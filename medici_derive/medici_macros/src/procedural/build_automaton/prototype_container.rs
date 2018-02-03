use syn::Ident;
use syn::synom::Synom;

pub struct ProtoTypeContainer {
	pub ident: Ident,
}

impl Synom for ProtoTypeContainer {
    named!(parse -> Self, do_parse!(
    	ident: syn!(Ident) >>
    	({
    		ProtoTypeContainer {
    			ident, 
    		}
    	})
    ));
}
