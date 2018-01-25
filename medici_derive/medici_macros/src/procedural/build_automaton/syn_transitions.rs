use syn::{Field, Ident, FieldsNamed};
use syn::synom::{Synom, ParseError};

use super::StateReference;

pub struct Transitions {
	pub group_ident: Ident,
	pub into_transitions: TransitionGroup,
	pub pushdown_transitions: TransitionGroup,
}

impl Synom for Transitions {
    named!(parse -> Self, do_parse!(
    	group_ident: syn!(Ident) >>
    	body: braces!(do_parse!(
    		into_transitions: call!(TransitionGroup::parse_single_right) >>
            // cond_reduce!(&into_transitions.group_ident == "into_transitions") >>
    		pushdown_transitions: call!(TransitionGroup::parse_reflexive) >>
            // cond_reduce!(&pushdown_transitions.group_ident == "pushdown_transitions") >>
    		(into_transitions, pushdown_transitions)
    	)) >>
    	({
            let (_braces, (into_transitions, pushdown_transitions)) = body;
            Transitions {
                group_ident,
                into_transitions,
                pushdown_transitions,
            }
    	})
    ));
}


pub struct TransitionGroup {
	pub group_ident: Ident,
	pub transitions: Vec<TransitionEntry>,
}

impl TransitionGroup {
    named!(parse_single_right -> Self, do_parse!(
        group_ident: syn!(Ident) >>
        body: braces!(do_parse!(
            transitions: many0!(call!(TransitionEntry::parse_single_right)) >>
            (transitions)
        )) >>
        ({
            let (_braces, transitions) = body;
            TransitionGroup {
                group_ident,
                transitions,
            }
        })
    ));

    named!(parse_reflexive -> Self, do_parse!(
        group_ident: syn!(Ident) >>
        body: braces!(do_parse!(
            transitions: many0!(call!(TransitionEntry::parse_reflexive)) >>
            (transitions)
        )) >>
        ({
            let (_braces, transitions) = body;
            TransitionGroup {
                group_ident,
                transitions,
            }
        })
    ));
}

pub struct TransitionEntry  {
	pub left: StateReference,
	pub lt: Option<Token![<]>,
	pub hp: Token![-],
	pub gt: Option<Token![>]>,
	pub right: StateReference,
    pub comma: Option<Token![,]>,
}

impl TransitionEntry {
    named!(parse_single_right -> Self, do_parse!(
        left: syn!(StateReference) >>

        lt: value!(None) >>
        hp: punct!(-) >>
        gt: punct!(>) >>
        gt: value!(Some(gt)) >>

        right: syn!(StateReference) >>
        comma: option!(punct!(,)) >>
        ({
            TransitionEntry {
                left,
                lt,
                hp,
                gt,
                right,
                comma,
            }
        })
    ));

    named!(parse_reflexive -> Self, do_parse!(
        left: syn!(StateReference) >>
        lt: punct!(<) >>
        lt: value!(Some(lt)) >>
        hp: punct!(-) >>
        gt: punct!(>) >>
        gt: value!(Some(gt)) >>
        right: syn!(StateReference) >>
        comma: option!(punct!(,)) >>
        ({
            TransitionEntry {
                left,
                lt,
                hp,
                gt,
                right,
                comma,
            }
        })
    ));
}
