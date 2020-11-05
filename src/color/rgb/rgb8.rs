use super::{RGB32, RGB64, RGB8};
use crate::color::{rgb_hsl::hsl_to_rgb, Convert, HSL};

impl Convert for RGB8 {
    fn from_rgb8(_: RGB8) -> Self {
        unimplemented!();
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

    fn from_hsl(src: HSL) -> Self {
        Self::from_rgb64(hsl_to_rgb(src))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_rgb32() {
        let rgb = RGB8::from_rgb32(RGB32::from([1.0, 0.5, 0.0]));
        assert_eq!(rgb, RGB8::from([255, 128, 0]));

        let rgb = RGB8::from_rgb32(RGB32::from([0.5, 0.0, 1.0]));
        assert_eq!(rgb, RGB8::from([128, 0, 255]));

        let rgb = RGB8::from_rgb32(RGB32::from([0.0, 1.0, 0.25]));
        assert_eq!(rgb, RGB8::from([0, 255, 64]));
    }

    #[test]
    fn from_rgb64() {
        let rgb = RGB8::from_rgb64(RGB64::from([0.0, 1.0, 0.5]));
        assert_eq!(rgb, RGB8::from([0, 255, 128]));

        let rgb = RGB8::from_rgb64(RGB64::from([0.0, 0.0, 0.0]));
        assert_eq!(rgb, RGB8::from([0, 0, 0]));

        let rgb = RGB8::from_rgb64(RGB64::from([0.0, 1.0, 0.25]));
        assert_eq!(rgb, RGB8::from([0, 255, 64]));
    }

    #[test]
    fn from_hsl() {
        let rgb = RGB8::from_hsl(HSL::from([300.0, 1.0, 0.25]));
        assert_eq!(rgb, RGB8::from([128, 0, 127]));

        let rgb = RGB8::from_hsl(HSL::from([240.0, 1.0, 0.3]));
        assert_eq!(rgb, RGB8::from([0, 0, 153]));

        let rgb = RGB8::from_hsl(HSL::from([0.0, 0.0, 0.5]));
        assert_eq!(rgb, RGB8::from([128, 128, 128]));
    }
}
