#![feature(proc_macro)]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::Diagnostic;
use proc_macro2::{Span, TokenStream};
use syn::{Ident, Path};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::buffer::TokenBuffer;
use quote::ToTokens;

// The proc macro name is chosen so it would overwrite the macro behind feature gate
// '#![feature(concat_idents)]'.
// All macos share the same namespace so either you don't enable the feature or 
// this macro will overwrite the feature-gated macro when explicitly imported!
#[proc_macro]
pub fn cc_idents(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	match concat_idents_impl(input) {
    	Ok(v) => v,
    	Err(e) => {
    		e.emit();
    		// panic!("See emitted errors");
    		proc_macro::TokenStream::empty()
    	}
    }
}

// Path is used because it allows the widest range of allowed characters.
// This part is not intended to already filter characters which would (possibly) NOT be legal in
// an identifier.
type ArgsType = Punctuated<Ident, Token![,]>;

fn manual_parse_args(input: TokenStream) -> Result<ArgsType, Diagnostic> {
	let buffer = TokenBuffer::new2(input);
	let cursor = buffer.begin();

	let (args, rest_cursor) = ArgsType::parse_terminated(cursor)
		.map_err(|e| {
			let msg = format!("Failed parsing arguments: {:}", e);
			cursor.span().unstable().error(msg)
		})?;
	
	let cursor = rest_cursor;
	if !cursor.eof() {
		let msg = "Expected arguments to end, but there are still elements present";
		return Err(cursor.span().unstable().error(msg));
	}

	Ok(args)
}

fn concat_idents_impl(input: proc_macro::TokenStream) -> Result<proc_macro::TokenStream, Diagnostic> {
	let input: TokenStream = input.into();
	let call_site = Span::call_site();
	
	let args = manual_parse_args(input)?;
    if args.is_empty() {
    	let msg = "You must pass at least 1 argument";
    	return Err(call_site.unstable().error(msg));
    }

    let mut ident_str = String::new();
    // Test each part for having valid identifier characters.
    // If all parts contain only valid characters, the concatenation will only
    // contain valid characters.
    for p in args.iter() {
    	let mut path_tokens = quote::Tokens::new();
    	p.to_tokens(&mut path_tokens);
    	let path_str = path_tokens.to_string();
    	let ident_check = format!("_{:}_a", &path_str);
    	println!("IDENT CHECK: {:}", &ident_check);
    	// Here we validate the characters inside each arg.
    	if let Err(e) = syn::parse_str::<Ident>(&ident_check) {
    		let msg = format!("This part contains illegal characters for an identifier: {:}", e);
    		return Err(p.span().unstable().error(msg));
    	} else {
    		ident_str.push_str(&path_str);
    	}
    };

    let result_ident = Ident::new(&ident_str, Span::call_site());
    let result_tokens = result_ident.into_tokens();
    return Ok(result_tokens.into());
}
