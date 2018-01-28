use proc_macro::Diagnostic;
use proc_macro2::Span;
use quote::Tokens;
use syn::{Ident, Item, Visibility};
use syn::synom::Synom;
use syn::token::Brace;
use syn::spanned::Spanned;

use heck::SnakeCase;

use super::state_container::StateContainer;

pub struct StateParentContainer {
	pub ident: Ident,
	pub open_b: Brace,
	pub content: Vec<StateContainer>,
}

impl Synom for StateParentContainer {
    named!(parse -> Self, do_parse!(
    	ident: syn!(Ident) >>
    	body: braces!(many0!(syn!(StateContainer))) >>
    	({
    		let (open_b, content) = body;
    		StateParentContainer {
    			ident,
    			open_b,
    			content,
    		}
    	})
    ));
}

impl StateParentContainer {
    pub fn validate(&self) -> Result<(), Diagnostic> {
        let states_ident_match = "states";
        if self.ident.as_ref() != states_ident_match {
            let msg = format!("Top level module containing states must be named `{:}`", states_ident_match);
            return Err(self.ident.span().unstable().error(msg));
        }

        if self.content.len() < 1 {
            let msg = format!("States container must have contents");
            return Err(self.ident.span().unstable().error(msg));
        }

        let global_submod_ident_match = "global";
        let mut global_submod = self.content.iter().filter(|s_mod| {
            let snek_mod_ident = s_mod.ident.as_ref().to_snake_case();
            if &snek_mod_ident == global_submod_ident_match { true } else { false }
        });

        if let None = global_submod.next() {
            let msg = format!("The states module must have a `{:}` container defined", global_submod_ident_match);
            return Err(self.ident.span().unstable().error(msg));
        }

        Ok(())
    }

    pub fn build_output(self) -> Result<Tokens, Diagnostic> {
    	let StateParentContainer {ident, content, ..} = self;
    	let call_site = Span::call_site();
    	let pub_vis: Visibility = parse_quote!{pub};

    	let sub_modules_iter = content.into_iter().map(|c| {
            let StateContainer {
                attrs, ident: sub_ident, contents, ..
            } = c;

    		let snek_sub_mod_name = sub_ident.as_ref().to_snake_case();
    		let sub_mod_name = Ident::new(&snek_sub_mod_name, sub_ident.span());
    		let sub_mod_site = sub_ident.span().resolved_at(call_site);
    		let sub_mod_items = contents.into_iter().map(|mut s| {
    			match s {
    			    Item::Use(ref mut u) => u.vis = pub_vis.clone(),
    			    Item::Struct(ref mut s) => s.vis = pub_vis.clone(),
    			    _ => {
    			    	// let msg = format!("Unexpected content");
    			    	// return Err(s.span().unstable().error(msg));
    			    }
    			}
    			s
    		});

    		let sub_mod_tokens = quote_spanned!{sub_mod_site=>
    			pub mod #sub_mod_name {
                    // We only parse inner attributes
                    #( #attrs )*

    				#( #sub_mod_items )*
    			}
    		};

            let sub_use_tokens = quote_spanned!{sub_mod_site=>
                // States must all be exported into top level state module because 
                // the transition module must be able to access them
                pub use self::#sub_mod_name::*;
            };
    		(sub_mod_tokens, sub_use_tokens)
    	});

        // Consumes and splits the entire iterator.
        let (sub_modules, sub_use_stmts): (Vec<_>, Vec<_>) = sub_modules_iter.unzip();

        let top_mod_site = ident.span().resolved_at(call_site);
    	let snek_top_mod_name = ident.as_ref().to_snake_case();
    	let top_mod_name = Ident::new(&snek_top_mod_name, top_mod_site);
    	let mod_tokens = quote_spanned!{top_mod_site=>
    		pub mod #top_mod_name {
                #( #sub_use_stmts )*

    			#( #sub_modules )*
    		}
    	};

    	// let state_module: ItemMod = syn::parse2(mod_tokens.into()).map_err(|e| {
    	// 	let msg = format!("Issue converting states into module layout: {:}", e);
    	// 	ident.span().unstable().error(msg)
    	// })?;
    	Ok(mod_tokens)
    }
}
