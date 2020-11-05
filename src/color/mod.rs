pub(crate) mod rgb_hsl;

pub(crate) mod rgb;
pub use rgb::{RGB32, RGB64, RGB8};
mod hsl;
pub use hsl::HSL;

pub trait Convert {
    fn from_rgb8(src: RGB8) -> Self;

    fn from_rgb32(src: RGB32) -> Self;

    fn from_rgb64(src: RGB64) -> Self;

    fn from_hsl(src: HSL) -> Self;
}

macro_rules! convert {
    ($T:ty, RGB8) => {
        impl From<RGB8> for $T {
            fn from(src: RGB8) -> Self {
                Self::from_rgb8(src)
            }
        }
    };

    ($T:ty, RGB32) => {
        impl From<RGB32> for $T {
            fn from(src: RGB32) -> Self {
                Self::from_rgb32(src)
            }
        }
    };

    ($T:ty, RGB64) => {
        impl From<RGB64> for $T {
            fn from(src: RGB64) -> Self {
                Self::from_rgb64(src)
            }
        }
    };

    ($T:ty, HSL) => {
        impl From<HSL> for $T {
            fn from(src: HSL) -> Self {
                Self::from_hsl(src)
            }
        }
    };
}

convert!(RGB8, HSL);
convert!(RGB8, RGB32);
convert!(RGB8, RGB64);

convert!(RGB32, RGB8);
convert!(RGB32, RGB64);
convert!(RGB32, HSL);

convert!(RGB64, RGB8);
convert!(RGB64, RGB32);
convert!(RGB64, HSL);
