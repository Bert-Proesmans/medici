#![feature(proc_macro)]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro2::{TokenStream, Span};
use syn::DeriveInput;

#[proc_macro_derive(ActionState)]
pub fn derive_action(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    const MACRO_NAME: &str = "ActionState";
    const MACRO_IMPL: fn(&DeriveInput) -> TokenStream = impl_derive_action;

    // Parse the rust code into an ast.
    // This node will encode an ENUM or STRUCT
    let ast: DeriveInput = syn::parse2(input.into()).unwrap();

    if let syn::Data::Struct(_) = ast.data {
        // Build the derived implementation for Timing
        let gen = (MACRO_IMPL)(&ast);
        // Return the parsed ast
        return gen.into();
    } else {
        panic!(format!("#[derive({})] is only defined for structs, not enums!", MACRO_NAME));
    };
}

fn impl_derive_action(ast: &DeriveInput) -> TokenStream {
    // Defines the location where the macro is called
    let macro_call_site = Span::call_site();

    // Retrieves identifier of AST node
    let name = &ast.ident;
    let name_access = quote_spanned!{macro_call_site=> #name};
    // Build tokens in an isolated location, so the user CAN NOT
    // fool the system by providing it's own implementation of Actionable.
    let tokens = quote!{
        mod scoped {
            extern crate action_traits;
            use self::action_traits::Actionable;

            #[automatically_derived]
            impl Actionable for #name_access {
                // TODO add method implementations here
            }
        }
    };

    return tokens.into();
}


#[proc_macro_derive(TriggerState)]
pub fn derive_trigger(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    const MACRO_NAME: &str = "TriggerState";
    const MACRO_IMPL: fn(&DeriveInput) -> TokenStream = impl_derive_trigger;

    // Parse the rust code into an ast.
    // This node will encode an ENUM or STRUCT
    let ast: DeriveInput = syn::parse2(input.into()).unwrap();

    if let syn::Data::Struct(_) = ast.data {
        // Build the derived implementation for Timing
        let gen = (MACRO_IMPL)(&ast);
        // Return the parsed ast
        return gen.into();
    } else {
        panic!(format!("#[derive({})] is only defined for structs, not enums!", MACRO_NAME));
    };
}

fn impl_derive_trigger(ast: &DeriveInput) -> TokenStream {
    // Defines the location where the macro is called
    let macro_call_site = Span::call_site();

    // Retrieves identifier of AST node
    let name = &ast.ident;
    let name_access = quote_spanned!{macro_call_site=> #name};
    let tokens = quote!{
        mod scoped {
            extern crate action_traits;
            use self::action_traits::Triggerable;

            #[automatically_derived]
            impl Triggerable for #name_access {
                // TODO add method implementations here
            }
        }
    };

    return tokens.into();
}
