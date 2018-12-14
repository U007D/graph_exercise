mod temp_fault_detector;
pub use self::temp_fault_detector::TempFaultDetector;

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
        write!(f, "<Event>")
    }
}
