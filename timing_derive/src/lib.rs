extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(Timing)]
pub fn timing(input: TokenStream) -> TokenStream {
    // Parse the rust code into an ast.
    // This node will encode an ENUM or STRUCT
    let ast: DeriveInput = match syn::parse(input) {
        Result::Ok(val) => val,
        Result::Err(err) => panic!("Failed to parse Rust code: {:?}", err),
    };

    if let syn::Data::Struct(_) = ast.data {
        // Build the derived implementation for Timing
        let gen = impl_timing(&ast);
        // Return the parsed ast
        return gen.into();
    } else {
        panic!("#[derive(Timing)] is only defined for structs, not enums!");
    };
}

fn impl_timing(ast: &DeriveInput) -> quote::Tokens {
    // Retrieves identifier of AST node
    let name = &ast.ident;
    quote!{
        impl Timing for #name {
            // TODO add method implementations here
        }
    }
}
