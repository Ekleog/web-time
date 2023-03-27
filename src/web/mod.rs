//! Re-implementation of [`std::time`].

mod instant;
mod js;
mod system_time;

pub use std::time::Duration;

pub use self::instant::Instant;
pub use self::system_time::{SystemTime, SystemTimeError};

/// See [`std::time::UNIX_EPOCH`].
pub const UNIX_EPOCH: SystemTime = SystemTime::UNIX_EPOCH;
