use sha2::{Digest, Sha256};
use svg::{
    node::element::{path::Data, Group, Path},
    Document, Node,
};

use crate::{Rings, SvgAvatar, SvgTheme, PATHS_PROPERTIES};

/// SvgAvatarBuilder is used to configure and construct a new [`SvgAvatar`].
///
/// [`SvgAvatar`]: crate::SvgAvatar
#[derive(Debug, Clone)]
pub struct SvgAvatarBuilder {
    hasher: Sha256,
    rings: Rings,
    stroke_color: String,
}

impl SvgAvatarBuilder {
    /// Constructs a new `SvgAvatarBuilder`. It's equivalent to
    /// [`Self::default`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Updates the builder with a new identifier.
    ///
    /// Identifiers are additive. Subsequent calls to this method _appends_ to
    /// previous identifiers, in opposition to overwriting them.
    ///
    ///  See also [`Self::identifier_bytes`].
    ///
    /// # Arguments
    ///
    /// * `id` - an identifier segment used in the SVG generation process.
    ///
    /// # Examples
    ///
    /// ```
    /// use svg_avatars::SvgAvatarBuilder;
    ///
    /// let builder = SvgAvatarBuilder::new()
    ///     .identifier("foo")
    ///     .identifier("bar");
    /// ```
    pub fn identifier(self, id: impl AsRef<str>) -> Self {
        self.identifier_bytes(id.as_ref().as_bytes())
    }

    /// Updates the builder with a new byte identifier.
    ///
    /// Identifiers are additive. Subsequent calls to this method _appends_ to
    /// previous identifiers, in opposition to overwriting them.
    ///
    ///  See also [`Self::identifier`].
    ///
    /// # Arguments
    ///
    /// * `data` - a byte identifier segment used in the SVG generation process.
    ///
    /// # Examples
    ///
    /// ```
    /// use svg_avatars::SvgAvatarBuilder;
    ///
    /// let builder = SvgAvatarBuilder::new()
    ///     .identifier_bytes(vec![12u8, 42u8])
    ///     .identifier_bytes(vec![32u8, 255u8]);
    /// ```
    pub fn identifier_bytes(mut self, data: impl AsRef<[u8]>) -> Self {
        Digest::update(&mut self.hasher, data.as_ref());
        self
    }

    /// Configures a new [`Rings`] variant.
    ///
    /// # Arguments
    ///
    /// * `rings` - a new [`Rings`] variant which controls how many rings the
    /// SVG will have.
    ///
    /// # Examples
    ///
    /// ```
    /// use svg_avatars::{Rings, SvgAvatarBuilder};
    ///
    /// let builder = SvgAvatarBuilder::new()
    ///     .rings(Rings::Three);
    /// ```
    ///
    /// [`Rings`]: crate::Rings
    pub fn rings(mut self, rings: Rings) -> Self {
        self.rings = rings;
        self
    }

    /// Sets the stroke color of [path] elements.
    ///
    /// # Arguments
    ///
    /// * `color` - a new stroke color. The builder does not process the color
    /// in any way; check ["Specifying paint"] to see which values are valid.
    ///
    /// # Examples
    ///
    /// ```
    /// use svg_avatars::SvgAvatarBuilder;
    ///
    /// let builder = SvgAvatarBuilder::new()
    ///     .stroke_color("blue");
    /// ```
    ///
    /// ```
    /// use svg_avatars::SvgAvatarBuilder;
    ///
    /// let builder = SvgAvatarBuilder::new()
    ///     .stroke_color("hsla(53, 100%, 50%, 1)");
    /// ```
    ///
    /// ```
    /// use svg_avatars::SvgAvatarBuilder;
    ///
    /// let builder = SvgAvatarBuilder::new()
    ///     .stroke_color("rgb(36, 138, 71)");
    /// ```
    ///
    /// [path]: https://www.w3.org/TR/SVG2/paths.html
    /// ["Specifying paint"]: https://www.w3.org/TR/SVG2/painting.html#SpecifyingPaint
    pub fn stroke_color(mut self, color: impl Into<String>) -> Self {
        self.stroke_color = color.into();
        self
    }

    /// Builds a new [`SvgAvatar`].
    ///
    /// # Examples
    ///
    /// ```
    /// use svg_avatars::{Rings, SvgAvatar, SvgAvatarBuilder};
    ///
    /// let svg: SvgAvatar = SvgAvatarBuilder::new()
    ///     .identifier("foo")
    ///     .rings(Rings::Three)
    ///     .build();
    /// ```
    ///
    /// [`SvgAvatar`]: crate::SvgAvatar
    pub fn build(self) -> SvgAvatar {
        let hash = self.hasher.finalize();

        // Safety: This will never panic, as a SHA256 hash has exactly 32 bytes.
        let hash: [u8; 32] = hash.try_into().unwrap();
        let theme = SvgTheme::new(&hash, self.stroke_color);

        let mut g = Group::new();

        for (ring_index, divider) in self.rings.to_dividers().iter().enumerate() {
            for index in 0..8 {
                let path = Self::create_path(ring_index, index, *divider, &theme);
                g.append(path);
            }
        }

        let document = Self::create_document().add(g);
        SvgAvatar::new(document)
    }

    fn create_document() -> Document {
        Document::new().set("viewBox", (-1.1, -1.1, 2.3, 2.3))
    }

    fn create_path(ring_index: usize, index: usize, divider: f64, theme: &SvgTheme) -> Path {
        let path_properties = &PATHS_PROPERTIES[index];

        let data = Data::new()
            .move_to((0, 0))
            .line_to((
                path_properties.line_x(divider),
                path_properties.line_y(divider),
            ))
            .elliptical_arc_to((
                divider,
                divider,
                0,
                0,
                1,
                path_properties.arc_x(divider),
                path_properties.arc_y(divider),
            ))
            .close();

        let color = Self::construct_color(ring_index, index, theme);

        Path::new()
            .set("fill", color)
            .set("stroke", theme.stroke_color())
            .set("stroke-width", "0.01")
            .set("d", data)
    }

    fn construct_color(ring_index: usize, index: usize, theme: &SvgTheme) -> String {
        let color = theme.ring(ring_index, index);

        let h = color >> 4;
        let s = (color >> 2) & 0x03;
        let l = color & 0x03;

        let normalized_h = (h as f32) / 16.0;
        let normalized_s = (s as f32) / 4.0;
        let normalized_l = (l as f32) / 4.0;

        let normalized_h = 360.0 * theme.normalized_global_theme()
            + 120.0 * theme.normalized_ring_theme(ring_index)
            + 30.0 * normalized_h;
        let normalized_s = 20.0 + 80.0 * normalized_s;
        let normalized_l = 40.0 + 50.0 * normalized_l;

        format!("hsl({normalized_h}, {normalized_s}%, {normalized_l}%)")
    }
}

impl Default for SvgAvatarBuilder {
    fn default() -> Self {
        Self {
            hasher: Default::default(),
            rings: Default::default(),
            stroke_color: String::from("black"),
        }
    }
}
