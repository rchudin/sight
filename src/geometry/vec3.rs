#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec3<T: Copy> {
    pub x: T,
    pub y: T,
    pub z: T,
}

macro_rules! vec3_min_max {
    ($($t:ty),*) => {
        $(
            impl Vec3<$t> {
                pub fn min(self, oth: Self) -> Self {
                    Self {
                        x: self.x.min(oth.x),
                        y: self.y.min(oth.y),
                        z: self.z.min(oth.z),
                    }
                }
                pub fn max(self, oth: Self) -> Self {
                    Self {
                        x: self.x.max(oth.x),
                        y: self.y.max(oth.y),
                        z: self.z.max(oth.z),
                    }
                }
            }
        )*
    };
}

vec3_min_max! {u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, f32, f64}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min() {
        let a: Vec3<u8> = Vec3 { x: 0, y: 0, z: 0 };
        let b: Vec3<u8> = Vec3 { x: 0, y: 0, z: 0 };
        assert_eq!(a.min(b), Vec3 { x: 0, y: 0, z: 0 });

        let a: Vec3<i32> = Vec3 { x: 100, y: 0, z: 0 };
        let b: Vec3<i32> = Vec3 { x: 3, y: 10, z: -4 };
        assert_eq!(a.min(b), Vec3 { x: 3, y: 0, z: -4 });

        let a: Vec3<f64> = Vec3 {
            x: 0.0,
            y: -45.0,
            z: 0.0,
        };
        let b: Vec3<f64> = Vec3 {
            x: 25.0,
            y: 41.0,
            z: 45.0,
        };
        assert_eq!(
            a.min(b),
            Vec3 {
                x: 0.0,
                y: -45.0,
                z: 0.0
            }
        );
    }

    #[test]
    fn max() {
        let a: Vec3<u8> = Vec3 { x: 0, y: 0, z: 0 };
        let b: Vec3<u8> = Vec3 { x: 0, y: 0, z: 0 };
        assert_eq!(a.max(b), Vec3 { x: 0, y: 0, z: 0 });

        let a: Vec3<i32> = Vec3 { x: 10, y: 0, z: 0 };
        let b: Vec3<i32> = Vec3 { x: 3, y: 10, z: -4 };
        assert_eq!(a.max(b), Vec3 { x: 10, y: 10, z: 0 });

        let a: Vec3<f64> = Vec3 {
            x: 0.0,
            y: -45.0,
            z: 0.0,
        };
        let b: Vec3<f64> = Vec3 {
            x: 25.0,
            y: 41.0,
            z: 45.0,
        };
        assert_eq!(
            a.max(b),
            Vec3 {
                x: 25.0,
                y: 41.0,
                z: 45.0
            }
        );
    }
}
