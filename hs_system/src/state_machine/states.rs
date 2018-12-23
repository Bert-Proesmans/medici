use medici_core::behaviours::{functions, markers};

#[derive(Clone, Copy)]
pub struct Empty;
impl markers::Transaction for Empty {}

pub struct Start;

impl functions::State for Start {
    type Transaction = Empty;
}

impl markers::TopLevelState for Start {}

pub struct Stop;

impl functions::State for Stop {
    type Transaction = Empty;
}

impl markers::TopLevelState for Stop {}
