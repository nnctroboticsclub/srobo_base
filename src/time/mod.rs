mod time;

pub use time::TimeImpl;

#[cfg(feature = "std")]
mod time_std;

#[cfg(feature = "std")]
pub use time_std::HostTime;
