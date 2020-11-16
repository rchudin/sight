mod buffer;
mod image;

pub use self::buffer::Buffer;
pub use self::image::Image;

pub trait Frame {
    type Pixel;

    fn width(&self) -> u32;

    fn height(&self) -> u32;

    fn pixel(&self, x: u32, y: u32) -> &Self::Pixel;

    fn pixel_mut(&mut self, x: u32, y: u32) -> &mut Self::Pixel;

    fn row(&self, row: u32) -> &[Self::Pixel];

    fn row_mut(&mut self, row: u32) -> &mut [Self::Pixel];
}

pub trait ComponentsRaw {
    type Output;

    fn raw(&self) -> &[Self::Output];

    fn raw_into_vec(self) -> Vec<Self::Output>;
}
