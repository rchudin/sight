mod rgb32;
mod rgb64;
mod rgb8;

use super::ComponentsCount;

pub type RGB8 = RGB<u8>;
pub type RGB32 = RGB<f32>;
pub type RGB64 = RGB<f64>;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct RGB<T> {
    /// Red
    pub r: T,
    /// Green
    pub g: T,
    /// Blue
    pub b: T,
}

impl<T> RGB<T> {
    pub fn percent32_to_byte(percent: f32) -> u8 {
        (percent * 255_f32).round() as u8
    }

    pub fn percent64_to_byte(percent: f64) -> u8 {
        (percent * 255_f64).round() as u8
    }

    pub fn byte_to_percent32(byte: u8) -> f32 {
        byte as f32 / 255_f32
    }

    pub fn byte_to_percent64(byte: u8) -> f64 {
        byte as f64 / 255_f64
    }
}

impl<T> ComponentsCount for RGB<T> {
    type Component = T;

    fn count() -> usize {
        3
    }
}

impl<T: Copy> From<[T; 3]> for RGB<T> {
    fn from(src: [T; 3]) -> Self {
        Self {
            r: src[0],
            g: src[1],
            b: src[2],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percent32_to_byte() {
        let percent: f32 = 0.5;
        let byte: u8 = RGB64::percent32_to_byte(percent);
        assert_eq!(byte, 128);
    }

    #[test]
    fn percent64_to_byte() {
        let percent: f64 = 0.25;
        let byte: u8 = RGB8::percent64_to_byte(percent);
        assert_eq!(byte, 64);
    }

    #[test]
    fn byte_to_percent32() {
        let byte: u8 = 255;
        let percent: f32 = RGB32::byte_to_percent32(byte);
        assert_eq!(percent, 1.0);
    }

    #[test]
    fn byte_to_percent64() {
        let byte: u8 = 0;
        let percent: f64 = RGB8::byte_to_percent64(byte);
        assert_eq!(percent, 0.0);
    }

    #[test]
    fn partial_eq() {
        let rgb1 = RGB::from([1.0, 1.0, 1.0]);
        let rgb2 = RGB::from([1.0, 1.0, 1.0]);
        assert_eq!(rgb1, rgb2);
        let rgb2 = RGB::from([0.0, 1.0, 1.0]);
        assert_ne!(rgb1, rgb2);
        let rgb2 = RGB::from([1.0, 0.0, 1.0]);
        assert_ne!(rgb1, rgb2);
        let rgb2 = RGB::from([1.0, 1.0, 0.0]);
        assert_ne!(rgb1, rgb2);
        let rgb2 = RGB::from([0.0, 0.0, 0.0]);
        assert_ne!(rgb1, rgb2);
    }
}
