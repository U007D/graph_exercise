use std::fmt::{
    Debug,
    Formatter,
    Result as FmtResult,
};

pub trait FaultDetector {
    fn is_faulted(&self) -> bool;
}

impl Debug for FaultDetector {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "<FaultDetector>")
    }
}
