use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;

use crate::effect::Effect;
use crate::error::{ShakeError, ShakeResult};

lazy_static::lazy_static! {
    static ref MOCK_EFFECTS: Mutex<HashMap<i32, Effect>> = Mutex::new(HashMap::new());
}

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

pub fn upload_effect(_file: &File, effect: &Effect) -> ShakeResult<i32> {
    let mut map = MOCK_EFFECTS.lock().unwrap();

    let id = (map.len() as i32) + 1;

    map.insert(id, effect.clone());

    println!("[MOCK] Uploaded effect ID: {}", id);

    Ok(id)
}

pub fn play_effect(_file: &File, id: i32) -> ShakeResult<()> {
    println!("[MOCK] Playing effect ID: {}", id);

    let effects = MOCK_EFFECTS.lock().unwrap();

    if let Some(effect) = effects.get(&id) {
        match effect {
            Effect::Periodic(p) => {
                println!("[MOCK] Oscilloscope (Periodic):");
                for t in (0..p.duration as u32).step_by(50) {
                    let sine = (2.0 * std::f32::consts::PI * (t as f32) / p.period as f32).sin();
                    let val = (sine * p.magnitude as f32) as i32;
                    let bar = "*".repeat((val.abs() / 2000) as usize);
                    println!("{:4}ms | {:7} | {}", t, val, bar);
                }
            }

            Effect::Ramp(r) => {
                println!("[MOCK] Oscilloscope (Ramp):");
                for t in (0..r.duration as u32).step_by(50) {
                    let frac = t as f32 / r.duration as f32;
                    let val = r.start_level as f32 * (1.0 - frac) + r.end_level as f32 * frac;
                    let bar = "*".repeat((val.abs() as i32 / 2000) as usize);
                    println!("{:4}ms | {:7.0} | {}", t, val, bar);
                }
            }

            Effect::Rumble(r) => {
                println!(
                    "[MOCK] Rumble: strong={}, weak={}, duration={}ms",
                    r.strong_magnitude, r.weak_magnitude, r.duration
                );
            }

            _ => println!("[MOCK] Effect type not visualized"),
        }
    } else {
        println!("[MOCK] No effect found for ID {}", id);
    }

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
