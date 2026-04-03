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
fn simple_periodic_uses_correct_waveform() {
    let e = simple_periodic(PeriodicWaveform::Triangle, 0.5, 0.1, 0.1, 0.1);
    match e {
        Effect::Periodic(p) => assert!(matches!(p.waveform, PeriodicWaveform::Triangle)),
        _ => panic!("Expected Periodic effect"),
    }
}
