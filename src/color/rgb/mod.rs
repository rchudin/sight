mod rgb32;
mod rgb64;
mod rgb8;

use super::ComponentsCount;
use std::ops::{Add, Div, Mul};

pub type RGB8 = RGB<u8>;
pub type RGB32 = RGB<f32>;
pub type RGB64 = RGB<f64>;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct RGB<T: Copy> {
    /// Red
    pub r: T,
    /// Green
    pub g: T,
    /// Blue
    pub b: T,
}

impl<T: Copy> RGB<T> {
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

impl<T> Add for RGB<T>
where
    T: Copy + Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl<T> Add<T> for RGB<T>
where
    T: Copy + Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        Self::Output {
            r: self.r + rhs,
            g: self.g + rhs,
            b: self.b + rhs,
        }
    }
}

impl<T> Mul for RGB<T>
where
    T: Copy + Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl<T> Mul<T> for RGB<T>
where
    T: Copy + Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self::Output {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl<T> Div for RGB<T>
where
    T: Copy + Div<Output = T>,
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r / rhs.r,
            g: self.g / rhs.g,
            b: self.b / rhs.b,
        }
    }
}

impl<T> Div<T> for RGB<T>
where
    T: Copy + Div<Output = T>,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self::Output {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}

impl<T: Copy> ComponentsCount for RGB<T> {
    type Component = T;

    fn components_count() -> usize {
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

    #[test]
    fn add() {
        let rgb: RGB<i32> = RGB { r: 0, g: 0, b: 0 };
        let rth: RGB<i32> = RGB { r: 0, g: 0, b: 0 };
        assert_eq!(rgb.add(rth), RGB { r: 0, g: 0, b: 0 });

        let rgb: RGB<f32> = RGB {
            r: 1.0,
            g: 5.0,
            b: 0.0,
        };
        let rth: RGB<f32> = RGB {
            r: 45.0,
            g: 10.0,
            b: 17.0,
        };
        assert_eq!(
            rgb.add(rth),
            RGB {
                r: 46.0,
                g: 15.0,
                b: 17.0
            }
        );

        let rgb: RGB<u8> = RGB { r: 0, g: 0, b: 0 };
        assert_eq!(rgb.add(7), RGB { r: 7, g: 7, b: 7 });

        let rgb: RGB<u8> = RGB { r: 0, g: 2, b: 4 };
        assert_eq!(rgb.add(2), RGB { r: 2, g: 4, b: 6 });
    }

    #[test]
    fn mul() {
        let rgb: RGB<u8> = RGB { r: 0, g: 0, b: 0 };
        let rth: RGB<u8> = RGB { r: 0, g: 0, b: 0 };
        assert_eq!(rgb.mul(rth), RGB { r: 0, g: 0, b: 0 });

        let rgb: RGB<u8> = RGB { r: 1, g: 5, b: 0 };
        let rth: RGB<u8> = RGB {
            r: 45,
            g: 10,
            b: 17,
        };
        assert_eq!(rgb.mul(rth), RGB { r: 45, g: 50, b: 0 });

        let rgb: RGB<f64> = RGB {
            r: 1.0,
            g: 5.0,
            b: 0.0,
        };
        let rth: RGB<f64> = RGB {
            r: 45.0,
            g: 10.0,
            b: 17.0,
        };
        assert_eq!(
            rgb.mul(rth),
            RGB {
                r: 45.0,
                g: 50.0,
                b: 0.0
            }
        );

        let rgb: RGB<u8> = RGB { r: 0, g: 0, b: 0 };
        assert_eq!(rgb.mul(7), RGB { r: 0, g: 0, b: 0 });

        let rgb: RGB<u8> = RGB { r: 0, g: 2, b: 4 };
        assert_eq!(rgb.mul(2), RGB { r: 0, g: 4, b: 8 });
    }

    #[test]
    fn div() {
        let rgb: RGB<i32> = RGB { r: 0, g: 0, b: 0 };
        let rth: RGB<i32> = RGB { r: 8, g: 4, b: 4 };
        assert_eq!(rgb.div(rth), RGB { r: 0, g: 0, b: 0 });

        let rgb: RGB<f32> = RGB {
            r: 6.0,
            g: 4.0,
            b: 3.0,
        };
        let rth: RGB<f32> = RGB {
            r: 2.0,
            g: 4.0,
            b: 2.0,
        };
        assert_eq!(
            rgb.div(rth),
            RGB {
                r: 3.0,
                g: 1.0,
                b: 1.5
            }
        );

        let rgb: RGB<u8> = RGB {
            r: 19,
            g: 20,
            b: 44,
        };
        assert_eq!(rgb.div(2), RGB { r: 9, g: 10, b: 22 });

        let rgb: RGB<u8> = RGB { r: 10, g: 25, b: 5 };
        assert_eq!(rgb.div(5), RGB { r: 2, g: 5, b: 1 });
    }
}
