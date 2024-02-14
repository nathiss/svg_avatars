//! This crate provides functionality for generating [SVG] avatars from
//! identifiers.
//!
//! # Example
//!
//! The crate exposes a builder for constructing a new SVG avatar.
//!
//! ```
//! use svg_avatars::{Rings, SvgAvatarBuilder};
//!
//! SvgAvatarBuilder::new()
//!     .identifier("foo")
//!     .rings(Rings::Three)
//!     .build()
//!     .save("foo.svg")
//!     .unwrap();
//! ```
//!
//! This saves the `foo.svg` file with this image:
//!
//! <style id="svg-style-block">
//!     #svg-style-block + svg {
//!         width: 180px;
//!         height: 180px;
//!     }
//! </style>
//!
#![doc=include_str!("foo.svg")]
//!
//! # Note
//!
//! The above example is slightly modified --- this docblock contains a `style`
//! tag, which sets both width and height of the svg to `180px`.
//!
//! Also, if you plan to use the avatars on websites with light/dark themes, you
//! probably want to override the default stroke color. See
//! [SvgAvatarBuilder::stroke_color] for details.
//!
//! [SVG]: https://en.wikipedia.org/wiki/SVG

mod path_properties;
mod rings;
mod svg_avatar;
mod svg_avatar_builder;
mod svg_theme;

pub use rings::Rings;
pub use svg_avatar::SvgAvatar;
pub use svg_avatar_builder::SvgAvatarBuilder;

pub(crate) use path_properties::PATHS_PROPERTIES;
pub(crate) use svg_theme::SvgTheme;
