mod syn_game_struct;
mod syn_state_obj;
mod syn_global_states;
mod syn_custom_states;
mod syn_transitions;

use quote;
use proc_macro::{self, Diagnostic};
use proc_macro2::{Span, TokenStream, TokenNode};
use syn::synom::Synom;
use syn::{self, Ident};
use syn::spanned::Spanned;


use self::syn_game_struct::GameStruct;
use self::syn_global_states::GlobalStates;
use self::syn_custom_states::CustomStates;
use self::syn_transitions::Transitions;

use syn::Path as StateReference;

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
    game_struct: GameStruct,
    global_states: Option<GlobalStates>,
    custom_states: Option<CustomStates>,
    transitions: Option<Transitions>,
}

impl Synom for Automaton {
    named!(parse -> Self, do_parse!(
        game_struct: syn!(GameStruct) >>
        global_states: option!(syn!(GlobalStates)) >>
        custom_states: option!(syn!(CustomStates)) >>
        transitions: option!(syn!(Transitions)) >>
        ({
            Automaton {
                game_struct,
                global_states,
                custom_states,
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

    let game_struct = &subject.game_struct;
    println!("Game struct identifier: {:}", &game_struct.ident);

    let mut result_tokens = quote_spanned!{def_site=>
        extern crate medici_traits;
    };

    let game_struct_tokens = build_game_struct(game_struct)?;
    result_tokens.append_all(game_struct_tokens);

    let state_tokens = build_states(&subject.global_states, &subject.custom_states)?;
    result_tokens.append_all(state_tokens);

    return Ok(result_tokens.into());
}

fn build_game_struct(data: &GameStruct) -> Result<quote::Tokens, Diagnostic> {
    let call_site = Span::call_site();
    let def_site = Span::def_site();

    let g_ident = data.ident;

    let state_ident = data.state_field.ident.unwrap();

    let other_fields: Vec<_> = data.other_fields
            .iter()
            .cloned()
            .map(|f| {
                // TODO; Possibly do something to their span.
                f
            }).collect();

    // let constraints = data.other_fields
    //         .iter()
    //         .map(|f| {
    //             // Possibly add constraints to types contained by Game struct.
    //             let test_site = def_site.located_at(f.ty.span());
    //             quote_spanned!{test_site=>
    //                 // TODO
    //                 #f
    //             }
    //         });

    let tokens = quote_spanned!{def_site=>       
        pub struct #g_ident<X: medici_traits::prelude::Global> {
            #state_ident: X,

            #( #other_fields ),*
        }

        // #( #constraints )*
    };
    Ok(tokens)
}

fn build_states(global: &Option<GlobalStates>, custom: &Option<CustomStates>) 
    -> Result<quote::Tokens, Diagnostic> 
{
    let call_site = Span::call_site();
    let def_site = Span::def_site();

    let global_states = match *global {
        Some(ref s) => build_global_states(s)?,
        None => build_global_states(&GlobalStates::default())?,
    };

    let ns_ident = Ident::new("states", call_site);
    let tokens = quote_spanned!{def_site=> 
        pub mod #ns_ident {
            #global_states

            // #custom_states
        }
    };
    Ok(tokens)
}

fn build_global_states(data: &GlobalStates) -> Result<quote::Tokens, Diagnostic> {
    let call_site = Span::call_site();
    let def_site = Span::def_site();

    let ref wait_state = data.wait_state;
    let ref action_state = data.action_state;
    let ref finished_state = data.finished_state;
    let ref effect_state = data.effect_state;
    let ref trigger_state = data.trigger_state;

    let ref other_states = data.other_states;

    // TODO; Constrain each state

    // Collect all states and process them into tokens
    let implementations = [
        wait_state, action_state, finished_state, effect_state, trigger_state
    ];
    // Need cloned here because [T; n] does NOT implement IntoIter.
    // The type gets converted to &[T; n], which does 
    let implementations = implementations.into_iter().cloned().chain(other_states.iter());

    let implementations = implementations
            .map(|s| {
                let attrs = &s.attrs;
                let s_ident = s.ident;
                let fields = &s.fields;

                if let Some(ref g) = s.generics {
                    let (_impl_generics, ty_generics, where_clause) = g.split_for_impl();

                    let tokens = quote_spanned!{def_site=>
                        #( #attrs )*
                        pub struct #s_ident #ty_generics #where_clause
                        #fields;
                    };

                    println!("DBG - BUILDING: {:}", tokens.clone().to_string());
                    // Manually override all token spans into call_site.
                    tokens
                } else {
                    let tokens = quote_spanned!{def_site=>
                        #( #attrs )*
                        pub struct #s_ident
                        #fields;
                    };

                    println!("DBG - BUILDING: {:}", tokens.clone().to_string());
                    tokens
                }                
            });

    let ns_ident = Ident::new("global", call_site);
    let tokens = quote_spanned!{def_site=>
        pub mod #ns_ident {
            #( #implementations )*
        }
    };
    Ok(tokens)
}
