use syn::{Field, Ident, FieldsNamed};
use syn::synom::{Synom, ParseError};

use super::syn_state_obj::StateObject;

pub struct CustomStates {
	pub group_ident: Ident,
	pub states: Vec<StateObjContainer>,
}

impl Synom for CustomStates {
    named!(parse -> Self, do_parse!(
    	group_ident: syn!(Ident) >>
    	body: braces!(many0!(
    		syn!(StateObjContainer)
    	)) >>
    	({
    		let (_braces, states) = body;
    		CustomStates {
    			group_ident,
    			states
    		}
    	})
    ));
}

pub struct StateObjContainer {
	pub trait_name: Ident,
	pub states: Vec<StateObject>,
}

impl Synom for StateObjContainer {
    named!(parse -> Self, do_parse!(
    	trait_name: syn!(Ident) >>
    	body: braces!(many0!(
    		syn!(StateObject)
    	)) >>
    	({
    		let (_braces, states) = body;
    		StateObjContainer {
    			trait_name,
    			states,
    		}
    	})
    ));
}
