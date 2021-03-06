//! Definitions for the various output modes used by the [rev-11-1105](https://www.revrobotics.com/rev-11-1105/) LED driver
//!
//! This crate only provides the transcribed output values, not any PWM implementation. 
//! The user is expected to use this crate with something that implements 
//! [`embedded-hal`](https://github.com/rust-embedded/embedded-hal) for actual output
//!
//! All data in this crate has been directly transposed from the [datasheet](https://www.revrobotics.com/content/docs/REV-11-1105-UM.pdf)'s color table.

#![no_std]

extern crate num;
use num::{Num, NumCast};

/// Expression of each valid driver colour mode as a value from `0..200`.
///
/// Values are expressed in this range so they can be converted back to their 
/// float form when used (since enums can not store `f32` types in a useful way)
///
/// These values are not intended to be passes directly to a PWM output, but to 
/// be converted to the correct duty cycle format before use
///
/// These values are transcribed from the [user manual](https://www.revrobotics.com/content/docs/REV-11-1105-UM.pdf)
#[derive(Debug, Clone, Copy)]
pub enum Pattern {
    Rainbow = 1,
    RainbowParty = 3,
    RainbowOcean = 5,
    RainbowLava = 7,
    RainbowForest = 9,
    RainbowGlitter = 11,
    Confetti = 13,
    RedShot = 15,
    BlueShot = 17,
    WhiteShot = 19,
    SinelonRainbow = 21,
    SinelonParty = 23,
    SinelonOcean = 25,
    SinelonLava = 27,
    SinelonForest = 29,
    BpmRainbow = 31,
    BpmOcean = 35,
    BpmLava = 37,
    BpmForest = 39,
    FireMedium = 41,
    FireLarge = 43,
    TwinklesRainbow = 45,
    TwinklesParty = 47,
    TwinklesOcean = 49,
    TwinklesLava = 51,
    TwinklesForest = 53,
    WavesRainbow = 55,
    WavesParty = 57,
    WavesOcean = 59,
    WavesLava = 61,
    WavesForest = 63,
    LarsonRed = 65,
    LarsonGray = 67,
    ChaseRed = 69,
    ChaseBlue = 71,
    ChaseGray = 73,
    HeartbeatRed = 75,
    HeartbeatBlue = 77,
    HeartbeatWhite = 79,
    HeartbeatGray = 81,
    BreathRed = 83,
    BreathBlue = 85,
    BreathGray = 87,
    StrobeBlue = 91,
    StrobeGold = 93,
    StrobeWhite = 95,
    Color1BlendToBlack = 97,
    Color1Larson = 99,
    Color1Chase = 101,
    Color1HeartbeatSlow = 103,
    Color1HeartbeatMedium = 105,
    Color1HeartbeatFast = 107,
    Color1BreathSlow = 109,
    Color1BreathFast = 111,
    Color1Shot = 113,
    Color1Strobe = 115,
    Color2BlendToBlack = 117,
    Color2Larson = 119,
    Color2Chase = 121,
    Color2HeartbeatSlow = 123,
    Color2HeartbeatMedium = 125,
    Color2HeartbeatFast = 127,
    Color2BreathSlow = 129,
    Color2BreathFast = 131,
    Color2Shot = 133,
    Color2Strobe = 135,
    Sparkle1On2 = 137,
    Sparkle2On1 = 139,
    Gradient1And2 = 141,
    Bpm1And2 = 143,
    EndBlend1And2 = 145,
    EndBlend = 147,
    Color1And2NoBlend = 149,
    Twinkle1And2 = 151,
    Waves1And2 = 153,
    Sinelon1And2 = 155,
    HotPink = 157,
    DarkRed = 159,
    Red = 161,
    RedOrange = 163,
    Orange = 165,
    Gold = 167,
    Yellow = 169,
    LawnGreen = 171,
    Lime = 173,
    DarkGreen = 175,
    Green = 177,
    BlueGreen = 179,
    Aqua = 181,
    SkyBlue = 183,
    DarkBlue = 185,
    Blue = 187,
    BlueViolet = 189,
    Violet = 191,
    White = 193,
    Gray = 195,
    DarkGray = 197,
    Black = 199,
}

impl Pattern {

    /// Get the pattern duty cycle as a percentage value from `-1.0` to `1.0`
    pub fn as_percentage(&self) -> f32 {
        return ((*self as u8) as f32 - 100.0) / 100.0;
    }

    /// Get the pattern duty cycle as a percentage value from `0.0` to `1.0`
    pub fn as_abs_percentage(&self) -> f32 {
        return (self.as_percentage() + 1.0) / 2.0;
    }

    /// Get the pattern duty cycle as a value from `0` to `max_duty`.
    ///
    /// The `max_duty` should be the output of [`embedded_hal::PwmPin::get_max_duty()`](https://docs.rs/embedded-hal/0.2.4/embedded_hal/trait.PwmPin.html#tymethod.get_max_duty)
    pub fn as_duty<T: Num + NumCast + PartialOrd + Copy>(&self, max_duty: T) -> T {
        let max_as_float: f32 = NumCast::from(max_duty).unwrap();
        return (max_duty / max_duty)
            * NumCast::from(self.as_abs_percentage() * max_as_float).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_percentage_test() {
        assert_eq!(Pattern::FireMedium.as_percentage(), -0.59);
        assert_eq!(Pattern::Aqua.as_percentage(), 0.81);
    }

    #[test]
    fn as_abs_percentage_test() {
        assert_eq!(Pattern::Color1Larson.as_abs_percentage(), 0.495);
        assert_eq!(Pattern::Color1Chase.as_abs_percentage(), 0.505);
    }

    #[test]
    fn as_duty_test() {
        assert_eq!(Pattern::Color1Larson.as_duty(u8::MAX), 126);
    }
}
