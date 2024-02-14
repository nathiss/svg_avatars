#[derive(Debug)]
pub(crate) struct SvgTheme<'a> {
    rings: &'a [u8],
    normalized_global_theme: f32,
    normalized_ring_themes: Vec<f32>,
    stroke_color: String,
}

impl<'a> SvgTheme<'a> {
    pub(crate) fn new(hash: &'a [u8; 32], stroke_color: impl Into<String>) -> Self {
        let normalized_ring_themes = hash
            .windows(8)
            .map(Self::calculate_theme)
            .map(|ring_theme| (ring_theme as f32) / (u8::MAX as f32))
            .collect::<Vec<_>>();

        let global_theme = Self::calculate_theme(hash.as_ref());
        let normalized_global_theme = (global_theme as f32) / (u8::MAX as f32);

        Self {
            rings: hash,
            normalized_global_theme,
            normalized_ring_themes,
            stroke_color: stroke_color.into(),
        }
    }

    #[inline(always)]
    pub(crate) fn ring(&self, ring_index: usize, index: usize) -> u8 {
        self.rings[ring_index * 8 + index]
    }

    #[inline(always)]
    pub(crate) fn normalized_global_theme(&self) -> f32 {
        self.normalized_global_theme
    }

    #[inline(always)]
    pub(crate) fn normalized_ring_theme(&self, index: usize) -> f32 {
        self.normalized_ring_themes[index]
    }

    #[inline(always)]
    pub(crate) fn stroke_color(&self) -> &str {
        &self.stroke_color
    }

    #[inline(always)]
    fn calculate_theme(hash: &[u8]) -> u8 {
        hash.iter().fold(0u8, |acc, h| acc ^ h)
    }
}
