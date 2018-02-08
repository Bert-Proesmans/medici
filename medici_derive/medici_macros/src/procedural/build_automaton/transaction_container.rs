use proc_macro::Diagnostic;
use proc_macro2::Span;
use quote::{Tokens, ToTokens};
use syn::{Ident, ItemStruct, Item, ItemUse, Visibility};
use syn::synom::Synom;
use syn::token::Brace;
use syn::spanned::Spanned;

use heck::SnakeCase;

pub struct TransactionContainer {
	pub ident: Ident,
	pub open_b: Brace,
	pub contents: Vec<Item>,	
}

impl Synom for TransactionContainer {
    named!(parse -> Self, do_parse!(
    	ident: syn!(Ident) >>
    	body: braces!(do_parse!(
            contents: many0!(alt!(
                syn!(ItemUse) => { |a| a.into() }
                |
                syn!(ItemStruct) => { |a| a.into() }
            )) >>
            (contents)
        )) >>
    	({
    		let (open_b, contents) = body;
    		TransactionContainer {
                ident, open_b, contents,
    		}
    	})
    ));
}

impl TransactionContainer {

    pub fn validate(&self) -> Result<(), Diagnostic> {
        let transactions_ident_match = "transactions";
        if self.ident.as_ref() != transactions_ident_match {
            let msg = format!("Top level module containing transactions must be named `{:}`", transactions_ident_match);
            return Err(self.ident.span().unstable().error(msg));
        }

        Ok(())
    }

    pub fn build_output(self) -> Result<Tokens, Diagnostic> {
        let TransactionContainer {ident, contents, ..} = self;
        let call_site = Span::call_site();
        let def_site = Span::def_site();
        let pub_vis: Visibility = parse_quote!{pub};

        let mod_content = contents.into_iter().map(|mut c| {
            let mut tokens = Tokens::new();
            
            if let Item::Struct(ref mut s) = c {
                s.vis = pub_vis.clone();
                
                // Build an implementation of Transaction for this object
                let struct_ident = s.ident;
                let impl_site = struct_ident.span().resolved_at(def_site);
                let (impl_generics, ty_generics, where_clause) = s.generics.split_for_impl();
                let impl_tokens = quote_spanned!{impl_site=>
                    mod scoped {
                        extern crate medici_traits;
                        use self::medici_traits::automata::Transaction;

                        impl #impl_generics Transaction for #struct_ident #ty_generics 
                        #where_clause 
                        {}
                    }
                };

                impl_tokens.to_tokens(&mut tokens);
            }

            c.to_tokens(&mut tokens);
            tokens            
        });

        let top_mod_site = ident.span().resolved_at(call_site);
        let snek_top_mod_name = ident.as_ref().to_snake_case();
        let top_mod_name = Ident::new(&snek_top_mod_name, top_mod_site);
        let mod_tokens = quote_spanned!{top_mod_site=>
            pub mod #top_mod_name {
                #( #mod_content )*
            }
        };

        Ok(mod_tokens)
    }
}
