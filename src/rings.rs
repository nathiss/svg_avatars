use std::{f64::consts::FRAC_1_SQRT_2, fmt::Display};

/// sqrt(1/3)
const SQRT_FRAC_1_3: f64 = 0.5773502691896258;

/// sqrt(2/3)
const SQRT_FRAC_2_3: f64 = 0.816496580927726;

/// sqrt(3/4)
const SQRT_FRAC_3_4: f64 = 0.8660254037844386;

/// sqrt(1/4)
const SQRT_FRAC_1_4: f64 = 0.5;

const ONE_DIVIDER: [f64; 1] = [1.0];
const TWO_DIVIDERS: [f64; 2] = [1.0, (FRAC_1_SQRT_2 + 0.5) / 2.0];
const THREE_DIVIDERS: [f64; 3] = [
    1.0,
    (SQRT_FRAC_2_3 + (2.0 / 3.0)) / 2.0,
    (SQRT_FRAC_1_3 + (1.0 / 3.0)) / 2.0,
];
const FOUR_DIVIDERS: [f64; 4] = [
    1.0,
    (SQRT_FRAC_3_4 + 0.75) / 2.0,
    (FRAC_1_SQRT_2 + 0.5) / 2.0,
    (SQRT_FRAC_1_4 + 0.25) / 2.0,
];

/// Rings variants control the number of rings generated in avatars.
#[non_exhaustive]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rings {
    /// `One` configures [`SvgAvatarBuilder`] to generate an avatar with a
    /// single ring.
    ///
    /// [`SvgAvatarBuilder`]: crate::SvgAvatarBuilder
    One = 1,

    /// `Two` configures [`SvgAvatarBuilder`] to generate an avatar with two
    /// rings.
    ///
    /// [`SvgAvatarBuilder`]: crate::SvgAvatarBuilder
    Two = 2,

    /// `Three` configures [`SvgAvatarBuilder`] to generate an avatar with three
    /// rings.
    ///
    /// [`SvgAvatarBuilder`]: crate::SvgAvatarBuilder
    Three = 3,

    /// `Four` configures [`SvgAvatarBuilder`] to generate an avatar with four
    /// rings.
    ///
    /// This is the default option.
    ///
    /// [`SvgAvatarBuilder`]: crate::SvgAvatarBuilder
    Four = 4,
}

impl Rings {
    pub(crate) fn to_dividers(&self) -> &'static [f64] {
        match self {
            Rings::One => &ONE_DIVIDER,
            Rings::Two => &TWO_DIVIDERS,
            Rings::Three => &THREE_DIVIDERS,
            Rings::Four => &FOUR_DIVIDERS,
        }
    }
}

impl Display for Rings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rings::One => write!(f, "One"),
            Rings::Two => write!(f, "Two"),
            Rings::Three => write!(f, "Three"),
            Rings::Four => write!(f, "Four"),
        }
    }
}

impl Default for Rings {
    /// Returns the [`Self::Four`] variant.
    fn default() -> Self {
        Self::Four
    }
}
