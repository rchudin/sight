use super::ComponentsRaw;
use crate::{
    color::{rgb::RGB, HSL, RGB32, RGB64, RGB8},
    error::IncorrectData,
};
use std::{
    ops::{Deref, Index, IndexMut},
    slice,
    slice::SliceIndex,
};

pub struct Buffer<T> {
    width: u32,
    height: u32,
    buffer: Vec<T>,
}

impl<T: Clone> Buffer<T> {
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
}

impl<T> Buffer<T> {
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

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

impl<T> Deref for Buffer<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.buffer.deref()
    }
}

impl<T, I: SliceIndex<[T]>> Index<I> for Buffer<T> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.buffer[index]
    }
}

impl<T, I: SliceIndex<[T]>> IndexMut<I> for Buffer<T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.buffer[index]
    }
}

impl<T> ComponentsRaw for Buffer<RGB<T>> {
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
}
