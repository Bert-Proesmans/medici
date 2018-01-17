#![feature(proc_macro)]
#![feature(conservative_impl_trait)]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro2::{Span, TokenStream, TokenTree};
use syn::{Ident, Item, ItemMod, ItemStruct, ItemEnum, ItemImpl, ItemExternCrate};
use syn::synom::Synom;
use syn::spanned::Spanned;
use quote::ToTokens;

const PROC_ATTR_NAME: &str = "value_from_type";

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
    match value_from_type_impl(args, input) {
    	Ok(v) => v,
    	Err(e) => panic!("{:}", e)
    }
}

fn value_from_type_impl(args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> Result<proc_macro::TokenStream, &'static str> {
    let input: TokenStream = input.into();
    println!("ARG TOKENS: {:?}", args.to_string());
    let args: AttrArgs = match syn::parse2(args.into()) {
        Ok(v) => v,
        Err(e) => panic!("{:?} - Failed parsing arguments: {:?}", PROC_ATTR_NAME, e),
    };

    // Parse module code
    // IMPORTANT: Our own macro attribute is automatically stripped!
	let mut module_def: ItemMod = match syn::parse2(input.into()) {
		Ok(v) => v,
		Err(e) => panic!(
		    "{:?} - You have syntax errors in your module: {:?}",
		    PROC_ATTR_NAME, e
		),
	};

	{ // Push usage of value_from_type_traits
		let fab_import: ItemExternCrate = parse_quote!{
			extern crate value_from_type_traits;
		}; 
		// use self::value_from_type_traits::FromType;
		// self will resolve to the local module
		push_into_module(&mut module_def, fab_import.into())?;
	}
	

    let all_structs: Vec<_> = locate_structs(&module_def).map(|s| s.clone()).collect();
    if all_structs.len() < 1 {
    	return Err("You have no structs defined in your module");
    }

    let enum_name = Ident::from(args.enum_name.as_ref());
    let struct_names: Vec<_> = all_structs.iter().map(|s| s.ident).collect();
    let enum_variant_names: Vec<_> = all_structs.iter().map(|s| Ident::from(s.ident.as_ref())).collect();

    { // Build enum from structs
		let variants = enum_variant_names.iter();
	    let fab_enum: ItemEnum = parse_quote!{
	    	pub enum #enum_name {
	    		#( #variants ),*
	    	}
	    };

	    push_into_module(&mut module_def, fab_enum.into())?;
	}

	{ // Build conversion implementations
	    for (struct_name, enum_variant) in struct_names.iter().zip(enum_variant_names.iter()) {
	    	let fab_impl: ItemImpl = parse_quote!{
	    		impl self::value_from_type_traits::FromType<#struct_name> for #enum_name {
	    			fn from_type() -> Self {
	    				#enum_name::#enum_variant
	    			}
	    		}
	    	};
	    	push_into_module(&mut module_def, fab_impl.into())?;
	    }
	}
    
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

fn push_into_module(module: &mut ItemMod, i: Item) -> Result<(), &'static str> {
	match module.content.as_mut() {
		Some(&mut (_, ref mut c)) => c.push(i),
		None => {},
	};
	Ok(())
}
