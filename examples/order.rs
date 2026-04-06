use rand::seq::SliceRandom;
use shake::device::Device;
use shake::effect::{Effect, Envelope, PeriodicEffect, PeriodicWaveform, RumbleEffect};
use shake::error::ShakeResult;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

fn main() -> ShakeResult<()> {
    let device: Arc<Device> = Device::open(0)?;

    let effects = vec![
        Effect::Periodic(PeriodicEffect {
            waveform: PeriodicWaveform::Sine,
            period: 100,
            magnitude: 0x5000,
            offset: 0,
            phase: 0,
            envelope: Envelope::new(0, 0, 0, 0),
            duration: 1000,
            delay: 0,
            direction: 0,
        }),
        Effect::Periodic(PeriodicEffect {
            waveform: PeriodicWaveform::Square,
            period: 100,
            magnitude: 0x5000,
            offset: 0,
            phase: 0,
            envelope: Envelope::new(0, 0, 0, 0),
            duration: 1000,
            delay: 0,
            direction: 0,
        }),
        Effect::Rumble(RumbleEffect {
            strong_magnitude: 0x6000,
            weak_magnitude: 0x6000,
            duration: 1000,
            delay: 0,
            direction: 0,
        }),
        Effect::Rumble(RumbleEffect {
            strong_magnitude: 0x5000,
            weak_magnitude: 0x5000,
            duration: 1000,
            delay: 0,
            direction: 0,
        }),
    ];

    let mut handles = Vec::new();
    for e in &effects {
        handles.push(device.upload(e)?);
    }

    let mut order = vec![0, 1, 2, 3];
    order.shuffle(&mut rand::thread_rng());

    for idx in order {
        println!("Effect #{}", idx);
        handles[idx].play()?;
        sleep(Duration::from_secs(1));
    }

    drop(handles);

    Ok(())
}
