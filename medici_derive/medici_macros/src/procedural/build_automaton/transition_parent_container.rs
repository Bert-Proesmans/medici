use proc_macro::Diagnostic;
use proc_macro2::Span;

use quote::{Tokens, ToTokens};
use syn::{self, Ident, ItemMod, ItemStruct, Visibility, ItemImpl};
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
    pub fn build_ast_module(self, game_struct: &ItemStruct) -> Result<ItemMod, Diagnostic> {
        let TransitionParentContainer{ ident, into_transitions, pushdown_transitions, ..} = self;
        let call_site = Span::call_site();
        let def_site = Span::def_site();

        // All imports for transition code
        let site_imports = quote_spanned!{call_site=>            
            use std::marker::PhantomData;
            use std::convert::From;

            use medici_traits::automata;
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
                    impl automata::PushdownFrom<#game_ident<#left>> for #game_ident<#right> {
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
                    impl automata::PullupFrom<#game_ident<#right>> for #game_ident<#left> {
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
                quote_spanned!{transition_site=>
                    impl From<#game_ident<#left>> for #game_ident<#right> {
                        fn from(x: #game_ident<#left>) -> Self {
                            Self {
                                state: PhantomData,
                                #( #field_left: x.#field_right ),*
                            }
                        }
                    }
                }
            },
            _ => unreachable!(),
        }
    }
}


