use super::{RGB32, RGB64, RGB8};
use crate::color::{rgb_hsl::hsl_to_rgb, Convert, BGR32, BGR64, BGR8, HSL};

impl Convert for RGB32 {
    fn from_rgb32(_: RGB32) -> Self {
        unimplemented!();
    }

    fn from_rgb8(src: RGB8) -> Self {
        Self {
            r: Self::byte_to_percent32(src.r),
            g: Self::byte_to_percent32(src.g),
            b: Self::byte_to_percent32(src.b),
        }
    }

    fn from_rgb64(src: RGB64) -> Self {
        Self {
            r: src.r as f32,
            g: src.g as f32,
            b: src.b as f32,
        }
    }

    fn from_bgr8(src: BGR8) -> Self {
        Self {
            r: Self::byte_to_percent32(src.r),
            g: Self::byte_to_percent32(src.g),
            b: Self::byte_to_percent32(src.b),
        }
    }

    fn from_bgr32(src: BGR32) -> Self {
        Self {
            r: src.r,
            g: src.g,
            b: src.b,
        }
    }

    fn from_bgr64(src: BGR64) -> Self {
        Self {
            r: src.r as f32,
            g: src.g as f32,
            b: src.b as f32,
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
        let rgb = RGB32::from_rgb8(RGB8::from([0, 0, 255]));
        assert_eq!(rgb, RGB32::from([0.0, 0.0, 1.0]));

        let rgb = RGB32::from_rgb8(RGB8::from([0, 0, 0]));
        assert_eq!(rgb, RGB32::from([0.0, 0.0, 0.0]));

        let rgb = RGB32::from_rgb8(RGB8::from([255, 255, 255]));
        assert_eq!(rgb, RGB32::from([1.0, 1.0, 1.0]));
    }

    #[test]
    fn from_rgb64() {
        let rgb = RGB32::from_rgb64(RGB64::from([0.0, 0.0, 0.0]));
        assert_eq!(rgb, RGB32::from([0.0, 0.0, 0.0]));

        let rgb = RGB32::from_rgb64(RGB64::from([1.0, 0.5, 0.0]));
        assert_eq!(rgb, RGB32::from([1.0, 0.5, 0.0]));

        let rgb = RGB32::from_rgb64(RGB64::from([1.0, 1.0, 1.0]));
        assert_eq!(rgb, RGB32::from([1.0, 1.0, 1.0]));
    }

    #[test]
    fn from_bgr8() {
        let rgb = RGB32::from_bgr8(BGR8::from([0, 0, 255]));
        assert_eq!(rgb, RGB32::from([1.0, 0.0, 0.0]));

        let rgb = RGB32::from_bgr8(BGR8::from([0, 0, 0]));
        assert_eq!(rgb, RGB32::from([0.0, 0.0, 0.0]));

        let rgb = RGB32::from_bgr8(BGR8::from([255, 255, 255]));
        assert_eq!(rgb, RGB32::from([1.0, 1.0, 1.0]));
    }

    #[test]
    fn from_bgr32() {
        let rgb = RGB32::from_bgr32(BGR32::from([0.0, 0.0, 1.0]));
        assert_eq!(rgb, RGB32::from([1.0, 0.0, 0.0]));

        let rgb = RGB32::from_bgr32(BGR32::from([0.0, 0.0, 0.0]));
        assert_eq!(rgb, RGB32::from([0.0, 0.0, 0.0]));

        let rgb = RGB32::from_bgr32(BGR32::from([1.0, 1.0, 0.0]));
        assert_eq!(rgb, RGB32::from([0.0, 1.0, 1.0]));
    }

    #[test]
    fn from_bgr64() {
        let rgb = RGB32::from_bgr64(BGR64::from([0.0, 0.0, 0.0]));
        assert_eq!(rgb, RGB32::from([0.0, 0.0, 0.0]));

        let rgb = RGB32::from_bgr64(BGR64::from([0.0, 0.5, 1.0]));
        assert_eq!(rgb, RGB32::from([1.0, 0.5, 0.0]));

        let rgb = RGB32::from_bgr64(BGR64::from([1.0, 1.0, 1.0]));
        assert_eq!(rgb, RGB32::from([1.0, 1.0, 1.0]));
    }

    #[test]
    fn from_hsl() {
        let rgb = RGB32::from_hsl(HSL::from([0.0, 0.0, 1.0]));
        assert_eq!(rgb, RGB32::from([1.0, 1.0, 1.0]));

        let rgb = RGB32::from_hsl(HSL::from([180.0, 1.0, 0.5]));
        assert_eq!(rgb, RGB32::from([0.0, 1.0, 1.0]));

        let rgb = RGB32::from_hsl(HSL::from([0.0, 1.0, 0.5]));
        assert_eq!(rgb, RGB32::from([1.0, 0.0, 0.0]));
    }
}
