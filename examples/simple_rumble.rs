use shake::device::Device;
use shake::effect::{Effect, RumbleEffect};
use shake::error::ShakeResult;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

fn main() -> ShakeResult<()> {
    let device: Arc<Device> = Device::open(0)?;

    let effect = Effect::Rumble(RumbleEffect {
        strong_magnitude: 0x6000,
        weak_magnitude: 0x6000,
        duration: 2000,
        delay: 0,
        direction: 0,
    });

    let handle = device.upload(&effect)?;

    println!("Playing (2 sec)");
    handle.play()?;
    sleep(Duration::from_secs(2));

    drop(handle);

    Ok(())
}
