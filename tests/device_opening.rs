use shake::device::Device;
use shake::error::ShakeError;

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
    }
}
