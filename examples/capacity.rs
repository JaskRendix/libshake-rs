use shake::device::Device;
use shake::effect::{Effect, Envelope, PeriodicEffect, PeriodicWaveform};
use shake::error::ShakeResult;

fn main() -> ShakeResult<()> {
    let device = Device::open(0)?;
    let capacity = device.capacity();

    println!("Reported capacity: {}", capacity);

    let mut ids = Vec::new();

    for i in 0..capacity {
        let effect = Effect::Periodic(PeriodicEffect {
            waveform: PeriodicWaveform::Sine,
            period: 100, // ms
            magnitude: 0x4000,
            offset: 0,
            phase: 0,
            envelope: Envelope {
                attack_length: 0,
                attack_level: 0,
                fade_length: 0,
                fade_level: 0,
            },
            duration: 1000, // ms
            delay: 0,
        });

        let id = device.upload(&effect)?;
        println!("Uploaded {} as {}", i, id);
        ids.push(id);
    }

    for id in ids {
        device.erase(id)?;
    }

    Ok(())
}
