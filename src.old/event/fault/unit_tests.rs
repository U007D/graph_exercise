use super::*;

// Each FaultLevel variant listed explicitly to ensure FaultLevel testing is exhaustive (i.e. don't use _ => true)
#[test]
fn should_trigger(fault_level: FaultLevel) -> bool {
    match fault_level {
        FaultLevel::None => false,
        FaultLevel::Warn => true,
        FaultLevel::Severe => true,
        FaultLevel::Critical => true,
    }
}

#[test]
fn when_fault_level_is_none_is_triggered_yields_false() {
    // given a fault at FaultLevel::None
    let fault_level = FaultLevel::None;
    let fault = FaultMock::new(fault_level);

    // when is_triggered() is queried
    let result = fault.is_triggered();

    // then it returns the expected result
    assert_eq!(result, should_trigger(fault_level))
}

#[test]
fn when_fault_level_is_warn_is_triggered_yields_true() {
    // given a fault at FaultLevel::Warn
    let fault_level = FaultLevel::Warn;
    let fault = FaultMock::new(fault_level);

    // when is_triggered() is queried
    let result = fault.is_triggered();

    // then it returns the expected result
    assert_eq!(result, should_trigger(fault_level))
}

#[test]
fn when_fault_level_is_severe_is_triggered_yields_true() {
    // given a fault at FaultLevel::Severe
    let fault_level = FaultLevel::Severe;
    let fault = FaultMock::new(fault_level);

    // when is_triggered() is queried
    let result = fault.is_triggered();

    // then it returns the expected result
    assert_eq!(result, should_trigger(fault_level))
}

#[test]
fn when_fault_level_is_critical_is_triggered_yields_true() {
    // given a fault at FaultLevel::Critical
    let fault_level = FaultLevel::Critical;
    let fault = FaultMock::new(fault_level);

    // when is_triggered() is queried
    let result = fault.is_triggered();

    // then it returns the expected result
    assert_eq!(result, should_trigger(fault_level))
}
