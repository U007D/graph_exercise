use std::sync::atomic::{
    AtomicUsize,
    Ordering,
};
use super::*;

enum GenerateFault {
    No,
    Yes,
}

struct MockTempSensor {
    should_fault: bool,
    times_called: AtomicUsize,
}

impl MockTempSensor {
    fn new(should_fault: bool) -> Self {
        Self {
            should_fault,
            times_called: AtomicUsize::new(0),
        }
    }

    fn times_called(&self) -> usize {
        self.times_called.load(Ordering::Relaxed)
    }
}

impl FaultDetector for MockTempSensor {
    fn is_faulted(&self) -> bool {
        self.times_called.fetch_add(1, Ordering::Relaxed);
        self.should_fault
    }
}

#[test]
fn fault_detector_does_not_trigger_when_bound_to_non_faulting_sensor() {
    // given a FaultDetector bound to a MockTemperatureSensor
    let mock = MockTempSensor::new(false);
    let detector = TempFaultDetector::new(mock);

    // when fault status is checked
    let result = detector.is_faulted();

    // then there should be no fault
    assert_eq!(result, false);

    // and the sensor should have been called once
    assert_eq!(mock.times_called(), 1);

}
