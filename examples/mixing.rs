use shake::device::Device;
use shake::effect::{Effect, Envelope, PeriodicEffect, PeriodicWaveform};
use shake::error::ShakeResult;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

fn main() -> ShakeResult<()> {
    let device: Arc<Device> = Device::open(0)?;

    let e1 = Effect::Periodic(PeriodicEffect {
        waveform: PeriodicWaveform::Sine,
        period: 100,
        magnitude: (0.6 * 0x7FFF as f32) as i16,
        offset: 0,
        phase: 0,
        envelope: Envelope::new(0, 0, 0, 0),
        duration: 4000,
        delay: 0,
        direction: 0,
    });

    let e2 = Effect::Periodic(PeriodicEffect {
        waveform: PeriodicWaveform::Square,
        period: 100,
        magnitude: (0.2 * 0x7FFF as f32) as i16,
        offset: 0,
        phase: 0,
        envelope: Envelope::new(0, 0, 0, 0),
        duration: 2000,
        delay: 0,
        direction: 0,
    });

    let e3 = Effect::Periodic(PeriodicEffect {
        waveform: PeriodicWaveform::Sine,
        period: 100,
        magnitude: (0.2 * 0x7FFF as f32) as i16,
        offset: 0,
        phase: 0,
        envelope: Envelope::new(0, 0, 0, 0),
        duration: 1000,
        delay: 0,
        direction: 0,
    });

    let h1 = device.upload(&e1)?;
    let h2 = device.upload(&e2)?;
    let h3 = device.upload(&e3)?;

    println!("Playing #1 (0.6 mag)");
    h1.play()?;
    sleep(Duration::from_secs(1));

    println!("Adding #2 (+0.2 mag)");
    h2.play()?;
    sleep(Duration::from_secs(1));

    println!("Adding #3 (+0.2 mag)");
    h3.play()?;
    sleep(Duration::from_secs(1));

    println!("Removing #2 and #3 (-0.4 mag)");
    sleep(Duration::from_secs(1));

    drop(h1);
    drop(h2);
    drop(h3);

    Ok(())
}
