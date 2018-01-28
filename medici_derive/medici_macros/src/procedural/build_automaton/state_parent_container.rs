
use proc_macro::{self, Diagnostic};
use proc_macro2::Span;

use syn::{self, Ident, ItemStruct, ItemMod, Visibility, Item};
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
    pub fn build_ast_modules(self) -> Result<ItemMod, Diagnostic> {
    	let StateParentContainer {ident, content, ..} = self;
    	let call_site = Span::call_site();
    	let pub_vis: Visibility = parse_quote!{pub};

    	let sub_modules_iter = content.into_iter().map(|c| {
    		let snek_sub_mod_name = c.ident.as_ref().to_snake_case();
    		let sub_mod_name = Ident::new(&snek_sub_mod_name, c.ident.span());
    		let sub_mod_site = c.ident.span().resolved_at(call_site);
    		let sub_mod_items = c.content.into_iter().map(|mut s| {
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
    				#( #sub_mod_items )*
    			}
    		};

            let sub_use_tokens = quote_spanned!{sub_mod_site=>
                pub use self::#sub_mod_name::*;
            };
    		(sub_mod_tokens, sub_use_tokens)
    	});

        // Consumes and splits the entire iterator.
        let (sub_modules, sub_use_stmts): (Vec<_>, Vec<_>) = sub_modules_iter.unzip();

    	let snek_top_mod_name = ident.as_ref().to_snake_case();
    	let top_mod_name = Ident::new(&snek_top_mod_name, ident.span());
    	let mod_tokens = quote_spanned!{call_site=>
    		pub mod #top_mod_name {
                #( #sub_use_stmts )*

    			#( #sub_modules )*
    		}
    	};

    	let state_module: ItemMod = syn::parse2(mod_tokens.into()).map_err(|e| {
    		let msg = format!("Issue converting states into module layout: {:}", e);
    		ident.span().unstable().error(msg)
    	})?;
    	Ok(state_module)
    }
}
