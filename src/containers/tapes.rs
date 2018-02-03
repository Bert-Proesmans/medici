use std::default::Default;

use automaton::prelude::*;

#[derive(Debug)]
pub struct TapeService {
    // This contains all global data which can be accessed by all states.
}

// DBG
impl Default for TapeService {
    fn default() -> Self {
    	TapeService {}
    }
}

impl TapeService {
    pub fn new(c: &SetupConfig) -> Result<Self, ()> {
        Ok(Self {})
    }
}
