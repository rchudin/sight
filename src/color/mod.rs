pub(crate) mod rgb_hsl;

mod rgb;
pub use rgb::{RGB32, RGB64, RGB8};
mod hsl;
pub use hsl::HSL;

pub trait Convert {
    fn from_rgb8(src: RGB8) -> Self;

    fn from_rgb32(src: RGB32) -> Self;

    fn from_rgb64(src: RGB64) -> Self;

    fn from_hsl(src: HSL) -> Self;
}
