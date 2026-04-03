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
        assert_eq!(dev.capacity(), info.capacity);
        assert_eq!(dev.features(), info.features.as_slice());
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

        // These should never panic regardless of device support
        let _ = dev.supports_rumble();
        let _ = dev.supports_periodic();
    }
}

#[cfg(feature = "mock-backend")]
mod mock_tests {
    use super::*;
    use shake::simple::*;

    #[test]
    fn mock_upload_play_stop_erase_cycle() {
        let list = Device::enumerate().unwrap();
        let info = list
            .first()
            .expect("mock backend should provide at least one device");

        let dev = Device::open_info(info).unwrap();

        let effect = simple_rumble(1.0, 0.5, 0.2);
        let id = dev.upload(&effect).expect("upload failed");

        dev.play(id).expect("play failed");
        dev.stop(id).expect("stop failed");
        dev.erase(id).expect("erase failed");
    }

    #[test]
    fn mock_rumble_convenience_method_works() {
        let list = Device::enumerate().unwrap();
        let info = list.first().unwrap();
        let dev = Device::open_info(info).unwrap();

        let id = dev.rumble(1.0, 0.5, 0.1).expect("rumble failed");
        assert!(id >= 0);
    }

    #[test]
    fn mock_open_by_path_works() {
        let list = Device::enumerate().unwrap();
        let info = list.first().unwrap();

        let dev = Device::open_info(info).unwrap();
        let dev2 = Device::open_info(info).unwrap();

        assert_eq!(dev.name(), dev2.name());
    }
}
