use num::complex::Complex;
use sight::{
    color::RGB8,
    display::{ComponentsRaw, Frame, Image},
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
    let mut img: Image<RGB8> = Image::new(571, 600, RGB8::from([255, 0, 0])).unwrap();

    for y in 0..img.height() {
        for x in 0..img.width() {
            *img.pixel_mut(x, y) = mandelbrot_red_black(x, y, img.width(), img.height());
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
