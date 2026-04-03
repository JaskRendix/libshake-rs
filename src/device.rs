use std::fs::File;
use std::path::PathBuf;

use crate::effect::Effect;
use crate::error::{ShakeError, ShakeResult};
use crate::simple::*;

// Backend selection
#[cfg(all(feature = "linux-backend", not(feature = "mock-backend")))]
use crate::linux as backend;

#[cfg(all(feature = "linux-backend", not(feature = "mock-backend")))]
use crate::linux::{FF_PERIODIC, FF_RUMBLE};

#[cfg(feature = "mock-backend")]
use crate::mock as backend;

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

                devices.push(DeviceInfo {
                    id: devices.len() as u32,
                    name: info.name,
                    capacity: info.capacity,
                    features: info.features,
                    path,
                });
            }
        }

        Ok(devices)
    }

    pub fn open(id: u32) -> ShakeResult<Device> {
        let infos = Device::enumerate()?;
        let info = infos
            .into_iter()
            .find(|d| d.id == id)
            .ok_or(ShakeError::Device)?;

        Device::open_info(&info)
    }

    pub fn open_info(info: &DeviceInfo) -> ShakeResult<Device> {
        let file = backend::open_device(&info.path)?;
        Ok(Device {
            id: info.id,
            name: info.name.clone(),
            capacity: info.capacity,
            features: info.features.clone(),
            file,
            path: info.path.clone(),
        })
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

    pub fn path(&self) -> &std::path::Path {
        &self.path
    }

    pub fn upload(&self, effect: &Effect) -> ShakeResult<i32> {
        backend::upload_effect(&self.file, effect)
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

    #[cfg_attr(feature = "mock-backend", allow(dead_code))]
    fn has_feature(&self, bit: u16) -> bool {
        let idx = (bit / 64) as usize;
        let b = bit % 64;

        self.features
            .get(idx)
            .map(|chunk| (chunk & (1 << b)) != 0)
            .unwrap_or(false)
    }

    #[cfg(all(feature = "linux-backend", not(feature = "mock-backend")))]
    pub fn supports_rumble(&self) -> bool {
        self.has_feature(FF_RUMBLE)
    }

    #[cfg(all(feature = "linux-backend", not(feature = "mock-backend")))]
    pub fn supports_periodic(&self) -> bool {
        self.has_feature(FF_PERIODIC)
    }

    #[cfg(feature = "mock-backend")]
    pub fn supports_rumble(&self) -> bool {
        true
    }

    #[cfg(feature = "mock-backend")]
    pub fn supports_periodic(&self) -> bool {
        true
    }

    pub fn rumble(&self, strong: f32, weak: f32, secs: f32) -> ShakeResult<i32> {
        let effect = simple_rumble(strong, weak, secs);
        let id = self.upload(&effect)?;
        self.play(id)?;
        Ok(id)
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
