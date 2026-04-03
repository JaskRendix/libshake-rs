pub mod device;
pub mod effect;
pub mod error;
pub mod simple;

// Linux backend (default)
#[cfg(feature = "linux-backend")]
pub mod linux;

// Mock backend (overrides Linux when enabled)
#[cfg(feature = "mock-backend")]
pub mod mock;
