use shake::device::Device;
use shake::effect::{Effect, Envelope, PeriodicEffect, PeriodicWaveform};
use shake::error::ShakeResult;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

fn main() -> ShakeResult<()> {
    let device: Arc<Device> = Device::open(0)?;

    let effect = Effect::Periodic(PeriodicEffect {
        waveform: PeriodicWaveform::Sine,
        period: 100, // milliseconds
        magnitude: 0x6000,
        offset: 0,
        phase: 0,
        envelope: Envelope::new(10, 0, 10, 0),
        duration: 2000, // milliseconds
        delay: 0,
        direction: 0,
    });

    let handle = device.upload(&effect)?;
    handle.play()?;

    sleep(Duration::from_secs(2));

    drop(handle);

    Ok(())
}
