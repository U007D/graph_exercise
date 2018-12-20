use crate::{
    Error,
    FaultEmitter,
    Temp,
};
use crossbeam_channel::{
    TryRecvError,
    bounded,
};
use std::{
    thread::{
        sleep,
        spawn,
    },
    time::Duration,
};
use super::*;

#[test]
pub fn new_should_report_error_when_no_initial_temp_msg_is_available() {
    // given a TempFaultEmitter with no Temp data in the channel
    let thresholds = TempEmitterThresholds::new(Temp(0.0), Temp(1.0), Temp(2.0)).unwrap();
    let (_s, r) = bounded(0);

    // when it is constructed
    let result = TempFaultEmitter::new(r, thresholds);

    // then it should report an error
    assert_eq!(result.err().unwrap(), Error::TryRecvError(TryRecvError::Empty));
}

#[test]
pub fn fault_level_should_report_none_when_temp_state_is_below_warn_threshold() {
    // given an event with defined trigger thresholds
    let thresholds = TempEmitterThresholds::new(Temp(50.0), Temp(75.0), Temp(100.0)).unwrap();
    let (s, r) = bounded(0);
    spawn(move || s.send(Temp(25.0)).unwrap());
    // avoid race for zero-capacity channel
    sleep(Duration::from_millis(10));
    let emitter = TempFaultEmitter::new(r, thresholds).unwrap();

    // when temperature event thresholds are queried()
    let result = emitter.fault_level();

    // then the thresholds should report as expected
    assert_eq!(result, FaultLevel::None);
}

#[test]
pub fn fault_level_should_report_warn_when_temp_state_is_within_warn_threshold() {
    // given an event with defined trigger thresholds
    let thresholds = TempEmitterThresholds::new(Temp(-15.0), Temp(-10.0), Temp(-5.0)).unwrap();
    let (s, r) = bounded(0);
    spawn(move || s.send(Temp(-12.0)).unwrap());
    // avoid race for zero-capacity channel
    sleep(Duration::from_millis(10));
    let emitter = TempFaultEmitter::new(r, thresholds).unwrap();

    // when temperature event thresholds are queried()
    let result = emitter.fault_level();

    // then the thresholds should report as expected
    assert_eq!(result, FaultLevel::Warn);
}

#[test]
pub fn fault_level_should_report_serious_when_temp_state_is_within_serious_threshold() {
    // given an event with defined trigger thresholds
    let thresholds = TempEmitterThresholds::new(Temp(1000.0), Temp(1000.5), Temp(1001.0)).unwrap();
    let (s, r) = bounded(0);
    spawn(move || s.send(Temp(1000.5)).unwrap());
    // avoid race for zero-capacity channel
    sleep(Duration::from_millis(10));
    let emitter = TempFaultEmitter::new(r, thresholds).unwrap();

    // when temperature event thresholds are queried()
    let result = emitter.fault_level();

    // then the thresholds should report as expected
    assert_eq!(result, FaultLevel::Severe);
}

#[test]
pub fn fault_level_should_report_critical_when_temp_state_is_within_critical_threshold() {
    // given an event with defined trigger thresholds
    let thresholds = TempEmitterThresholds::new(Temp(-1000_f64), Temp(-100_f64), Temp(-10_f64)).unwrap();
    let (s, r) = bounded(0);
    spawn(move || s.send(Temp(-10_f64)).unwrap());
    // avoid race for zero-capacity channel
    sleep(Duration::from_millis(10));
    let emitter = TempFaultEmitter::new(r, thresholds).unwrap();

    // when temperature event thresholds are queried()
    let result = emitter.fault_level();

    // then the thresholds should report as expected
    assert_eq!(result, FaultLevel::Critical);
}

