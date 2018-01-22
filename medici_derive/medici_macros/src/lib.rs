#![feature(proc_macro)]
#![feature(plugin)]
#![plugin(interpolate_idents)]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

#[macro_use]
mod derived;
#[macro_use]
mod procedural;

macro_rules! derive_impl {
    (# [ $m_name:ident ] X => $m_func:path) => {
        interpolate_idents! {
        	#[proc_macro_derive($m_name)]
            #[allow(non_snake_case)]
    		pub fn [$m_name _action](input: proc_macro::TokenStream) -> proc_macro::TokenStream {
                let proc_name = stringify!($m_name);
                println!("[BUILD] Running proc (DERIVE) macro: {:}", proc_name);
                match $m_func(input) {
                    Ok(v) => v,
                    Err(e) => {
                        e.emit();
                        proc_macro::TokenStream::empty()
                    }
                }
    		}
        }
    }
}

macro_rules! proc_impl {
    ($m_name:ident !() => $m_func:path) => {
        interpolate_idents! {
            #[proc_macro]
            pub fn [$m_name](input: proc_macro::TokenStream) -> proc_macro::TokenStream {
                let proc_name = stringify!($m_name);
                println!("[BUILD] Running proc macro: {:}", proc_name);
                match $m_func(input) {
                    Ok(v) => v,
                    Err(e) => {
                        e.emit();
                        proc_macro::TokenStream::empty()
                    }
                }
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////

derive_impl!(#[ActionState] X => derived::impl_derive_action);

derive_impl!(#[TriggerState] X => derived::impl_derive_trigger);

derive_impl!(#[TimingState] X => derived::impl_derive_timing);

derive_impl!(#[WaitState] X => derived::impl_derive_wait);

////////////////////////////////////////////////////////////////////////

proc_impl!(build_automaton!() => procedural::impl_build_automaton);
