use shake::device::Device;
use shake::effect::{Effect, PeriodicEffect, RumbleEffect, Envelope, PeriodicWaveform};
use shake::error::ShakeResult;
use rand::seq::SliceRandom;
use std::thread::sleep;
use std::time::Duration;

fn main() -> ShakeResult<()> {
    let device = Device::open(0)?;

    let effects = vec![
        Effect::Periodic(PeriodicEffect {
            waveform: PeriodicWaveform::Sine,
            period: 100,
            magnitude: 0x5000,
            offset: 0,
            phase: 0,
            envelope: Envelope {
                attack_length: 0,
                attack_level: 0,
                fade_length: 0,
                fade_level: 0,
            },
            duration: 1000,
            delay: 0,
        }),
        Effect::Periodic(PeriodicEffect {
            waveform: PeriodicWaveform::Square,
            period: 100,
            magnitude: 0x5000,
            offset: 0,
            phase: 0,
            envelope: Envelope {
                attack_length: 0,
                attack_level: 0,
                fade_length: 0,
                fade_level: 0,
            },
            duration: 1000,
            delay: 0,
        }),
        Effect::Rumble(RumbleEffect {
            strong_magnitude: 0x6000,
            weak_magnitude: 0x6000,
            duration: 1000,
            delay: 0,
        }),
        Effect::Rumble(RumbleEffect {
            strong_magnitude: 0x5000,
            weak_magnitude: 0x5000,
            duration: 1000,
            delay: 0,
        }),
    ];

    let mut ids = Vec::new();
    for e in &effects {
        ids.push(device.upload(e)?);
    }

    let mut order = vec![0, 1, 2, 3];
    order.shuffle(&mut rand::thread_rng());

    for idx in order {
        println!("Effect #{}", idx);
        device.play(ids[idx])?;
        sleep(Duration::from_secs(1));
    }

    for id in ids {
        device.erase(id)?;
    }

    Ok(())
}
