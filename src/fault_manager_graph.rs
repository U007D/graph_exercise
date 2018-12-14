#[cfg(test)]
mod unit_tests;

#[derive(Debug, PartialEq)]
pub struct FaultManagerGraph {}

impl FaultManagerGraph {
    pub fn new() -> Self {
        Self {}
    }

    pub fn len(&self) -> usize {
        0
    }
}