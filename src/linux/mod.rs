#![cfg(target_os = "linux")]

use crate::backend::Backend;
use std::fs;
use std::fs::File;
use std::os::fd::{AsRawFd, BorrowedFd, FromRawFd};
use std::path::Path;
use std::path::PathBuf;

use nix::fcntl::{self, OFlag};
use nix::libc;
use nix::sys::stat::Mode;
use nix::unistd;

use crate::effect::{
    ConditionEffect, ConstantEffect, Effect, Envelope, PeriodicEffect, PeriodicWaveform,
    RampEffect, RumbleEffect,
};
use crate::error::{ShakeError, ShakeResult};

// Linux FF constants
const EV_FF: u16 = 0x15;

pub const FF_RUMBLE: u16 = 0x50;
pub const FF_PERIODIC: u16 = 0x51;
const FF_CONSTANT: u16 = 0x52;
const FF_RAMP: u16 = 0x57;

const FF_SQUARE: u16 = 0x58;
const FF_TRIANGLE: u16 = 0x59;
const FF_SINE: u16 = 0x5A;
const FF_SAW_UP: u16 = 0x5B;
const FF_SAW_DOWN: u16 = 0x5C;
const FF_CUSTOM: u16 = 0x5D;

const FF_GAIN: u16 = 0x60;
const FF_AUTOCENTER: u16 = 0x61;

pub const FF_SPRING: u16 = 0x53;
pub const FF_FRICTION: u16 = 0x54;
pub const FF_DAMPER: u16 = 0x55;
pub const FF_INERTIA: u16 = 0x56;

pub struct DeviceInfo {
    pub id: u32,
    pub name: String,
    pub capacity: u32,
    pub features: Vec<u64>,
    pub path: PathBuf,
}

// ioctl definitions
const EVIOCGNAME_LEN: usize = 256;
const EVIOCGBIT_EV_FF_LEN: usize = 16;

nix::ioctl_read!(eviocgeffects, b'E', 0x84, libc::c_int);
nix::ioctl_read_buf!(eviocgname, b'E', 0x06, u8);
nix::ioctl_read_buf!(eviocgbit_ff, b'E', 0x20 + EV_FF as u8, u8);
nix::ioctl_write_ptr!(eviocsff, b'E', 0x80, libc::ff_effect);
nix::ioctl_write_int!(eviocrmff, b'E', 0x81);

fn scan_event_nodes_in(dir: &Path) -> ShakeResult<Vec<PathBuf>> {
    let mut nodes = Vec::new();

    let entries = fs::read_dir(dir).map_err(|e| match e.kind() {
        std::io::ErrorKind::PermissionDenied => ShakeError::Permission,
        _ => ShakeError::Device,
    })?;

    for entry in entries {
        let entry = entry.map_err(|e| match e.kind() {
            std::io::ErrorKind::PermissionDenied => ShakeError::Permission,
            _ => ShakeError::Device,
        })?;
        let name = entry.file_name();
        let name = match name.to_str() {
            Some(s) => s,
            None => continue,
        };

        if name.starts_with("event") {
            nodes.push(entry.path());
        }
    }

    if nodes.is_empty() {
        return Err(ShakeError::Device);
    }

    Ok(nodes)
}

pub fn scan_event_nodes() -> ShakeResult<Vec<PathBuf>> {
    scan_event_nodes_in(Path::new("/dev/input"))
}

// Device probing
pub fn probe_device(path: &Path) -> ShakeResult<bool> {
    let file = match open_device(path) {
        Ok(f) => f,
        Err(_) => return Ok(false),
    };

    let info = match query_device(&file) {
        Ok(i) => i,
        Err(_) => return Ok(false),
    };

    Ok(info.capacity > 0)
}

// Device opening
pub fn open_device(path: &Path) -> ShakeResult<File> {
    let fd =
        fcntl::open(path, OFlag::O_RDWR | OFlag::O_NONBLOCK, Mode::empty()).map_err(
            |e| match e {
                nix::Error::EACCES | nix::Error::EPERM => ShakeError::Permission,
                _ => ShakeError::Device,
            },
        )?;

    Ok(unsafe { File::from_raw_fd(fd) })
}

// Query device capabilities
pub fn query_device(fd: &File) -> ShakeResult<DeviceInfo> {
    let raw_fd = fd.as_raw_fd();

    // EVIOCGEFFECTS
    let mut effects: libc::c_int = 0;
    unsafe {
        eviocgeffects(raw_fd, &mut effects as *mut libc::c_int).map_err(|e| match e {
            nix::Error::EACCES | nix::Error::EPERM => ShakeError::Permission,
            _ => ShakeError::Query,
        })?;
    }

    // EVIOCGNAME
    let mut name_buf = [0u8; EVIOCGNAME_LEN];
    unsafe {
        eviocgname(raw_fd, &mut name_buf).map_err(|e| match e {
            nix::Error::EACCES | nix::Error::EPERM => ShakeError::Permission,
            _ => ShakeError::Query,
        })?;
    }
    let name = String::from_utf8_lossy(
        &name_buf[..name_buf
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(name_buf.len())],
    )
    .to_string();

    // EVIOCGBIT(EV_FF)
    let mut ff_bits = [0u8; EVIOCGBIT_EV_FF_LEN];
    unsafe {
        eviocgbit_ff(raw_fd, &mut ff_bits).map_err(|e| match e {
            nix::Error::EACCES | nix::Error::EPERM => ShakeError::Permission,
            _ => ShakeError::Query,
        })?;
    }

    let mut features = Vec::new();
    for chunk in ff_bits.chunks(8) {
        let mut v = 0u64;
        for (i, b) in chunk.iter().enumerate() {
            v |= (*b as u64) << (i * 8);
        }
        features.push(v);
    }

    Ok(DeviceInfo {
        id: 0, // Device::enumerate() will assign the real ID
        name,
        capacity: effects as u32,
        features,
        path: PathBuf::new(), // Device::enumerate() overwrites this too
    })
}

// Effect conversion helpers
fn fill_replay(ff: &mut libc::ff_effect, duration: u16, delay: u16) {
    ff.replay.length = duration;
    ff.replay.delay = delay;
}

fn fill_envelope(dst: &mut libc::ff_envelope, src: &Envelope) {
    dst.attack_length = src.attack_length;
    dst.attack_level = src.attack_level;
    dst.fade_length = src.fade_length;
    dst.fade_level = src.fade_level;
}

fn rumble_to_ff(r: &RumbleEffect) -> libc::ff_effect {
    let mut ff: libc::ff_effect = unsafe { std::mem::zeroed() };
    ff.type_ = FF_RUMBLE;

    unsafe {
        let rumble = ff.u.as_mut_ptr().cast::<libc::ff_rumble_effect>();
        (*rumble).strong_magnitude = r.strong_magnitude;
        (*rumble).weak_magnitude = r.weak_magnitude;
    }

    fill_replay(&mut ff, r.duration, r.delay);
    ff.id = -1;
    ff.direction = r.direction;

    ff
}

fn periodic_to_ff(p: &PeriodicEffect) -> libc::ff_effect {
    let mut ff: libc::ff_effect = unsafe { std::mem::zeroed() };
    ff.type_ = FF_PERIODIC;

    unsafe {
        let per = ff.u.as_mut_ptr().cast::<libc::ff_periodic_effect>();
        (*per).waveform = match p.waveform {
            PeriodicWaveform::Square => FF_SQUARE,
            PeriodicWaveform::Triangle => FF_TRIANGLE,
            PeriodicWaveform::Sine => FF_SINE,
            PeriodicWaveform::SawUp => FF_SAW_UP,
            PeriodicWaveform::SawDown => FF_SAW_DOWN,
            PeriodicWaveform::Custom => FF_CUSTOM,
        };

        (*per).period = p.period;
        (*per).magnitude = p.magnitude;
        (*per).offset = p.offset;
        (*per).phase = p.phase;

        fill_envelope(&mut (*per).envelope, &p.envelope);
    }

    fill_replay(&mut ff, p.duration, p.delay);
    ff.id = -1;
    ff.direction = p.direction;

    ff
}

fn constant_to_ff(c: &ConstantEffect) -> libc::ff_effect {
    let mut ff: libc::ff_effect = unsafe { std::mem::zeroed() };
    ff.type_ = FF_CONSTANT;

    unsafe {
        let ce = ff.u.as_mut_ptr().cast::<libc::ff_constant_effect>();
        (*ce).level = c.level;
        fill_envelope(&mut (*ce).envelope, &c.envelope);
    }

    fill_replay(&mut ff, c.duration, c.delay);
    ff.id = -1;
    ff.direction = c.direction;

    ff
}

fn ramp_to_ff(r: &RampEffect) -> libc::ff_effect {
    let mut ff: libc::ff_effect = unsafe { std::mem::zeroed() };
    ff.type_ = FF_RAMP;

    unsafe {
        let re = ff.u.as_mut_ptr().cast::<libc::ff_ramp_effect>();
        (*re).start_level = r.start_level;
        (*re).end_level = r.end_level;
        fill_envelope(&mut (*re).envelope, &r.envelope);
    }

    fill_replay(&mut ff, r.duration, r.delay);
    ff.id = -1;
    ff.direction = r.direction;

    ff
}

fn condition_to_ff(c: &ConditionEffect, effect_type: u16) -> libc::ff_effect {
    let mut ff: libc::ff_effect = unsafe { std::mem::zeroed() };
    ff.type_ = effect_type;

    unsafe {
        // ff.u is a union; for condition effects it contains 2 axes
        let cond_ptr = ff.u.as_mut_ptr().cast::<[libc::ff_condition_effect; 2]>();

        for i in 0..2 {
            (*cond_ptr)[i].right_saturation = c.right_saturation;
            (*cond_ptr)[i].left_saturation = c.left_saturation;
            (*cond_ptr)[i].right_coeff = c.right_coeff;
            (*cond_ptr)[i].left_coeff = c.left_coeff;
            (*cond_ptr)[i].deadband = c.deadband;
            (*cond_ptr)[i].center = c.center;
        }
    }

    // Condition effects ignore replay length/delay
    ff.id = -1;
    ff.direction = 0;

    ff
}

fn effect_to_ff(effect: &Effect) -> ShakeResult<libc::ff_effect> {
    match effect {
        Effect::Rumble(r) => Ok(rumble_to_ff(r)),
        Effect::Periodic(p) => Ok(periodic_to_ff(p)),
        Effect::Constant(c) => Ok(constant_to_ff(c)),
        Effect::Ramp(r) => Ok(ramp_to_ff(r)),
        Effect::Spring(c) => Ok(condition_to_ff(c, FF_SPRING)),
        Effect::Friction(c) => Ok(condition_to_ff(c, FF_FRICTION)),
        Effect::Damper(c) => Ok(condition_to_ff(c, FF_DAMPER)),
        Effect::Inertia(c) => Ok(condition_to_ff(c, FF_INERTIA)),
    }
}

// Effect operations
pub fn upload_effect(fd: &File, effect: &Effect) -> ShakeResult<i32> {
    let raw_fd = fd.as_raw_fd();
    let mut ff = effect_to_ff(effect)?;

    unsafe {
        eviocsff(raw_fd, &mut ff as *mut libc::ff_effect).map_err(|e| match e {
            nix::Error::EACCES | nix::Error::EPERM => ShakeError::Permission,
            _ => ShakeError::Io,
        })?;
    }

    Ok(ff.id as i32)
}

pub fn erase_effect(fd: &File, id: i32) -> ShakeResult<()> {
    let raw_fd = fd.as_raw_fd();

    unsafe {
        eviocrmff(raw_fd, id as u64).map_err(|e| match e {
            nix::Error::EACCES | nix::Error::EPERM => ShakeError::Permission,
            _ => ShakeError::Io,
        })?;
    }

    Ok(())
}

fn send_ff_event(fd: &File, code: u16, value: i32) -> ShakeResult<()> {
    let raw_fd = fd.as_raw_fd();

    let mut ev: libc::input_event = unsafe { std::mem::zeroed() };
    ev.type_ = EV_FF;
    ev.code = code;
    ev.value = value;

    let bytes = unsafe {
        std::slice::from_raw_parts(
            &ev as *const libc::input_event as *const u8,
            std::mem::size_of::<libc::input_event>(),
        )
    };

    let borrowed = unsafe { BorrowedFd::borrow_raw(raw_fd) };
    unistd::write(borrowed, bytes).map_err(|e| match e {
        nix::Error::EACCES | nix::Error::EPERM => ShakeError::Permission,
        _ => ShakeError::Io,
    })?;
    Ok(())
}

pub fn play_effect(fd: &File, id: i32) -> ShakeResult<()> {
    send_ff_event(fd, id as u16, 1)
}

pub fn stop_effect(fd: &File, id: i32) -> ShakeResult<()> {
    send_ff_event(fd, id as u16, 0)
}

pub fn set_gain(fd: &File, gain: u16) -> ShakeResult<()> {
    send_ff_event(fd, FF_GAIN, gain as i32)
}

pub fn set_autocenter(fd: &File, value: u16) -> ShakeResult<()> {
    send_ff_event(fd, FF_AUTOCENTER, value as i32)
}

pub struct LinuxBackend;

impl Backend for LinuxBackend {
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
            id: 0, // Device::enumerate() will overwrite this
            name: raw.name,
            capacity: raw.capacity,
            features: raw.features,
            path: PathBuf::new(), // Device::enumerate() overwrites this too
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
    use std::ffi::OsString;
    use std::fs::File;
    use std::os::unix::ffi::OsStringExt;
    use tempfile::tempdir;

    use crate::effect::{Envelope, PeriodicWaveform};

    #[test]
    fn scan_event_nodes_in_finds_event_files() {
        let dir = tempdir().unwrap();
        let path = dir.path();

        File::create(path.join("event0")).unwrap();
        File::create(path.join("event1")).unwrap();
        File::create(path.join("not_event")).unwrap();

        let nodes = scan_event_nodes_in(path).unwrap();

        assert_eq!(nodes.len(), 2);
        assert!(nodes.iter().any(|p| p.ends_with("event0")));
        assert!(nodes.iter().any(|p| p.ends_with("event1")));
    }

    #[test]
    fn scan_event_nodes_in_returns_error_when_empty() {
        let dir = tempdir().unwrap();
        let path = dir.path();

        let result = scan_event_nodes_in(path);
        assert!(result.is_err());
    }

    #[test]
    fn scan_event_nodes_in_ignores_non_utf8_filenames() {
        let dir = tempdir().unwrap();
        let path = dir.path();

        let bad_name = OsString::from_vec(vec![0xFF, 0xFE, 0xFD]);
        File::create(path.join(bad_name)).unwrap();

        let result = scan_event_nodes_in(path);
        assert!(result.is_err());
    }

    fn dummy_envelope() -> Envelope {
        Envelope {
            attack_length: 10,
            attack_level: 20,
            fade_length: 30,
            fade_level: 40,
        }
    }

    #[test]
    fn rumble_to_ff_converts_correctly() {
        let r = RumbleEffect {
            strong_magnitude: 1234,
            weak_magnitude: 5678,
            duration: 1000,
            delay: 50,
            direction: 0,
        };

        let ff = rumble_to_ff(&r);

        assert_eq!(ff.type_, FF_RUMBLE);
        unsafe {
            let rumble =
                ff.u.as_ptr()
                    .cast::<libc::ff_rumble_effect>()
                    .as_ref()
                    .unwrap();
            assert_eq!(rumble.strong_magnitude, 1234);
            assert_eq!(rumble.weak_magnitude, 5678);
        }
        assert_eq!(ff.replay.length, 1000);
        assert_eq!(ff.replay.delay, 50);
        assert_eq!(ff.direction, 0);
        assert_eq!(ff.id, -1);
    }

    #[test]
    fn periodic_to_ff_converts_correctly() {
        let p = PeriodicEffect {
            waveform: PeriodicWaveform::Sine,
            period: 200,
            magnitude: 3000,
            offset: -100,
            phase: 90,
            envelope: dummy_envelope(),
            duration: 500,
            delay: 10,
            direction: 0,
        };

        let ff = periodic_to_ff(&p);

        assert_eq!(ff.type_, FF_PERIODIC);
        unsafe {
            let per =
                ff.u.as_ptr()
                    .cast::<libc::ff_periodic_effect>()
                    .as_ref()
                    .unwrap();
            assert_eq!(per.waveform, FF_SINE);
            assert_eq!(per.period, 200);
            assert_eq!(per.magnitude, 3000);
            assert_eq!(per.offset, -100);
            assert_eq!(per.phase, 90);

            assert_eq!(per.envelope.attack_length, 10);
            assert_eq!(per.envelope.attack_level, 20);
            assert_eq!(per.envelope.fade_length, 30);
            assert_eq!(per.envelope.fade_level, 40);
        }
        assert_eq!(ff.replay.length, 500);
        assert_eq!(ff.replay.delay, 10);
    }

    #[test]
    fn constant_to_ff_converts_correctly() {
        let c = ConstantEffect {
            level: -2000,
            envelope: dummy_envelope(),
            duration: 700,
            delay: 20,
            direction: 0,
        };

        let ff = constant_to_ff(&c);

        assert_eq!(ff.type_, FF_CONSTANT);
        unsafe {
            let ce =
                ff.u.as_ptr()
                    .cast::<libc::ff_constant_effect>()
                    .as_ref()
                    .unwrap();
            assert_eq!(ce.level, -2000);
            assert_eq!(ce.envelope.attack_length, 10);
            assert_eq!(ce.envelope.attack_level, 20);
            assert_eq!(ce.envelope.fade_length, 30);
            assert_eq!(ce.envelope.fade_level, 40);
        }
        assert_eq!(ff.replay.length, 700);
        assert_eq!(ff.replay.delay, 20);
    }

    #[test]
    fn ramp_to_ff_converts_correctly() {
        let r = RampEffect {
            start_level: -5000,
            end_level: 5000,
            envelope: dummy_envelope(),
            duration: 900,
            delay: 30,
            direction: 0,
        };

        let ff = ramp_to_ff(&r);

        assert_eq!(ff.type_, FF_RAMP);
        unsafe {
            let re =
                ff.u.as_ptr()
                    .cast::<libc::ff_ramp_effect>()
                    .as_ref()
                    .unwrap();
            assert_eq!(re.start_level, -5000);
            assert_eq!(re.end_level, 5000);
            assert_eq!(re.envelope.attack_length, 10);
            assert_eq!(re.envelope.attack_level, 20);
            assert_eq!(re.envelope.fade_length, 30);
            assert_eq!(re.envelope.fade_level, 40);
        }
        assert_eq!(ff.replay.length, 900);
        assert_eq!(ff.replay.delay, 30);
    }

    #[test]
    fn effect_to_ff_dispatches_correctly() {
        let rumble = Effect::Rumble(RumbleEffect {
            strong_magnitude: 1,
            weak_magnitude: 2,
            duration: 3,
            delay: 4,
            direction: 0,
        });

        let ff = effect_to_ff(&rumble).unwrap();
        assert_eq!(ff.type_, FF_RUMBLE);

        let periodic = Effect::Periodic(PeriodicEffect {
            waveform: PeriodicWaveform::Triangle,
            period: 10,
            magnitude: 20,
            offset: 30,
            phase: 40,
            envelope: dummy_envelope(),
            duration: 50,
            delay: 60,
            direction: 0,
        });

        let ff = effect_to_ff(&periodic).unwrap();
        assert_eq!(ff.type_, FF_PERIODIC);

        let constant = Effect::Constant(ConstantEffect {
            level: 100,
            envelope: dummy_envelope(),
            duration: 200,
            delay: 300,
            direction: 0,
        });

        let ff = effect_to_ff(&constant).unwrap();
        assert_eq!(ff.type_, FF_CONSTANT);

        let ramp = Effect::Ramp(RampEffect {
            start_level: -1,
            end_level: 1,
            envelope: dummy_envelope(),
            duration: 10,
            delay: 20,
            direction: 0,
        });

        let ff = effect_to_ff(&ramp).unwrap();
        assert_eq!(ff.type_, FF_RAMP);
    }
}
