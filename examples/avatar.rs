use sight::{
    color::RGB8,
    display::{ComponentsRaw, Frame, Image},
};
use std::convert::TryInto;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let name: [u8; 16] = if args.len() > 1 {
        md5::compute(&args[1]).0
    } else {
        md5::compute("name").0
    };

    let mut img: Image<RGB8> = Image::new(144, 144, RGB8::from([219, 219, 219])).unwrap();

    let buffer: [u8; 9] = name[0..9].try_into().unwrap();
    let mut color: [u8; 3] = [name[9], name[10], name[11]];

    for i in 0..3 {
        color[i] = (color[i] as f32 * 0.8) as u8;
    }

    draw(&mut img, buffer, RGB8::from(color));

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

fn draw<T: Copy>(image: &mut Image<T>, bytes: [u8; 9], color: T) {
    // 12 * (12 / 2) = 72
    // 9byte * 8bit = 72
    const NEED_MATRIX: usize = 12;

    let image_width = image.width() as usize;
    let block_width = image_width / NEED_MATRIX;
    let block_height = image.height() as usize / NEED_MATRIX;

    let mut index: usize = 0;
    for byte in &bytes {
        for bit_index in 0..8 {
            if (byte >> bit_index & 0x01) > 0 {
                let x = index % (NEED_MATRIX / 2);
                let y = index / (NEED_MATRIX / 2);

                // padding
                if x == 0 || y == 0 || y == (NEED_MATRIX - 1) {
                    continue;
                }

                // draw
                for need_y in 0..block_height {
                    for need_x in 0..block_width {
                        let x = x * block_width + need_x;
                        let y = y * block_height + need_y;
                        let index = y * image_width + x;
                        image[index] = color;
                        let index = y * image_width + image_width - x - 1;
                        image[index] = color;
                    }
                }
            }

            index += 1;
        }
    }
}
