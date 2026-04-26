use shake::device::Device;
use shake::effect::*;
use shake::error::ShakeResult;
use shake::simple::*;

use std::thread;
use std::time::Duration;

fn main() -> ShakeResult<()> {
    let dev = Device::open(0)?;
    println!("Using device: {}", dev.name());

    let periodic = simple_periodic(
        PeriodicWaveform::Sine,
        1.0, // magnitude
        0.2, // attack
        0.6, // sustain
        0.2, // fade
    );

    let Effect::Periodic(p) = periodic else {
        eprintln!("Not a periodic effect");
        return Ok(());
    };

    println!("\n=== Visualizing PeriodicEffect ===");
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

    println!("\nUploading periodic effect...");
    let periodic_handle = dev.upload(&periodic)?;
    periodic_handle.play()?;
    println!("Periodic effect playing for 2 seconds...");
    thread::sleep(Duration::from_secs(2));
    periodic_handle.stop()?;

    println!("\n=== Testing Spring ConditionEffect ===");

    println!("Device supports Spring: {}", dev.supports_spring());

    let spring = simple_spring(
        0.8, // strength
        0.1, // deadzone
    );

    println!("Uploading Spring effect...");
    let spring_handle = dev.upload(&spring)?;

    println!("Spring active for 3 seconds...");
    spring_handle.play()?;
    thread::sleep(Duration::from_secs(3));
    spring_handle.stop()?;

    println!("\nDone.");
    Ok(())
}
