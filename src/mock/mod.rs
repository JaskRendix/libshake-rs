use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

use crate::effect::Effect;
use crate::error::{ShakeError, ShakeResult};

pub struct DeviceInfo {
    pub name: String,
    pub capacity: u32,
    pub features: Vec<u64>,
}

pub fn scan_event_nodes() -> ShakeResult<Vec<PathBuf>> {
    Ok(vec![PathBuf::from("/dev/mock0")])
}

pub fn probe_device(_path: &Path) -> ShakeResult<bool> {
    Ok(true)
}

pub fn open_device(_path: &Path) -> ShakeResult<File> {
    File::open("/dev/null").map_err(|_| ShakeError::Device)
}

pub fn query_device(_file: &File) -> ShakeResult<DeviceInfo> {
    Ok(DeviceInfo {
        name: "Mock Device".into(),
        capacity: 16,
        features: vec![u64::MAX, u64::MAX],
    })
}

pub fn upload_effect(_file: &File, _effect: &Effect) -> ShakeResult<i32> {
    Ok(1)
}

pub fn play_effect(_file: &File, _id: i32) -> ShakeResult<()> {
    Ok(())
}

pub fn stop_effect(_file: &File, _id: i32) -> ShakeResult<()> {
    Ok(())
}

pub fn erase_effect(_file: &File, _id: i32) -> ShakeResult<()> {
    Ok(())
}

pub fn set_gain(_file: &File, _value: u16) -> ShakeResult<()> {
    Ok(())
}

pub fn set_autocenter(_file: &File, _value: u16) -> ShakeResult<()> {
    Ok(())
}
