pub mod fault_emitter;

pub trait EventEmitter {
    fn is_triggered(&self) -> bool;
}
