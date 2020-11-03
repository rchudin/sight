use super::{HSL, RGB64};

pub(crate) fn rgb_to_hsl(rgb: RGB64) -> HSL {
    let (r, g, b) = (rgb.r, rgb.g, rgb.b);
    debug_assert!(r >= 0_f64 && r <= 1_f64);
    debug_assert!(g >= 0_f64 && g <= 1_f64);
    debug_assert!(b >= 0_f64 && b <= 1_f64);

    let max = r.max(g.max(b));
    let min = r.min(g.min(b));

    let l = (max + min) / 2_f64;

    let delta: f64 = max - min;
    if delta == 0_f64 {
        return HSL {
            h: 0_f64,
            s: 0_f64,
            l: l,
        };
    }

    let s = if l < 0.5_f64 {
        delta / (max + min)
    } else {
        delta / (2_f64 - max - min)
    };

    let r2 = (((max - r) / 6_f64) + (delta / 2_f64)) / delta;
    let g2 = (((max - g) / 6_f64) + (delta / 2_f64)) / delta;
    let b2 = (((max - b) / 6_f64) + (delta / 2_f64)) / delta;

    let mut h = match max {
        x if x == r => b2 - g2,
        x if x == g => (1_f64 / 3_f64) + r2 - b2,
        _ => (2_f64 / 3_f64) + g2 - r2,
    };

    if h < 0_f64 {
        h += 1_f64;
    } else if h > 1_f64 {
        h -= 1_f64;
    }

    let h_degrees = (h * 360_f64 * 100_f64).round() / 100_f64;

    HSL {
        h: h_degrees,
        s: s,
        l: l,
    }
}

pub(crate) fn hsl_to_rgb(hsl: HSL) -> RGB64 {
    debug_assert!(hsl.h >= 0_f64 && hsl.h <= 360_f64);
    debug_assert!(hsl.s >= 0_f64 && hsl.s <= 1_f64);
    debug_assert!(hsl.l >= 0_f64 && hsl.l <= 1_f64);

    if hsl.s == 0.0 {
        return RGB64 {
            r: hsl.l,
            g: hsl.l,
            b: hsl.l,
        };
    }

    let h = hsl.h / 360.0;
    let s = hsl.s;
    let l = hsl.l;

    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - (l * s)
    };
    let p = 2.0 * l - q;

    RGB64 {
        r: hue_to_rgb(p, q, h + 1.0 / 3.0),
        g: hue_to_rgb(p, q, h),
        b: hue_to_rgb(p, q, h - 1.0 / 3.0),
    }
}

fn hue_to_rgb(p: f64, q: f64, t: f64) -> f64 {
    let t = if t < 0.0 {
        t + 1.0
    } else if t > 1.0 {
        t - 1.0
    } else {
        t
    };

    if t < 1.0 / 6.0 {
        p + (q - p) * 6.0 * t
    } else if t < 1.0 / 2.0 {
        q
    } else if t < 2.0 / 3.0 {
        p + (q - p) * (2.0 / 3.0 - t) * 6.0
    } else {
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rgb_hsl_rgb(range: std::ops::RangeInclusive<u8>) {
        for r in range {
            for g in 0..=255 as u8 {
                for b in 0..=255 as u8 {
                    let rgb = RGB64::from([
                        RGB64::byte_to_percent64(r),
                        RGB64::byte_to_percent64(g),
                        RGB64::byte_to_percent64(b),
                    ]);

                    let hsl = rgb_to_hsl(rgb);
                    let rgb2 = hsl_to_rgb(hsl);

                    let rgb = (r, g, b);
                    let rgb2 = (
                        RGB64::percent64_to_byte(rgb2.r),
                        RGB64::percent64_to_byte(rgb2.g),
                        RGB64::percent64_to_byte(rgb2.b),
                    );

                    assert_eq!(rgb, rgb2);
                }
            }
        }
    }

    #[test]
    fn rgb_hsl_rgb1() {
        rgb_hsl_rgb(0..=86);
    }

    #[test]
    fn rgb_hsl_rgb2() {
        rgb_hsl_rgb(87..=170);
    }

    #[test]
    fn rgb_hsl_rgb3() {
        rgb_hsl_rgb(171..=255);
    }
}
