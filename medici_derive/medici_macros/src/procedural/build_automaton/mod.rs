mod state_container;
mod state_parent_container;
mod transition_container;
mod transition_parent_container;
mod prototype_container;
mod prototype_parent_container;
mod transaction_container;

use quote::{Tokens, ToTokens};
use proc_macro::{self, Diagnostic};
use proc_macro2::{Span, TokenStream};
use syn::synom::Synom;
use syn::{self, Type, ItemStruct, Path, Fields, Visibility};
use syn::spanned::Spanned;

// use self::state_container::StateContainer;
use self::state_parent_container::StateParentContainer;
use self::transition_container::{TransitionContainer, TransitionEntry};
use self::transition_parent_container::TransitionParentContainer;
// use self::prototype_container::ProtoTypeContainer;
use self::prototype_parent_container::ProtoTypeParentContainer;
use self::transaction_container::TransactionContainer;


struct Automaton {
    game_struct: ItemStruct,
    entity_struct: ItemStruct,
    card_struct: ItemStruct,
    states: StateParentContainer,
    transactions: TransactionContainer,
    transitions: TransitionParentContainer,
    prototypes: ProtoTypeParentContainer,
}

impl Synom for Automaton {
    named!(parse -> Self, do_parse!(
        game_struct: syn!(ItemStruct) >>
        entity_struct: syn!(ItemStruct) >>
        card_struct: syn!(ItemStruct) >>
        states: syn!(StateParentContainer) >>
        transactions: syn!(TransactionContainer) >>
        transitions: syn!(TransitionParentContainer) >>
        prototypes: syn!(ProtoTypeParentContainer) >>
        ({
            Automaton {
                game_struct,
                entity_struct,
                card_struct,
                states,
                transactions,
                transitions,
                prototypes,
            }
        })
    ));
}

pub fn impl_build_automaton(
    input: proc_macro::TokenStream,
) -> Result<proc_macro::TokenStream, Diagnostic> {
    let input: TokenStream = input.into();
    let call_site = Span::call_site();

    // println!("DBG: {:}", input.clone().to_string());

    let subject: Automaton = syn::parse2(input).map_err(|e| {
        let msg = format!("Failed parsing macro contents: {:?}", e);
        call_site.unstable().error(msg)
    })?;

    // Deconstruct subject and build state machine.
    let Automaton {
        game_struct, mut entity_struct, mut card_struct,
        states, transitions, prototypes, transactions,
    } = subject;

    validate_game_struct(&game_struct)?;
    // No validation for the entity_structure
    states.validate()?;
    transitions.validate()?;
    transactions.validate()?;
    
    let mut return_tokens = Tokens::new();
    // Note: This method also performs transformations on the game structure!
    let game_struct = write_game_struct(game_struct, &mut return_tokens)?;

    let pub_vis: Visibility = parse_quote!{pub};
    entity_struct.vis = pub_vis.clone();
    entity_struct.to_tokens(&mut return_tokens); 

    card_struct.vis = pub_vis.clone();
    card_struct.to_tokens(&mut return_tokens);   
    
    let state_module = states.build_output()?; 
    state_module.to_tokens(&mut return_tokens);

    let transaction_module = transactions.build_output()?;
    transaction_module.to_tokens(&mut return_tokens);

    let transition_module = transitions.build_ast_module(&game_struct)?;    
    transition_module.to_tokens(&mut return_tokens);

    let prototype_module = prototypes.build_output(&entity_struct)?;
    prototype_module.to_tokens(&mut return_tokens);

    return Ok(return_tokens.into());
}

fn validate_game_struct(game_struct: &ItemStruct) -> Result<(), Diagnostic> {
    let mut field_iter = game_struct.fields.iter();
    
    /* Test struct field: STATE */
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

    /* Test struct field: TRANSACTION */
    // Note; The iterator is mutated internally, so the 0-th element after getting the 
    // 0-the element above will result in the 1st field.
    // This basically is the same as calling iterator::next() twice!
    let second_field = field_iter.nth(0).ok_or_else(|| {
        let msg = format!("Transaction field is missing from game struct!");
        game_struct.span().unstable().error(msg)
    })?;

    let transaction_field_match_ident = "transaction";
    if let Some(ident) = second_field.ident {
        if ident.as_ref() != transaction_field_match_ident {
            let msg = format!("Expected second field to be named `{:}`", transaction_field_match_ident);
            return Err(second_field.span().unstable().error(msg));
        }
    } 
    // We already tested for named/unnamed through the first field.
    // else {
    //     let msg = format!("Game struct must have named fields!");
    //     return Err(second_field.span().unstable().error(msg));
    // }

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
