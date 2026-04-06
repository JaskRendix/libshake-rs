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
        period: 100,
        magnitude: 0x6000,
        offset: 0,
        phase: 0,
        envelope: Envelope::new(0, 0, 0, 0),
        duration: 2000,
        delay: 0,
        direction: 0,
    });

    let handle = device.upload(&effect)?;

    println!("Playing (2 sec)");
    handle.play()?;
    sleep(Duration::from_secs(1));

    println!("Stopping (at 1 sec)");
    handle.stop()?;
    sleep(Duration::from_secs(1));

    println!("Replaying (2 sec)");
    handle.play()?;
    sleep(Duration::from_secs(2));

    drop(handle);

    Ok(())
}
