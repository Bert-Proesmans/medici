mod state_container;
mod state_parent_container;
mod transition_container;
mod transition_parent_container;

use quote::{self, Tokens, ToTokens};
use proc_macro::{self, Diagnostic};
use proc_macro2::{Span, TokenStream, TokenNode};
use syn::synom::Synom;
use syn::{self, Ident, Type, ItemStruct, Path, Item, ItemMod, Fields, ItemUse, Visibility};
use syn::spanned::Spanned;

use heck::SnakeCase;

use self::state_container::StateContainer;
use self::state_parent_container::StateParentContainer;
use self::transition_container::{TransitionContainer, TransitionEntry};
use self::transition_parent_container::TransitionParentContainer;

// Code ripped from 
// https://github.com/alexcrichton/weird-proc-macro-spans/blob/af3b0ac5a5376679f8a2017bed758884e6df4e8e/src/lib.rs#L21
fn call_site_all(t: TokenStream) -> TokenStream {
    t.into_iter().map(|mut tt| {
        tt.span = Span::call_site();
        tt.kind = match tt.kind {
            TokenNode::Group(d, ts) => TokenNode::Group(d, call_site_all(ts)),
            node => node,
        };
        tt
    }).collect()
}


struct Automaton {
    game_struct: ItemStruct,
    states: StateParentContainer,
    transitions: TransitionParentContainer,
}

impl Synom for Automaton {
    named!(parse -> Self, do_parse!(
        game_struct: syn!(ItemStruct) >>
        states: syn!(StateParentContainer) >>
        transitions: syn!(TransitionParentContainer) >>
        ({
            Automaton {
                game_struct,
                states,
                transitions,
            }
        })
    ));
}

pub fn impl_build_automaton(
    input: proc_macro::TokenStream,
) -> Result<proc_macro::TokenStream, Diagnostic> {
    let input: TokenStream = input.into();
    let call_site = Span::call_site();
    let def_site = Span::def_site();

    println!("DBG: {:}", input.clone().to_string());

    let subject: Automaton = syn::parse2(input).map_err(|e| {
        let msg = format!("Failed parsing macro contents: {:?}", e);
        call_site.unstable().error(msg)
    })?;

    // Deconstruct subject and build state machine.
    let Automaton {game_struct, states, transitions} = subject;

    validate_game_struct(&game_struct)?;
    validate_states(&states)?;
    validate_transitions(&transitions)?;
    
    let mut return_tokens = Tokens::new();
    // This method also performs transformations.
    let game_struct = write_game_struct(game_struct, &mut return_tokens)?;    
    
    let state_module = states.build_ast_modules()?; 
    state_module.to_tokens(&mut return_tokens);

    let transition_module = transitions.build_ast_module(&game_struct)?;    
    transition_module.to_tokens(&mut return_tokens);

    return Ok(return_tokens.into());
}

fn validate_game_struct(game_struct: &ItemStruct) -> Result<(), Diagnostic> {
    let mut field_iter = game_struct.fields.iter();
    let first_field = field_iter.nth(0).ok_or_else(|| {
        let msg = format!("State field is missing from game struct!");
        game_struct.span().unstable().error(msg)
    })?;

    let state_field_match_ident = "state";
    if let Some(ident) = first_field.ident {
        if ident.as_ref() != state_field_match_ident {
            let msg = format!("Expected first field to be named `{:}`", state_field_match_ident);
            return Err(first_field.span().unstable().error(msg));
        }
    } else {
        let msg = format!("Game struct must have named fields!");
        return Err(first_field.span().unstable().error(msg));
    }

    let state_match_path: Path = parse_quote!{X};
    if let Type::Path(ref p) = first_field.ty {
        if p.path != state_match_path {
            let msg = format!("First field's type must be equal to: {:}", state_match_path.into_tokens());
            return Err(p.span().unstable().error(msg));
        }
    } else {
        let msg = format!("Unexpected AST type for state field type");
        return Err(first_field.span().unstable().error(msg));
    }

    let generic_params = &game_struct.generics.params;
    if !generic_params.is_empty() {
        let params_iter = game_struct.generics.type_params();
        let type_str_match = stringify!{X};
        // Find generic parameter which exactly indicates 
        let mut filtered = params_iter.filter(|tp| tp.ident.as_ref() == type_str_match);
        if let None = filtered.next() {
            let msg = format!("Expected a generic parameter with identifier: {:}", type_str_match);
            return Err(game_struct.generics.span().unstable().error(msg));
        }
    } else {
        let msg = format!("The game structure must have at least ONE generic constraint for the state field type");
        return Err(game_struct.ident.span().unstable().error(msg));
    }        

    Ok(())
}

fn validate_states(states: &StateParentContainer) -> Result<(), Diagnostic> {
    let states_ident_match = "states";
    if states.ident.as_ref() != states_ident_match {
        let msg = format!("Top level module containing states must be named `{:}`", states_ident_match);
        return Err(states.ident.span().unstable().error(msg));
    }

    if states.content.len() < 1 {
        let msg = format!("States container must have contents");
        return Err(states.ident.span().unstable().error(msg));
    }

    let global_submod_ident_match = "global";
    let mut global_submod = states.content.iter().filter(|s_mod| {
        let snek_mod_ident = s_mod.ident.as_ref().to_snake_case();
        if &snek_mod_ident == global_submod_ident_match { true } else { false }
    });

    if let None = global_submod.next() {
        let msg = format!("The states module must have a `{:}` container defined", global_submod_ident_match);
        return Err(states.ident.span().unstable().error(msg));
    }

    Ok(())
}

fn validate_transitions(states: &TransitionParentContainer) -> Result<(), Diagnostic> {
    let intos = &states.into_transitions;
    let pushdowns = &states.pushdown_transitions;
    
    if intos.transitions.len() < 1 {
        let msg = format!("No Into transitions defined");
        return Err(intos.ident.span().unstable().error(msg));
    }

    let invalid_intos = intos.transitions.iter().any(|e| match *e {
        TransitionEntry::IntoTR(_) => false,
        _ => true,
    });

    if pushdowns.transitions.len() < 1 {
        let msg = format!("No Pushdown transitions defined");
        return Err(pushdowns.ident.span().unstable().error(msg));
    }

    let invalid_pushdowns = pushdowns.transitions.iter().any(|e| match *e {
        TransitionEntry::PushdownTR(_) => false,
        _ => true,
    });

    if invalid_intos {
        let msg = format!("Invalid Into transitions detected"); 
        return Err(intos.ident.span().unstable().error(msg));
    }

    if invalid_pushdowns {
        let msg = format!("Invalid Pushdown transitions detected"); 
        return Err(pushdowns.ident.span().unstable().error(msg));
    }

    Ok(())
}

fn write_game_struct(mut game_struct: ItemStruct, mut output: &mut Tokens) -> Result<ItemStruct, Diagnostic> {
    // Build the game struct into a new submodule and re-export it within it's parent
    // module.
    let call_site = Span::call_site();

    {
        let pub_vis: Visibility = parse_quote!{pub};
        // Force public visibility.
        game_struct.vis = pub_vis;

        let state_field_match_ident = "state";
        let mut state_field = match game_struct.fields {
            Fields::Named(ref mut fields) => fields.named.iter_mut().find(|f| {
                match f.ident {
                    Some(i) => i.as_ref() == state_field_match_ident,
                    None => false,
                }
            }),
            _ => unreachable!(),
        };

        // .. and update it's type
        match state_field {
            Some(ref mut f) => {
                // Build phantom type wrapper
                let x_type = f.ty.clone();
                let phantom_type_tokens = quote_spanned!{call_site=>
                    PhantomData<#x_type>
                };
                let phantom_type: Type = syn::parse2(phantom_type_tokens.into()).unwrap();
                // Replace original with phantom type
                f.ty = phantom_type;
            },
            _ => unreachable!(),
        };
    }

    // Push game struct onto tokenstream.
    // DO NOT implement the structure in some submodule because the implementations and transitions
    // MUST be able to access the fields. This is only possible if these tokens are defined in the same
    // module are within a submodule of the game structure!
    let sub_mod_site = game_struct.span().resolved_at(call_site);
    let mod_tokens = quote_spanned!{sub_mod_site=>        
        use std::marker::PhantomData;
        // Actual game struct implementation
        #game_struct
    };

    mod_tokens.to_tokens(&mut output);
    Ok(game_struct)
}
