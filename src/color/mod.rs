pub(crate) mod rgb_hsl;

pub(crate) mod bgr;
pub use bgr::{BGR32, BGR64, BGR8};

pub(crate) mod rgb;
pub use rgb::{RGB32, RGB64, RGB8};

mod hsl;
pub use hsl::HSL;

pub trait Convert {
    fn from_rgb8(_: RGB8) -> Self;

    fn from_rgb32(_: RGB32) -> Self;

    fn from_rgb64(_: RGB64) -> Self;

    fn from_bgr8(_: BGR8) -> Self;

    fn from_bgr32(_: BGR32) -> Self;

    fn from_bgr64(_: BGR64) -> Self;

    fn from_hsl(_: HSL) -> Self;
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

    ($T:ty, BGR8) => {
        impl From<BGR8> for $T {
            fn from(src: BGR8) -> Self {
                Self::from_bgr8(src)
            }
        }
    };

    ($T:ty, BGR32) => {
        impl From<BGR32> for $T {
            fn from(src: BGR32) -> Self {
                Self::from_bgr32(src)
            }
        }
    };

    ($T:ty, BGR64) => {
        impl From<BGR64> for $T {
            fn from(src: BGR64) -> Self {
                Self::from_bgr64(src)
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

convert!(RGB8, RGB32);
convert!(RGB8, RGB64);
convert!(RGB8, BGR8);
convert!(RGB8, BGR32);
convert!(RGB8, BGR64);
convert!(RGB8, HSL);

convert!(RGB32, RGB8);
convert!(RGB32, RGB64);
convert!(RGB32, BGR8);
convert!(RGB32, BGR32);
convert!(RGB32, BGR64);
convert!(RGB32, HSL);

convert!(RGB64, RGB8);
convert!(RGB64, RGB32);
convert!(RGB64, BGR8);
convert!(RGB64, BGR32);
convert!(RGB64, BGR64);
convert!(RGB64, HSL);

convert!(BGR8, RGB8);
convert!(BGR8, RGB32);
convert!(BGR8, RGB64);
convert!(BGR8, BGR32);
convert!(BGR8, BGR64);
convert!(BGR8, HSL);

convert!(BGR32, RGB8);
convert!(BGR32, RGB32);
convert!(BGR32, RGB64);
convert!(BGR32, BGR8);
convert!(BGR32, BGR64);
convert!(BGR32, HSL);

convert!(BGR64, RGB8);
convert!(BGR64, RGB32);
convert!(BGR64, RGB64);
convert!(BGR64, BGR8);
convert!(BGR64, BGR32);
convert!(BGR64, HSL);

convert!(HSL, RGB8);
convert!(HSL, RGB32);
convert!(HSL, RGB64);
convert!(HSL, BGR8);
convert!(HSL, BGR32);
convert!(HSL, BGR64);
