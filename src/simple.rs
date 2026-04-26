use crate::effect::{
    ConditionEffect, ConstantEffect, Effect, Envelope, PeriodicEffect, PeriodicWaveform,
    RampEffect, RumbleEffect, CONSTANT_LEVEL_MAX, CONSTANT_LEVEL_MIN, PERIODIC_MAGNITUDE_MAX,
    PERIODIC_MAGNITUDE_MIN, RUMBLE_STRONG_MAGNITUDE_MAX, RUMBLE_WEAK_MAGNITUDE_MAX,
};

fn clamp_u16(value: f32, max: u16) -> u16 {
    value.clamp(0.0, max as f32) as u16
}

fn clamp_i16(value: f32, min: i16, max: i16) -> i16 {
    value.clamp(min as f32, max as f32) as i16
}

pub fn simple_rumble(strong: f32, weak: f32, secs: f32) -> Effect {
    simple_rumble_dir(strong, weak, secs, 0.0)
}

pub fn simple_rumble_dir(strong: f32, weak: f32, secs: f32, direction_deg: f32) -> Effect {
    let strong_mag = clamp_u16(
        strong * RUMBLE_STRONG_MAGNITUDE_MAX as f32,
        RUMBLE_STRONG_MAGNITUDE_MAX,
    );
    let weak_mag = clamp_u16(
        weak * RUMBLE_WEAK_MAGNITUDE_MAX as f32,
        RUMBLE_WEAK_MAGNITUDE_MAX,
    );

    let dir = ((direction_deg.rem_euclid(360.0) / 360.0) * 65535.0) as u16;

    Effect::Rumble(RumbleEffect {
        strong_magnitude: strong_mag,
        weak_magnitude: weak_mag,
        direction: dir,
        duration: (secs * 1000.0) as u16,
        delay: 0,
    })
}

/// Simple periodic effect with default 100 ms period.
pub fn simple_periodic(
    waveform: PeriodicWaveform,
    force: f32,
    attack: f32,
    sustain: f32,
    fade: f32,
) -> Effect {
    simple_periodic_with_period(waveform, force, attack, sustain, fade, 100)
}

/// Same as `simple_periodic` but with custom period (ms).
pub fn simple_periodic_with_period(
    waveform: PeriodicWaveform,
    force: f32,
    attack: f32,
    sustain: f32,
    fade: f32,
    period_ms: u16,
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
        period: period_ms,
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

fn scale_condition_coeff(strength: f32) -> i16 {
    // Same signed scaling style as your ramp helper
    if strength >= 0.0 {
        (strength * 0x7FFF as f32).clamp(0.0, 0x7FFF as f32) as i16
    } else {
        (strength * 0x8000 as f32).clamp(-0x8000 as f32, 0.0) as i16
    }
}

fn scale_deadband(deadzone: f32) -> u16 {
    (deadzone * 0x7FFF as f32).clamp(0.0, 0x7FFF as f32) as u16
}

fn make_condition_effect(strength: f32, deadzone: f32) -> ConditionEffect {
    let coeff = scale_condition_coeff(strength);
    let deadband = scale_deadband(deadzone);

    ConditionEffect {
        right_saturation: 0x7FFF,
        left_saturation: 0x7FFF,
        right_coeff: coeff,
        left_coeff: coeff,
        deadband,
        center: 0,
    }
}

pub fn simple_spring(strength: f32, deadzone: f32) -> Effect {
    Effect::Spring(make_condition_effect(strength, deadzone))
}

pub fn simple_friction(strength: f32) -> Effect {
    // Friction has no deadzone; use 0
    Effect::Friction(make_condition_effect(strength, 0.0))
}

pub fn simple_damper(strength: f32) -> Effect {
    Effect::Damper(make_condition_effect(strength, 0.0))
}

pub fn simple_inertia(strength: f32) -> Effect {
    Effect::Inertia(make_condition_effect(strength, 0.0))
}
