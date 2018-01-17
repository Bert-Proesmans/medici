use proc_macro2::{Span, TokenStream};
use syn::DeriveInput;


pub fn impl_derive_action(ast: &DeriveInput) -> TokenStream {
    // Defines the location where the macro is called
    let macro_call_site = Span::call_site();

    // Retrieves identifier of AST node
    let name = &ast.ident;
    let name_access = quote_spanned!{macro_call_site=> #name};
    // Build tokens in an isolated location, so the user CAN NOT
    // fool the system by providing it's own implementation of Actionable.
    let tokens = quote!{
        mod scoped {
            extern crate medici_traits;
            use self::medici_traits::action_traits::Actionable;

            impl Actionable for #name_access {
                // TODO add method implementations here
            }
        }
    };

    return tokens.into();
}
