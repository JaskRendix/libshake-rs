use crate::backend::Backend;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use crate::effect::{ConditionEffect, Effect, PeriodicEffect, RampEffect};
use crate::error::{ShakeError, ShakeResult};

lazy_static::lazy_static! {
    static ref MOCK_EFFECTS: Mutex<HashMap<i32, Effect>> = Mutex::new(HashMap::new());
    static ref MOCK_TIMELINE: Mutex<Vec<(i32, u32)>> = Mutex::new(Vec::new()); // (effect_id, start_time_ms)
    static ref MOCK_GAIN: Mutex<u16> = Mutex::new(100);
    static ref MOCK_AUTOCENTER: Mutex<u16> = Mutex::new(0);
    static ref MOCK_LOG: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

pub struct DeviceInfo {
    pub name: String,
    pub capacity: u32,
    pub features: Vec<u64>,
}

// Device discovery

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
    println!("[MOCK] Device Inspector:");
    println!("  Name: Mock Device");
    println!("  Capacity: 16 effects");
    println!("  Features: Rumble, Periodic, Constant, Ramp, Spring, Friction, Damper, Inertia");

    mock_dashboard();

    Ok(DeviceInfo {
        name: "Mock Device".into(),
        capacity: 16,
        features: vec![u64::MAX, u64::MAX],
    })
}

// Effect lifecycle

pub fn upload_effect(_file: &File, effect: &Effect) -> ShakeResult<i32> {
    let mut map = MOCK_EFFECTS.lock().unwrap();
    let id = (map.len() as i32) + 1;
    map.insert(id, effect.clone());

    log(format!("Uploaded effect {}", id));
    println!("[MOCK] Uploaded effect ID: {}", id);
    Ok(id)
}

pub fn play_effect(_file: &File, id: i32) -> ShakeResult<()> {
    println!("[MOCK] Playing effect ID: {}", id);
    log(format!("Play effect {}", id));

    // Add to timeline
    {
        let mut timeline = MOCK_TIMELINE.lock().unwrap();
        timeline.push((id, now_ms()));
    }

    // Fetch effect
    let effects = MOCK_EFFECTS.lock().unwrap();
    let Some(effect) = effects.get(&id) else {
        println!("[MOCK] No effect found for ID {}", id);
        return Ok(());
    };

    // Rumble mixer
    if let Effect::Rumble(r) = effect {
        let gain = *MOCK_GAIN.lock().unwrap() as f32 / 100.0;
        let strong = (r.strong_magnitude as f32 * gain) as i32;
        let weak = (r.weak_magnitude as f32 * gain) as i32;

        println!(
            "[MOCK] Rumble Mixer: strong={} weak={} (gain={}%)",
            strong,
            weak,
            *MOCK_GAIN.lock().unwrap()
        );
    }

    // Oscilloscope
    match effect {
        Effect::Periodic(p) => visualize_periodic(p),
        Effect::Ramp(r) => visualize_ramp(r),
        Effect::Rumble(r) => println!(
            "[MOCK] Rumble: strong={}, weak={}, duration={}ms",
            r.strong_magnitude, r.weak_magnitude, r.duration
        ),
        Effect::Constant(c) => println!(
            "[MOCK] Constant: level={}, duration={}ms",
            c.level, c.duration
        ),
        Effect::Spring(c) => visualize_condition(c, "SPRING"),
        Effect::Friction(c) => visualize_condition(c, "FRICTION"),
        Effect::Damper(c) => visualize_condition(c, "DAMPER"),
        Effect::Inertia(c) => visualize_condition(c, "INERTIA"),
    }

    visualize_timeline();
    mock_dashboard();
    mock_profiler();

    Ok(())
}

pub fn stop_effect(_file: &File, id: i32) -> ShakeResult<()> {
    println!("[MOCK] Stopping effect ID: {}", id);
    log(format!("Stop effect {}", id));
    Ok(())
}

pub fn erase_effect(_file: &File, id: i32) -> ShakeResult<()> {
    println!("[MOCK] Erasing effect ID: {}", id);
    log(format!("Erase effect {}", id));
    MOCK_EFFECTS.lock().unwrap().remove(&id);
    Ok(())
}

// Device settings

pub fn set_gain(_file: &File, value: u16) -> ShakeResult<()> {
    *MOCK_GAIN.lock().unwrap() = value;
    println!("[MOCK] Gain set to {}%", value);
    log(format!("Set gain {}", value));
    mock_dashboard();
    Ok(())
}

pub fn set_autocenter(_file: &File, value: u16) -> ShakeResult<()> {
    *MOCK_AUTOCENTER.lock().unwrap() = value;
    println!("[MOCK] Autocenter set to {}%", value);
    log(format!("Set autocenter {}", value));
    mock_dashboard();
    Ok(())
}

// Helpers

fn now_ms() -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u32
}

fn log(msg: String) {
    MOCK_LOG
        .lock()
        .unwrap()
        .push(format!("[{}] {}", now_ms(), msg));
}

// Oscilloscopes

fn visualize_periodic(p: &PeriodicEffect) {
    println!("[MOCK] Oscilloscope (Periodic):");
    for t in (0..p.duration as u32).step_by(50) {
        let sine = (2.0 * std::f32::consts::PI * (t as f32) / p.period as f32).sin();
        let val = (sine * p.magnitude as f32) as i32;
        let bar = "*".repeat((val.abs() / 2000) as usize);
        println!("{:4}ms | {:7} | {}", t, val, bar);
    }
}

fn visualize_ramp(r: &RampEffect) {
    println!("[MOCK] Oscilloscope (Ramp):");
    for t in (0..r.duration as u32).step_by(50) {
        let frac = t as f32 / r.duration as f32;
        let val = r.start_level as f32 * (1.0 - frac) + r.end_level as f32 * frac;
        let bar = "*".repeat((val.abs() as i32 / 2000) as usize);
        println!("{:4}ms | {:7.0} | {}", t, val, bar);
    }
}

fn visualize_condition(c: &ConditionEffect, label: &str) {
    println!("[MOCK] Condition Monitor ({}):", label);
    println!("  Center: {}", c.center);
    println!("  Deadband: {}", c.deadband);
    println!("  Coefficients: L:{} R:{}", c.left_coeff, c.right_coeff);
    println!(
        "  Saturation:  L:{} R:{}",
        c.left_saturation, c.right_saturation
    );

    let left = if c.left_coeff > 0 { "<<" } else { "--" };
    let right = if c.right_coeff > 0 { ">>" } else { "--" };

    println!("  Feel: [ {} | center | {} ]", left, right);
}

// Timeline visualizer

pub fn visualize_timeline() {
    let timeline = MOCK_TIMELINE.lock().unwrap();
    let effects = MOCK_EFFECTS.lock().unwrap();

    println!("\n[MOCK] Timeline Visualizer:");
    println!("--------------------------------");

    for (id, start) in timeline.iter() {
        if let Some(effect) = effects.get(id) {
            println!("Effect {} started at {}ms:", id, start);

            match effect {
                Effect::Periodic(p) => {
                    println!("  Type: Periodic");
                    println!("  Duration: {}ms", p.duration);
                }
                Effect::Ramp(r) => {
                    println!("  Type: Ramp");
                    println!("  Duration: {}ms", r.duration);
                }
                Effect::Rumble(r) => {
                    println!("  Type: Rumble");
                    println!("  Duration: {}ms", r.duration);
                }
                _ => println!("  Type: Other"),
            }
        }
    }

    println!("--------------------------------\n");
}

// Device dashboard

pub fn mock_dashboard() {
    let gain = *MOCK_GAIN.lock().unwrap();
    let autocenter = *MOCK_AUTOCENTER.lock().unwrap();
    let effects = MOCK_EFFECTS.lock().unwrap();
    let timeline = MOCK_TIMELINE.lock().unwrap();

    println!("\n[MOCK] Device Dashboard");
    println!("===========================");
    println!("Gain:        {}%", gain);
    println!("Autocenter:  {}%", autocenter);
    println!("Uploaded:    {} effects", effects.len());
    println!("Active:      {} effects", timeline.len());
    println!("===========================\n");
}

// Mock profiler (effect density + fake CPU load)

pub fn mock_profiler() {
    let timeline = MOCK_TIMELINE.lock().unwrap();
    let active = timeline.len();

    let cpu_load = (active as f32 * 7.5).min(100.0);

    println!("[MOCK] Profiler:");
    println!("  Active effects: {}", active);
    println!("  Effect density: {} per second", active * 2);
    println!("  Simulated CPU load: {:.1}%", cpu_load);
}

// Log exporter

pub fn export_mock_log(path: &Path) -> ShakeResult<()> {
    let log = MOCK_LOG.lock().unwrap();

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .map_err(|_| ShakeError::Device)?;

    for line in log.iter() {
        writeln!(file, "{}", line).unwrap();
    }

    println!("[MOCK] Log exported to {:?}", path);
    Ok(())
}

pub struct MockBackend;

impl Backend for MockBackend {
    type Handle = File;

    fn scan() -> ShakeResult<Vec<PathBuf>> {
        scan_event_nodes()
    }

    fn open(path: &Path) -> ShakeResult<Self::Handle> {
        open_device(path)
    }

    fn query(handle: &Self::Handle) -> ShakeResult<crate::device::DeviceInfo> {
        let raw = query_device(handle)?;

        Ok(crate::device::DeviceInfo {
            id: 0,
            name: raw.name,
            capacity: raw.capacity,
            features: raw.features,
            path: PathBuf::from("/dev/mock0"),
        })
    }

    fn upload(handle: &Self::Handle, effect: &Effect) -> ShakeResult<i32> {
        upload_effect(handle, effect)
    }

    fn play(handle: &Self::Handle, id: i32) -> ShakeResult<()> {
        play_effect(handle, id)
    }

    fn stop(handle: &Self::Handle, id: i32) -> ShakeResult<()> {
        stop_effect(handle, id)
    }

    fn erase(handle: &Self::Handle, id: i32) -> ShakeResult<()> {
        erase_effect(handle, id)
    }

    fn set_gain(handle: &Self::Handle, value: u16) -> ShakeResult<()> {
        set_gain(handle, value)
    }

    fn set_autocenter(handle: &Self::Handle, value: u16) -> ShakeResult<()> {
        set_autocenter(handle, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Read;

    use crate::effect::{Envelope, PeriodicWaveform, RumbleEffect};

    fn reset_mock_state() {
        MOCK_EFFECTS.lock().unwrap().clear();
        MOCK_TIMELINE.lock().unwrap().clear();
        *MOCK_GAIN.lock().unwrap() = 100;
        *MOCK_AUTOCENTER.lock().unwrap() = 0;
        MOCK_LOG.lock().unwrap().clear();
    }

    fn dummy_periodic_effect() -> Effect {
        Effect::Periodic(PeriodicEffect {
            waveform: PeriodicWaveform::Sine,
            period: 100,
            magnitude: 0x7FFF,
            offset: 0,
            phase: 0,
            envelope: Envelope {
                attack_length: 100,
                attack_level: 0,
                fade_length: 100,
                fade_level: 0,
            },
            duration: 500,
            delay: 0,
            direction: 0,
        })
    }

    fn dummy_ramp_effect() -> Effect {
        Effect::Ramp(RampEffect {
            start_level: -0x4000,
            end_level: 0x4000,
            envelope: Envelope {
                attack_length: 0,
                attack_level: 0,
                fade_length: 0,
                fade_level: 0,
            },
            duration: 500,
            delay: 0,
            direction: 0,
        })
    }

    fn dummy_rumble_effect() -> Effect {
        Effect::Rumble(RumbleEffect {
            strong_magnitude: 0x7FFF,
            weak_magnitude: 0x3FFF,
            duration: 500,
            delay: 0,
            direction: 0,
        })
    }

    #[test]
    fn scan_event_nodes_returns_mock_path() {
        reset_mock_state();
        let nodes = scan_event_nodes().unwrap();
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0], PathBuf::from("/dev/mock0"));
    }

    #[test]
    fn probe_device_always_true() {
        reset_mock_state();
        let ok = probe_device(Path::new("/dev/mock0")).unwrap();
        assert!(ok);
    }

    #[test]
    fn open_device_returns_file() {
        reset_mock_state();
        let f = open_device(Path::new("/dev/mock0"));
        assert!(f.is_ok());
    }

    #[test]
    fn query_device_returns_mock_info() {
        reset_mock_state();
        let f = open_device(Path::new("/dev/mock0")).unwrap();
        let info = query_device(&f).unwrap();
        assert_eq!(info.name, "Mock Device");
        assert_eq!(info.capacity, 16);
        assert_eq!(info.features.len(), 2);
    }

    #[test]
    fn upload_effect_stores_effect_and_returns_id() {
        reset_mock_state();
        let f = open_device(Path::new("/dev/mock0")).unwrap();
        let e = dummy_periodic_effect();

        let id = upload_effect(&f, &e).unwrap();
        assert_eq!(id, 1);

        let map = MOCK_EFFECTS.lock().unwrap();
        assert_eq!(map.len(), 1);
        assert!(map.contains_key(&1));
    }

    #[test]
    fn play_effect_adds_to_timeline() {
        reset_mock_state();
        let f = open_device(Path::new("/dev/mock0")).unwrap();
        let e = dummy_periodic_effect();
        let id = upload_effect(&f, &e).unwrap();

        play_effect(&f, id).unwrap();

        let timeline = MOCK_TIMELINE.lock().unwrap();
        assert_eq!(timeline.len(), 1);
        assert_eq!(timeline[0].0, id);
    }

    #[test]
    fn play_rumble_uses_rumble_mixer() {
        reset_mock_state();
        let f = open_device(Path::new("/dev/mock0")).unwrap();
        let e = dummy_rumble_effect();
        let id = upload_effect(&f, &e).unwrap();

        set_gain(&f, 50).unwrap();
        play_effect(&f, id).unwrap();

        // Just assert that timeline and log were updated
        let timeline = MOCK_TIMELINE.lock().unwrap();
        assert_eq!(timeline.len(), 1);

        let log = MOCK_LOG.lock().unwrap();
        assert!(log.iter().any(|l| l.contains("Play effect")));
    }

    #[test]
    fn erase_effect_removes_from_store() {
        reset_mock_state();
        let f = open_device(Path::new("/dev/mock0")).unwrap();
        let e = dummy_ramp_effect();
        let id = upload_effect(&f, &e).unwrap();

        {
            let map = MOCK_EFFECTS.lock().unwrap();
            assert!(map.contains_key(&id));
        }

        erase_effect(&f, id).unwrap();

        let map = MOCK_EFFECTS.lock().unwrap();
        assert!(!map.contains_key(&id));
    }

    #[test]
    fn set_gain_updates_state_and_logs() {
        reset_mock_state();
        let f = open_device(Path::new("/dev/mock0")).unwrap();

        set_gain(&f, 75).unwrap();

        let gain = *MOCK_GAIN.lock().unwrap();
        assert_eq!(gain, 75);

        let log = MOCK_LOG.lock().unwrap();
        assert!(log.iter().any(|l| l.contains("Set gain 75")));
    }

    #[test]
    fn set_autocenter_updates_state_and_logs() {
        reset_mock_state();
        let f = open_device(Path::new("/dev/mock0")).unwrap();

        set_autocenter(&f, 30).unwrap();

        let ac = *MOCK_AUTOCENTER.lock().unwrap();
        assert_eq!(ac, 30);

        let log = MOCK_LOG.lock().unwrap();
        assert!(log.iter().any(|l| l.contains("Set autocenter 30")));
    }

    #[test]
    fn mock_profiler_runs_with_no_panic() {
        reset_mock_state();
        mock_profiler(); // just ensure it doesn't panic
    }

    #[test]
    fn export_mock_log_writes_file() {
        reset_mock_state();
        let f = open_device(Path::new("/dev/mock0")).unwrap();
        let e = dummy_periodic_effect();
        let id = upload_effect(&f, &e).unwrap();
        play_effect(&f, id).unwrap();

        let path = PathBuf::from("target/mock_log_test.log");
        export_mock_log(&path).unwrap();

        let mut file = File::open(&path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        assert!(contents.contains("Uploaded effect"));
        assert!(contents.contains("Play effect"));

        let _ = fs::remove_file(path);
    }

    #[test]
    fn visualize_timeline_does_not_panic() {
        reset_mock_state();
        let f = open_device(Path::new("/dev/mock0")).unwrap();
        let e = dummy_periodic_effect();
        let id = upload_effect(&f, &e).unwrap();
        play_effect(&f, id).unwrap();

        visualize_timeline(); // smoke test
    }

    #[test]
    fn mock_dashboard_does_not_panic() {
        reset_mock_state();
        mock_dashboard();
    }
}
