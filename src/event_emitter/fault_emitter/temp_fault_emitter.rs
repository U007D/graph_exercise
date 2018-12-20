#[cfg(test)]
mod unit_tests;
use crate::{
    Result,
    Temp,
    TempEmitterThresholds,
};
use crossbeam_channel::Receiver;
use super::{
    FaultEmitter,
    FaultLevel,
};

pub struct TempFaultEmitter {
    _recvr: Receiver<Temp>,
    temp: Temp,
    thresholds: TempEmitterThresholds,
}

impl TempFaultEmitter {
    pub fn new(_recvr: Receiver<Temp>, thresholds: TempEmitterThresholds) -> Result<Self> {
        Ok(Self {
            temp: _recvr.try_recv()?,
            _recvr,
            thresholds,
        })
    }

    pub fn thresholds(&self) -> TempEmitterThresholds {
        self.thresholds
    }
}

impl FaultEmitter for TempFaultEmitter {
    fn fault_level(&self) -> FaultLevel {
        match self.temp {
            t if t >= self.thresholds.critical() => FaultLevel::Critical,
            t if t >= self.thresholds.severe() && t < self.thresholds.critical() => FaultLevel::Severe,
            t if t >= self.thresholds.warn() && t < self.thresholds.severe() => FaultLevel::Warn,
            t if t < self.thresholds.warn() => FaultLevel::None,
            _ => unreachable!(),
        }
    }
}
