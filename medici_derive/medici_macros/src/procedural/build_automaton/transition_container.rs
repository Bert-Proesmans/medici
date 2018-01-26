use syn::{Ident, ExprPath};
use syn::punctuated::Punctuated;
use syn::synom::Synom;
use syn::token::Brace;

pub struct TransitionContainer {
	pub ident: Ident,
	pub open_b: Brace,
	pub transitions: Vec<TransitionEntry>,
}

impl Synom for TransitionContainer {
    named!(parse -> Self, do_parse!(
    	ident: syn!(Ident) >>
    	body: braces!(alt!(
    		call!(Punctuated::<PushdownTR, Token![,]>::parse_terminated_nonempty) => {
    			|i| i.into_iter().map(|e| e.into()).collect()
    		}
    		|
    		call!(Punctuated::<IntoTR, Token![,]>::parse_terminated_nonempty) => {
    			|i| i.into_iter().map(|e| e.into()).collect()
    		}
		)) >>
    	({
    		let (open_b, transitions) = body;
    		TransitionContainer {
    			ident,
    			open_b,
    			transitions,
    		}
    	})
    ));
}

pub enum TransitionEntry {
	PushdownTR(PushdownTR),
	// SingleLeft,
	IntoTR(IntoTR),
}

impl From<PushdownTR> for TransitionEntry {
    fn from(x: PushdownTR) -> Self {
    	TransitionEntry::PushdownTR(x)
    }
}

impl From<IntoTR> for TransitionEntry {
    fn from(x: IntoTR) -> Self {
    	TransitionEntry::IntoTR(x)
    }
}

pub struct PushdownTR {
	pub left: ExprPath,
	pub right: ExprPath,
}

impl Synom for PushdownTR {
    named!(parse -> Self, do_parse!(
    	left: syn!(ExprPath) >>
    	// Note: Always try to parse as much as possible!
    	punct!(<-) >> punct!(>) >>
    	right: syn!(ExprPath) >>
    	({
    		PushdownTR {
    			left,
    			right,
    		}
    	})
    ));
}

pub struct IntoTR {
	pub left: ExprPath,
	pub right: ExprPath,
}

impl Synom for IntoTR {
    named!(parse -> Self, do_parse!(
    	left: syn!(ExprPath) >>
    	// Note: Always try to parse as much as possible!
    	punct!(->) >>
    	right: syn!(ExprPath) >>
    	({
    		IntoTR {
    			left,
    			right,
    		}
    	})
    ));
}


