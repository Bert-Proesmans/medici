use proc_macro::{self, Diagnostic};
use proc_macro2::{Span, TokenStream};
use syn::{self, ItemStruct, Path};
use syn::synom::Synom;
use quote::{Tokens, ToTokens};

struct AttrArgs {
    game_struct: Path,
    transaction: Path,
}

impl Synom for AttrArgs {
    named!(parse -> Self, do_parse!(
        body: parens!(do_parse!(
            game_struct: syn!(Path) >>
            punct!(,) >>
            transaction: syn!(Path) >>
            (game_struct, transaction)
        )) >>
        ({
            let (_parens, (game_struct, transaction)) = body;
            AttrArgs {
                game_struct,
                transaction
            }
        })
    ));
}

pub fn impl_attr_state(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> Result<proc_macro::TokenStream, Diagnostic> {
    let args: TokenStream = args.into();
    let input: TokenStream = input.into();
    let call_site = Span::call_site();

    let AttrArgs {game_struct, transaction} = syn::parse2(args).map_err(|e| {
        let msg = format!("Failed parsing arguments: {:}", e);
        call_site.unstable().error(msg)
    })?;

    let subject: ItemStruct = syn::parse2(input).map_err(|e| {
        let msg = format!("Failed parsing subject: {:}", e);
        call_site.unstable().error(msg)
    })?;

    // This is the name of the thing we're implementing for
    let subj_name = subject.ident;
    let (impl_generics, ty_generics, where_clause) = subject.generics.split_for_impl();

    println!("Implementing trait 'State' for type {:}", subj_name.as_ref());

    // Build tokens in an isolated location, so the user CAN NOT
    // fool the system by providing it's own implementation of Actionable.
    let impl_tokens = quote!{
        mod scoped {
            extern crate medici_traits;
            use self::medici_traits::automata::State;

            // Implementation of State trait
            impl #impl_generics State for #subj_name #ty_generics #where_clause {
                type Transaction = #transaction;
            }

            // Linking of actual state with ""container state""
            impl #impl_generics State for #game_struct<#subj_name #ty_generics> #where_clause {
                type Transaction = <#subj_name #ty_generics as State>::Transaction;
            }            
        }
    };

    let mut result_tokens = Tokens::new();
    subject.to_tokens(&mut result_tokens);
    impl_tokens.to_tokens(&mut result_tokens);

    return Ok(result_tokens.into());
}
