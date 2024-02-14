use std::{
    fmt::Display,
    io,
    ops::{Deref, DerefMut},
    path::Path,
};

use svg::Document;

/// SvgAvatar represents an avatar generated from an identifier.
///
/// See also: [`SvgAvatarBuilder`].
///
/// [`SvgAvatarBuilder`]: crate::SvgAvatarBuilder
#[derive(Debug, Clone)]
pub struct SvgAvatar {
    document: Document,
}

impl SvgAvatar {
    pub(crate) fn new(document: Document) -> Self {
        Self { document }
    }

    /// Saves the SVG avatar to a file under the provided `path`.
    ///
    /// # Arguments
    ///
    /// * `path` - A path where to save the SVG avatar.
    ///
    /// # Examples
    ///
    /// ```
    /// use svg_avatars::SvgAvatarBuilder;
    ///
    /// let svg = SvgAvatarBuilder::new()
    ///     .identifier("foo")
    ///     .build();
    ///
    /// svg.save("bar.svg").unwrap();
    /// ```
    ///
    /// # Returns
    ///
    /// This method returns [`io::Error`] if it fails to save the SVG.
    ///
    /// [`io::Error`]: std::io::Error
    pub fn save(&self, path: impl AsRef<Path>) -> io::Result<()> {
        svg::save(path, &self.document)
    }
}

impl Display for SvgAvatar {
    /// Formats a string-representation of the SVG.
    ///
    /// To print the SVG on stdout you can do:
    ///
    /// ```
    /// use svg_avatars::SvgAvatarBuilder;
    ///
    /// let svg = SvgAvatarBuilder::new()
    ///     .identifier("foo")
    ///     .build();
    ///
    /// println!("{svg}");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.document.fmt(f)
    }
}

impl Deref for SvgAvatar {
    type Target = Document;

    fn deref(&self) -> &Self::Target {
        &self.document
    }
}

impl DerefMut for SvgAvatar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.document
    }
}

impl From<SvgAvatar> for String {
    fn from(value: SvgAvatar) -> Self {
        value.to_string()
    }
}

impl From<SvgAvatar> for Vec<u8> {
    fn from(value: SvgAvatar) -> Self {
        value.to_string().into_bytes()
    }
}

impl From<SvgAvatar> for Document {
    fn from(value: SvgAvatar) -> Self {
        value.document
    }
}
