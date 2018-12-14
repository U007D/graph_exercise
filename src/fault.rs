mod fault_detector;
mod temp_fault_detector;
pub use self::{
    fault_detector::FaultDetector,
    temp_fault_detector::TempFaultDetector,
};
