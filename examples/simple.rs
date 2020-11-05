extern crate sight;

use sight::{
    color::{HSL, RGB8},
    display::{Buffer, ComponentsRaw},
};

fn main() {
    let width: u32 = 800;
    let height: u32 = 200;

    let mut buff: Vec<HSL> = Vec::with_capacity(width as usize * height as usize);
    let k = width as f64 / 100.0;
    for _ in 0..height {
        for x in 0..width {
            let h = x as f64 / k * 3.4;
            buff.push(HSL { h, s: 1.0, l: 0.5 })
        }
    }
    let img: Buffer<RGB8> = Buffer::from_vec(width, height, buff).unwrap().into();

    image::save_buffer_with_format(
        "tmp.png",
        img.raw(),
        img.width(),
        img.height(),
        image::ColorType::Rgb8,
        image::ImageFormat::Png,
    )
    .unwrap();
}
