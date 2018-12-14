use super::FaultDetector;

pub struct TempFaultDetector {}

impl FaultDetector for TempFaultDetector {
    fn is_faulted(&self) -> bool {
        false
    }
}
