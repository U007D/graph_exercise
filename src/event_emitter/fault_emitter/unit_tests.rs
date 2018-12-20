// unwrap() must not be used in release code; tests are OK.
#![allow(clippy::result_unwrap_used, clippy::option_unwrap_used)]
use super::*;

struct FaultFake {
    fault_level: FaultLevel,
}

impl FaultFake {
    fn new(fault_level: FaultLevel) -> Self {
        Self {
            fault_level,
        }
    }
}

impl FaultEmitter for FaultFake {
    fn fault_level(&self) -> FaultLevel {
        self.fault_level
    }
}
// Each FaultLevel variant listed explicitly to ensure FaultLevel testing is exhaustive (i.e. don't use _ => true)
fn should_trigger(fault_level: FaultLevel) -> bool {
    match fault_level {
        FaultLevel::None => false,
        FaultLevel::Warn => true,
        FaultLevel::Severe => true,
        FaultLevel::Critical => true,
    }
}

#[test]
fn is_triggered_yields_false_when_fault_level_is_none() {
    // given a fault at FaultLevel::None
    let fault_level = FaultLevel::None;
    let fault = FaultFake::new(fault_level);

    // when is_triggered() is queried
    let result = fault.is_triggered();

    // then it returns the expected result
    assert_eq!(result, should_trigger(fault_level))
}

#[test]
fn is_triggered_yields_true_when_fault_level_is_warn() {
    // given a fault at FaultLevel::Warn
    let fault_level = FaultLevel::Warn;
    let fault = FaultFake::new(fault_level);

    // when is_triggered() is queried
    let result = fault.is_triggered();

    // then it returns the expected result
    assert_eq!(result, should_trigger(fault_level))
}

#[test]
fn is_triggered_yields_true_when_fault_level_is_severe() {
    // given a fault at FaultLevel::Severe
    let fault_level = FaultLevel::Severe;
    let fault = FaultFake::new(fault_level);

    // when is_triggered() is queried
    let result = fault.is_triggered();

    // then it returns the expected result
    assert_eq!(result, should_trigger(fault_level))
}

#[test]
fn is_triggered_yields_true_when_fault_level_is_critical() {
    // given a fault at FaultLevel::Critical
    let fault_level = FaultLevel::Critical;
    let fault = FaultFake::new(fault_level);

    // when is_triggered() is queried
    let result = fault.is_triggered();

    // then it returns the expected result
    assert_eq!(result, should_trigger(fault_level))
}
