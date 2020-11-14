use super::{rgb_hsl::rgb_to_hsl, Convert, BGR32, BGR64, BGR8, RGB32, RGB64, RGB8};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct HSL {
    /// Hue
    pub h: f64,
    /// Saturation
    pub s: f64,
    /// Lightness
    pub l: f64,
}

impl Convert for HSL {
    fn from_hsl(_: HSL) -> Self {
        unimplemented!();
    }

    fn from_rgb8(src: RGB8) -> Self {
        let rgb = RGB64::from_rgb8(src);
        rgb_to_hsl(rgb.r, rgb.g, rgb.b)
    }

    fn from_rgb32(src: RGB32) -> Self {
        let rgb = RGB64::from_rgb32(src);
        rgb_to_hsl(rgb.r, rgb.g, rgb.b)
    }

    fn from_bgr8(src: BGR8) -> Self {
        let rgb = RGB64::from_bgr8(src);
        rgb_to_hsl(rgb.r, rgb.g, rgb.b)
    }

    fn from_bgr32(src: BGR32) -> Self {
        let rgb = RGB64::from_bgr32(src);
        rgb_to_hsl(rgb.r, rgb.g, rgb.b)
    }

    fn from_rgb64(src: RGB64) -> Self {
        rgb_to_hsl(src.r, src.g, src.b)
    }

    fn from_bgr64(src: BGR64) -> Self {
        rgb_to_hsl(src.r, src.g, src.b)
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

    #[test]
    fn from_rgb8() {
        let hsl: HSL = HSL::from_rgb8(RGB8::from([255, 255, 255]));
        assert_eq!(hsl, HSL::from([0.0, 0.0, 1.0]));

        let hsl: HSL = HSL::from_rgb8(RGB8::from([0, 0, 0]));
        assert_eq!(hsl, HSL::from([0.0, 0.0, 0.0]));

        let hsl: HSL = HSL::from_rgb8(RGB8::from([0, 0, 255]));
        assert_eq!(hsl, HSL::from([240.0, 1.0, 0.5]));
    }

    #[test]
    fn from_rgb32() {
        let hsl: HSL = HSL::from_rgb32(RGB32::from([0.5, 1.0, 0.0]));
        assert_eq!(hsl, HSL::from([90.0, 1.0, 0.5]));

        let hsl: HSL = HSL::from_rgb32(RGB32::from([0.25, 0.75, 0.25]));
        assert_eq!(hsl, HSL::from([120.0, 0.5, 0.5]));

        let hsl: HSL = HSL::from_rgb32(RGB32::from([1.0, 0.0, 0.0]));
        assert_eq!(hsl, HSL::from([0.0, 1.0, 0.5]));
    }

    #[test]
    fn from_rgb64() {
        let hsl: HSL = HSL::from_rgb64(RGB64::from([0.5, 1.0, 0.0]));
        assert_eq!(hsl, HSL::from([90.0, 1.0, 0.5]));

        let hsl: HSL = HSL::from_rgb64(RGB64::from([0.25, 0.75, 0.25]));
        assert_eq!(hsl, HSL::from([120.0, 0.5, 0.5]));

        let hsl: HSL = HSL::from_rgb64(RGB64::from([1.0, 0.0, 0.0]));
        assert_eq!(hsl, HSL::from([0.0, 1.0, 0.5]));
    }

    #[test]
    fn from_bgr8() {
        let hsl: HSL = HSL::from_bgr8(BGR8::from([255, 255, 255]));
        assert_eq!(hsl, HSL::from([0.0, 0.0, 1.0]));

        let hsl: HSL = HSL::from_bgr8(BGR8::from([0, 0, 0]));
        assert_eq!(hsl, HSL::from([0.0, 0.0, 0.0]));

        let hsl: HSL = HSL::from_bgr8(BGR8::from([255, 0, 0]));
        assert_eq!(hsl, HSL::from([240.0, 1.0, 0.5]));
    }

    #[test]
    fn from_bgr32() {
        let hsl: HSL = HSL::from_bgr32(BGR32::from([0.0, 1.0, 0.5]));
        assert_eq!(hsl, HSL::from([90.0, 1.0, 0.5]));

        let hsl: HSL = HSL::from_bgr32(BGR32::from([0.25, 0.75, 0.25]));
        assert_eq!(hsl, HSL::from([120.0, 0.5, 0.5]));

        let hsl: HSL = HSL::from_bgr32(BGR32::from([0.0, 0.0, 1.0]));
        assert_eq!(hsl, HSL::from([0.0, 1.0, 0.5]));
    }

    #[test]
    fn from_bgr64() {
        let hsl: HSL = HSL::from_bgr64(BGR64::from([0.0, 1.0, 0.5]));
        assert_eq!(hsl, HSL::from([90.0, 1.0, 0.5]));

        let hsl: HSL = HSL::from_bgr64(BGR64::from([0.25, 0.75, 0.25]));
        assert_eq!(hsl, HSL::from([120.0, 0.5, 0.5]));

        let hsl: HSL = HSL::from_bgr64(BGR64::from([0.0, 0.0, 1.0]));
        assert_eq!(hsl, HSL::from([0.0, 1.0, 0.5]));
    }
}
