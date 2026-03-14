use shake::device::Device;

#[test]
fn enumerate_returns_ok() {
    let result = Device::enumerate();
    assert!(result.is_ok());
}

#[test]
fn enumerate_returns_empty_on_macos() {
    #[cfg(target_os = "macos")]
    {
        let list = Device::enumerate().unwrap();
        assert!(list.is_empty());
    }
}

#[test]
fn enumerate_never_panics_or_errors() {
    for _ in 0..5 {
        let result = Device::enumerate();
        assert!(result.is_ok());
    }
}
