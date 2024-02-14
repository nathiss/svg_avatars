use std::fmt::Display;

const ONE_DIVIDER: [f64; 1] = [1.0];
const TWO_DIVIDERS: [f64; 2] = [1.0, 0.65];
const THREE_DIVIDERS: [f64; 3] = [1.0, 0.77, 0.50];
const FOUR_DIVIDERS: [f64; 4] = [1.0, 0.77, 0.55, 0.3];

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
