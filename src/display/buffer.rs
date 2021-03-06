use super::{ComponentsRaw, Frame};
use crate::{color::ComponentsCount, error::IncorrectData, math::index2d_to_index};
use std::{
    ops::{Deref, Index, IndexMut},
    slice,
    slice::SliceIndex,
};

pub struct Buffer<T: Copy> {
    pub width: u32,
    pub height: u32,
    pub data: Vec<T>,
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
            data: vec![color; capacity],
        })
    }

    pub fn from_vec(width: u32, height: u32, data: Vec<T>) -> Result<Self, IncorrectData> {
        let expected = width as usize * height as usize;

        let got = data.len();
        if expected != got {
            return Err(IncorrectData::Size { expected, got });
        }

        if got > isize::MAX as usize {
            return Err(IncorrectData::Overflow);
        }

        Ok(Self {
            width,
            height,
            data,
        })
    }
}

impl<T: Copy> Frame for Buffer<T> {
    type Pixel = T;
    #[inline]
    fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    fn height(&self) -> u32 {
        self.height
    }

    #[inline]
    fn pixel(&self, x: u32, y: u32) -> &Self::Pixel {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);
        &self.data[index2d_to_index(self.width, x, y)]
    }

    #[inline]
    fn pixel_mut(&mut self, x: u32, y: u32) -> &mut Self::Pixel {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);
        &mut self.data[index2d_to_index(self.width, x, y)]
    }

    #[inline]
    fn row(&self, row: u32) -> &[Self::Pixel] {
        debug_assert!(row < self.height);
        let start = self.width as usize * row as usize;
        &self.data[start..start + self.width as usize]
    }

    #[inline]
    fn row_mut(&mut self, row: u32) -> &mut [Self::Pixel] {
        debug_assert!(row < self.height);
        let start = self.width as usize * row as usize;
        &mut self.data[start..start + self.width as usize]
    }
}

impl<T: Copy> Deref for Buffer<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.data.deref()
    }
}

impl<T: Copy> IntoIterator for Buffer<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<T: Copy, I: SliceIndex<[T]>> Index<I> for Buffer<T> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: Copy, I: SliceIndex<[T]>> IndexMut<I> for Buffer<T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T: Copy + ComponentsCount> ComponentsRaw for Buffer<T> {
    type Output = <T as ComponentsCount>::Component;

    fn raw(&self) -> &[Self::Output] {
        let ptr = self.data.as_ptr();
        let len = self.data.len() * T::components_count();
        unsafe { slice::from_raw_parts(ptr as *const _, len) }
    }

    fn raw_into_vec(mut self) -> Vec<Self::Output> {
        let ptr = self.data.as_mut_ptr();
        let len = self.data.len() * T::components_count();
        let cap = self.data.capacity() * T::components_count();
        std::mem::forget(self.data);
        unsafe { Vec::from_raw_parts(ptr as *mut _, len, cap) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{rgb::RGB, RGB8};

    #[test]
    fn new() {
        let width: u32 = 20;
        let height: u32 = 20;
        let capacity = width as usize * height as usize;
        let color = RGB8::from([255, 255, 255]);

        let buffer = Buffer::new(width, height, color).unwrap();

        assert_eq!(buffer.data.len(), capacity);
        assert_eq!(buffer.width as usize * buffer.height as usize, capacity);
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

        assert_eq!(buffer.data.len(), capacity);
        assert_eq!(buffer.width as usize * buffer.height as usize, capacity);
        for x in buffer {
            assert_eq!(x, color);
        }
    }

    #[test]
    fn width_and_height() {
        let width: u32 = 5;
        let height: u32 = 10;
        let buffer = Buffer::new(width, height, RGB8::from([255, 255, 255])).unwrap();
        assert_eq!(width, buffer.width());
        assert_eq!(height, buffer.height());
    }

    #[test]
    fn pixel() {
        let color = RGB8::from([255, 255, 255]);
        let mut buffer = Buffer::new(45, 45, color).unwrap();
        assert_eq!(*buffer.pixel(1, 1), color);
        assert_eq!(
            *buffer.pixel(1, 1),
            buffer[index2d_to_index(buffer.width, 1, 1)]
        );

        let color = RGB8::from([0, 0, 0]);
        *buffer.pixel_mut(0, 0) = color;
        assert_eq!(*buffer.pixel(0, 0), color);
        assert_eq!(
            *buffer.pixel(0, 0),
            buffer[index2d_to_index(buffer.width, 0, 0)]
        );

        let color = RGB8::from([0, 255, 0]);
        *buffer.pixel_mut(buffer.width() - 1, buffer.height() - 1) = color;
        assert_eq!(
            buffer.pixel(buffer.width() - 1, buffer.height() - 1),
            &color
        );
        assert_eq!(
            *buffer.pixel(buffer.height() - 1, buffer.height() - 1),
            buffer[index2d_to_index(buffer.width, buffer.width() - 1, buffer.height() - 1)]
        );
    }

    #[test]
    fn row() {
        let mut buffer = Buffer::new(45, 45, RGB8::from([255, 255, 255])).unwrap();

        let row = buffer.row(0);
        assert_eq!(row.len() as u32, buffer.height());

        let row = buffer.row_mut(buffer.height() - 1);
        assert_eq!(row.len() as u32, buffer.height());
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
}
