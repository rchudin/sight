use super::{BGR32, BGR64, BGR8};
use crate::color::{rgb_hsl::hsl_to_rgb, Convert, HSL, RGB32, RGB64, RGB8};

impl Convert for BGR8 {
    fn from_bgr8(_: BGR8) -> Self {
        unimplemented!();
    }

    fn from_rgb8(src: RGB8) -> Self {
        Self {
            r: src.r,
            g: src.g,
            b: src.b,
        }
    }

    fn from_rgb32(src: RGB32) -> Self {
        Self {
            r: Self::percent32_to_byte(src.r),
            g: Self::percent32_to_byte(src.g),
            b: Self::percent32_to_byte(src.b),
        }
    }

    fn from_rgb64(src: RGB64) -> Self {
        Self {
            r: Self::percent64_to_byte(src.r),
            g: Self::percent64_to_byte(src.g),
            b: Self::percent64_to_byte(src.b),
        }
    }

    fn from_bgr32(src: BGR32) -> Self {
        Self {
            r: Self::percent32_to_byte(src.r),
            g: Self::percent32_to_byte(src.g),
            b: Self::percent32_to_byte(src.b),
        }
    }

    fn from_bgr64(src: BGR64) -> Self {
        Self {
            r: Self::percent64_to_byte(src.r),
            g: Self::percent64_to_byte(src.g),
            b: Self::percent64_to_byte(src.b),
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
        let bgr = BGR8::from_rgb8(RGB8::from([255, 0, 0]));
        assert_eq!(bgr, BGR8::from([0, 0, 255]));

        let bgr = BGR8::from_rgb8(RGB8::from([0, 255, 255]));
        assert_eq!(bgr, BGR8::from([255, 255, 0]));

        let bgr = BGR8::from_rgb8(RGB8::from([255, 0, 255]));
        assert_eq!(bgr, BGR8::from([255, 0, 255]));
    }

    #[test]
    fn from_rgb32() {
        let bgr = BGR8::from_rgb32(RGB32::from([1.0, 0.0, 0.0]));
        assert_eq!(bgr, BGR8::from([0, 0, 255]));

        let bgr = BGR8::from_rgb32(RGB32::from([0.0, 1.0, 0.5]));
        assert_eq!(bgr, BGR8::from([128, 255, 0]));

        let bgr = BGR8::from_rgb32(RGB32::from([0.0, 0.0, 0.0]));
        assert_eq!(bgr, BGR8::from([0, 0, 0]));
    }

    #[test]
    fn from_rgb64() {
        let bgr = BGR8::from_rgb64(RGB64::from([1.0, 0.0, 0.0]));
        assert_eq!(bgr, BGR8::from([0, 0, 255]));

        let bgr = BGR8::from_rgb64(RGB64::from([0.0, 1.0, 0.5]));
        assert_eq!(bgr, BGR8::from([128, 255, 0]));

        let bgr = BGR8::from_rgb64(RGB64::from([0.0, 0.0, 0.0]));
        assert_eq!(bgr, BGR8::from([0, 0, 0]));
    }

    #[test]
    fn from_bgr32() {
        let bgr = BGR8::from_bgr32(BGR32::from([1.0, 0.0, 0.0]));
        assert_eq!(bgr, BGR8::from([255, 0, 0]));

        let bgr = BGR8::from_bgr32(BGR32::from([0.0, 1.0, 0.5]));
        assert_eq!(bgr, BGR8::from([0, 255, 128]));

        let bgr = BGR8::from_bgr32(BGR32::from([0.0, 0.0, 0.0]));
        assert_eq!(bgr, BGR8::from([0, 0, 0]));
    }

    #[test]
    fn from_bgr64() {
        let bgr = BGR8::from_bgr64(BGR64::from([1.0, 0.0, 0.0]));
        assert_eq!(bgr, BGR8::from([255, 0, 0]));

        let bgr = BGR8::from_bgr64(BGR64::from([0.0, 1.0, 0.5]));
        assert_eq!(bgr, BGR8::from([0, 255, 128]));

        let bgr = BGR8::from_bgr64(BGR64::from([0.0, 0.0, 1.0]));
        assert_eq!(bgr, BGR8::from([0, 0, 255]));
    }

    #[test]
    fn from_hsl() {
        let bgr = BGR8::from_hsl(HSL::from([300.0, 1.0, 0.25]));
        assert_eq!(bgr, BGR8::from([127, 0, 128]));

        let bgr = BGR8::from_hsl(HSL::from([240.0, 1.0, 0.3]));
        assert_eq!(bgr, BGR8::from([153, 0, 0]));

        let bgr = BGR8::from_hsl(HSL::from([0.0, 0.0, 0.5]));
        assert_eq!(bgr, BGR8::from([128, 128, 128]));
    }
}
