use shake::device::Device;
use shake::effect::{Effect, Envelope, PeriodicEffect, PeriodicWaveform};
use shake::error::ShakeResult;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

fn main() -> ShakeResult<()> {
    let device: Arc<Device> = Device::open(0)?;

    let mut effect = PeriodicEffect {
        waveform: PeriodicWaveform::Sine,
        period: 100,
        magnitude: 0x4000,
        offset: 0,
        phase: 0,
        envelope: Envelope::new(0, 0, 0, 0),
        duration: 4000,
        delay: 0,
        direction: 0,
    };

    let mut handle = device.upload(&Effect::Periodic(effect))?;

    println!("Original");
    handle.play()?;
    sleep(Duration::from_secs(2));

    effect.magnitude = 0x6000;

    handle = device.upload(&Effect::Periodic(effect))?;

    println!("Updated");
    handle.play()?;
    sleep(Duration::from_secs(2));

    drop(handle);

    Ok(())
}
