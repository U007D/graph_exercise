use crate::{
    Temp,
    TempEmitterThresholds,
};
use crossbeam_channel::bounded;
use std::{
    thread::{
        sleep,
        spawn,
    },
    time::Duration,
};
use super::*;

#[test]
pub fn thresholds_should_report_correct_thresholds() {
    // given an event with defined trigger thresholds
    let thresholds = TempEmitterThresholds::new(Temp(50.0), Temp(75.0), Temp(100.0)).unwrap();
    let (s, r) = bounded(0);
    spawn(move || s.send(Temp(-12.0)).unwrap());
    // avoid race for zero-capacity channel
    sleep(Duration::from_millis(10));
    let emitter = TempFaultEmitter::new(r, thresholds).unwrap();

    // when temperature event thresholds are queried()
    let result = emitter.thresholds();

    // then the thresholds should report as expected
    assert_eq!(result, thresholds);
}
