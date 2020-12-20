#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec2<T: Copy> {
    pub x: T,
    pub y: T,
}

macro_rules! vec2_min_max {
    ($($t:ty),*) => {
        $(
            impl Vec2<$t> {
                pub fn min(self, oth: Self) -> Self {
                    Self {
                        x: self.x.min(oth.x),
                        y: self.y.min(oth.y),
                    }
                }
                pub fn max(self, oth: Self) -> Self {
                    Self {
                        x: self.x.max(oth.x),
                        y: self.y.max(oth.y),
                    }
                }
            }
        )*
    };
}

vec2_min_max! {u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, f32, f64}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min() {
        let a: Vec2<u8> = Vec2 { x: 0, y: 0 };
        let b: Vec2<u8> = Vec2 { x: 0, y: 0 };
        assert_eq!(a.min(b), Vec2 { x: 0_u8, y: 0_u8 });

        let a: Vec2<i32> = Vec2 { x: 100, y: 0 };
        let b: Vec2<i32> = Vec2 { x: 3, y: 1000 };
        assert_eq!(a.min(b), Vec2 { x: 3_i32, y: 0_i32 });

        let a: Vec2<f64> = Vec2 { x: 0.0, y: -45.0 };
        let b: Vec2<f64> = Vec2 { x: 25.0, y: 41.0 };
        assert_eq!(a.min(b), Vec2 { x: 0.0, y: -45.0 });
    }

    #[test]
    fn max() {
        let a: Vec2<u8> = Vec2 { x: 0, y: 0 };
        let b: Vec2<u8> = Vec2 { x: 0, y: 0 };
        assert_eq!(a.max(b), Vec2 { x: 0_u8, y: 0_u8 });

        let a: Vec2<i32> = Vec2 { x: 100, y: 0 };
        let b: Vec2<i32> = Vec2 { x: 3, y: 54 };
        assert_eq!(a.max(b), Vec2 { x: 100, y: 54 });

        let a: Vec2<f64> = Vec2 { x: 0.0, y: -45.0 };
        let b: Vec2<f64> = Vec2 { x: 25.0, y: 41.0 };
        assert_eq!(a.max(b), Vec2 { x: 25.0, y: 41.0 });
    }
}
