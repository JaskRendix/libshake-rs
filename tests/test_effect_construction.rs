use shake::effect::*;
use shake::simple::*;

#[test]
fn rumble_effect_values_are_scaled_correctly() {
    let e = simple_rumble(1.0, 0.5, 1.0);

    match e {
        Effect::Rumble(r) => {
            assert_eq!(r.strong_magnitude, 0x7FFF);
            assert_eq!(r.weak_magnitude, 0x3FFF);
            assert_eq!(r.duration, 1000);
            assert_eq!(r.delay, 0);
        }
        _ => panic!("Expected Rumble effect"),
    }
}

#[test]
fn periodic_effect_has_correct_envelope() {
    let e = simple_periodic(PeriodicWaveform::Sine, 0.5, 0.1, 0.2, 0.3);

    match e {
        Effect::Periodic(p) => {
            assert_eq!(p.magnitude, (0.5 * 0x7FFF as f32) as i16);
            assert_eq!(p.envelope.attack_length, 100);
            assert_eq!(p.envelope.fade_length, 300);
            assert_eq!(p.duration, 600);
        }
        _ => panic!("Expected Periodic effect"),
    }
}

#[test]
fn constant_effect_respects_force_range() {
    let e = simple_constant(1.0, 0.0, 0.0, 0.0);

    match e {
        Effect::Constant(c) => {
            assert_eq!(c.level, 0x7FFF);
        }
        _ => panic!("Expected Constant effect"),
    }
}

#[test]
fn ramp_effect_start_and_end_levels_are_scaled() {
    let e = simple_ramp(-1.0, 1.0, 0.0, 0.0, 0.0);

    match e {
        Effect::Ramp(r) => {
            assert_eq!(r.start_level, -0x8000);
            assert_eq!(r.end_level, 0x7FFF);
        }
        _ => panic!("Expected Ramp effect"),
    }
}

#[test]
fn effect_type_matches_variant() {
    let e = simple_rumble(1.0, 1.0, 1.0);
    assert_eq!(e.effect_type(), EffectType::Rumble);
}

#[test]
fn simple_periodic_with_period_sets_period() {
    let e = simple_periodic_with_period(PeriodicWaveform::Sine, 0.5, 0.1, 0.1, 0.1, 250);
    match e {
        Effect::Periodic(p) => assert_eq!(p.period, 250),
        _ => panic!("Expected Periodic effect"),
    }
}
