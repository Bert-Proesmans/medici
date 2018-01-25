use proc_macro::{self, Diagnostic};
use proc_macro2::{Span, TokenStream, TokenNode};
use syn::{self, Data, DeriveInput};
use syn::spanned::Spanned;
use quote::ToTokens;

pub fn impl_derive_global(
    input: proc_macro::TokenStream,
) -> Result<proc_macro::TokenStream, Diagnostic> {
    let input: TokenStream = input.into();
    let call_site = Span::call_site();

    let subject: DeriveInput = syn::parse2(input).map_err(|e| {
        let msg = format!("Failed parsing subject: {:}", e);
        call_site.unstable().error(msg)
    })?;

    // This is the name of the thing we're implementing for
    let subj_name = subject.ident;
    let (impl_generics, ty_generics, where_clause) = subject.generics.split_for_impl();

    let _struct_data = match subject.data {
        Data::Struct(d) => d,
        _ => {
            let msg = "This macro is only applicable to structs";
            return Err(subj_name.span().unstable().error(msg));
        }
    };

    // let mut where_clause_cs = TokenStream::empty();
    // if let Some(wc) = where_clause {
    //     where_clause_cs = call_site_all(wc.into_tokens().into());
    // }

    // Build tokens in an isolated location, so the user CAN NOT
    // fool the system by providing it's own implementation of Actionable.
    let tokens = quote!{
        mod scoped {
            extern crate medici_traits;
            use self::medici_traits::prelude::*;

            impl #impl_generics Global for #subj_name #ty_generics #where_clause {
                // TODO add method implementations here
            }
        }
    };

    return Ok(tokens.into());
}

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
