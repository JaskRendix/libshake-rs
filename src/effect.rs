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
    pub fn new(attack_length: u16, attack_level: u16, fade_length: u16, fade_level: u16) -> Self {
        Self {
            attack_length,
            attack_level,
            fade_length,
            fade_level,
        }
    }
}

//
// Rumble effect
//
#[derive(Debug, Clone, Copy)]
pub struct RumbleEffect {
    pub strong_magnitude: u16,
    pub weak_magnitude: u16,
    pub duration: u16, // milliseconds
    pub delay: u16,    // milliseconds
}

//
// Periodic effect
//
#[derive(Debug, Clone, Copy)]
pub struct PeriodicEffect {
    pub waveform: PeriodicWaveform,
    pub period: u16,
    pub magnitude: i16,
    pub offset: i16,
    pub phase: u16,
    pub envelope: Envelope,
    pub duration: u16, // milliseconds
    pub delay: u16,    // milliseconds
}

//
// Constant effect
//
#[derive(Debug, Clone, Copy)]
pub struct ConstantEffect {
    pub level: i16,
    pub envelope: Envelope,
    pub duration: u16, // milliseconds
    pub delay: u16,    // milliseconds
}

//
// Ramp effect
//
#[derive(Debug, Clone, Copy)]
pub struct RampEffect {
    pub start_level: i16,
    pub end_level: i16,
    pub envelope: Envelope,
    pub duration: u16, // milliseconds
    pub delay: u16,    // milliseconds
}

//
// Main Effect enum
//
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
