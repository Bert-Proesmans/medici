use syn::{Ident, Generics, Fields, FieldsUnnamed, Attribute};
use syn::synom::{Synom, ParseError};


pub struct StateObject {
    pub attrs: Vec<Attribute>,
	pub ident: Ident,
	pub generics: Option<Generics>,
	pub fields: FieldsUnnamed,
}

impl Synom for StateObject {
    named!(parse -> Self, do_parse!(
        attrs: many0!(call!(Attribute::parse_outer)) >>
    	ident: syn!(Ident) >>
    	generics: option!(syn!(Generics)) >>
    	// FieldsUnnamed also consumes the parens!
    	fields: syn!(FieldsUnnamed) >>
    	({
    		StateObject {
                attrs,
    			ident,
    			generics,
    			fields,
    		}
    	})
    ));
}
