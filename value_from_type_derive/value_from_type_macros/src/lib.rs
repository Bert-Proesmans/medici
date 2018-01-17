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
use syn::{Ident, Item, ItemMod, ItemStruct, ItemEnum, ItemImpl, ItemExternCrate, Visibility};
use syn::synom::Synom;
use syn::spanned::Spanned;

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
    let def_site = Span::def_site();

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
    	let msg = "You have no structs defined in your module";
    	return Err(module_def.span().unstable().error(msg));
    }

    let enum_name = Ident::from(args.enum_name.as_ref());
    let struct_names: Vec<_> = all_structs.iter().map(|s| s.ident).collect();
    let enum_variant_names: Vec<_> = all_structs.iter().map(|s| Ident::from(s.ident.as_ref())).collect();

    { // Build enum from structs
		let variants = enum_variant_names.iter();
	    let fab_enum: ItemEnum = parse_quote!{
	    	#[derive(Debug, Clone, PartialEq)]
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
    
    // Make sure module is public!  
    let public_vis: Visibility = parse_quote!{pub};
    module_def.vis = public_vis;
    let output_span = module_def.span().resolved_at(def_site);
    let module_tokens = quote_spanned!{output_span=>
    	#module_def
    };
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
