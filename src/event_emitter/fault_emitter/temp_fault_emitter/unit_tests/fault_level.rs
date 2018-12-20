use crate::{
    FaultEmitter,
    Temp,
};
use super::*;
use crossbeam_channel::unbounded;

#[test]
pub fn new_should_report_error_when_no_initial_temp_msg_is_available() {
    // given an event with defined trigger thresholds
    let thresholds = TempEmitterThresholds::new(Temp(0_f64), Temp(0_f64), Temp(0_f64)).unwrap();
    let (s, r) = unbounded();
    let emitter = TempFaultEmitter::new(r, thresholds).unwrap();
    let temp = Temp(25_f64);
    s.send(temp).unwrap();

    // when temperature event thresholds are queried()
    let result = emitter.fault_level();

    // then the thresholds should report as expected
    assert_eq!(result, FaultLevel::None);
}

#[test]
pub fn fault_level_should_report_none_when_temp_state_is_below_warn_threshold() {
    // given an event with defined trigger thresholds
    let thresholds = TempEmitterThresholds::new(Temp(50_f64), Temp(75_f64), Temp(100_f64)).unwrap();
    let (s, r) = unbounded();
    let emitter = TempFaultEmitter::new(r, thresholds).unwrap();
    let temp = Temp(25_f64);
    s.send(temp).unwrap();

    // when temperature event thresholds are queried()
    let result = emitter.fault_level();

    // then the thresholds should report as expected
    assert_eq!(result, FaultLevel::None);
}

#[test]
pub fn fault_level_should_report_warn_when_temp_state_is_within_warn_threshold() {
    // given an event with defined trigger thresholds
    let thresholds = TempEmitterThresholds::new(Temp(-15_f64), Temp(-10_f64), Temp(-5_f64)).unwrap();
    let (s, r) = unbounded();
    let emitter = TempFaultEmitter::new(r, thresholds).unwrap();
    let temp = Temp(-12_f64);
    s.send(temp).unwrap();

    // when temperature event thresholds are queried()
    let result = emitter.fault_level();

    // then the thresholds should report as expected
    assert_eq!(result, FaultLevel::Warn);
}

