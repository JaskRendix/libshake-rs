pub const ENVELOPE_ATTACK_LENGTH_MAX: u16 = 0x7FFF;
pub const ENVELOPE_FADE_LENGTH_MAX: u16 = 0x7FFF;
pub const ENVELOPE_ATTACK_LEVEL_MAX: u16 = 0x7FFF;
pub const ENVELOPE_FADE_LEVEL_MAX: u16 = 0x7FFF;

pub const RUMBLE_STRONG_MAGNITUDE_MAX: u16 = 0x7FFF;
pub const RUMBLE_WEAK_MAGNITUDE_MAX: u16 = 0x7FFF;

pub const PERIODIC_PERIOD_MAX: u16 = 0x7FFF;
pub const PERIODIC_MAGNITUDE_MIN: i16 = -0x8000;
pub const PERIODIC_MAGNITUDE_MAX: i16 = 0x7FFF;
pub const PERIODIC_OFFSET_MIN: i16 = -0x8000;
pub const PERIODIC_OFFSET_MAX: i16 = 0x7FFF;
pub const PERIODIC_PHASE_MAX: u16 = 0x7FFF;

pub const CONSTANT_LEVEL_MIN: i16 = -0x8000;
pub const CONSTANT_LEVEL_MAX: i16 = 0x7FFF;

pub const RAMP_START_LEVEL_MIN: i16 = -0x8000;
pub const RAMP_START_LEVEL_MAX: i16 = 0x7FFF;
pub const RAMP_END_LEVEL_MIN: i16 = -0x8000;
pub const RAMP_END_LEVEL_MAX: i16 = 0x7FFF;

pub const EFFECT_ID_NEW: i16 = -1;
pub const EFFECT_DIRECTION_MAX: u16 = 0xFFFE;
pub const EFFECT_LENGTH_MAX: u16 = 0x7FFF;
pub const EFFECT_DELAY_MAX: u16 = 0x7FFF;

#[derive(Debug, Clone, Copy)]
pub enum PeriodicWaveform {
    Square,
    Triangle,
    Sine,
    SawUp,
    SawDown,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectType {
    Rumble,
    Periodic,
    Constant,
    Ramp,
}

#[derive(Debug, Clone, Copy)]
pub struct Envelope {
    pub attack_length: u16,
    pub attack_level: u16,
    pub fade_length: u16,
    pub fade_level: u16,
}

impl Envelope {
    pub fn new(a_len: u16, a_lvl: u16, f_len: u16, f_lvl: u16) -> Self {
        Self {
            attack_length: a_len.min(ENVELOPE_ATTACK_LENGTH_MAX),
            attack_level: a_lvl.min(ENVELOPE_ATTACK_LEVEL_MAX),
            fade_length: f_len.min(ENVELOPE_FADE_LENGTH_MAX),
            fade_level: f_lvl.min(ENVELOPE_FADE_LEVEL_MAX),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RumbleEffect {
    pub strong_magnitude: u16,
    pub weak_magnitude: u16,
    pub direction: u16, // 0..=0xFFFF (Linux FF units)
    pub duration: u16,
    pub delay: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct PeriodicEffect {
    pub waveform: PeriodicWaveform,
    pub period: u16,
    pub magnitude: i16,
    pub offset: i16,
    pub phase: u16,
    pub envelope: Envelope,
    pub duration: u16,
    pub delay: u16,
    pub direction: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct ConstantEffect {
    pub level: i16,
    pub envelope: Envelope,
    pub duration: u16,
    pub delay: u16,
    pub direction: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct RampEffect {
    pub start_level: i16,
    pub end_level: i16,
    pub envelope: Envelope,
    pub duration: u16,
    pub delay: u16,
    pub direction: u16,
}

#[derive(Debug, Clone)]
pub enum Effect {
    Rumble(RumbleEffect),
    Periodic(PeriodicEffect),
    Constant(ConstantEffect),
    Ramp(RampEffect),
}

impl Effect {
    pub fn effect_type(&self) -> EffectType {
        match self {
            Effect::Rumble(_) => EffectType::Rumble,
            Effect::Periodic(_) => EffectType::Periodic,
            Effect::Constant(_) => EffectType::Constant,
            Effect::Ramp(_) => EffectType::Ramp,
        }
    }
}

pub fn visualize_direction(direction: u16) -> &'static str {
    // 0–65535 maps to 0–360 degrees
    let angle = (direction as f32 / 65535.0) * 360.0;

    match angle {
        a if a < 22.5 => "→",
        a if a < 67.5 => "↗",
        a if a < 112.5 => "↑",
        a if a < 157.5 => "↖",
        a if a < 202.5 => "←",
        a if a < 247.5 => "↙",
        a if a < 292.5 => "↓",
        a if a < 337.5 => "↘",
        _ => "→",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visualize_periodic_sine() {
        // A periodic effect with envelope so we can see the shape evolve
        let effect = PeriodicEffect {
            waveform: PeriodicWaveform::Sine,
            period: 100, // ms
            magnitude: 0x7FFF,
            offset: 0,
            phase: 0,
            envelope: Envelope {
                attack_length: 500,
                attack_level: 0,
                fade_length: 500,
                fade_level: 0,
            },
            duration: 2000,
            delay: 0,
            direction: 0,
        };

        println!("--- Visualizing Sine PeriodicEffect ---");
        println!(
            "period={}ms, magnitude={}, duration={}ms",
            effect.period, effect.magnitude, effect.duration
        );

        // Step through the effect in 50ms increments
        for t in (0..effect.duration as u32).step_by(50) {
            // Base sine wave
            let sine = (2.0 * std::f32::consts::PI * (t as f32) / effect.period as f32).sin();

            // Envelope multiplier
            let env = if t < effect.envelope.attack_length as u32 {
                // Attack
                t as f32 / effect.envelope.attack_length as f32
            } else if t > (effect.duration as u32 - effect.envelope.fade_length as u32) {
                // Fade
                let fade_start = (effect.duration - effect.envelope.fade_length) as f32;
                1.0 - ((t as f32 - fade_start) / effect.envelope.fade_length as f32)
            } else {
                // Sustain
                1.0
            };

            // Final value
            let val = (sine * effect.magnitude as f32 * env) as i32;

            // ASCII bar graph
            let bar_len = (val.abs() / 1000) as usize;
            let bar = "*".repeat(bar_len);

            println!("{:4}ms | {:7} | {}", t, val, bar);
        }
    }

    #[test]
    fn visualize_ramp_effect() {
        let effect = RampEffect {
            start_level: -0x8000,
            end_level: 0x7FFF,
            envelope: Envelope::new(200, 0, 200, 0),
            duration: 2000,
            delay: 0,
            direction: 0,
        };

        println!("--- Visualizing RampEffect ---");
        println!(
            "start={}, end={}, duration={}ms",
            effect.start_level, effect.end_level, effect.duration
        );

        for t in (0..effect.duration as u32).step_by(50) {
            let frac = t as f32 / effect.duration as f32;
            let val = effect.start_level as f32 * (1.0 - frac) + effect.end_level as f32 * frac;

            let bar = "*".repeat((val.abs() as i32 / 1000) as usize);
            println!("{:4}ms | {:7.0} | {}", t, val, bar);
        }
    }

    #[test]
    fn visualize_direction_ascii() {
        let dirs = [
            0,     // 0°   →
            8192,  // 45°  ↗
            16384, // 90°  ↑
            24576, // 135° ↖
            32768, // 180° ←
            40960, // 225° ↙
            49152, // 270° ↓
            57344, // 315° ↘
        ];

        for d in dirs {
            println!("{:5} -> {}", d, crate::effect::visualize_direction(d));
        }
    }
}
