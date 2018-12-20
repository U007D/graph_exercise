use super::*;

#[test]
fn new_with_valid_thresholds_succeeds() {
    // given valid thresholds
    let warn = Temp(2_f64);
    let serious = Temp(4_f64);
    let critical = Temp(6_f64);

    // when constructing a TempEmitterThresholds
    let result = TempEmitterThresholds::new(warn, serious, critical);

    // then the construction should succeed
    assert_eq!(result.is_ok(), true)
}

// warn, severe, critical return correct values
// new with invalid thresholds w = s, w > s, s = c, s > c fails
