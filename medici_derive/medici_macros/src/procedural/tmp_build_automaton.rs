use proc_macro::{self, Diagnostic};
use proc_macro2::{Span, TokenStream};
use syn::{self, Data, DeriveInput, Field, Ident, Type};
use syn::spanned::Spanned;
use syn::synom::Synom;
use syn::buffer::TokenBuffer;





struct GlobalStates {
    group_ident: Ident,
}

impl Synom for GlobalStates {
    named!(parse -> Self, do_parse!(
        group_ident: syn!(Ident) >>
        body: braces!(do_parse!(
            many0!()
        )) >>
        ({
            GlobalStates {
                group_ident
            }
        })
    ));
}

struct CustomStates {
    group_ident: Ident,
}

impl Synom for CustomStates {
    named!(parse -> Self, do_parse!(
        (CustomStates{})
    ));
}

struct Transitions {
    group_ident: Ident,
}

impl Synom for Transitions {
    named!(parse -> Self, do_parse!(
        (Transitions{})
    ));
}



