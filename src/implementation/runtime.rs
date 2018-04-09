//! Module pending removal

use medici_core::build_exec_triggers;

use state_machine::prelude::*;
use state_machine::state::prelude::*;
use state_machine::transaction::*;

// Invoking this macro must be accompanied with importing all used types!
build_exec_triggers!(Machine);
