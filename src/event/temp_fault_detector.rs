use super::Event;

pub struct TempFaultDetector {}

impl Event for TempFaultDetector {
    fn is_triggered(&self) -> bool {
        false
    }
}
