use shake::device::Device;
use shake::effect::{Effect, Envelope, PeriodicEffect, PeriodicWaveform};
use shake::error::ShakeResult;
use std::thread::sleep;
use std::time::Duration;

fn main() -> ShakeResult<()> {
    let device = Device::open(0)?;

    let e1 = Effect::Periodic(PeriodicEffect {
        waveform: PeriodicWaveform::Sine,
        period: 100,
        magnitude: (0.6 * 0x7FFF as f32) as i16,
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
    });

    let e2 = Effect::Periodic(PeriodicEffect {
        waveform: PeriodicWaveform::Square,
        period: 100,
        magnitude: (0.2 * 0x7FFF as f32) as i16,
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

    let e3 = Effect::Periodic(PeriodicEffect {
        waveform: PeriodicWaveform::Sine,
        period: 100,
        magnitude: (0.2 * 0x7FFF as f32) as i16,
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
    });

    let id1 = device.upload(&e1)?;
    let id2 = device.upload(&e2)?;
    let id3 = device.upload(&e3)?;

    println!("Playing #1 (0.6 mag)");
    device.play(id1)?;
    sleep(Duration::from_secs(1));

    println!("Adding #2 (+0.2 mag)");
    device.play(id2)?;
    sleep(Duration::from_secs(1));

    println!("Adding #3 (+0.2 mag)");
    device.play(id3)?;
    sleep(Duration::from_secs(1));

    println!("Removing #2 and #3 (-0.4 mag)");
    sleep(Duration::from_secs(1));

    device.erase(id1)?;
    device.erase(id2)?;
    device.erase(id3)?;

    Ok(())
}
