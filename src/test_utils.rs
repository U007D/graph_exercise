use crate::Event;
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

pub struct TempSensorMock {
    should_fault: ShouldGenerateFault,
    times_called: AtomicUsize,
}

impl TempSensorMock {
    pub fn new(should_fault: ShouldGenerateFault) -> Self {
        Self {
            should_fault,
            times_called: AtomicUsize::new(0),
        }
    }

    pub fn times_called(&self) -> usize {
        self.times_called.load(Ordering::Relaxed)
    }
}

impl Event for TempSensorMock {
    fn is_triggered(&self) -> bool {
        self.times_called.fetch_add(1, Ordering::Relaxed);
        self.should_fault.as_bool()
    }
}

