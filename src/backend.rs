//! Backend abstraction for force‑feedback device implementations.
//!
//! Backends are responsible for:
//! - discovering devices
//! - opening and closing device handles
//! - querying raw device info and capabilities
//! - uploading / updating / playing / stopping / erasing effects
//! - setting device parameters (gain, autocenter)
//!
//! The `Device` struct wraps these operations into a stable public API.

use std::path::{Path, PathBuf};

use crate::effect::Effect;
use crate::error::ShakeResult;

/// Raw backend‑provided device information.
///
/// This contains only what the backend can directly observe.
/// `Device::enumerate()` assigns stable IDs and paths.
pub struct RawDeviceInfo {
    pub name: String,
    pub capacity: u32,
    pub features: Vec<u64>,
}

/// Backend‑agnostic capability model.
///
/// Derived from `RawDeviceInfo` unless a backend overrides it.
#[derive(Debug)]
pub struct DeviceCapabilities {
    pub rumble: bool,
    pub periodic: bool,
    pub spring: bool,
    pub friction: bool,
    pub damper: bool,
    pub inertia: bool,
    pub max_effects: u32,
}

/// Trait implemented by each backend (Linux, Mock, future backends).
pub trait Backend: Send + Sync + 'static {
    /// Backend‑specific device handle (e.g., `File` on Linux).
    type Handle;

    /// Scan for available devices and return their paths.
    fn scan() -> ShakeResult<Vec<PathBuf>>;

    /// Open a device at the given path.
    fn open(path: &Path) -> ShakeResult<Self::Handle>;

    /// Close a device handle.
    fn close(handle: Self::Handle);

    /// Query raw device information.
    fn query(handle: &Self::Handle) -> ShakeResult<RawDeviceInfo>;

    /// Query high‑level capabilities.
    ///
    /// Default implementation derives from `RawDeviceInfo`.
    fn capabilities(handle: &Self::Handle) -> ShakeResult<DeviceCapabilities> {
        let info = Self::query(handle)?;
        let f0 = info.features.first().copied().unwrap_or(0);

        Ok(DeviceCapabilities {
            rumble: (f0 & (1 << 0)) != 0,
            periodic: (f0 & (1 << 1)) != 0,
            spring: (f0 & (1 << 2)) != 0,
            friction: (f0 & (1 << 3)) != 0,
            damper: (f0 & (1 << 4)) != 0,
            inertia: (f0 & (1 << 5)) != 0,
            max_effects: info.capacity,
        })
    }

    /// Upload a new effect, returning a backend‑specific effect ID.
    fn upload(handle: &Self::Handle, effect: &Effect) -> ShakeResult<i32>;

    /// Update an existing effect in‑place.
    fn update(handle: &Self::Handle, id: i32, effect: &Effect) -> ShakeResult<()>;

    /// Play an uploaded effect.
    fn play(handle: &Self::Handle, id: i32) -> ShakeResult<()>;

    /// Stop a running effect.
    fn stop(handle: &Self::Handle, id: i32) -> ShakeResult<()>;

    /// Erase an uploaded effect.
    fn erase(handle: &Self::Handle, id: i32) -> ShakeResult<()>;

    /// Set device gain (0–100%).
    fn set_gain(handle: &Self::Handle, value: u16) -> ShakeResult<()>;

    /// Set device autocenter (0–100%).
    fn set_autocenter(handle: &Self::Handle, value: u16) -> ShakeResult<()>;
}
