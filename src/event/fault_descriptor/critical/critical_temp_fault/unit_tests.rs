use std::sync::atomic::{
    AtomicUsize,
    Ordering,
};
use super::*;

#[derive(Debug, Copy, Clone)]
enum ShouldGenerateFault {
    No,
    Yes,
}

impl ShouldGenerateFault {
    fn as_bool(&self) -> bool {
        match self {
            ShouldGenerateFault::Yes => true,
            ShouldGenerateFault::No => false,
        }
    }
}

struct MockTempSensor {
    should_fault: ShouldGenerateFault,
    times_called: AtomicUsize,
}

impl MockTempSensor {
    fn new(should_fault: ShouldGenerateFault) -> Self {
        Self {
            should_fault,
            times_called: AtomicUsize::new(0),
        }
    }

    fn times_called(&self) -> usize {
        self.times_called.load(Ordering::Relaxed)
    }
}

impl Event for MockTempSensor {
    fn is_triggered(&self) -> bool {
        self.times_called.fetch_add(1, Ordering::Relaxed);
        self.should_fault.as_bool()
    }
}

#[test]
fn event_does_not_trigger_when_bound_to_triggering_sensor() {
    // given a Event bound to a MockTemperatureSensor
    let mock = MockTempSensor::new(ShouldGenerateFault::No);
    let event = CriticalTempFault::new(mock);

    // when fault status is checked
    let result = event.is_triggered();

    // then there should be no fault
    assert_eq!(result, false);

    // and the sensor should have been called once
    assert_eq!(event.event.times_called(), 1);

}

#[test]
fn event_triggers_when_bound_to_a_triggering_sensor() {
    // given a Event bound to a MockTemperatureSensor
    let mock = MockTempSensor::new(ShouldGenerateFault::Yes);
    let event = CriticalTempFault::new(mock);

    // when fault status is checked
    let result = event.is_triggered();

    // then there should be a fault
    assert_eq!(result, true);

    // and the sensor should have been called once
    assert_eq!(event.event.times_called(), 1);

}
