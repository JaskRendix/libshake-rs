use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::backend::{Backend, DeviceCapabilities, RawDeviceInfo};
use crate::effect::Effect;
use crate::error::{ShakeError, ShakeResult};

// Backend selection
#[cfg(all(feature = "linux-backend", not(feature = "mock-backend")))]
use crate::linux::LinuxBackend as ActiveBackend;

#[cfg(feature = "mock-backend")]
use crate::mock::MockBackend as ActiveBackend;

pub struct EffectHandle {
    id: i32,
    device: Arc<Device>,
}

impl EffectHandle {
    pub fn new(id: i32, device: Arc<Device>) -> Self {
        Self { id, device }
    }

    pub fn play(&self) -> ShakeResult<()> {
        self.device.play(self.id)
    }

    pub fn stop(&self) -> ShakeResult<()> {
        self.device.stop(self.id)
    }

    pub fn id(&self) -> i32 {
        self.id
    }
}

impl Drop for EffectHandle {
    fn drop(&mut self) {
        if let Err(e) = self.device.erase(self.id) {
            log::warn!("libShake: failed to erase effect {}: {:?}", self.id, e);
        }
    }
}

/// Public, ergonomic device metadata.
pub struct DeviceInfo {
    pub id: u32,
    pub name: String,
    pub max_effects: u32,
    pub raw_features: Vec<u64>,
    pub path: PathBuf,
}

impl DeviceInfo {
    pub fn supports(&self, bit: u16) -> bool {
        let idx = (bit / 64) as usize;
        let b = bit % 64;

        self.raw_features
            .get(idx)
            .map(|chunk| (chunk & (1 << b)) != 0)
            .unwrap_or(false)
    }
}

pub struct Device {
    id: u32,
    name: String,
    max_effects: u32,
    raw_features: Vec<u64>,
    capabilities: DeviceCapabilities,
    handle: <ActiveBackend as Backend>::Handle,
    path: PathBuf,
}

impl Device {
    // -------------------------------------------------------------------------
    // Enumeration
    // -------------------------------------------------------------------------

    pub fn enumerate() -> ShakeResult<Vec<DeviceInfo>> {
        let mut devices = Vec::new();
        let entries = ActiveBackend::scan()?;

        for (idx, path) in entries.into_iter().enumerate() {
            if let Ok(handle) = ActiveBackend::open(&path) {
                let RawDeviceInfo {
                    name,
                    capacity,
                    features,
                } = ActiveBackend::query(&handle)?;

                ActiveBackend::close(handle);

                let stable_id = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .and_then(|s| s.strip_prefix("event"))
                    .and_then(|n| n.parse::<u32>().ok())
                    .unwrap_or(idx as u32);

                devices.push(DeviceInfo {
                    id: stable_id,
                    name,
                    max_effects: capacity,
                    raw_features: features,
                    path,
                });
            }
        }

        Ok(devices)
    }

    // -------------------------------------------------------------------------
    // Opening
    // -------------------------------------------------------------------------

    pub fn open(id: u32) -> ShakeResult<Arc<Device>> {
        let infos = Device::enumerate()?;
        let info = infos
            .into_iter()
            .find(|d| d.id == id)
            .ok_or(ShakeError::Device)?;
        Device::open_info(&info)
    }

    pub fn open_info(info: &DeviceInfo) -> ShakeResult<Arc<Device>> {
        let handle = ActiveBackend::open(&info.path)?;
        let capabilities = ActiveBackend::capabilities(&handle)?;

        Ok(Arc::new(Device {
            id: info.id,
            name: info.name.clone(),
            max_effects: info.max_effects, // ← FIXED HERE
            raw_features: info.raw_features.clone(),
            capabilities,
            handle,
            path: info.path.clone(),
        }))
    }

    pub fn open_path(path: &Path) -> ShakeResult<Arc<Device>> {
        let handle = ActiveBackend::open(path)?;
        let RawDeviceInfo {
            name,
            capacity,
            features,
        } = ActiveBackend::query(&handle)?;
        let capabilities = ActiveBackend::capabilities(&handle)?;

        Ok(Arc::new(Device {
            id: 0,
            name,
            max_effects: capacity,
            raw_features: features,
            capabilities,
            handle,
            path: path.to_path_buf(),
        }))
    }

    // -------------------------------------------------------------------------
    // Effect operations
    // -------------------------------------------------------------------------

    pub fn upload(self: &Arc<Self>, effect: &Effect) -> ShakeResult<EffectHandle> {
        let id = ActiveBackend::upload(&self.handle, effect)?;
        Ok(EffectHandle::new(id, Arc::clone(self)))
    }

    pub fn update(&self, id: i32, effect: &Effect) -> ShakeResult<()> {
        ActiveBackend::update(&self.handle, id, effect)
    }

    pub fn erase(&self, id: i32) -> ShakeResult<()> {
        ActiveBackend::erase(&self.handle, id)
    }

    pub fn play(&self, id: i32) -> ShakeResult<()> {
        ActiveBackend::play(&self.handle, id)
    }

    pub fn stop(&self, id: i32) -> ShakeResult<()> {
        ActiveBackend::stop(&self.handle, id)
    }

    pub fn set_gain(&self, gain: u16) -> ShakeResult<()> {
        ActiveBackend::set_gain(&self.handle, gain)
    }

    pub fn set_autocenter(&self, value: u16) -> ShakeResult<()> {
        ActiveBackend::set_autocenter(&self.handle, value)
    }

    // -------------------------------------------------------------------------
    // Getters
    // -------------------------------------------------------------------------

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn max_effects(&self) -> u32 {
        self.max_effects
    }

    pub fn raw_features(&self) -> &[u64] {
        &self.raw_features
    }

    pub fn capabilities(&self) -> &DeviceCapabilities {
        &self.capabilities
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    // -------------------------------------------------------------------------
    // Simple helpers
    // -------------------------------------------------------------------------

    pub fn rumble(
        self: &Arc<Self>,
        strong: f32,
        weak: f32,
        secs: f32,
    ) -> ShakeResult<EffectHandle> {
        let effect = crate::simple::simple_rumble(strong, weak, secs);
        self.upload(&effect)
    }

    pub fn rumble_dir(
        self: &Arc<Self>,
        strong: f32,
        weak: f32,
        secs: f32,
        direction_deg: f32,
    ) -> ShakeResult<EffectHandle> {
        let effect = crate::simple::simple_rumble_dir(strong, weak, secs, direction_deg);
        self.upload(&effect)
    }
}

impl std::fmt::Debug for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Device")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("max_effects", &self.max_effects)
            .field("raw_features", &self.raw_features)
            .field("capabilities", &self.capabilities)
            .field("path", &self.path)
            .finish()
    }
}
