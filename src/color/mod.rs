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
    (RGB8) => (
        Self::from_rgb8
    );

    (RGB32) => (
        Self::from_rgb32
    );

    (RGB64) => (
        Self::from_rgb64
    );

    (BGR8) => (
        Self::from_bgr8
    );

    (BGR32) => (
        Self::from_bgr32
    );

    (BGR64) => (
        Self::from_bgr64
    );

    (HSL) => (
        Self::from_hsl
    );

    ( $t:ty, $( $x:tt ),* ) => {
        $(
            impl From<$x> for $t{
                fn from(src: $x) -> Self {
                    convert!($x)(src)
                }
            }
        )*
    };
}

convert!(RGB8, RGB32, RGB64, BGR8, BGR32, BGR64, HSL);
convert!(RGB32, RGB8, RGB64, BGR8, BGR32, BGR64, HSL);
convert!(RGB64, RGB8, RGB32, BGR8, BGR32, BGR64, HSL);

convert!(BGR8, RGB8, RGB32, RGB64, BGR32, BGR64, HSL);
convert!(BGR32, RGB8, RGB32, RGB64, BGR8, BGR64, HSL);
convert!(BGR64, RGB8, RGB32, RGB64, BGR8, BGR32, HSL);

convert!(HSL, RGB8, RGB32, RGB64, BGR8, BGR32, BGR64);
