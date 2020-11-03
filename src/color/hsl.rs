use super::{rgb_hsl::rgb_to_hsl, Convert, RGB32, RGB64, RGB8};

// Hue | Saturation | Lightness
#[derive(Default, Debug, Clone, Copy)]
pub struct HSL {
    pub h: f64,
    pub s: f64,
    pub l: f64,
}

impl PartialEq for HSL {
    fn eq(&self, other: &Self) -> bool {
        self.h == other.h && self.s == other.s && self.l == other.l
    }
}

impl Convert for HSL {
    fn from_hsl(_src: HSL) -> Self {
        unimplemented!();
    }

    fn from_rgb8(_src: RGB8) -> Self {
        // rgb_to_hsl(src.into())
        unimplemented!();
    }

    fn from_rgb32(_src: RGB32) -> Self {
        unimplemented!();
        // rgb_to_hsl(src.into())
    }

    fn from_rgb64(src: RGB64) -> Self {
        rgb_to_hsl(src)
    }
}

impl From<[f64; 3]> for HSL {
    fn from(src: [f64; 3]) -> Self {
        Self {
            h: src[0],
            s: src[1],
            l: src[2],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_rgb64() {
        let rgb: RGB64 = RGB64::from([0.5, 1.0, 0.0]);
        let hsl: HSL = HSL::from_rgb64(rgb);
        let (h, s, l) = (90.0, 1.0, 0.5);
        assert_eq!(hsl, HSL { h, s, l });

        let rgb: RGB64 = RGB64::from([0.25, 0.75, 0.25]);
        let hsl: HSL = HSL::from_rgb64(rgb);
        let (h, s, l) = (120.0, 0.5, 0.5);
        assert_eq!(hsl, HSL { h, s, l });

        let rgb: RGB64 = RGB64::from([1.0, 0.0, 0.0]);
        let hsl: HSL = HSL::from_rgb64(rgb);
        let (h, s, l) = (0.0, 1.0, 0.5);
        assert_eq!(hsl, HSL { h, s, l });
    }

    #[test]
    fn partial_eq() {
        let rgb1 = HSL::from([1.0, 1.0, 1.0]);
        let rgb2 = HSL::from([1.0, 1.0, 1.0]);
        assert_eq!(rgb1, rgb2);
        let rgb2 = HSL::from([0.0, 1.0, 1.0]);
        assert_ne!(rgb1, rgb2);
        let rgb2 = HSL::from([1.0, 0.0, 1.0]);
        assert_ne!(rgb1, rgb2);
        let rgb2 = HSL::from([1.0, 1.0, 0.0]);
        assert_ne!(rgb1, rgb2);
        let rgb2 = HSL::from([0.0, 0.0, 0.0]);
        assert_ne!(rgb1, rgb2);
    }
}
