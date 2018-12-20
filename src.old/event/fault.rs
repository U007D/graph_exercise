mod temp_fault;
#[cfg(test)]
mod unit_tests;
use crate::{
    Event,
    FaultLevel,
};
pub use self::temp_fault::TempFault;

pub trait Fault {
    fn fault_level(&self) -> FaultLevel;
}

impl<T: Fault> Event for T {
    fn is_triggered(&self) -> bool {
        self.fault_level() != FaultLevel::None
    }
}
