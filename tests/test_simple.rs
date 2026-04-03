use shake::effect::*;
use shake::simple::*;

#[test]
fn simple_rumble_creates_rumble_effect() {
    let e = simple_rumble(0.2, 0.1, 0.5);
    match e {
        Effect::Rumble(_) => {}
        _ => panic!("Expected Rumble effect"),
    }
}

#[test]
fn simple_rumble_clamps_values() {
    let e = simple_rumble(2.0, -1.0, 1.0); // out of range
    match e {
        Effect::Rumble(r) => {
            assert_eq!(r.strong_magnitude, 0x7FFF);
            assert_eq!(r.weak_magnitude, 0); // clamped
        }
        _ => panic!("Expected Rumble effect"),
    }
}

#[test]
fn simple_rumble_sets_duration_correctly() {
    let e = simple_rumble(0.5, 0.5, 1.5);
    match e {
        Effect::Rumble(r) => assert_eq!(r.duration, 1500),
        _ => panic!("Expected Rumble effect"),
    }
}

#[test]
fn simple_periodic_uses_correct_waveform() {
    let e = simple_periodic(PeriodicWaveform::Triangle, 0.5, 0.1, 0.1, 0.1);
    match e {
        Effect::Periodic(p) => assert!(matches!(p.waveform, PeriodicWaveform::Triangle)),
        _ => panic!("Expected Periodic effect"),
    }
}

#[test]
fn simple_periodic_sets_envelope_correctly() {
    let e = simple_periodic(PeriodicWaveform::Sine, 0.5, 0.2, 0.3, 0.4);
    match e {
        Effect::Periodic(p) => {
            assert_eq!(p.envelope.attack_length, 200);
            assert_eq!(p.envelope.fade_length, 400);
            assert_eq!(p.duration, 900);
        }
        _ => panic!("Expected Periodic effect"),
    }
}

#[test]
fn simple_periodic_clamps_magnitude() {
    let e = simple_periodic(PeriodicWaveform::Sine, 2.0, 0.1, 0.1, 0.1);
    match e {
        Effect::Periodic(p) => assert_eq!(p.magnitude, 0x7FFF),
        _ => panic!("Expected Periodic effect"),
    }
}

#[test]
fn simple_constant_creates_constant_effect() {
    let e = simple_constant(0.5, 0.1, 0.1, 0.1);
    match e {
        Effect::Constant(_) => {}
        _ => panic!("Expected Constant effect"),
    }
}

#[test]
fn simple_constant_clamps_level() {
    let e = simple_constant(2.0, 0.1, 0.1, 0.1);
    match e {
        Effect::Constant(c) => assert_eq!(c.level, 0x7FFF),
        _ => panic!("Expected Constant effect"),
    }
}

#[test]
fn simple_constant_sets_envelope_and_duration() {
    let e = simple_constant(0.5, 0.2, 0.3, 0.4);
    match e {
        Effect::Constant(c) => {
            assert_eq!(c.envelope.attack_length, 200);
            assert_eq!(c.envelope.fade_length, 400);
            assert_eq!(c.duration, 900);
        }
        _ => panic!("Expected Constant effect"),
    }
}

#[test]
fn simple_ramp_creates_ramp_effect() {
    let e = simple_ramp(0.0, 1.0, 0.1, 0.1, 0.1);
    match e {
        Effect::Ramp(_) => {}
        _ => panic!("Expected Ramp effect"),
    }
}

#[test]
fn simple_ramp_clamps_levels() {
    let e = simple_ramp(-2.0, 3.0, 0.1, 0.1, 0.1);
    match e {
        Effect::Ramp(r) => {
            assert_eq!(r.start_level, -0x8000);
            assert_eq!(r.end_level, 0x7FFF);
        }
        _ => panic!("Expected Ramp effect"),
    }
}

#[test]
fn simple_ramp_sets_envelope_and_duration() {
    let e = simple_ramp(0.5, -0.5, 0.2, 0.3, 0.4);
    match e {
        Effect::Ramp(r) => {
            assert_eq!(r.envelope.attack_length, 200);
            assert_eq!(r.envelope.fade_length, 400);
            assert_eq!(r.duration, 900);
        }
        _ => panic!("Expected Ramp effect"),
    }
}
