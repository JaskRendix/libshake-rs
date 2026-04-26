use shake::device::Device;
use shake::error::ShakeResult;
use shake::simple::*;

use std::thread;
use std::time::Duration;

fn main() -> ShakeResult<()> {
    let dev = Device::open(0)?;
    println!("Using device: {}", dev.name());

    println!("\nUploading directional rumble...");
    let rumble = simple_rumble_dir(
        1.0,  // strong
        0.5,  // weak
        1.0,  // seconds
        45.0, // direction in degrees
    );

    let r_handle = dev.upload(&rumble)?;
    r_handle.play()?;
    println!("Rumble for 1 second...");
    thread::sleep(Duration::from_secs(1));
    r_handle.stop()?;

    println!("\nDevice supports Spring: {}", dev.capabilities().spring);

    let spring = simple_spring(
        0.8, // strength
        0.1, // deadzone
    );

    println!("Uploading spring effect...");
    let s_handle = dev.upload(&spring)?;
    s_handle.play()?;
    println!("Spring active for 3 seconds...");
    thread::sleep(Duration::from_secs(3));
    s_handle.stop()?;

    println!("\nDone.");
    Ok(())
}
