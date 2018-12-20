use super::State;

pub trait Temp {
    fn temp() -> Temp;
}

impl <T: Temp> State for T {}
