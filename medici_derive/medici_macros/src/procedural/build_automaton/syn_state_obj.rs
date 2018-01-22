use syn::{Ident, Generics, Fields, FieldsUnnamed};
use syn::synom::{Synom, ParseError};


pub struct StateObject {
	pub ident: Ident,
	pub generics: Option<Generics>,
	pub fields: FieldsUnnamed,
}

impl Synom for StateObject {
    named!(parse -> Self, do_parse!(
    	ident: syn!(Ident) >>
    	generics: option!(syn!(Generics)) >>
    	// FieldsUnnamed also consumes the parens!
    	fields: syn!(FieldsUnnamed) >>
    	({
    		StateObject {
    			ident,
    			generics,
    			fields,
    		}
    	})
    ));
}
