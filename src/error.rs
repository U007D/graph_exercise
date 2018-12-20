use crate::Temp;
use crossbeam_channel::TryRecvError;

#[derive(Debug, PartialEq)]
pub enum TempThresholdError {
    WarnGreaterThanSevere(Temp, Temp),
    SevereGreaterThanCritical(Temp, Temp),
}

#[derive(Debug, PartialEq)]
pub enum Error {
    TryRecvError(TryRecvError),
    InvalidTempThreshold(TempThresholdError),
}

impl From<TryRecvError> for Error {
    fn from(err: TryRecvError) -> Self {
        Error::TryRecvError(err)
    }
}
