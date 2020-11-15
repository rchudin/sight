use super::{Buffer, ComponentsRaw};
use crate::{
    error::IncorrectData,
    math::transpose::{transpose, transpose_square},
};
use std::{
    ops::{Deref, Index, IndexMut},
    slice::SliceIndex,
};

pub struct Image<T: Copy> {
    buffer: Buffer<T>,
}

impl<T: Copy> Image<T> {
    pub fn new(width: u32, height: u32, color: T) -> Result<Self, IncorrectData> {
        match Buffer::new(width, height, color) {
            Ok(buffer) => Ok(Self { buffer }),
            Err(e) => Err(e),
        }
    }

    pub fn from_vec(width: u32, height: u32, data: Vec<T>) -> Result<Self, IncorrectData> {
        match Buffer::from_vec(width, height, data) {
            Ok(buffer) => Ok(Self { buffer }),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn as_vec(self) -> Vec<T> {
        self.buffer.data
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.buffer.width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.buffer.height
    }

    #[inline]
    pub fn index2d_to_index(&self, x: u32, y: u32) -> usize {
        self.buffer.index2d_to_index(x, y)
    }

    #[inline]
    pub fn index_to_index2d(&self, index: usize) -> (u32, u32) {
        self.buffer.index_to_index2d(index)
    }

    #[inline]
    pub fn flip_vertically(&mut self) {
        for y in 0..self.buffer.height {
            let first = self.index2d_to_index(0, y);
            let width = self.buffer.width as usize;
            self.buffer[first..first + width].reverse()
        }
    }

    #[inline]
    pub fn flip_horizontally(&mut self) {
        self.buffer.data.reverse();
        self.flip_vertically();
    }

    #[inline]
    pub fn rotate90(&mut self) {
        if self.buffer.width == self.buffer.height {
            transpose_square(self.buffer.width, &mut self.buffer.data)
        } else {
            transpose(self.buffer.width, self.buffer.height, &mut self.buffer.data);
            std::mem::swap(&mut self.buffer.width, &mut self.buffer.height);
        }
        self.flip_vertically();
    }
}

impl<T: Copy> Deref for Image<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.buffer.deref()
    }
}

impl<T: Copy> IntoIterator for Image<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.into_iter()
    }
}

impl<T: Copy, I: SliceIndex<[T]>> Index<I> for Image<T> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.buffer[index]
    }
}

impl<T: Copy, I: SliceIndex<[T]>> IndexMut<I> for Image<T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.buffer[index]
    }
}

impl<T: Copy> ComponentsRaw for Image<T>
where
    Buffer<T>: ComponentsRaw,
{
    type Output = <Buffer<T> as ComponentsRaw>::Output;

    #[inline]
    fn raw(&self) -> &[Self::Output] {
        self.buffer.raw()
    }

    #[inline]
    fn raw_into_vec(self) -> Vec<Self::Output> {
        self.buffer.raw_into_vec()
    }
}

impl<T: Copy + Into<F>, F: Copy> Into<Vec<F>> for Image<T> {
    fn into(self) -> Vec<F> {
        self.buffer.into_iter().map(|x| x.into()).collect()
    }
}

impl<T: Copy + Into<F>, F: Copy> Into<Vec<F>> for &Image<T> {
    fn into(self) -> Vec<F> {
        self.buffer.iter().map(|x| x.clone().into()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{color::RGB8, math::index2d_to_index};

    #[test]
    fn rotate90_square() {
        let width: u32 = 20;
        let height: u32 = 20;
        let mut img: Image<RGB8> = Image::new(width, height, RGB8::from([0, 0, 0])).unwrap();
        let color = RGB8::from([255, 0, 0]);
        img[0] = color;
        assert_eq!(img[0], color);

        img.rotate90();
        assert_eq!(img[index2d_to_index(width, width - 1, 0)], color);

        img.rotate90();
        assert_eq!(img[index2d_to_index(width, width - 1, height - 1)], color);

        img.rotate90();
        assert_eq!(img[index2d_to_index(width, 0, height - 1)], color);

        img.rotate90();
        assert_eq!(img[index2d_to_index(width, 0, 0)], color);
    }

    #[test]
    fn rotate90() {
        let mut img: Image<RGB8> = Image::new(450, 20, RGB8::from([255, 255, 255])).unwrap();
        let color = RGB8::from([0, 255, 0]);
        img[0] = color;
        assert_eq!(img[0], color);

        img.rotate90();
        assert_eq!(
            img[index2d_to_index(img.width(), img.width() - 1, 0)],
            color
        );

        img.rotate90();
        assert_eq!(
            img[index2d_to_index(img.width(), img.width() - 1, img.height() - 1)],
            color
        );

        img.rotate90();
        assert_eq!(
            img[index2d_to_index(img.width(), 0, img.height() - 1)],
            color
        );

        img.rotate90();
        assert_eq!(img[index2d_to_index(img.width(), 0, 0)], color);
    }

    #[test]
    fn flip_vertically() {
        let mut buffer: Image<RGB8> = Image::new(21, 21, RGB8::from([0, 0, 0])).unwrap();
        let color1 = RGB8::from([255, 0, 0]);
        let color2 = RGB8::from([0, 255, 0]);
        let color3 = RGB8::from([0, 0, 255]);
        buffer[0] = color1;
        let len = buffer.len();
        buffer[len - 1] = color2;
        let index = index2d_to_index(buffer.width(), buffer.width() / 2, buffer.height() / 2);
        buffer[index] = color3;

        buffer.flip_vertically();
        assert_eq!(
            buffer[index2d_to_index(buffer.width(), buffer.width() - 1, 0)],
            color1
        );
        assert_eq!(
            buffer[index2d_to_index(buffer.width(), 0, buffer.height() - 1)],
            color2
        );
        assert_eq!(
            buffer[index2d_to_index(buffer.width(), buffer.width() / 2, buffer.height() / 2)],
            color3
        );
    }

    #[test]
    fn flip_horizontally() {
        let mut buffer: Image<RGB8> = Image::new(43, 43, RGB8::from([0, 0, 0])).unwrap();
        let color1 = RGB8::from([255, 0, 0]);
        let color2 = RGB8::from([0, 255, 0]);
        let color3 = RGB8::from([0, 0, 255]);
        buffer[0] = color1;
        let len = buffer.len();
        buffer[len - 1] = color2;
        let index = index2d_to_index(buffer.width(), buffer.width() / 2, buffer.height() / 2);
        buffer[index] = color3;

        buffer.flip_horizontally();
        assert_eq!(
            buffer[index2d_to_index(buffer.width(), 0, buffer.height() - 1)],
            color1
        );
        assert_eq!(
            buffer[index2d_to_index(buffer.width(), buffer.width() - 1, 0)],
            color2
        );
        assert_eq!(
            buffer[index2d_to_index(buffer.width(), buffer.width() / 2, buffer.height() / 2)],
            color3
        );
    }
}
