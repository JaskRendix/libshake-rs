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
        let dev = Device::open_info(info);

        // macOS returns Support, Linux returns Ok(Device)
        match dev {
            Ok(d) => {
                assert_eq!(d.id(), info.id);
                assert_eq!(d.name(), info.name);
                assert_eq!(d.capacity(), info.capacity);
            }
            Err(ShakeError::Support) => {} // expected on macOS
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }
}
