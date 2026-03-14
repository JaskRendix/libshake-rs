use crate::effect::{
    Effect, RumbleEffect, PeriodicEffect, ConstantEffect, RampEffect, Envelope, PeriodicWaveform,
};

pub fn simple_rumble(strong: f32, weak: f32, secs: f32) -> Effect {
    let scale = 0x7FFF as f32;

    let strong_mag = (strong * scale)
        .clamp(0.0, scale) as u16;

    let weak_mag = (weak * scale)
        .clamp(0.0, scale) as u16;

    Effect::Rumble(RumbleEffect {
        strong_magnitude: strong_mag,
        weak_magnitude: weak_mag,
        duration: (secs * 1000.0) as u16,
        delay: 0,
    })
}

pub fn simple_periodic(
    waveform: PeriodicWaveform,
    force: f32,
    attack: f32,
    sustain: f32,
    fade: f32,
) -> Effect {
    let total = attack + sustain + fade;

    Effect::Periodic(PeriodicEffect {
        waveform,
        period: 100, // 0.1s
        magnitude: (force * 0x7FFF as f32) as i16,
        offset: 0,
        phase: 0,
        envelope: Envelope {
            attack_length: (attack * 1000.0) as u16,
            attack_level: 0,
            fade_length: (fade * 1000.0) as u16,
            fade_level: 0,
        },
        duration: (total * 1000.0) as u16,
        delay: 0,
    })
}

pub fn simple_constant(force: f32, attack: f32, sustain: f32, fade: f32) -> Effect {
    let total = attack + sustain + fade;

    Effect::Constant(ConstantEffect {
        level: (force * 0x7FFF as f32) as i16,
        envelope: Envelope {
            attack_length: (attack * 1000.0) as u16,
            attack_level: 0,
            fade_length: (fade * 1000.0) as u16,
            fade_level: 0,
        },
        duration: (total * 1000.0) as u16,
        delay: 0,
    })
}

pub fn simple_ramp(start: f32, end: f32, attack: f32, sustain: f32, fade: f32) -> Effect {
    let total = attack + sustain + fade;

    Effect::Ramp(RampEffect {
        start_level: (start * 0x8000 as f32) as i16,
        end_level:   (end   * 0x8000 as f32) as i16,
        envelope: Envelope {
            attack_length: (attack * 1000.0) as u16,
            attack_level: 0,
            fade_length: (fade * 1000.0) as u16,
            fade_level: 0,
        },
        duration: (total * 1000.0) as u16,
        delay: 0,
    })
}
