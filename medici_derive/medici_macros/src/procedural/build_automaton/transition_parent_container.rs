use proc_macro::Diagnostic;
use proc_macro2::Span;

use quote::{Tokens, ToTokens};
use syn::{self, Ident, ItemMod, ItemStruct};
use syn::synom::Synom;
use syn::token::Brace;
use syn::spanned::Spanned;

use heck::SnakeCase;

use super::{TransitionContainer, TransitionEntry};

pub struct TransitionParentContainer {
	pub ident: Ident,
	pub open_b: Brace,
	pub into_transitions: TransitionContainer,
    pub pushdown_transitions: TransitionContainer,
}

impl Synom for TransitionParentContainer {
    named!(parse -> Self, do_parse!(
    	ident: syn!(Ident) >>
    	body: braces!(do_parse!(
    		into: syn!(TransitionContainer) >>
    		pushdown: syn!(TransitionContainer) >>
    		(into, pushdown)
    	)) >>
    	({
    		let (open_b, (into_transitions, pushdown_transitions)) = body;
    		TransitionParentContainer {
    			ident, 
    			open_b,
    			into_transitions,
    			pushdown_transitions,
    		}
    	})
    ));
}

impl TransitionParentContainer {
    pub fn validate(&self) -> Result<(), Diagnostic> {
        let intos = &self.into_transitions;
        let pushdowns = &self.pushdown_transitions;
        
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

    pub fn build_ast_module(self, game_struct: &ItemStruct) -> Result<ItemMod, Diagnostic> {
        let TransitionParentContainer{ ident, into_transitions, pushdown_transitions, ..} = self;
        let call_site = Span::call_site();
        
        // All imports for transition code
        let site_imports = quote_spanned!{call_site=>            
            use std::marker::PhantomData;
            use std::convert::From;

            use medici_traits::automata::State;
            use medici_traits::automata::deterministic_automaton::TransitionFrom;
            use medici_traits::automata::pushdown_automaton::{PushdownFrom, PullupFrom};
        };

        let collector = [into_transitions, pushdown_transitions];
        let sub_modules = collector.into_iter().map(|t| {
            let snek_sub_mod_name = t.ident.as_ref().to_snake_case();
            let sub_mod_name = Ident::new(&snek_sub_mod_name, t.ident.span());
            let sub_mod_site = t.ident.span().resolved_at(call_site);
            let sub_mod_impls = t.transitions.iter().map(|e| {
                Self::build_impl(e, game_struct)
            });

            let sub_mod_tokens = quote_spanned!{sub_mod_site=>
                #[allow(unused_imports)]
                mod #sub_mod_name {
                    // Load external types
                    #site_imports

                    // Load states from parent module
                    use super::*;

                    #( #sub_mod_impls )*
                }
            };
            sub_mod_tokens
        });

        let top_mod_site = ident.span().resolved_at(call_site);
        let snek_top_mod_name = ident.as_ref().to_snake_case();
        let top_mod_name = Ident::new(&snek_top_mod_name, ident.span());
        let mod_tokens = quote_spanned!{top_mod_site=>
            mod #top_mod_name {
                // Load states from sibling module states
                use super::states::*;

                #( #sub_modules )*
            }
        };

        let transition_module: ItemMod = syn::parse2(mod_tokens.into()).map_err(|e| {
            let msg = format!("Issue converting transitions into module layout: {:}", e);
            ident.span().unstable().error(msg)
        })?;
        Ok(transition_module)        
    }

    fn build_impl(data: &TransitionEntry, game_struct: &ItemStruct) -> Tokens {
        let call_site = Span::call_site();

        let game_ident = game_struct.ident;
        let game_fields = game_struct.fields.iter();
        let field_access = game_fields.filter_map(|f| {
            let ref_ident = f.ident.as_ref().unwrap();
            // TODO; Filter on transaction as well!
            // See below for part two of this TODO.
            if ref_ident == "state" {
                return None
            } else {
                Some((ref_ident, ref_ident))
            }
        });
        let (field_left, field_right): (Vec<&Ident>, Vec<&Ident>) = field_access.unzip();

        match *data {
            TransitionEntry::PushdownTR(ref e) => {
                let left = &e.left;
                let right = &e.right;
                let transition_site = left.span().resolved_at(call_site);

                let field_left_2 = field_left.clone();
                let field_right_2 = field_right.clone();
                
                let mut tokens = Tokens::new();
                let x = quote_spanned!{transition_site=>
                    impl PushdownFrom<#game_ident<#left>> for #game_ident<#right> {
                        fn pushdown_from(x: #game_ident<#left>) -> Self {
                            Self {
                                state: PhantomData,
                                #( #field_left: x.#field_right ),*
                            }
                        }
                    }
                };
                x.to_tokens(&mut tokens);

                let x = quote_spanned!{transition_site=>
                    impl PullupFrom<#game_ident<#right>> for #game_ident<#left> {
                        fn pullup_from(x: #game_ident<#right>) -> Self {
                            Self {
                                state: PhantomData,
                                #( #field_left_2: x.#field_right_2 ),*
                            }
                        }
                    }
                };
                x.to_tokens(&mut tokens);

                tokens
            },
            TransitionEntry::IntoTR(ref e) => {
                let left = &e.left;
                let right = &e.right;
                let transition_site = left.span().resolved_at(call_site);

                // TODO; Remove this hack when the above TODO is resolved
                let field_left = field_left.into_iter().filter(|i| i.as_ref() != "transaction");
                let field_right = field_right.into_iter().filter(|i| i.as_ref() != "transaction");
                
                quote_spanned!{transition_site=>
                    impl TransitionFrom<#game_ident<#left>> for #game_ident<#right> {
                        fn transition_from(x: #game_ident<#left>, transaction: <Self as State>::Transaction) -> Self {
                            Self {
                                state: PhantomData,
                                transaction: transaction,
                                #( #field_left: x.#field_right ),*
                            }
                        }
                    }
                }
            },
        }
    }
}


