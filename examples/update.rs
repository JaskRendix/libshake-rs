use shake::device::Device;
use shake::effect::{Effect, Envelope, PeriodicEffect, PeriodicWaveform};
use shake::error::ShakeResult;
use std::thread::sleep;
use std::time::Duration;

fn main() -> ShakeResult<()> {
    let device = Device::open(0)?;

    let mut effect = PeriodicEffect {
        waveform: PeriodicWaveform::Sine,
        period: 100,
        magnitude: 0x4000,
        offset: 0,
        phase: 0,
        envelope: Envelope {
            attack_length: 0,
            attack_level: 0,
            fade_length: 0,
            fade_level: 0,
        },
        duration: 4000,
        delay: 0,
    };

    let mut id = device.upload(&Effect::Periodic(effect.clone()))?;

    println!("Original");
    device.play(id)?;
    sleep(Duration::from_secs(2));

    effect.magnitude = 0x6000;
    id = device.upload(&Effect::Periodic(effect.clone()))?;

    println!("Updated");
    device.play(id)?;
    sleep(Duration::from_secs(2));

    device.erase(id)?;
    Ok(())
}
