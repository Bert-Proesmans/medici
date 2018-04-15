//! Module pending removal

use medici_core::build_exec_triggers_checked;

use state_machine::prelude::*;
use state_machine::state::prelude::*;

// Invoking this macro must be accompanied with importing all used types!
build_exec_triggers_checked!(Machine);
