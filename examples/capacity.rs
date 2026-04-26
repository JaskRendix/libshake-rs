use shake::device::Device;
use shake::effect::{Effect, Envelope, PeriodicEffect, PeriodicWaveform};
use shake::error::ShakeResult;
use std::sync::Arc;

fn main() -> ShakeResult<()> {
    let device: Arc<Device> = Device::open(0)?;
    let capacity = device.max_effects(); // ← FIXED

    println!("Reported capacity: {}", capacity);

    let mut handles = Vec::new();

    for i in 0..capacity {
        let effect = Effect::Periodic(PeriodicEffect {
            waveform: PeriodicWaveform::Sine,
            period: 100, // ms
            magnitude: 0x4000,
            offset: 0,
            phase: 0,
            envelope: Envelope::new(0, 0, 0, 0),
            duration: 1000, // ms
            delay: 0,
            direction: 0,
        });

        let handle = device.upload(&effect)?;
        println!("Uploaded {} as effect #{}", i, handle.id());
        handles.push(handle);
    }

    drop(handles);

    Ok(())
}
