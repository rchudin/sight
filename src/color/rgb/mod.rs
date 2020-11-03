#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct RGB<T = u8> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl RGB {
    pub fn percent32_to_byte(percent: f32) -> u8 {
        (percent * 255_f32).round() as u8
    }

    pub fn percent64_to_byte(percent: f64) -> u8 {
        (percent * 255_f64).round() as u8
    }

    pub fn byte_to_percent32(byte: u8) -> f32 {
        byte as f32 / 255_f32
    }

    pub fn byte_to_percent64(byte: u8) -> f64 {
        byte as f64 / 255_f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percent32_to_byte() {
        let percent: f32 = 0.5;
        let byte: u8 = RGB::percent32_to_byte(percent);
        assert_eq!(byte, 128);
    }

    #[test]
    fn percent64_to_byte() {
        let percent: f64 = 0.25;
        let byte: u8 = RGB::percent64_to_byte(percent);
        assert_eq!(byte, 64);
    }

    #[test]
    fn byte_to_percent32() {
        let byte: u8 = 255;
        let percent: f32 = RGB::byte_to_percent32(byte);
        assert_eq!(percent, 1.0);
    }

    #[test]
    fn byte_to_percent64() {
        let byte: u8 = 0;
        let percent: f64 = RGB::byte_to_percent64(byte);
        assert_eq!(percent, 0.0);
    }
}
