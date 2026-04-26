use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::effect::Effect;
use crate::error::{ShakeError, ShakeResult};
use crate::simple::*;

// Backend selection
#[cfg(all(feature = "linux-backend", not(feature = "mock-backend")))]
use crate::linux as backend;

#[cfg(all(feature = "linux-backend", not(feature = "mock-backend")))]
use crate::linux::{FF_DAMPER, FF_FRICTION, FF_INERTIA, FF_PERIODIC, FF_RUMBLE, FF_SPRING};

#[cfg(feature = "mock-backend")]
use crate::mock as backend;

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

pub struct Device {
    id: u32,
    name: String,
    capacity: u32,
    features: Vec<u64>,
    file: File,
    path: PathBuf,
}

pub struct DeviceInfo {
    pub id: u32,
    pub name: String,
    pub capacity: u32,
    pub features: Vec<u64>,
    pub path: PathBuf,
}

impl Device {
    pub fn enumerate() -> ShakeResult<Vec<DeviceInfo>> {
        let mut devices = Vec::new();
        let entries: Vec<PathBuf> = backend::scan_event_nodes()?;

        for path in entries {
            if backend::probe_device(&path)? {
                let file = backend::open_device(&path)?;
                let info = backend::query_device(&file)?;

                // Stable ID: extract event number if possible
                let stable_id = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .and_then(|s| s.strip_prefix("event"))
                    .and_then(|n| n.parse::<u32>().ok())
                    .unwrap_or(devices.len() as u32);

                devices.push(DeviceInfo {
                    id: stable_id,
                    name: info.name,
                    capacity: info.capacity,
                    features: info.features,
                    path,
                });
            }
        }

        Ok(devices)
    }

    pub fn open(id: u32) -> ShakeResult<Arc<Device>> {
        let infos = Device::enumerate()?;
        let info = infos
            .into_iter()
            .find(|d| d.id == id)
            .ok_or(ShakeError::Device)?;

        Device::open_info(&info)
    }

    pub fn open_info(info: &DeviceInfo) -> ShakeResult<Arc<Device>> {
        let file = backend::open_device(&info.path)?;
        Ok(Arc::new(Device {
            id: info.id,
            name: info.name.clone(),
            capacity: info.capacity,
            features: info.features.clone(),
            file,
            path: info.path.clone(),
        }))
    }

    pub fn open_path(path: &Path) -> ShakeResult<Arc<Device>> {
        let file = backend::open_device(path)?;
        let info = backend::query_device(&file)?;
        Ok(Arc::new(Device {
            id: 0,
            name: info.name,
            capacity: info.capacity,
            features: info.features,
            file,
            path: path.to_path_buf(),
        }))
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn capacity(&self) -> u32 {
        self.capacity
    }

    pub fn features(&self) -> &[u64] {
        &self.features
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn upload(self: &Arc<Self>, effect: &Effect) -> ShakeResult<EffectHandle> {
        let id = backend::upload_effect(&self.file, effect)?;
        Ok(EffectHandle::new(id, Arc::clone(self)))
    }

    pub fn erase(&self, id: i32) -> ShakeResult<()> {
        backend::erase_effect(&self.file, id)
    }

    pub fn play(&self, id: i32) -> ShakeResult<()> {
        backend::play_effect(&self.file, id)
    }

    pub fn stop(&self, id: i32) -> ShakeResult<()> {
        backend::stop_effect(&self.file, id)
    }

    pub fn set_gain(&self, gain: u16) -> ShakeResult<()> {
        backend::set_gain(&self.file, gain)
    }

    pub fn set_autocenter(&self, value: u16) -> ShakeResult<()> {
        backend::set_autocenter(&self.file, value)
    }

    /// Generic feature check
    pub fn supports(&self, bit: u16) -> bool {
        let idx = (bit / 64) as usize;
        let b = bit % 64;

        self.features
            .get(idx)
            .map(|chunk| (chunk & (1 << b)) != 0)
            .unwrap_or(false)
    }

    #[cfg(all(feature = "linux-backend", not(feature = "mock-backend")))]
    pub fn supports_rumble(&self) -> bool {
        self.supports(FF_RUMBLE)
    }

    #[cfg(all(feature = "linux-backend", not(feature = "mock-backend")))]
    pub fn supports_periodic(&self) -> bool {
        self.supports(FF_PERIODIC)
    }

    #[cfg(all(feature = "linux-backend", not(feature = "mock-backend")))]
    pub fn supports_spring(&self) -> bool {
        self.supports(FF_SPRING)
    }

    #[cfg(all(feature = "linux-backend", not(feature = "mock-backend")))]
    pub fn supports_friction(&self) -> bool {
        self.supports(FF_FRICTION)
    }

    #[cfg(all(feature = "linux-backend", not(feature = "mock-backend")))]
    pub fn supports_damper(&self) -> bool {
        self.supports(FF_DAMPER)
    }

    #[cfg(all(feature = "linux-backend", not(feature = "mock-backend")))]
    pub fn supports_inertia(&self) -> bool {
        self.supports(FF_INERTIA)
    }

    #[cfg(feature = "mock-backend")]
    pub fn supports_spring(&self) -> bool {
        true
    }

    #[cfg(feature = "mock-backend")]
    pub fn supports_friction(&self) -> bool {
        true
    }

    #[cfg(feature = "mock-backend")]
    pub fn supports_damper(&self) -> bool {
        true
    }

    #[cfg(feature = "mock-backend")]
    pub fn supports_inertia(&self) -> bool {
        true
    }

    #[cfg(feature = "mock-backend")]
    pub fn supports_rumble(&self) -> bool {
        true
    }

    #[cfg(feature = "mock-backend")]
    pub fn supports_periodic(&self) -> bool {
        true
    }

    pub fn rumble(
        self: &Arc<Self>,
        strong: f32,
        weak: f32,
        secs: f32,
    ) -> ShakeResult<EffectHandle> {
        let effect = simple_rumble(strong, weak, secs);
        self.upload(&effect)
    }

    pub fn rumble_dir(
        self: &Arc<Self>,
        strong: f32,
        weak: f32,
        secs: f32,
        direction_deg: f32,
    ) -> ShakeResult<EffectHandle> {
        let effect = simple_rumble_dir(strong, weak, secs, direction_deg);
        self.upload(&effect)
    }
}

impl std::fmt::Debug for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Device")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("capacity", &self.capacity)
            .field("features", &self.features)
            .field("path", &self.path)
            .finish()
    }
}
