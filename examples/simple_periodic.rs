use shake::device::Device;
use shake::effect::{Effect, Envelope, PeriodicEffect, PeriodicWaveform};
use shake::error::ShakeResult;
use std::thread::sleep;
use std::time::Duration;

fn main() -> ShakeResult<()> {
    let device = Device::open(0)?;

    let effect = Effect::Periodic(PeriodicEffect {
        waveform: PeriodicWaveform::Sine,
        period: 100, // milliseconds
        magnitude: 0x6000,
        offset: 0,
        phase: 0,
        envelope: Envelope {
            attack_length: 10,
            attack_level: 0,
            fade_length: 10,
            fade_level: 0,
        },
        duration: 2000, // milliseconds
        delay: 0,
    });

    let id = device.upload(&effect)?;
    device.play(id)?;

    sleep(Duration::from_secs(2));

    device.erase(id)?;
    Ok(())
}
