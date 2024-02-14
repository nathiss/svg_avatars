use std::f64::consts::FRAC_1_SQRT_2;

pub(crate) const PATHS_PROPERTIES: [PathProperties; 8] = [
    PathProperties::new(0.0, -1.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
    PathProperties::new(FRAC_1_SQRT_2, -FRAC_1_SQRT_2, 1.0, 0.0),
    PathProperties::new(1.0, 0.0, FRAC_1_SQRT_2, FRAC_1_SQRT_2),
    PathProperties::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.0, 1.0),
    PathProperties::new(0.0, 1.0, -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
    PathProperties::new(-FRAC_1_SQRT_2, FRAC_1_SQRT_2, -1.0, 0.0),
    PathProperties::new(-1.0, 0.0, -FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
    PathProperties::new(-FRAC_1_SQRT_2, -FRAC_1_SQRT_2, -0.0, -1.0),
];

pub(crate) struct PathProperties {
    line_x: f64,
    line_y: f64,
    arc_x: f64,
    arc_y: f64,
}

impl PathProperties {
    const fn new(line_x: f64, line_y: f64, arc_x: f64, arc_y: f64) -> Self {
        Self {
            line_x,
            line_y,
            arc_x,
            arc_y,
        }
    }

    #[inline(always)]
    pub(crate) fn line_x(&self, divider: f64) -> f64 {
        self.line_x * divider
    }

    #[inline(always)]
    pub(crate) fn line_y(&self, divider: f64) -> f64 {
        self.line_y * divider
    }

    #[inline(always)]
    pub(crate) fn arc_x(&self, divider: f64) -> f64 {
        self.arc_x * divider
    }

    #[inline(always)]
    pub(crate) fn arc_y(&self, divider: f64) -> f64 {
        self.arc_y * divider
    }
}
