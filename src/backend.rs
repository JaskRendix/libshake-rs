//! Backend abstraction for force‑feedback device implementations.

use std::path::{Path, PathBuf};

use crate::device::DeviceInfo;
use crate::effect::Effect;
use crate::error::ShakeResult;

/// Trait implemented by each backend (Linux, Mock, future backends).
///
/// Backends are responsible for:
/// - discovering devices
/// - opening device handles
/// - querying raw device info
/// - uploading / playing / stopping / erasing effects
/// - setting device parameters (gain, autocenter)
///
/// The `Device` struct wraps these operations into a stable public API.
pub trait Backend: Send + Sync + 'static {
    /// Backend-specific device handle (e.g., `File` on Linux).
    type Handle;

    /// Scan for available devices.
    fn scan() -> ShakeResult<Vec<PathBuf>>;

    /// Open a device at the given path.
    fn open(path: &Path) -> ShakeResult<Self::Handle>;

    /// Query raw device information.
    ///
    /// Backends return a `DeviceInfo` with placeholder `id` and `path`.
    /// `Device::enumerate()` normalizes these fields.
    fn query(handle: &Self::Handle) -> ShakeResult<DeviceInfo>;

    /// Upload an effect to the device.
    fn upload(handle: &Self::Handle, effect: &Effect) -> ShakeResult<i32>;

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
