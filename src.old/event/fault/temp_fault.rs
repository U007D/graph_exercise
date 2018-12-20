use crate::{
    Fault,
    Event,
};

pub struct FaultThresholds
{
    warn: Temp,
    severe: Temp,
    critical: Temp,
}

impl FaultThresholds {
    pub fn new(warn: Temp, severe: Temp, critical: Temp) -> Result<Self> {
        match (warn, severe, critical) {
            (w, s, _) if w >= s => Err(Error::WarnThresholdGreaterThanSevere(w, s)),
            (_, s, c) if s >= c => Err(Error::SevereThresholdGreaterThanCritical(s, c)),
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
    pub fn critical(&self) -> Temp {self.critical }
}

pub struct TempFault<T: Event> {
    event: T,
    thresholds: FaultThresholds
}

impl<T: Event> TempFault<T> {
    pub fn new(event: T, thresholds: FaultThresholds) -> Self {
        Self {
            event,
            thresholds,
        }
    }
}

impl<T: Event> Event for TempFault<T> {
    fn fault_level(&self) -> FaultLevel {
        match self.event {
            v if v >= self.thresholds.critical => FaultLevel::Critical,
            v if v >= self.thresholds.severe() && v < self.thresholds.critical => FaultLevel::Severe,
            v if v >= self.thresholds.warn() && v < self.thresholds.severe() => FaultLevel::Warn,
            _ => FaultLevel::None,
        }
    }
}
