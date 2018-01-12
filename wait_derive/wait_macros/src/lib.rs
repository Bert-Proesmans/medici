#![feature(proc_macro)]

extern crate proc_macro2;
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro2::{Span, TokenStream};
use syn::DeriveInput;

#[proc_macro_derive(WaitState)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    const MACRO_NAME: &str = "WaitState";

    // Parse the rust code into an ast.
    // This node will encode an ENUM or STRUCT
    let ast: DeriveInput = syn::parse2(input.into()).unwrap();

    if let syn::Data::Struct(_) = ast.data {
        let gen = impl_derive(&ast);
        // Return the parsed ast
        return gen.into();
    } else {
        panic!(format!(
            "#[derive({})] is only defined for structs, not enums!",
            MACRO_NAME
        ));
    };
}

fn impl_derive(ast: &DeriveInput) -> TokenStream {
    // Defines the location where the macro is called
    let macro_call_site = Span::call_site();

    // Retrieves identifier of AST node
    let name = &ast.ident;
    let name_access = quote_spanned!{macro_call_site=> #name};
    // Build tokens in an isolated location, so the user CAN NOT
    // fool the system by providing it's own implementation of relevant traits.
    let tokens = quote!{
        mod scoped {
            extern crate wait_traits;
            use self::wait_traits::Waitable;

            #[automatically_derived]
            impl Waitable for #name_access {
                // TODO add method implementations here
            }
        }
    };

    return tokens.into();
}
