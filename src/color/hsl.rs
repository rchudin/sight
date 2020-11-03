// Hue | Saturation | Lightness
#[derive(Default, Debug, Clone, Copy)]
pub struct HSL {
    pub h: f64,
    pub s: f64,
    pub l: f64,
}

impl PartialEq for HSL {
    fn eq(&self, other: &Self) -> bool {
        self.h == other.h && self.s == other.s && self.l == other.l
    }
}

impl From<[f64; 3]> for HSL {
    fn from(src: [f64; 3]) -> Self {
        Self {
            h: src[0],
            s: src[1],
            l: src[2],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partial_eq() {
        let rgb1 = HSL::from([1.0, 1.0, 1.0]);
        let rgb2 = HSL::from([1.0, 1.0, 1.0]);
        assert_eq!(rgb1, rgb2);
        let rgb2 = HSL::from([0.0, 1.0, 1.0]);
        assert_ne!(rgb1, rgb2);
        let rgb2 = HSL::from([1.0, 0.0, 1.0]);
        assert_ne!(rgb1, rgb2);
        let rgb2 = HSL::from([1.0, 1.0, 0.0]);
        assert_ne!(rgb1, rgb2);
        let rgb2 = HSL::from([0.0, 0.0, 0.0]);
        assert_ne!(rgb1, rgb2);
    }
}
