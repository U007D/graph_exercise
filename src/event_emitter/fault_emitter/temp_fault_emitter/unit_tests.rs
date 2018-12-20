// unwrap() must not be used in release code; tests are OK.
#![allow(clippy::result_unwrap_used, clippy::option_unwrap_used)]
mod thresholds;
mod fault_level;
use super::*;
