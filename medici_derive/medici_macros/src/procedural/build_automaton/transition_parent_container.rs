
use syn::Ident;
use syn::synom::Synom;
use syn::token::Brace;

use super::TransitionContainer;

pub struct TransitionParentContainer {
	pub ident: Ident,
	pub open_b: Brace,
	pub into_transitions: TransitionContainer,
    pub pushdown_transitions: TransitionContainer,
}

impl Synom for TransitionParentContainer {
    named!(parse -> Self, do_parse!(
    	ident: syn!(Ident) >>
    	body: braces!(do_parse!(
    		into: syn!(TransitionContainer) >>
    		pushdown: syn!(TransitionContainer) >>
    		(into, pushdown)
    	)) >>
    	({
    		let (open_b, (into_transitions, pushdown_transitions)) = body;
    		TransitionParentContainer {
    			ident, 
    			open_b,
    			into_transitions,
    			pushdown_transitions,
    		}
    	})
    ));
}


