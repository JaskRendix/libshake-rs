use shake::device::Device;
use shake::effect::{Effect, RumbleEffect};
use shake::error::ShakeResult;
use std::thread::sleep;
use std::time::Duration;

fn main() -> ShakeResult<()> {
    let device = Device::open(0)?;

    let effect = Effect::Rumble(RumbleEffect {
        strong_magnitude: 0x6000,
        weak_magnitude: 0x6000,
        duration: 2000,
        delay: 0,
    });

    let id = device.upload(&effect)?;
    device.play(id)?;

    sleep(Duration::from_secs(2));

    device.erase(id)?;
    Ok(())
}
