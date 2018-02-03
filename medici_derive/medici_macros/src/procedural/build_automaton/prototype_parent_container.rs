use proc_macro::Diagnostic;
use proc_macro2::Span;
use quote::{Tokens, ToTokens};
use syn::{Ident, Attribute, ItemStruct};
use syn::synom::Synom;
use syn::token::Brace;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

use heck::SnakeCase;

use super::prototype_container::ProtoTypeContainer;

pub struct ProtoTypeParentContainer {
    pub attrs: Vec<Attribute>,
	pub ident: Ident,
	pub open_b: Brace,
	pub contents: Vec<ProtoTypeContainer>,
}

impl Synom for ProtoTypeParentContainer {
    named!(parse -> Self, do_parse!(
    	ident: syn!(Ident) >>
    	body: braces!(do_parse!(
            attrs: many0!(call!(Attribute::parse_inner)) >>
            contents: call!(Punctuated::<ProtoTypeContainer, Token![,]>::parse_terminated) >>
            (attrs, contents)
        )) >>
    	({
    		let (open_b, (attrs, contents)) = body;
            let contents = contents.into_iter().collect();
    		ProtoTypeParentContainer {
    			attrs, ident, open_b, contents,
    		}
    	})
    ));
}

impl ProtoTypeParentContainer {
    pub fn build_output(self, entity_struct: &ItemStruct) -> Result<Tokens, Diagnostic> {
        let ProtoTypeParentContainer {attrs, ident, contents, ..} = self;
        let call_site = Span::call_site();
        let def_site = Span::def_site();
        let entity_struct_name = entity_struct.ident;

        let proto_iter = contents.into_iter().map(|p| {
            let ProtoTypeContainer {
                ident: proto_ident, ..
            } = p;

            let proto_mut = format!("{:}Mut", proto_ident.as_ref());
            let proto_ident_mut = Ident::new(&proto_mut, proto_ident.span());

            let proto_site = proto_ident.span().resolved_at(call_site);
            let mut def_tokens = quote_spanned!{proto_site=>
                // TODO; limit visibility of first variable
                #[derive(Debug)]
                pub struct #proto_ident<'a>(pub &'a Entity);
                #[derive(Debug)]
                pub struct #proto_ident_mut<'a>(pub &'a mut Entity);

                // Transition methods for the defined entity structure
                impl<'a> From<&'a #entity_struct_name> for #proto_ident<'a> {
                    fn from(e: &'a #entity_struct_name) -> Self {
                        #proto_ident(e)
                    }
                }

                impl<'a> From<&'a mut #entity_struct_name> for #proto_ident_mut<'a> {
                    fn from(e: &'a mut #entity_struct_name) -> Self {
                        #proto_ident_mut(e)
                    }
                }
            };

            let proto_site = proto_ident.span().resolved_at(def_site);
            let impl_tokens = quote_spanned!{proto_site=>
                extern crate medici_traits;
                use medici_traits::entities::EntityPrototype;

                impl<'a> EntityPrototype for #proto_ident<'a> {}
                impl<'a> EntityPrototype for #proto_ident_mut<'a> {}
            };

            impl_tokens.to_tokens(&mut def_tokens);
            def_tokens
        });

        let top_mod_site = ident.span().resolved_at(call_site);
        let snek_top_mod_name = ident.as_ref().to_snake_case();
        let top_mod_name = Ident::new(&snek_top_mod_name, top_mod_site);
        let mod_tokens = quote_spanned!{top_mod_site=>
            pub mod #top_mod_name {
                // We only process inner attributes
                #( #attrs )*

                // We use the declared entity struct. This MUST be defined
                // within the parent module!
                use automaton::prelude::Entity;

                #( #proto_iter )*
            }
        };

        // let proto_module: ItemMod = syn::parse2(mod_tokens.into()).map_err(|e| {
        //     let msg = format!("Issue converting protos into module layout: {:}", e);
        //     ident.span().unstable().error(msg)
        // })?;
        Ok(mod_tokens)
    }
}
