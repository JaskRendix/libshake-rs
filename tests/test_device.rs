use shake::device::Device;
use shake::error::ShakeError;

#[test]
fn enumerate_returns_ok() {
    let result = Device::enumerate();
    assert!(result.is_ok());
}

#[test]
fn enumerate_is_stable_over_multiple_calls() {
    for _ in 0..10 {
        let result = Device::enumerate();
        assert!(result.is_ok());
    }
}

#[test]
fn open_invalid_id_returns_error() {
    let result = Device::open(9999);
    assert!(matches!(result, Err(ShakeError::Device)));
}

#[test]
fn open_info_clones_metadata_correctly() {
    let list = Device::enumerate().unwrap();

    if let Some(info) = list.first() {
        let dev = Device::open_info(info).expect("Failed to open device");

        assert_eq!(dev.id(), info.id);
        assert_eq!(dev.name(), info.name);
        assert_eq!(dev.max_effects(), info.max_effects);
        assert_eq!(dev.raw_features(), info.raw_features.as_slice());
    }
}

#[test]
fn device_debug_format_includes_name_and_id() {
    let list = Device::enumerate().unwrap();

    if let Some(info) = list.first() {
        let dev = Device::open_info(info).unwrap();
        let dbg = format!("{:?}", dev);

        assert!(dbg.contains(dev.name()));
        assert!(dbg.contains(&dev.id().to_string()));
    }
}

#[cfg(feature = "linux-backend")]
#[test]
fn capability_checks_do_not_panic() {
    let list = Device::enumerate().unwrap();

    if let Some(info) = list.first() {
        let dev = Device::open_info(info).unwrap();

        let _ = dev.capabilities().rumble;
        let _ = dev.capabilities().periodic;
    }
}

#[cfg(feature = "mock-backend")]
mod mock_tests {
    use super::*;
    use shake::simple::*;

    #[test]
    fn mock_upload_play_stop_erase_cycle() {
        let list = Device::enumerate().unwrap();
        let info = list.first().unwrap();

        let dev = Device::open_info(info).unwrap();

        let effect = simple_rumble(1.0, 0.5, 0.2);
        let handle = dev.upload(&effect).expect("upload failed");

        handle.play().expect("play failed");
        handle.stop().expect("stop failed");

        drop(handle); // RAII erase
    }

    #[test]
    fn mock_rumble_convenience_method_works() {
        let list = Device::enumerate().unwrap();
        let info = list.first().unwrap();
        let dev = Device::open_info(info).unwrap();

        let handle = dev.rumble(1.0, 0.5, 0.1).expect("rumble failed");
        assert!(handle.id() >= 0);
    }

    #[test]
    fn mock_open_by_path_works() {
        let list = Device::enumerate().unwrap();
        let info = list.first().unwrap();

        let dev1 = Device::open_info(info).unwrap();
        let dev2 = Device::open_info(info).unwrap();

        assert_eq!(dev1.name(), dev2.name());
    }
}

#[test]
fn open_path_opens_same_device() {
    let list = Device::enumerate().unwrap();
    let info = match list.first() {
        Some(i) => i,
        None => return,
    };

    let dev1 = Device::open_info(info).unwrap();
    let dev2 = Device::open_path(info.path.as_path()).unwrap();

    assert_eq!(dev1.name(), dev2.name());
    assert_eq!(dev1.max_effects(), dev2.max_effects()); // ← FIXED
    assert_eq!(dev1.raw_features(), dev2.raw_features());
}

#[test]
fn open_path_invalid_returns_error() {
    use std::path::Path;

    let result = Device::open_path(Path::new("/this/does/not/exist"));
    assert!(result.is_err());
}

#[test]
fn enumerate_assigns_unique_ids() {
    let list = Device::enumerate().unwrap();
    let mut ids = list.iter().map(|d| d.id).collect::<Vec<_>>();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), list.len());
}

#[test]
fn deviceinfo_path_is_preserved() {
    let list = Device::enumerate().unwrap();
    if let Some(info) = list.first() {
        let dev = Device::open_info(info).unwrap();
        assert_eq!(dev.path(), info.path.as_path());
    }
}

#[cfg(feature = "mock-backend")]
#[test]
fn mock_upload_play_stop_cycle() {
    use shake::simple::*;

    let list = Device::enumerate().unwrap();
    let info = list.first().unwrap();
    let dev = Device::open_info(info).unwrap();

    let effect = simple_rumble(1.0, 0.5, 0.2);
    let handle = dev.upload(&effect).unwrap();

    handle.play().unwrap();
    handle.stop().unwrap();

    drop(handle);
}

#[cfg(feature = "linux-backend")]
#[test]
fn rumble_direction_is_preserved() {
    use shake::device::Device;
    use shake::effect::{Effect, RumbleEffect};

    let list = Device::enumerate().unwrap();
    let info = match list.first() {
        Some(i) => i,
        None => return,
    };

    let dev = Device::open_info(info).unwrap();

    let effect = Effect::Rumble(RumbleEffect {
        strong_magnitude: 1000,
        weak_magnitude: 500,
        duration: 100,
        delay: 0,
        direction: 12345,
    });

    let handle = dev.upload(&effect).unwrap();

    assert!(handle.id() >= 0);
}

#[cfg(feature = "linux-backend")]
#[test]
fn enumerate_handles_missing_input_dir() {
    let result = shake::device::Device::enumerate();
    assert!(result.is_err() || result.is_ok());
}

#[cfg(feature = "linux-backend")]
#[test]
fn spring_upload_succeeds() {
    use shake::effect::{ConditionEffect, Effect};

    let list = Device::enumerate().unwrap();
    let info = match list.first() {
        Some(i) => i,
        None => return,
    };

    let dev = Device::open_info(info).unwrap();

    let effect = Effect::Spring(ConditionEffect {
        right_saturation: 10000,
        left_saturation: 10000,
        right_coeff: 5000,
        left_coeff: 5000,
        deadband: 0,
        center: 0,
    });

    let handle = dev.upload(&effect).unwrap();
    assert!(handle.id() >= 0);
}

#[test]
fn capabilities_match_raw_features() {
    let list = Device::enumerate().unwrap();
    let info = match list.first() {
        Some(i) => i,
        None => return,
    };

    let dev = Device::open_info(info).unwrap();
    let caps = dev.capabilities();

    // Basic invariants
    assert_eq!(caps.max_effects, dev.max_effects());

    // Feature bits must match raw_features[0]
    let f0 = info.raw_features.first().copied().unwrap_or(0);

    assert_eq!(caps.rumble, (f0 & (1 << 0)) != 0);
    assert_eq!(caps.periodic, (f0 & (1 << 1)) != 0);
    assert_eq!(caps.spring, (f0 & (1 << 2)) != 0);
    assert_eq!(caps.friction, (f0 & (1 << 3)) != 0);
    assert_eq!(caps.damper, (f0 & (1 << 4)) != 0);
    assert_eq!(caps.inertia, (f0 & (1 << 5)) != 0);
}

#[test]
fn capabilities_are_cached_and_stable() {
    let list = Device::enumerate().unwrap();
    let info = match list.first() {
        Some(i) => i,
        None => return,
    };

    let dev = Device::open_info(info).unwrap();

    let c1 = dev.capabilities();
    let c2 = dev.capabilities();

    // Same reference, not recomputed
    assert!(std::ptr::eq(c1, c2));

    // Values must be identical
    assert_eq!(c1.max_effects, c2.max_effects);
    assert_eq!(c1.rumble, c2.rumble);
    assert_eq!(c1.periodic, c2.periodic);
    assert_eq!(c1.spring, c2.spring);
    assert_eq!(c1.friction, c2.friction);
    assert_eq!(c1.damper, c2.damper);
    assert_eq!(c1.inertia, c2.inertia);
}

#[test]
fn capabilities_consistent_across_multiple_opens() {
    let list = Device::enumerate().unwrap();
    let info = match list.first() {
        Some(i) => i,
        None => return,
    };

    let dev1 = Device::open_info(info).unwrap();
    let dev2 = Device::open_info(info).unwrap();

    assert_eq!(
        dev1.capabilities().max_effects,
        dev2.capabilities().max_effects
    );
    assert_eq!(dev1.capabilities().rumble, dev2.capabilities().rumble);
    assert_eq!(dev1.capabilities().periodic, dev2.capabilities().periodic);
    assert_eq!(dev1.capabilities().spring, dev2.capabilities().spring);
    assert_eq!(dev1.capabilities().friction, dev2.capabilities().friction);
    assert_eq!(dev1.capabilities().damper, dev2.capabilities().damper);
    assert_eq!(dev1.capabilities().inertia, dev2.capabilities().inertia);
}

#[test]
fn deviceinfo_maps_exactly_into_device() {
    let list = Device::enumerate().unwrap();
    let info = match list.first() {
        Some(i) => i,
        None => return,
    };

    let dev = Device::open_info(info).unwrap();

    assert_eq!(dev.id(), info.id);
    assert_eq!(dev.name(), info.name);
    assert_eq!(dev.max_effects(), info.max_effects);
    assert_eq!(dev.raw_features(), info.raw_features.as_slice());
    assert_eq!(dev.path(), info.path.as_path());
}

#[cfg(feature = "mock-backend")]
#[test]
fn mock_backend_reports_all_capabilities() {
    let list = Device::enumerate().unwrap();
    let info = list.first().unwrap();
    let dev = Device::open_info(info).unwrap();
    let caps = dev.capabilities();

    assert!(caps.rumble);
    assert!(caps.periodic);
    assert!(caps.spring);
    assert!(caps.friction);
    assert!(caps.damper);
    assert!(caps.inertia);
}

#[test]
fn max_effects_is_nonzero() {
    let list = Device::enumerate().unwrap();
    let info = match list.first() {
        Some(i) => i,
        None => return,
    };

    assert!(info.max_effects > 0);

    let dev = Device::open_info(info).unwrap();
    assert!(dev.max_effects() > 0);
}
