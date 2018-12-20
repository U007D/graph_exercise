use crate::{
    Temp,
    TempEmitterThresholds,
};
use crossbeam_channel::unbounded;
use super::*;

#[test]
pub fn thresholds_should_report_correct_thresholds() {
    // given an event with defined trigger thresholds
    let thresholds = TempEmitterThresholds::new(Temp(50_f64), Temp(75_f64), Temp(100_f64)).unwrap();
    let (_s, r) = unbounded();
    let emitter = TempFaultEmitter::new(r, thresholds).unwrap();

    // when temperature event thresholds are queried()
    let result = emitter.thresholds();

    // then the thresholds should report as expected
    assert_eq!(result, thresholds);
}
