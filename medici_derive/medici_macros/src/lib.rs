#![feature(proc_macro)]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

#[macro_use]
mod derived;
#[macro_use]
mod procedural;

use proc_macro2::{Span, TokenStream};
use syn::DeriveInput;

macro_rules! derive_impl {
    (# [ $m_name:ident ] struct X => $m_func:path) => {
    	#[proc_macro_derive($m_name)]

    	// TODO build name for method
		pub fn derive_action(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
		    const MACRO_NAME: &str = stringify!($m_name);

		    let ast: DeriveInput = syn::parse2(input.into()).unwrap();
		    if let syn::Data::Struct(_) = ast.data {
		        // Build the derived implementation for Timing
		        return $m_func(&ast).into();
		    } else {
		        panic!(format!("#[derive({})] is only defined for structs, not enums!", MACRO_NAME));
		    };
		}
    }
}

////////////////////////////////////////////////////////////////////////

// derive_impl!(#[ActionState] struct X => derived::action_state::impl_derive_action);

/*#[proc_macro_derive(ActionState)]
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
}*/

////////////////////////////////////////////////////////////////////////

// derive_impl!(#[TriggerState] struct X => derived::trigger_state::impl_derive_trigger);

/*#[proc_macro_derive(TriggerState)]
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
}*/
