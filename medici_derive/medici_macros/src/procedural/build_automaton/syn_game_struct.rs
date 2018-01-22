use syn::{Field, Ident, FieldsNamed};
use syn::synom::{Synom, ParseError};
use syn::buffer::TokenBuffer;

pub struct GameStruct {
    pub ident: Ident,
    pub state_field: Field,
    pub other_fields: Vec<Field>,
}

impl Synom for GameStruct {
    named!(parse -> Self, do_parse!(
        ident: syn!(Ident) >>
        fields: syn!(FieldsNamed) >>
        fields: value!({
            let mut field_iter = fields.named.into_iter();
            let state_field = match field_iter.next() {
                Some(f) => f,
                _ => return Err(ParseError::new("No state_field present!")),
            };
            let other_fields: Vec<_> = field_iter.collect();
            (state_field, other_fields)
        }) >>
        cond_reduce!({
                // We match the first field against a hardcoded one.
                // This has to be done manually since Field doesn't implement
                // Synom.
                let ref state_field = fields.0;
                let field_invariant_tokens = quote!{state: X};
                let buff = TokenBuffer::new(field_invariant_tokens.into());
                match Field::parse_named(buff.begin())
                        .map(|(f, _)| state_field == &f) {
                            Ok(b) => b,
                            _ => false,
                }             
        }) >>
        ({
            let (state_field, other_fields) = fields;
            GameStruct {
                ident,
                state_field,
                other_fields,
            }
        })
    ));
}
