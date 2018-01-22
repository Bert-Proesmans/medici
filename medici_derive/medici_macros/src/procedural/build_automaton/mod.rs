mod syn_game_struct;
mod syn_state_obj;
mod syn_global_states;
mod syn_custom_states;


use proc_macro::{self, Diagnostic};
use proc_macro2::{Span, TokenStream};
use syn::synom::Synom;
use syn;
// use syn::spanned::Spanned;
// use syn::buffer::TokenBuffer;


use self::syn_game_struct::GameStruct;
use self::syn_global_states::GlobalStates;

struct Automaton {
    game_struct: GameStruct,
    global_states: Option<GlobalStates>,
    // custom_states: Option<CustomStates>,
    // transitions: Option<Transitions>,
}

impl Synom for Automaton {
    named!(parse -> Self, do_parse!(
        game_struct: syn!(GameStruct) >>
        global_states: option!(syn!(GlobalStates)) >>
        // custom_states: option!(syn!(CustomStates)) >>
        // transitions: option!(syn!(Transitions)) >>
        ({
            Automaton {
                game_struct,
                global_states,
                // custom_states,
                // transitions,
            }
        })
    ));
}

pub fn impl_build_automaton(
    input: proc_macro::TokenStream,
) -> Result<proc_macro::TokenStream, Diagnostic> {
    let input: TokenStream = input.into();
    let call_site = Span::call_site();

    println!("DBG: {:}", input.clone().to_string());

    let subject: Automaton = syn::parse2(input).map_err(|e| {
        let msg = format!("Failed parsing macro contents: {:?}", e);
        call_site.unstable().error(msg)
    })?;

    let game_struct = &subject.game_struct;
    println!("Game struct identifier: {:}", &game_struct.ident);

    return Ok(proc_macro::TokenStream::empty());
}
