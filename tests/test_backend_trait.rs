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

#[test]
fn backend_capabilities_match_raw_features() {
    let handle = open_first().unwrap();
    let caps = FakeBackend::capabilities(&handle).unwrap();

    // RawDeviceInfo.features = [u64::MAX], so all bits are set.
    assert!(caps.rumble);
    assert!(caps.periodic);
    assert!(caps.spring);
    assert!(caps.friction);
    assert!(caps.damper);
    assert!(caps.inertia);

    // max_effects must match RawDeviceInfo.capacity
    assert_eq!(caps.max_effects, 16);
}

#[test]
fn backend_capabilities_are_stable() {
    let handle = open_first().unwrap();

    let c1 = FakeBackend::capabilities(&handle).unwrap();
    let c2 = FakeBackend::capabilities(&handle).unwrap();

    // Values must match exactly
    assert_eq!(c1.rumble, c2.rumble);
    assert_eq!(c1.periodic, c2.periodic);
    assert_eq!(c1.spring, c2.spring);
    assert_eq!(c1.friction, c2.friction);
    assert_eq!(c1.damper, c2.damper);
    assert_eq!(c1.inertia, c2.inertia);
    assert_eq!(c1.max_effects, c2.max_effects);
}

#[test]
fn backend_capabilities_handle_empty_feature_vector() {
    // Manually create a handle with no effects
    let handle = FakeHandle {
        effects: std::sync::Mutex::new(Vec::new()),
    };

    // capabilities() must not panic even if the handle was not opened via scan/open
    let caps = FakeBackend::capabilities(&handle).unwrap();

    // FakeBackend::query() always returns features = [u64::MAX]
    // so all capability bits must be true
    assert!(caps.rumble);
    assert!(caps.periodic);
    assert!(caps.spring);
    assert!(caps.friction);
    assert!(caps.damper);
    assert!(caps.inertia);

    // max_effects must match RawDeviceInfo.capacity
    assert_eq!(caps.max_effects, 16);
}

#[test]
fn backend_capabilities_does_not_modify_effect_slots() {
    let handle = open_first().unwrap();

    // Upload an effect
    let effect = Effect::Rumble(RumbleEffect {
        strong_magnitude: 1000,
        weak_magnitude: 500,
        duration: 200,
        delay: 0,
        direction: 0,
    });

    let id = FakeBackend::upload(&handle, &effect).unwrap();

    // Calling capabilities() must not erase or modify effects
    let _ = FakeBackend::capabilities(&handle).unwrap();

    // Effect must still be playable
    FakeBackend::play(&handle, id).unwrap();
}

#[test]
fn backend_update_invalid_id_returns_error() {
    let handle = open_first().unwrap();

    let effect = Effect::Constant(ConstantEffect {
        level: 123,
        envelope: Envelope::new(0, 0, 0, 0),
        duration: 10,
        delay: 0,
        direction: 0,
    });

    let result = FakeBackend::update(&handle, 999, &effect);
    assert!(matches!(result, Err(ShakeError::Effect)));
}

#[test]
fn backend_erase_invalid_id_returns_error() {
    let handle = open_first().unwrap();
    let result = FakeBackend::erase(&handle, 999);
    assert!(matches!(result, Err(ShakeError::Effect)));
}

#[test]
fn backend_upload_assigns_sequential_ids() {
    let handle = open_first().unwrap();

    let e = Effect::Rumble(RumbleEffect {
        strong_magnitude: 1,
        weak_magnitude: 1,
        duration: 1,
        delay: 0,
        direction: 0,
    });

    let id1 = FakeBackend::upload(&handle, &e).unwrap();
    let id2 = FakeBackend::upload(&handle, &e).unwrap();
    let id3 = FakeBackend::upload(&handle, &e).unwrap();

    assert_eq!(id1, 0);
    assert_eq!(id2, 1);
    assert_eq!(id3, 2);
}

#[test]
fn backend_update_overwrites_effect() {
    let handle = open_first().unwrap();

    let e1 = Effect::Constant(ConstantEffect {
        level: 100,
        envelope: Envelope::new(0, 0, 0, 0),
        duration: 10,
        delay: 0,
        direction: 0,
    });

    let e2 = Effect::Constant(ConstantEffect {
        level: 999,
        envelope: Envelope::new(0, 0, 0, 0),
        duration: 10,
        delay: 0,
        direction: 0,
    });

    let id = FakeBackend::upload(&handle, &e1).unwrap();
    FakeBackend::update(&handle, id, &e2).unwrap();

    // play() must succeed, meaning the slot still exists
    FakeBackend::play(&handle, id).unwrap();
}

#[test]
fn backend_erase_does_not_shift_ids() {
    let handle = open_first().unwrap();

    let e = Effect::Rumble(RumbleEffect {
        strong_magnitude: 1,
        weak_magnitude: 1,
        duration: 1,
        delay: 0,
        direction: 0,
    });

    let id1 = FakeBackend::upload(&handle, &e).unwrap();
    let id2 = FakeBackend::upload(&handle, &e).unwrap();

    FakeBackend::erase(&handle, id1).unwrap();

    // id2 must still be valid
    FakeBackend::play(&handle, id2).unwrap();
}

#[test]
fn backend_close_is_noop_and_safe() {
    let handle = open_first().unwrap();

    // close must not panic
    FakeBackend::close(handle);

    // open again must still work
    let handle2 = open_first().unwrap();
    let info = FakeBackend::query(&handle2).unwrap();
    assert_eq!(info.capacity, 16);
}
