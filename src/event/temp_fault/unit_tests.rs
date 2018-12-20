use crate::test_utils::*;
use super::*;

#[test]
fn event_does_not_trigger_when_bound_to_triggering_sensor() {
    // given a Event bound to a MockTemperatureSensor
    let event = TempSensorMock::new(ShouldGenerateFault::No);
    let fault = Fault::new(event);

    // when fault status is checked
    let result = fault.is_triggered();

    // then there should be no fault
    assert_eq!(result, false);

    // and the sensor should have been called once
    assert_eq!(fault.event.times_called(), 1);

}

#[test]
fn event_triggers_when_bound_to_a_triggering_sensor() {
    // given a Event bound to a MockTemperatureSensor
    let event = TempSensorMock::new(ShouldGenerateFault::Yes);
    let fault = Fault::new(event);

    // when fault status is checked
    let result = fault.is_triggered();

    // then there should be a fault
    assert_eq!(result, true);

    // and the sensor should have been called once
    assert_eq!(fault.event.times_called(), 1);

}
