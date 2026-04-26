use std::thread;
use std::time::Duration;

use shake::device::Device;
use shake::error::ShakeResult;

fn main() -> ShakeResult<()> {
    // 1. Enumerate devices
    let devices = Device::enumerate()?;
    let info = match devices.first() {
        Some(i) => i,
        None => {
            println!("No devices found.");
            return Ok(());
        }
    };

    let dev = Device::open_info(info)?;
    println!("Testing condition effects on: {}", dev.name());

    // 2. Check hardware support
    println!("Supports Spring:   {}", dev.supports_spring());
    println!("Supports Friction: {}", dev.supports_friction());
    println!("Supports Damper:   {}", dev.supports_damper());
    println!("Supports Inertia:  {}", dev.supports_inertia());

    // 3. Upload a Spring effect
    println!("Uploading Spring effect...");
    let spring = shake::simple::simple_spring(0.8, 0.1);
    let handle = dev.upload(&spring)?;

    // 4. Play it
    println!("Spring active for 5 seconds...");
    handle.play()?;
    thread::sleep(Duration::from_secs(5));
    handle.stop()?;

    println!("Done.");
    Ok(())
}
