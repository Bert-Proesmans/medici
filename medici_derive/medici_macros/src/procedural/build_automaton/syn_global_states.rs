use syn::{self, Ident};
use syn::synom::{Synom, ParseError};
use syn::punctuated::Punctuated;
use std::default::Default;

use super::syn_state_obj::StateObject;

pub struct GlobalStates {
	pub group_ident: Ident,
	pub wait_state: StateObject,
	pub action_state: StateObject,
	pub finished_state: StateObject,
	pub effect_state: StateObject,
	pub trigger_state: StateObject,

	pub other_states: Vec<StateObject>,
}

impl Synom for GlobalStates {
    named!(parse -> Self, do_parse!(
    	group_ident: syn!(Ident) >>
        cond_reduce!(group_ident.as_ref() == "global_states") >>
    	mut body: braces!(do_parse!(
    		structs: call!(Punctuated::<StateObject, Token![,]>::parse_terminated_nonempty) >>
    		structs: value!(structs.into_iter()) >>
    		(structs)
    	)) >>
    	wait_state: value!(body.1.next()) >>
    	wait_state: cond_reduce!(wait_state.is_some(), value!(wait_state.unwrap())) >>
    	
    	action_state: value!(body.1.next()) >>
    	action_state: cond_reduce!(action_state.is_some(), value!(action_state.unwrap())) >>
    	
    	finished_state: value!(body.1.next()) >>
    	finished_state: cond_reduce!(finished_state.is_some(), value!(finished_state.unwrap())) >>

    	effect_state: value!(body.1.next()) >>
    	effect_state: cond_reduce!(effect_state.is_some(), value!(effect_state.unwrap())) >>

    	trigger_state: value!(body.1.next()) >>
		trigger_state: cond_reduce!(trigger_state.is_some(), value!(trigger_state.unwrap())) >>    	

    	other_states: value!(body.1.collect()) >>
    	({
    		GlobalStates {
    			group_ident,
    			wait_state,
    			action_state,
    			finished_state,
    			effect_state,
    			trigger_state,
    			other_states,
    		}
    	})
    ));
}	

impl Default for GlobalStates {
    fn default() -> Self {
    	let tokens = quote!{
    		global_states {
                #[derive(Debug, GlobalState)] Wait<W: Waitable>(W),
                #[derive(Debug, GlobalState)] Action<T: Timing, U: Actionable>(T, U),
                #[derive(Debug, GlobalState)] Finished(),
                #[derive(Debug, GlobalState)] Effect<T: Timing, U: Triggerable>(T, U),
                #[derive(Debug, GlobalState)] Trigger<T: Timing, U: Triggerable>(T, U),
			}
    	};
    	syn::parse::<GlobalStates>(tokens.into()).unwrap()
    }
}
