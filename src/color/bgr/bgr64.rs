use super::{BGR32, BGR64, BGR8};
use crate::color::{rgb_hsl::hsl_to_rgb, Convert, HSL, RGB32, RGB64, RGB8};

impl Convert for BGR64 {
    fn from_bgr64(_: BGR64) -> Self {
        unimplemented!();
    }

    fn from_rgb8(src: RGB8) -> Self {
        Self {
            r: Self::byte_to_percent64(src.r),
            g: Self::byte_to_percent64(src.g),
            b: Self::byte_to_percent64(src.b),
        }
    }

    fn from_rgb32(src: RGB32) -> Self {
        Self {
            r: src.r as f64,
            g: src.g as f64,
            b: src.b as f64,
        }
    }

    fn from_rgb64(src: RGB64) -> Self {
        Self {
            r: src.r,
            g: src.g,
            b: src.b,
        }
    }

    fn from_bgr8(src: BGR8) -> Self {
        Self {
            r: Self::byte_to_percent64(src.r),
            g: Self::byte_to_percent64(src.g),
            b: Self::byte_to_percent64(src.b),
        }
    }

    fn from_bgr32(src: BGR32) -> Self {
        Self {
            r: src.r as f64,
            g: src.g as f64,
            b: src.b as f64,
        }
    }

    fn from_hsl(src: HSL) -> Self {
        Self::from_rgb64(hsl_to_rgb(src))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_rgb8() {
        let bgr = BGR64::from_rgb8(RGB8::from([0, 0, 255]));
        assert_eq!(bgr, BGR64::from([1.0, 0.0, 0.0]));

        let bgr = BGR64::from_rgb8(RGB8::from([0, 0, 0]));
        assert_eq!(bgr, BGR64::from([0.0, 0.0, 0.0]));

        let bgr = BGR64::from_rgb8(RGB8::from([255, 255, 0]));
        assert_eq!(bgr, BGR64::from([0.0, 1.0, 1.0]));
    }

    #[test]
    fn from_rgb32() {
        let bgr = BGR64::from_rgb32(RGB32::from([1.0, 0.0, 0.0]));
        assert_eq!(bgr, BGR64::from([0.0, 0.0, 1.0]));

        let bgr = BGR64::from_rgb32(RGB32::from([1.0, 1.0, 1.0]));
        assert_eq!(bgr, BGR64::from([1.0, 1.0, 1.0]));

        let bgr = BGR64::from_rgb32(RGB32::from([0.0, 1.0, 1.0]));
        assert_eq!(bgr, BGR64::from([1.0, 1.0, 0.0]));
    }

    #[test]
    fn from_bgr8() {
        let bgr = BGR64::from_bgr8(BGR8::from([255, 0, 0]));
        assert_eq!(bgr, BGR64::from([1.0, 0.0, 0.0]));

        let bgr = BGR64::from_bgr8(BGR8::from([0, 0, 0]));
        assert_eq!(bgr, BGR64::from([0.0, 0.0, 0.0]));

        let bgr = BGR64::from_bgr8(BGR8::from([0, 255, 255]));
        assert_eq!(bgr, BGR64::from([0.0, 1.0, 1.0]));
    }

    #[test]
    fn from_bgr32() {
        let bgr = BGR64::from_bgr32(BGR32::from([1.0, 0.0, 0.0]));
        assert_eq!(bgr, BGR64::from([1.0, 0.0, 0.0]));

        let bgr = BGR64::from_bgr32(BGR32::from([1.0, 1.0, 1.0]));
        assert_eq!(bgr, BGR64::from([1.0, 1.0, 1.0]));

        let bgr = BGR64::from_bgr32(BGR32::from([0.0, 0.5, 1.0]));
        assert_eq!(bgr, BGR64::from([0.0, 0.5, 1.0]));
    }

    #[test]
    fn from_hsl() {
        let bgr = BGR64::from_hsl(HSL::from([0.0, 0.0, 1.0]));
        assert_eq!(bgr, BGR64::from([1.0, 1.0, 1.0]));

        let bgr = BGR64::from_hsl(HSL::from([0.0, 0.0, 0.0]));
        assert_eq!(bgr, BGR64::from([0.0, 0.0, 0.0]));

        let bgr = BGR64::from_hsl(HSL::from([0.0, 1.0, 0.5]));
        assert_eq!(bgr, BGR64::from([0.0, 0.0, 1.0]));
    }
}
