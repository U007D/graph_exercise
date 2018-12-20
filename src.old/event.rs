pub mod fault;
pub mod fault_level;

use std::fmt::{
    Debug,
    Formatter,
    Result as FmtResult,
};

pub trait Event {
    fn is_triggered(&self) -> bool;
}

impl Debug for Event {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        // TODO(anyone): Write event dump function
        write!(f, "<Event>")
    }
}
