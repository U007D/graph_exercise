#[cfg(test)]
mod unit_tests;
use crate::event::Event;

pub struct CriticalTempFault<T: Event> {
    event: T,
}

impl<T: Event> CriticalTempFault<T> {
    pub fn new(event: T) -> Self {
        Self {
            event,
        }
    }
}

impl<T: Event> Event for CriticalTempFault<T> {
    fn is_triggered(&self) -> bool {
        self.event.is_triggered()
    }
}
