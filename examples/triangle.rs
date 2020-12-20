use sight::{
    color::RGB8,
    display::{ComponentsRaw, Frame, Image},
    geometry::{Vec2, Vec3},
};

use std::ops::{Mul, Sub};

fn cross<T>(v1: Vec3<T>, v2: Vec3<T>) -> Vec3<T>
where
    T: Copy + Mul<Output = T> + Sub<Output = T>,
{
    Vec3 {
        x: v1.y * v2.z - v1.z * v2.y,
        y: v1.z * v2.x - v1.x * v2.z,
        z: v1.x * v2.y - v1.y * v2.x,
    }
}

fn barycentric(a: Vec2<i32>, b: Vec2<i32>, c: Vec2<i32>, p: Vec2<i32>) -> Vec3<f32> {
    let v1 = Vec3 {
        x: c.x - a.x,
        y: b.x - a.x,
        z: a.x - p.x,
    };

    let v2 = Vec3 {
        x: c.y - a.y,
        y: b.y - a.y,
        z: a.y - p.y,
    };

    let u = cross(v1, v2);

    if u.z.abs() < 1 {
        Vec3 {
            x: -1_f32,
            y: 1_f32,
            z: 1_f32,
        }
    } else {
        Vec3 {
            x: 1_f32 - (u.x as f32 + u.y as f32) / u.z as f32,
            y: u.y as f32 / u.z as f32,
            z: u.x as f32 / u.z as f32,
        }
    }
}

fn triangle_interpolation<T: Frame<Pixel = RGB8>>(
    frame: &mut T,
    points: [Vec2<i32>; 3],
    colors: [RGB8; 3],
) {
    let left_top = points[0].min(points[1].min(points[2].min(Vec2 { x: 0, y: 0 })));
    let right_bottom = points[0].max(points[1].max(points[2].max(Vec2 {
        x: frame.width() as i32 - 1,
        y: frame.height() as i32 - 1,
    })));

    for y in left_top.y..=right_bottom.y {
        for x in left_top.x..=right_bottom.x {
            let bc_screen = barycentric(points[0], points[1], points[2], Vec2 { x, y });
            if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
                continue;
            }

            let color = colors[0];

            // let color = ((colors[0] * bc_screen.x as u8)
            //     + (colors[1] * bc_screen.y as u8)
            //     + (colors[0] * bc_screen.z as u8))
            //     / (bc_screen.x + bc_screen.y + bc_screen.z) as u8;

            *frame.pixel_mut(x as u32, y as u32) = color;
        }
    }
}

fn main() {
    let mut img: Image<RGB8> = Image::new(600, 600, RGB8::from([0, 0, 0])).unwrap();

    let points: [Vec2<i32>; 3] = [
        Vec2 {
            x: (img.width() / 2) as i32,
            y: 0,
        },
        Vec2 {
            x: 0,
            y: (img.height() - 1) as i32,
        },
        Vec2 {
            x: (img.width() - 1) as i32,
            y: (img.height() - 1) as i32,
        },
    ];

    let color: [RGB8; 3] = [
        RGB8 { r: 255, g: 0, b: 0 },
        RGB8 { r: 0, g: 255, b: 0 },
        RGB8 { r: 0, g: 0, b: 255 },
    ];

    triangle_interpolation(&mut img, points, color);

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
