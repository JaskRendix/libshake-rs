use shake::device::Device;
use shake::effect::{Effect, PeriodicEffect, PeriodicWaveform, Envelope};
use shake::error::ShakeResult;
use std::thread::sleep;
use std::time::Duration;

fn main() -> ShakeResult<()> {
    let device = Device::open(0)?;

    let effect = Effect::Periodic(PeriodicEffect {
        waveform: PeriodicWaveform::Sine,
        period: 100,
        magnitude: 0x6000,
        offset: 0,
        phase: 0,
        envelope: Envelope {
            attack_length: 0,
            attack_level: 0,
            fade_length: 0,
            fade_level: 0,
        },
        duration: 2000,
        delay: 0,
    });

    let id = device.upload(&effect)?;

    println!("Playing (2 sec)");
    device.play(id)?;
    sleep(Duration::from_secs(1));

    println!("Stopping (at 1 sec)");
    device.stop(id)?;
    sleep(Duration::from_secs(1));

    println!("Replaying (2 sec)");
    device.play(id)?;
    sleep(Duration::from_secs(2));

    device.erase(id)?;
    Ok(())
}
