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

    println!("\nUploading periodic effect...");
    let p_handle = dev.upload(&periodic)?;
    p_handle.play()?;
    println!("Periodic effect playing for 2 seconds...");
    thread::sleep(Duration::from_secs(2));
    p_handle.stop()?;

    println!(
        "\nDevice supports Friction: {}",
        dev.capabilities().friction
    );

    let friction = simple_friction(0.7);

    println!("Uploading friction effect...");
    let f_handle = dev.upload(&friction)?;
    f_handle.play()?;
    println!("Friction active for 3 seconds...");
    thread::sleep(Duration::from_secs(3));
    f_handle.stop()?;

    println!("\nDone.");
    Ok(())
}
