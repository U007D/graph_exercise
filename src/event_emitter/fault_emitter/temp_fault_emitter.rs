#[cfg(test)]
mod unit_tests;
use crate::{
    Result,
    Temp,
    TempEmitterThresholds,
};
use crossbeam_channel::Receiver;
use super::{
//    EventEmitter,
    FaultEmitter,
    FaultLevel,
};

pub struct TempFaultEmitter {
    _recvr: Receiver<Temp>,
    _temp: Temp,
    thresholds: TempEmitterThresholds,
}

impl TempFaultEmitter {
    pub fn new(_recvr: Receiver<Temp>, thresholds: TempEmitterThresholds) -> Result<Self> {
        Ok(Self {
            _temp: _recvr.try_recv()?,
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
        FaultLevel::None
    }
}

//impl Event for Temp {
//    fn is_triggered(&self) -> bool {
//        self.fault_level() != FaultLevel::None
//    }
//
//    fn update(&mut self) -> &mut Self {
//        unimplemented!()
//    }
//}
