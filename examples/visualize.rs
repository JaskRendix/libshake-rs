use shake::effect::*;
use shake::simple::*;

fn main() {
    let effect = simple_periodic(
        PeriodicWaveform::Sine,
        1.0, // magnitude
        0.2, // attack
        0.6, // sustain
        0.2, // fade
    );

    let Effect::Periodic(p) = effect else {
        eprintln!("Not a periodic effect");
        return;
    };

    println!("Visualizing periodic effect:");
    println!(
        "waveform={:?}, magnitude={}, duration={}ms",
        p.waveform, p.magnitude, p.duration
    );

    for t in (0..p.duration as u32).step_by(50) {
        let sine = (2.0 * std::f32::consts::PI * (t as f32) / p.period as f32).sin();

        let env = if t < p.envelope.attack_length as u32 {
            t as f32 / p.envelope.attack_length as f32
        } else if t > (p.duration as u32 - p.envelope.fade_length as u32) {
            let fade_start = (p.duration - p.envelope.fade_length) as f32;
            1.0 - ((t as f32 - fade_start) / p.envelope.fade_length as f32)
        } else {
            1.0
        };

        let val = (sine * p.magnitude as f32 * env) as i32;

        let bar = "*".repeat((val.abs() / 1000) as usize);
        println!("{:4}ms | {:7} | {}", t, val, bar);
    }
}
