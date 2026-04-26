use shake::device::Device;
use shake::effect::*;
use shake::error::ShakeResult;
use shake::simple::*;

use std::thread;
use std::time::Duration;

fn main() -> ShakeResult<()> {
    let dev = Device::open(0)?;
    println!("=== libShake Demo Suite ===");
    println!("Device: {}", dev.name());

    println!("\n[Rumble]");
    let rumble = simple_rumble(1.0, 0.5, 1.0);
    let h_rumble = dev.upload(&rumble)?;
    h_rumble.play()?;
    thread::sleep(Duration::from_secs(1));
    h_rumble.stop()?;

    println!("\n[Periodic]");
    let periodic = simple_periodic(PeriodicWaveform::Sine, 1.0, 0.1, 0.8, 0.1);
    let h_periodic = dev.upload(&periodic)?;
    h_periodic.play()?;
    thread::sleep(Duration::from_secs(2));
    h_periodic.stop()?;

    println!("\n[Constant]");
    let constant = simple_constant(0.7, 0.1, 0.8, 0.1);
    let h_constant = dev.upload(&constant)?;
    h_constant.play()?;
    thread::sleep(Duration::from_secs(2));
    h_constant.stop()?;

    println!("\n[Ramp]");
    let ramp = simple_ramp(-1.0, 1.0, 0.1, 0.8, 0.1);
    let h_ramp = dev.upload(&ramp)?;
    h_ramp.play()?;
    thread::sleep(Duration::from_secs(2));
    h_ramp.stop()?;

    println!("\n[Spring]");
    let spring = simple_spring(0.8, 0.1);
    let h_spring = dev.upload(&spring)?;
    h_spring.play()?;
    thread::sleep(Duration::from_secs(3));
    h_spring.stop()?;

    println!("\n[Friction]");
    let friction = simple_friction(0.6);
    let h_friction = dev.upload(&friction)?;
    h_friction.play()?;
    thread::sleep(Duration::from_secs(3));
    h_friction.stop()?;

    println!("\n[Damper]");
    let damper = simple_damper(0.7);
    let h_damper = dev.upload(&damper)?;
    h_damper.play()?;
    thread::sleep(Duration::from_secs(3));
    h_damper.stop()?;

    println!("\n[Inertia]");
    let inertia = simple_inertia(0.5);
    let h_inertia = dev.upload(&inertia)?;
    h_inertia.play()?;
    thread::sleep(Duration::from_secs(3));
    h_inertia.stop()?;

    println!("\n=== Demo complete ===");
    Ok(())
}
