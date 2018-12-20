#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum FaultLevel {
    None,
    Warn,
    Severe,
    Critical,
}