mod bgr32;
mod bgr64;
mod bgr8;

use super::ComponentsCount;

pub type BGR8 = BGR<u8>;
pub type BGR32 = BGR<f32>;
pub type BGR64 = BGR<f64>;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct BGR<T> {
    /// Blue
    pub b: T,
    /// Green
    pub g: T,
    /// Red
    pub r: T,
}

impl<T> BGR<T> {
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

impl<T> ComponentsCount for BGR<T> {
    type Component = T;

    fn count() -> usize {
        3
    }
}

impl<T: Copy> From<[T; 3]> for BGR<T> {
    fn from(src: [T; 3]) -> Self {
        Self {
            b: src[0],
            g: src[1],
            r: src[2],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percent32_to_byte() {
        let percent: f32 = 0.5;
        let byte: u8 = BGR64::percent32_to_byte(percent);
        assert_eq!(byte, 128);
    }

    #[test]
    fn percent64_to_byte() {
        let percent: f64 = 0.25;
        let byte: u8 = BGR8::percent64_to_byte(percent);
        assert_eq!(byte, 64);
    }

    #[test]
    fn byte_to_percent32() {
        let byte: u8 = 255;
        let percent: f32 = BGR32::byte_to_percent32(byte);
        assert_eq!(percent, 1.0);
    }

    #[test]
    fn byte_to_percent64() {
        let byte: u8 = 0;
        let percent: f64 = BGR8::byte_to_percent64(byte);
        assert_eq!(percent, 0.0);
    }

    #[test]
    fn partial_eq() {
        let bgr1 = BGR::from([1.0, 1.0, 1.0]);
        let bgr2 = BGR::from([1.0, 1.0, 1.0]);
        assert_eq!(bgr1, bgr2);
        let bgr2 = BGR::from([0.0, 1.0, 1.0]);
        assert_ne!(bgr1, bgr2);
        let bgr2 = BGR::from([1.0, 0.0, 1.0]);
        assert_ne!(bgr1, bgr2);
        let bgr2 = BGR::from([1.0, 1.0, 0.0]);
        assert_ne!(bgr1, bgr2);
        let bgr2 = BGR::from([0.0, 0.0, 0.0]);
        assert_ne!(bgr1, bgr2);
    }
}
