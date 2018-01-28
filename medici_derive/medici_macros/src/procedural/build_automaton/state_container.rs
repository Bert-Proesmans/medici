use syn::{Ident, ItemStruct, Item, ItemUse, Attribute};
use syn::synom::Synom;
use syn::token::Brace;

pub struct StateContainer {
    pub attrs: Vec<Attribute>,
	pub ident: Ident,
	pub open_b: Brace,
	pub contents: Vec<Item>,	
}

impl Synom for StateContainer {
    named!(parse -> Self, do_parse!(
    	ident: syn!(Ident) >>
    	body: braces!(do_parse!(
            attrs: many0!(call!(Attribute::parse_inner)) >>
            contents: many0!(alt!(
                syn!(ItemUse) => { |a| a.into() }
                |
                syn!(ItemStruct) => { |a| a.into() }
            )) >>
            (attrs, contents)
        )) >>
    	({
    		let (open_b, (attrs, contents)) = body;
    		StateContainer {
                attrs, ident, open_b, contents,
    		}
    	})
    ));
}
