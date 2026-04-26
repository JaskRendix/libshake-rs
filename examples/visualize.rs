use shake::device::Device;
use shake::error::ShakeResult;

fn main() -> ShakeResult<()> {
    // With mock-backend enabled, this opens /dev/mock0
    let dev = Device::open(0)?;
    println!("--- Haptic Visualization Demo ---");

    // 1. Spring
    println!("\n[Spring]");
    let spring = shake::simple::simple_spring(1.0, 0.0);
    let h1 = dev.upload(&spring)?;
    h1.play()?;

    // 2. Friction
    println!("\n[Friction]");
    let friction = shake::simple::simple_friction(0.5);
    let h2 = dev.upload(&friction)?;
    h2.play()?;

    // 3. Damper
    println!("\n[Damper]");
    let damper = shake::simple::simple_damper(0.7);
    let h3 = dev.upload(&damper)?;
    h3.play()?;

    // 4. Inertia
    println!("\n[Inertia]");
    let inertia = shake::simple::simple_inertia(0.6);
    let h4 = dev.upload(&inertia)?;
    h4.play()?;

    println!("\nVisualization complete.");
    Ok(())
}
