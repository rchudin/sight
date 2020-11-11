use num::complex::Complex;
use sight::{
    color::RGB8,
    display::{Buffer, ComponentsRaw},
};

fn mandelbrot_red_black(x: u32, y: u32, width: u32, height: u32) -> RGB8 {
    let point = Complex::new(
        x as f32 / width as f32 - 1.5,
        y as f32 / height as f32 - 0.5,
    );

    let mut z: Complex<f32> = Complex::new(0.0, 0.0);
    let mut nb_iter: u32 = 0;

    while z.norm() < 2.0 && nb_iter <= 34 {
        z = z * z + point;
        nb_iter += 1;
    }

    if nb_iter < 34 {
        RGB8::from([(255 * nb_iter / 33) as u8, 0, 0])
    } else {
        RGB8::from([0, 0, 0])
    }
}

fn main() {
    let width: u32 = 550;
    let height: u32 = 600;

    let mut img: Buffer<RGB8> = Buffer::new(width, height, RGB8::from([255, 0, 0])).unwrap();

    for y in 0..height {
        for x in 0..width {
            let index = img.index2d_to_index(x, y);
            img[index] = mandelbrot_red_black(x, y, width, height);
        }
    }

    img.rotate90();

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
