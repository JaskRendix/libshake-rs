use shake::effect::*;
use shake::simple::*;

// --- Magnitude and level boundaries ----------------------------------------

#[test]
fn periodic_magnitude_respects_min_and_max() {
    let e = simple_periodic(PeriodicWaveform::Sine, 2.0, 0.0, 0.0, 0.0);
    match e {
        Effect::Periodic(p) => {
            assert!(p.magnitude <= PERIODIC_MAGNITUDE_MAX);
            assert!(p.magnitude >= PERIODIC_MAGNITUDE_MIN);
        }
        _ => panic!("Expected Periodic effect"),
    }
}

#[test]
fn constant_level_respects_min_and_max() {
    let e = simple_constant(-2.0, 0.0, 0.0, 0.0);
    match e {
        Effect::Constant(c) => {
            assert!(c.level <= CONSTANT_LEVEL_MAX);
            assert!(c.level >= CONSTANT_LEVEL_MIN);
        }
        _ => panic!("Expected Constant effect"),
    }
}

#[test]
fn ramp_levels_respect_boundaries() {
    let e = simple_ramp(-2.0, 2.0, 0.0, 0.0, 0.0);
    match e {
        Effect::Ramp(r) => {
            assert!(r.start_level >= RAMP_START_LEVEL_MIN);
            assert!(r.start_level <= RAMP_START_LEVEL_MAX);
            assert!(r.end_level >= RAMP_END_LEVEL_MIN);
            assert!(r.end_level <= RAMP_END_LEVEL_MAX);
        }
        _ => panic!("Expected Ramp effect"),
    }
}

// --- Duration and envelope edge cases ---------------------------------------

#[test]
fn zero_duration_effects_do_not_panic() {
    let e = simple_rumble(1.0, 1.0, 0.0);
    match e {
        Effect::Rumble(r) => assert_eq!(r.duration, 0),
        _ => panic!("Expected Rumble effect"),
    }
}

#[test]
fn envelope_zero_lengths_are_valid() {
    let env = Envelope::new(0, 0, 0, 0);
    assert_eq!(env.attack_length, 0);
    assert_eq!(env.fade_length, 0);
}

#[test]
fn envelope_max_lengths_are_valid() {
    let env = Envelope::new(
        ENVELOPE_ATTACK_LENGTH_MAX,
        ENVELOPE_ATTACK_LEVEL_MAX,
        ENVELOPE_FADE_LENGTH_MAX,
        ENVELOPE_FADE_LEVEL_MAX,
    );

    assert_eq!(env.attack_length, ENVELOPE_ATTACK_LENGTH_MAX);
    assert_eq!(env.fade_length, ENVELOPE_FADE_LENGTH_MAX);
}

// --- Float scaling edge cases ----------------------------------------------

#[test]
fn negative_force_is_clamped_or_preserved() {
    let e = simple_constant(-1.0, 0.0, 0.0, 0.0);
    match e {
        Effect::Constant(c) => {
            assert!(c.level <= 0);
            assert!(c.level >= CONSTANT_LEVEL_MIN);
        }
        _ => panic!("Expected Constant effect"),
    }
}

#[test]
fn extremely_large_force_does_not_overflow() {
    let e = simple_rumble(1000.0, 1000.0, 1.0);
    match e {
        Effect::Rumble(r) => {
            assert!(r.strong_magnitude <= RUMBLE_STRONG_MAGNITUDE_MAX);
            assert!(r.weak_magnitude <= RUMBLE_WEAK_MAGNITUDE_MAX);
        }
        _ => panic!("Expected Rumble effect"),
    }
}

// --- Repeated construction stability ----------------------------------------

#[test]
fn repeated_effect_construction_is_stable() {
    for _ in 0..1000 {
        let e = simple_periodic(PeriodicWaveform::Sine, 0.5, 0.1, 0.1, 0.1);
        match e {
            Effect::Periodic(p) => {
                assert!(p.magnitude <= PERIODIC_MAGNITUDE_MAX);
                assert!(p.magnitude >= PERIODIC_MAGNITUDE_MIN);
            }
            _ => panic!("Expected Periodic effect"),
        }
    }
}
