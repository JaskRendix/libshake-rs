use shake::effect::*;
use shake::simple::*;

#[test]
fn effect_type_matches_condition_variants() {
    assert_eq!(simple_spring(0.8, 0.1).effect_type(), EffectType::Spring);
    assert_eq!(simple_friction(0.6).effect_type(), EffectType::Friction);
    assert_eq!(simple_damper(0.7).effect_type(), EffectType::Damper);
    assert_eq!(simple_inertia(0.5).effect_type(), EffectType::Inertia);
}

#[cfg(feature = "linux-backend")]
mod linux_backend_tests {
    use super::*;
    use shake::device::Device;

    #[test]
    fn condition_effect_upload_succeeds() {
        let list = Device::enumerate().unwrap();
        let info = match list.first() {
            Some(i) => i,
            None => return,
        };
        let dev = Device::open_info(info).unwrap();

        let effects = [
            simple_spring(0.8, 0.1),
            simple_friction(0.6),
            simple_damper(0.7),
            simple_inertia(0.5),
        ];

        for e in effects {
            let handle = dev.upload(&e).unwrap();
            assert!(handle.id() >= 0);
        }
    }

    #[test]
    fn condition_effect_parameters_are_nonzero() {
        let e = simple_spring(0.8, 0.1);

        match e {
            Effect::Spring(c) => {
                assert!(c.right_saturation > 0);
                assert!(c.left_saturation > 0);
                assert!(c.right_coeff > 0);
                assert!(c.left_coeff > 0);
            }
            _ => panic!("Expected Spring effect"),
        }
    }
}

#[cfg(feature = "mock-backend")]
mod mock_backend_tests {
    use super::*;
    use shake::device::Device;

    #[test]
    fn mock_condition_effect_upload_and_play() {
        let list = Device::enumerate().unwrap();
        let info = list.first().unwrap();
        let dev = Device::open_info(info).unwrap();

        let e = simple_friction(0.6);
        let handle = dev.upload(&e).unwrap();

        handle.play().unwrap();
        handle.stop().unwrap();
    }

    #[test]
    fn mock_condition_effect_roundtrip_reuses_id() {
        let list = Device::enumerate().unwrap();
        let info = list.first().unwrap();
        let dev = Device::open_info(info).unwrap();

        let e = simple_spring(0.8, 0.1);

        let h1 = dev.upload(&e).unwrap();
        let id1 = h1.id();
        drop(h1);

        let h2 = dev.upload(&e).unwrap();
        let id2 = h2.id();

        assert_eq!(id1, id2);
    }

    #[test]
    fn mock_condition_effects_have_valid_parameters() {
        let e = simple_damper(0.7);

        match e {
            Effect::Damper(c) => {
                assert!(c.right_saturation > 0);
                assert!(c.left_saturation > 0);
                assert!(c.right_coeff > 0);
                assert!(c.left_coeff > 0);
            }
            _ => panic!("Expected Damper effect"),
        }
    }
}

#[test]
fn device_supports_condition_effects_queries_do_not_panic() {
    let list = shake::device::Device::enumerate().unwrap();

    if let Some(info) = list.first() {
        let dev = shake::device::Device::open_info(info).unwrap();

        let _ = dev.capabilities().spring;
        let _ = dev.capabilities().friction;
        let _ = dev.capabilities().damper;
        let _ = dev.capabilities().inertia;
    }
}
