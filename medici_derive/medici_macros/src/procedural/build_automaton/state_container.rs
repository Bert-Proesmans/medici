use syn::{Ident, ItemStruct, Item, ItemUse};
use syn::synom::Synom;
use syn::token::Brace;

pub struct StateContainer {
	pub ident: Ident,
	pub open_b: Brace,
	pub content: Vec<Item>,	
}

impl Synom for StateContainer {
    named!(parse -> Self, do_parse!(
    	ident: syn!(Ident) >>
    	body: braces!(many0!(alt!(
            syn!(ItemUse) => { |a| a.into() }
            |
            syn!(ItemStruct) => { |a| a.into() }
        ))) >>
    	({
    		let (open_b, content) = body;
    		StateContainer {
    			ident,
    			open_b,
    			content,
    		}
    	})
    ));
}
