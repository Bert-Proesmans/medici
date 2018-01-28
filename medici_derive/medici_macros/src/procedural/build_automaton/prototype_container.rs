use syn::{Ident, ImplItem};
use syn::synom::Synom;
use syn::token::Brace;

pub struct ProtoTypeContainer {
	pub ident: Ident,
	pub open_b: Brace,
	pub content: Vec<ImplItem>,
}

impl Synom for ProtoTypeContainer {
    named!(parse -> Self, do_parse!(
    	ident: syn!(Ident) >>
    	body: braces!(many0!(syn!(ImplItem))) >>
    	({
    		let (open_b, content) = body;
    		ProtoTypeContainer {
    			ident, open_b, content
    		}
    	})
    ));
}
