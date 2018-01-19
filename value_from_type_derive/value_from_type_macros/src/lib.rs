#![feature(proc_macro)]
#![feature(conservative_impl_trait)]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::Diagnostic;
use proc_macro2::{Span, TokenStream};
use syn::{Ident, Item, ItemMod, ItemStruct, ItemEnum, ItemImpl};
use syn::synom::Synom;
use syn::spanned::Spanned;
use quote::ToTokens;

struct AttrArgs {
    enum_name: Ident,
}

impl Synom for AttrArgs {
    named!(parse -> Self, do_parse!(
		all: parens!(
			do_parse!(
				enum_name: syn!(Ident) >>
				({
					AttrArgs { enum_name }
				})
		)) >>
		({
			let (_parens, args) = all;
			args
		})
	));
}

#[proc_macro_attribute]
pub fn value_from_type(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	println!("Running proc macro: value_from_type");

    match value_from_type_impl(args, input) {
    	Ok(v) => v,
    	Err(e) => {
    		e.emit();
    		panic!("See emitted errors");
    	}
    }
}

fn value_from_type_impl(args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> Result<proc_macro::TokenStream, Diagnostic> {
    let input: TokenStream = input.into();
    let call_site = Span::call_site();
    let _def_site = Span::def_site();

    // println!("ARG TOKENS: {:?}", args.to_string());
    
    let args: AttrArgs = syn::parse2(args.into())
    	.map_err(|e| {
    		let msg = format!("Failed parsing arguments: {:}", e);
    		call_site.unstable().error(msg)
    	})?;

    // Parse module code
    // IMPORTANT: Our own macro attribute is automatically stripped!
	let mut module_def: ItemMod = syn::parse2(input.into())
		.map_err(|e| {
			let msg = format!("You have syntax errors in your module: {:}", e);
			call_site.unstable().error(msg)
		})?;

    let all_structs: Vec<_> = locate_structs(&module_def).map(|s| s.clone()).collect();
    if all_structs.len() < 1 {
    	let msg = "You have no structs defined in your module";
    	return Err(module_def.span().unstable().error(msg));
    }

    let enum_site = args.enum_name.span().resolved_at(call_site);
    let enum_access = Ident::new(args.enum_name.as_ref(), enum_site);
    let struct_idents: Vec<_> = all_structs.iter().map(|s| s.ident).collect();
    let enum_variant_idents: Vec<_> = all_structs.iter().map(|s| Ident::new(s.ident.as_ref(), enum_site)).collect();

    // Prepare submodule which imports the necessary types so we can push implementation details into it.
	let lower_enum_name = String::from(args.enum_name.as_ref()).to_lowercase();
	let impl_mod_name = format!("_impl_{:}", lower_enum_name);
	let impl_mod_access = Ident::from(impl_mod_name);
	let mut impl_module: ItemMod = parse_quote!{
		mod #impl_mod_access {
			// self will resolve to the local module
			extern crate value_from_type_traits;
			use self::value_from_type_traits::FromType;
			use super::#enum_access;
		}
	}; 

    { // Build enum from structs
		let variants = enum_variant_idents.iter();
	    let fab_enum: ItemEnum = parse_quote!{
	    	#[derive(Debug, Clone, PartialEq)]
	    	pub enum #enum_access {
	    		#( #variants ),*
	    	}
	    };

	    push_into_module(&mut module_def, fab_enum.into())?;
	}

	{ // Build conversion implementations
	    for (struct_access, enum_variant_access) in struct_idents.into_iter().zip(enum_variant_idents) {
	    	let fab_impl: ItemImpl = parse_quote!{
	    		impl FromType<super::#struct_access> for #enum_access {
	    			fn from_type() -> Self {
	    				#enum_access::#enum_variant_access
	    			}
	    		}
	    	};
	    	// Note: Push this into the IMPLEMENTATION MODULE!
	    	push_into_module(&mut impl_module, fab_impl.into())?;
	    }
	}

	// Push implementations module into original module.
	push_into_module(&mut module_def, impl_module.into())?;
    
    let module_tokens = module_def.into_tokens();
    return Ok(module_tokens.into());
}

fn locate_structs(data: &ItemMod) -> impl Iterator<Item = &ItemStruct> {
    return data.content
        .iter()
        .flat_map(|content| content.1.iter())
        .flat_map(|item| {
            if let &Item::Struct(ref s) = item {
                Some(s)
            } else {
                None
            }
        });
}

fn push_into_module(module: &mut ItemMod, i: Item) -> Result<(), Diagnostic> {
	let mod_span = module.span();
	match module.content.as_mut() {
		Some(&mut (_, ref mut c)) => {
			c.push(i); Ok(())
		},
		None => {
			let msg = "This module doesn't have any contents!";
			Err(mod_span.unstable().error(msg))
		},
	}
}
