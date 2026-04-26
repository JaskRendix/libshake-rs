//! Tests for the Backend trait using a minimal in-test fake backend.
//! This ensures the tests run under plain `cargo test` with no features.

use shake::backend::{Backend, RawDeviceInfo};
use shake::effect::*;
use shake::error::{ShakeError, ShakeResult};

use std::path::{Path, PathBuf};

pub struct FakeHandle {
    effects: std::sync::Mutex<Vec<Option<Effect>>>,
}

pub struct FakeBackend;

impl Backend for FakeBackend {
    type Handle = FakeHandle;

    fn scan() -> ShakeResult<Vec<PathBuf>> {
        Ok(vec![PathBuf::from("/dev/fake0")])
    }

    fn open(_path: &Path) -> ShakeResult<Self::Handle> {
        Ok(FakeHandle {
            effects: std::sync::Mutex::new(Vec::new()),
        })
    }

    fn close(_handle: Self::Handle) {
        // no-op
    }

    fn query(_handle: &Self::Handle) -> ShakeResult<RawDeviceInfo> {
        Ok(RawDeviceInfo {
            name: "Fake Device".into(),
            capacity: 16,
            features: vec![u64::MAX],
        })
    }

    fn upload(handle: &Self::Handle, effect: &Effect) -> ShakeResult<i32> {
        let mut lock = handle.effects.lock().unwrap();
        lock.push(Some(effect.clone()));
        Ok((lock.len() - 1) as i32)
    }

    fn update(handle: &Self::Handle, id: i32, effect: &Effect) -> ShakeResult<()> {
        let mut lock = handle.effects.lock().unwrap();
        if let Some(slot) = lock.get_mut(id as usize) {
            *slot = Some(effect.clone());
            Ok(())
        } else {
            Err(ShakeError::Effect)
        }
    }

    fn play(handle: &Self::Handle, id: i32) -> ShakeResult<()> {
        let lock = handle.effects.lock().unwrap();
        lock.get(id as usize)
            .and_then(|e| e.as_ref())
            .ok_or(ShakeError::Effect)?;
        Ok(())
    }

    fn stop(_handle: &Self::Handle, _id: i32) -> ShakeResult<()> {
        Ok(())
    }

    fn erase(handle: &Self::Handle, id: i32) -> ShakeResult<()> {
        let mut lock = handle.effects.lock().unwrap();
        if let Some(slot) = lock.get_mut(id as usize) {
            *slot = None;
            Ok(())
        } else {
            Err(ShakeError::Effect)
        }
    }

    fn set_gain(_handle: &Self::Handle, _value: u16) -> ShakeResult<()> {
        Ok(())
    }

    fn set_autocenter(_handle: &Self::Handle, _value: u16) -> ShakeResult<()> {
        Ok(())
    }
}

fn open_first() -> ShakeResult<<FakeBackend as Backend>::Handle> {
    let paths = FakeBackend::scan()?;
    let path = paths.into_iter().next().unwrap();
    FakeBackend::open(&path)
}

#[test]
fn backend_scan_returns_paths() {
    let paths = FakeBackend::scan().unwrap();
    assert_eq!(paths.len(), 1);
    assert!(paths[0].is_absolute());
}

#[test]
fn backend_open_and_query_are_consistent() {
    let handle = open_first().unwrap();
    let info = FakeBackend::query(&handle).unwrap();

    assert_eq!(info.name, "Fake Device");
    assert_eq!(info.capacity, 16);
}

#[test]
fn backend_upload_play_stop_erase_roundtrip() {
    let handle = open_first().unwrap();

    let effect = Effect::Rumble(RumbleEffect {
        strong_magnitude: 1000,
        weak_magnitude: 500,
        duration: 200,
        delay: 0,
        direction: 0,
    });

    let id = FakeBackend::upload(&handle, &effect).unwrap();
    FakeBackend::play(&handle, id).unwrap();
    FakeBackend::stop(&handle, id).unwrap();
    FakeBackend::erase(&handle, id).unwrap();
}

#[test]
fn backend_erased_effect_cannot_be_played() {
    let handle = open_first().unwrap();

    let e = Effect::Constant(ConstantEffect {
        level: 100,
        envelope: Envelope::new(0, 0, 0, 0),
        duration: 50,
        delay: 0,
        direction: 0,
    });

    let id = FakeBackend::upload(&handle, &e).unwrap();
    FakeBackend::erase(&handle, id).unwrap();

    let result = FakeBackend::play(&handle, id);
    assert!(result.is_err());
}

#[test]
fn backend_gain_and_autocenter_accept_valid_ranges() {
    let handle = open_first().unwrap();

    FakeBackend::set_gain(&handle, 50).unwrap();
    FakeBackend::set_autocenter(&handle, 75).unwrap();
}
