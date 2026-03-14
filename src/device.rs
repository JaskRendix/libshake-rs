use std::fs::File;
use std::path::PathBuf;

use crate::error::{ShakeError, ShakeResult};
use crate::effect::Effect;

#[cfg(target_os = "linux")]
use crate::linux as backend;

#[cfg(target_os = "macos")]
use crate::osx as backend;

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
    //
    // Device enumeration
    //
    pub fn enumerate() -> ShakeResult<Vec<DeviceInfo>> {
        let mut devices = Vec::new();
        let entries = backend::scan_event_nodes()?;
    
        for path in entries {
            if backend::probe_device(&path)? {
                let file = backend::open_device(&path)?;
                let info = backend::query_device(&file)?; // backend::DeviceInfo
    
                devices.push(DeviceInfo {
                    id: devices.len() as u32,
                    name: info.name,
                    capacity: info.capacity,
                    features: info.features,
                    path: path.clone(),
                });
            }
        }
    
        Ok(devices)
    }

    //
    // Open by ID
    //
    pub fn open(id: u32) -> ShakeResult<Device> {
        let infos = Device::enumerate()?;
        let info = infos.into_iter()
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

    //
    // Metadata accessors
    //
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

    //
    // Effect management
    //
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

    //
    // Device settings
    //
    pub fn set_gain(&self, gain: u16) -> ShakeResult<()> {
        backend::set_gain(&self.file, gain)
    }

    pub fn set_autocenter(&self, value: u16) -> ShakeResult<()> {
        backend::set_autocenter(&self.file, value)
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

