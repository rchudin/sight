use super::ComponentsRaw;
use crate::{
    color::{rgb::RGB, HSL, RGB32, RGB64, RGB8},
    error::IncorrectData,
    math::{
        index2d_to_index, index_to_index2d,
        transpose::{transpose, transpose_square},
    },
};
use std::{
    ops::{Deref, Index, IndexMut},
    slice,
    slice::SliceIndex,
};

pub struct Buffer<T: Copy> {
    width: u32,
    height: u32,
    buffer: Vec<T>,
}

impl<T: Copy> Buffer<T> {
    pub fn new(width: u32, height: u32, color: T) -> Result<Self, IncorrectData> {
        let capacity = width as usize * height as usize;
        if let None = capacity.checked_mul(std::mem::size_of::<T>()) {
            return Err(IncorrectData::Overflow);
        }
        if capacity > isize::MAX as usize {
            return Err(IncorrectData::Overflow);
        }

        Ok(Self {
            width,
            height,
            buffer: vec![color; capacity],
        })
    }

    pub fn from_vec(width: u32, height: u32, buffer: Vec<T>) -> Result<Self, IncorrectData> {
        let expected = width as usize * height as usize;

        let got = buffer.len();
        if expected != got {
            return Err(IncorrectData::Size { expected, got });
        }

        if got > isize::MAX as usize {
            return Err(IncorrectData::Overflow);
        }

        Ok(Self {
            width,
            height,
            buffer,
        })
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }

    #[inline]
    pub fn index2d_to_index(&self, x: u32, y: u32) -> usize {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);
        index2d_to_index(self.width, x, y)
    }

    #[inline]
    pub fn index_to_index2d(&self, index: usize) -> (u32, u32) {
        debug_assert!(index < self.buffer.len());
        index_to_index2d(self.width, index)
    }

    #[inline]
    pub fn flip_vertically(&mut self) {
        for x in 0..self.height {
            let first = index2d_to_index(self.width, 0, x);
            self.buffer[first..first + self.width as usize].reverse()
        }
    }

    #[inline]
    pub fn flip_horizontally(&mut self) {
        self.buffer.reverse();
        self.flip_vertically();
    }

    #[inline]
    pub fn rotate90(&mut self) {
        if self.width == self.height {
            transpose_square(self.width, &mut self.buffer)
        } else {
            transpose(self.width, self.height, &mut self.buffer);
            std::mem::swap(&mut self.width, &mut self.height);
        }
        self.flip_vertically();
    }
}

impl<T: Copy> Deref for Buffer<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.buffer.deref()
    }
}

impl<T: Copy, I: SliceIndex<[T]>> Index<I> for Buffer<T> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.buffer[index]
    }
}

impl<T: Copy, I: SliceIndex<[T]>> IndexMut<I> for Buffer<T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.buffer[index]
    }
}

impl<T: Copy> ComponentsRaw for Buffer<RGB<T>> {
    type Output = T;

    fn raw(&self) -> &[Self::Output] {
        let ptr = self.buffer.as_ptr();
        let len = self.buffer.len() * 3;
        unsafe { slice::from_raw_parts(ptr as *const _, len) }
    }

    fn raw_into_vec(mut self) -> Vec<Self::Output> {
        let ptr = self.buffer.as_mut_ptr();
        let len = self.buffer.len() * 3;
        let cap = self.buffer.capacity() * 3;
        std::mem::forget(self.buffer);
        unsafe { Vec::from_raw_parts(ptr as *mut _, len, cap) }
    }
}

macro_rules! from {
    ($s:ident) => {{
        let width: u32 = $s.width();
        let height: u32 = $s.height();
        let len: usize = width as usize * height as usize;
        assert_eq!(len, $s.buffer.len());
        let mut buffer = Vec::with_capacity(len);
        for x in $s.buffer.iter() {
            buffer.push(x.clone().into());
        }
        Self {
            width,
            height,
            buffer,
        }
    }};

    ($s:ty, $f:ty) => {
        impl From<Buffer<$f>> for Buffer<$s> {
            fn from(src: Buffer<$f>) -> Self {
                from!(src)
            }
        }

        impl From<&Buffer<$f>> for Buffer<$s> {
            fn from(src: &Buffer<$f>) -> Self {
                from!(src)
            }
        }
    };
}

from!(RGB8, RGB32);
from!(RGB8, RGB64);
from!(RGB8, HSL);

from!(RGB32, RGB8);
from!(RGB32, RGB64);
from!(RGB32, HSL);

from!(RGB64, RGB8);
from!(RGB64, RGB32);
from!(RGB64, HSL);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let width: u32 = 20;
        let height: u32 = 20;
        let capacity = width as usize * height as usize;
        let color = RGB8::from([255, 255, 255]);

        let buffer = Buffer::new(width, height, color).unwrap();

        assert_eq!(buffer.len(), capacity);
        assert_eq!(buffer.width() as usize * buffer.height() as usize, capacity);
        for x in buffer.iter() {
            assert_eq!(x, &color);
        }
    }

    #[test]
    fn new_overflow() {
        let width: u32 = u32::MAX;
        let height: u32 = u32::MAX;
        let color = RGB8::from([255, 255, 255]);

        let buffer = Buffer::new(width, height, color);
        assert!(buffer.is_err());

        if let Err(e) = buffer {
            match e {
                IncorrectData::Overflow => assert!(true),
                _ => assert!(false),
            }
        }
    }

    #[test]
    fn from_vec() {
        let width: u32 = 10;
        let height: u32 = 5;
        let capacity = width as usize * height as usize;
        let color = RGB8::from([255, 255, 255]);

        let buffer = Buffer::from_vec(width, height, vec![color; capacity]).unwrap();

        assert_eq!(buffer.len(), capacity);
        assert_eq!(buffer.width() as usize * buffer.height() as usize, capacity);
        for x in buffer.iter() {
            assert_eq!(x, &color);
        }
    }

    #[test]
    fn slice_index() {
        let width: u32 = 7;
        let height: u32 = 15;
        let capacity = width as usize * height as usize;
        let color = RGB8::from([255, 255, 255]);
        let mut buffer = Buffer::new(width, height, color).unwrap();

        assert_eq!(buffer[..].len(), capacity);

        let color = RGB8::from([0, 0, 255]);
        buffer[0] = color;
        assert_eq!(buffer[0], color);
    }

    #[test]
    fn rgb_raw() {
        type T = u8;
        let width: u32 = 200;
        let height: u32 = 200;
        let color = 0;
        let canals = std::mem::size_of::<RGB<T>>();
        assert_eq!(canals, 3);

        let buff: Buffer<RGB<T>> =
            Buffer::new(width, height, RGB8::from([color, color, color])).unwrap();

        let raw: &[T] = buff.raw();
        assert_eq!(raw.len(), width as usize * height as usize * canals);

        for x in raw {
            assert!((*x) == color);
        }
    }

    #[test]
    fn rgb_raw_into_vec() {
        type T = f64;
        let width: u32 = 121;
        let height: u32 = 121;
        let color = 0_f64;
        let canals = 3;

        let buff: Buffer<RGB<T>> =
            Buffer::new(width, height, RGB::from([color, color, color])).unwrap();

        let raw: Vec<T> = buff.raw_into_vec();
        assert_eq!(raw.len(), width as usize * height as usize * canals);

        for x in raw {
            assert!(x == color);
        }
    }

    #[test]
    fn index2d() {
        let width: u32 = 600;
        let height: u32 = 600;
        let img: Buffer<RGB8> = Buffer::new(width, height, RGB8::from([0, 0, 0])).unwrap();

        let index = img.index2d_to_index(0, 0);
        assert_eq!(index, 0);
        let (x, y) = img.index_to_index2d(index);
        assert_eq!((x, y), (0, 0));

        let index = img.index2d_to_index(width - 1, 0);
        assert_eq!(index, width as usize - 1);
        let (x, y) = img.index_to_index2d(index);
        assert_eq!((x, y), (width - 1, 0));

        let index = img.index2d_to_index(0, height - 1);
        assert_eq!(index, (height - 1) as usize * height as usize);
        let (x, y) = img.index_to_index2d(index);
        assert_eq!((x, y), (0, height - 1));

        let index = img.index2d_to_index(599, 599);
        assert_eq!(index, img.buffer.len() - 1);
        let (x, y) = img.index_to_index2d(index);
        assert_eq!((x, y), (599, 599));
    }

    #[test]
    fn rotate90_square() {
        let width: u32 = 20;
        let height: u32 = 20;
        let mut img: Buffer<RGB8> = Buffer::new(width, height, RGB8::from([0, 0, 0])).unwrap();
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
        let mut img: Buffer<RGB8> = Buffer::new(450, 20, RGB8::from([255, 255, 255])).unwrap();
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
        let mut buffer: Buffer<RGB8> = Buffer::new(21, 21, RGB8::from([0, 0, 0])).unwrap();
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
        let mut buffer: Buffer<RGB8> = Buffer::new(43, 43, RGB8::from([0, 0, 0])).unwrap();
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
