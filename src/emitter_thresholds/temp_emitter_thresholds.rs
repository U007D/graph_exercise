#[cfg(test)]
mod unit_tests;
use crate::{
    Error,
    Result,
    Temp,
    TempThresholdError,
};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct TempEmitterThresholds
{
    warn: Temp,
    severe: Temp,
    critical: Temp,
}

impl TempEmitterThresholds {
    pub fn new(warn: Temp, severe: Temp, critical: Temp) -> Result<Self> {
        match (warn, severe, critical) {
            (w, s, _) if w >= s =>
                Err(Error::InvalidTempThreshold(TempThresholdError::WarnGreaterThanSevere(w, s))),
            (_, s, c) if s >= c =>
                Err(Error::InvalidTempThreshold(TempThresholdError::SevereGreaterThanCritical(s, c))),
            _ => Ok(Self {
                warn,
                severe,
                critical,
            }),
        }
    }

    #[inline]
    pub fn warn(&self) -> Temp { self.warn }

    #[inline]
    pub fn severe(&self) -> Temp { self.severe }

    #[inline]
    pub fn critical(&self) -> Temp { self.critical }
}
