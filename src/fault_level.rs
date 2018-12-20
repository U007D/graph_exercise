#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum FaultLevel {
    None,
    Warn,
    Severe,
    Critical,
}
