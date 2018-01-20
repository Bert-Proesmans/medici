use proc_macro::{self, Diagnostic};
use proc_macro2::{Span, TokenStream};
use syn::{self, Data, DeriveInput};
use syn::spanned::Spanned;

pub fn impl_derive_action(
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

    let _struct_data = match subject.data {
        Data::Struct(d) => d,
        _ => {
            let msg = "This macro is only applicable to structs";
            return Err(subj_name.span().unstable().error(msg));
        }
    };

    // Build tokens in an isolated location, so the user CAN NOT
    // fool the system by providing it's own implementation of Actionable.
    let tokens = quote!{
        mod scoped {
            extern crate medici_traits;
            use self::medici_traits::prelude::*;

            impl Actionable for #subj_name {
                // TODO add method implementations here
            }

            impl Triggerable for #subj_name {
                // TODO add method implementations here   
            }
        }
    };

    return Ok(tokens.into());
}
