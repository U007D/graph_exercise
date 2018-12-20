use crate::{
    Event,
};

use std::sync::atomic::{
    AtomicUsize,
    Ordering,
};

#[derive(Debug, Copy, Clone)]
pub enum ShouldGenerateFault {
    No,
    Yes,
}

impl ShouldGenerateFault {
    pub fn as_bool(&self) -> bool {
        match self {
            ShouldGenerateFault::Yes => true,
            ShouldGenerateFault::No => false,
        }
    }
}

pub struct FaultMock {
    fault_level: FaultLevel,
    times_called: AtomicUsize,
}

impl FaultMock {
    pub fn new(fault_level: FaultLevel) -> Self {
        Self {
            fault_level,
            times_called: AtomicUsize::new(0),
        }
    }

    pub fn times_called(&self) -> usize {
        self.times_called.load(Ordering::Relaxed)
    }

    pub fn is_triggered(&self) -> bool {
        self.times_called.fetch_add(1, Ordering::Relaxed);
        <Self as Event>::is_triggered()
    }
}

impl Event for TempSensorMock {}

