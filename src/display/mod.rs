mod buffer;

pub use self::buffer::Buffer;

pub trait ComponentsRaw {
    type Output;

    fn raw(&self) -> &[Self::Output];

    fn raw_into_vec(self) -> Vec<Self::Output>;
}
