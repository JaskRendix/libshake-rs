use crate::effect::{
    ConstantEffect, Effect, Envelope, PeriodicEffect, PeriodicWaveform, RampEffect, RumbleEffect,
    CONSTANT_LEVEL_MAX, CONSTANT_LEVEL_MIN, PERIODIC_MAGNITUDE_MAX, PERIODIC_MAGNITUDE_MIN,
    RUMBLE_STRONG_MAGNITUDE_MAX, RUMBLE_WEAK_MAGNITUDE_MAX,
};

fn clamp_u16(value: f32, max: u16) -> u16 {
    value.clamp(0.0, max as f32) as u16
}

fn clamp_i16(value: f32, min: i16, max: i16) -> i16 {
    value.clamp(min as f32, max as f32) as i16
}

/// Simple rumble without direction (direction = 0).
pub fn simple_rumble(strong: f32, weak: f32, secs: f32) -> Effect {
    simple_rumble_dir(strong, weak, secs, 0.0)
}

/// Simple rumble with direction in degrees (0–360).
pub fn simple_rumble_dir(strong: f32, weak: f32, secs: f32, direction_deg: f32) -> Effect {
    let strong_mag = clamp_u16(
        strong * RUMBLE_STRONG_MAGNITUDE_MAX as f32,
        RUMBLE_STRONG_MAGNITUDE_MAX,
    );
    let weak_mag = clamp_u16(
        weak * RUMBLE_WEAK_MAGNITUDE_MAX as f32,
        RUMBLE_WEAK_MAGNITUDE_MAX,
    );

    // Convert degrees (0–360) to Linux FF units (0–0xFFFF)
    let dir = ((direction_deg.rem_euclid(360.0) / 360.0) * 65535.0) as u16;

    Effect::Rumble(RumbleEffect {
        strong_magnitude: strong_mag,
        weak_magnitude: weak_mag,
        direction: dir,
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

    let magnitude = clamp_i16(
        force * PERIODIC_MAGNITUDE_MAX as f32,
        PERIODIC_MAGNITUDE_MIN,
        PERIODIC_MAGNITUDE_MAX,
    );

    let envelope = Envelope::new((attack * 1000.0) as u16, 0, (fade * 1000.0) as u16, 0);

    Effect::Periodic(PeriodicEffect {
        waveform,
        period: 100, // 0.1s
        magnitude,
        offset: 0,
        phase: 0,
        envelope,
        duration: (total * 1000.0) as u16,
        delay: 0,
        direction: 0,
    })
}

pub fn simple_constant(force: f32, attack: f32, sustain: f32, fade: f32) -> Effect {
    let total = attack + sustain + fade;

    let level = clamp_i16(
        force * CONSTANT_LEVEL_MAX as f32,
        CONSTANT_LEVEL_MIN,
        CONSTANT_LEVEL_MAX,
    );

    let envelope = Envelope::new((attack * 1000.0) as u16, 0, (fade * 1000.0) as u16, 0);

    Effect::Constant(ConstantEffect {
        level,
        envelope,
        duration: (total * 1000.0) as u16,
        delay: 0,
        direction: 0,
    })
}

pub fn simple_ramp(start: f32, end: f32, attack: f32, sustain: f32, fade: f32) -> Effect {
    let total = attack + sustain + fade;

    // Correct scaling:
    // - Negative values scale to -32768
    // - Positive values scale to  32767
    fn scale_signed(v: f32) -> i16 {
        if v >= 0.0 {
            (v * 0x7FFF as f32).clamp(0.0, 0x7FFF as f32) as i16
        } else {
            (v * 0x8000 as f32).clamp(-0x8000 as f32, 0.0) as i16
        }
    }

    let start_level = scale_signed(start);
    let end_level = scale_signed(end);

    let envelope = Envelope::new((attack * 1000.0) as u16, 0, (fade * 1000.0) as u16, 0);

    Effect::Ramp(RampEffect {
        start_level,
        end_level,
        envelope,
        duration: (total * 1000.0) as u16,
        delay: 0,
        direction: 0,
    })
}
