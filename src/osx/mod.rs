#![cfg(target_os = "macos")]

use std::fs::File;
use std::path::PathBuf;

use crate::error::{ShakeError, ShakeResult};
use crate::effect::Effect;

mod ffi;

//
// Public metadata returned by macOS enumeration
//
#[derive(Debug, Clone)]
pub struct OsxDeviceInfo {
    pub name: String,
    pub capacity: u32,
    pub features: Vec<u64>,
    pub path: PathBuf,
}

//
// macOS does not support ForceFeedback in this build.
// All functions return ShakeError::Support, but they must exist
// so the unified Device API compiles.
//
pub fn scan_event_nodes() -> ShakeResult<Vec<PathBuf>> {
    // macOS has no /dev/input/event*; return empty list
    Ok(Vec::new())
}

pub fn probe_device(_path: &PathBuf) -> ShakeResult<bool> {
    // No probing possible; nothing is supported
    Ok(false)
}

pub fn open_device(_path: &PathBuf) -> ShakeResult<File> {
    Err(ShakeError::Support)
}

pub fn query_device(_file: &File) -> ShakeResult<OsxDeviceInfo> {
    Err(ShakeError::Support)
}

pub fn upload_effect(_file: &File, _effect: &Effect) -> ShakeResult<i32> {
    Err(ShakeError::Support)
}

pub fn erase_effect(_file: &File, _id: i32) -> ShakeResult<()> {
    Err(ShakeError::Support)
}

pub fn play_effect(_file: &File, _id: i32) -> ShakeResult<()> {
    Err(ShakeError::Support)
}

pub fn stop_effect(_file: &File, _id: i32) -> ShakeResult<()> {
    Err(ShakeError::Support)
}

pub fn set_gain(_file: &File, _gain: u16) -> ShakeResult<()> {
    Err(ShakeError::Support)
}

pub fn set_autocenter(_file: &File, _value: u16) -> ShakeResult<()> {
    Err(ShakeError::Support)
}
