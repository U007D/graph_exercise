mod temp_fault_emitter;
#[cfg(test)]
mod unit_tests;
use crate::FaultLevel;
pub use self::temp_fault_emitter::TempFaultEmitter;
use super::EventEmitter;

pub trait FaultEmitter {
    fn fault_level(&self) -> FaultLevel;
}

impl <T: FaultEmitter> EventEmitter for T {
    fn is_triggered(&self) -> bool {
        self.fault_level() != FaultLevel::None
    }
}
