use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f64,
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub fn with_alpha(r: u8, g: u8, b: u8, a: f64) -> Self {
        Self {
            r,
            g,
            b,
            a: a.clamp(0.0, 1.0),
        }
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    pub fn from_hex(hex: &str) -> Option<Self> {
        let clean = hex.trim().trim_start_matches('#');
        match clean.len() {
            3 => {
                let r = u8::from_str_radix(&clean[0..1].repeat(2), 16).ok()?;
                let g = u8::from_str_radix(&clean[1..2].repeat(2), 16).ok()?;
                let b = u8::from_str_radix(&clean[2..3].repeat(2), 16).ok()?;
                Some(Self::new(r, g, b))
            }
            6 => {
                let r = u8::from_str_radix(&clean[0..2], 16).ok()?;
                let g = u8::from_str_radix(&clean[2..4], 16).ok()?;
                let b = u8::from_str_radix(&clean[4..6], 16).ok()?;
                Some(Self::new(r, g, b))
            }
            8 => {
                let r = u8::from_str_radix(&clean[0..2], 16).ok()?;
                let g = u8::from_str_radix(&clean[2..4], 16).ok()?;
                let b = u8::from_str_radix(&clean[4..6], 16).ok()?;
                let a = u8::from_str_radix(&clean[6..8], 16).ok()? as f64 / 255.0;
                Some(Self::with_alpha(r, g, b, a))
            }
            _ => None,
        }
    }

    pub fn to_linear(&self) -> LinearRgb {
        LinearRgb {
            r: srgb_to_linear(self.r as f64 / 255.0),
            g: srgb_to_linear(self.g as f64 / 255.0),
            b: srgb_to_linear(self.b as f64 / 255.0),
            a: self.a,
        }
    }

    pub fn to_hsl(&self) -> Hsl {
        let r = self.r as f64 / 255.0;
        let g = self.g as f64 / 255.0;
        let b = self.b as f64 / 255.0;

        let max = r.max(g.max(b));
        let min = r.min(g.min(b));
        let delta = max - min;
        let l = (max + min) / 2.0;

        if delta == 0.0 {
            return Hsl {
                h: 0.0,
                s: 0.0,
                l: l * 100.0,
                a: self.a,
            };
        }

        let s = if l > 0.5 {
            delta / (2.0 - max - min)
        } else {
            delta / (max + min)
        };

        let mut h = if max == r {
            (g - b) / delta + if g < b { 6.0 } else { 0.0 }
        } else if max == g {
            (b - r) / delta + 2.0
        } else {
            (r - g) / delta + 4.0
        };
        h *= 60.0;

        Hsl {
            h: (h % 360.0 + 360.0) % 360.0,
            s: (s * 100.0).clamp(0.0, 100.0),
            l: (l * 100.0).clamp(0.0, 100.0),
            a: self.a,
        }
    }

    pub fn to_hsv(&self) -> Hsv {
        let r = self.r as f64 / 255.0;
        let g = self.g as f64 / 255.0;
        let b = self.b as f64 / 255.0;

        let max = r.max(g.max(b));
        let min = r.min(g.min(b));
        let delta = max - min;
        let v = max;

        let s = if max == 0.0 { 0.0 } else { delta / max };

        if delta == 0.0 {
            return Hsv {
                h: 0.0,
                s: 0.0,
                v: v * 100.0,
                a: self.a,
            };
        }

        let mut h = if max == r {
            (g - b) / delta + if g < b { 6.0 } else { 0.0 }
        } else if max == g {
            (b - r) / delta + 2.0
        } else {
            (r - g) / delta + 4.0
        };
        h *= 60.0;

        Hsv {
            h: (h % 360.0 + 360.0) % 360.0,
            s: (s * 100.0).clamp(0.0, 100.0),
            v: (v * 100.0).clamp(0.0, 100.0),
            a: self.a,
        }
    }

    pub fn to_oklab(&self) -> Oklab {
        self.to_linear().to_oklab()
    }

    pub fn from_oklch(oklch: &Oklch) -> Self {
        oklch.to_rgb()
    }

    pub fn to_oklch(&self) -> Oklch {
        self.to_oklab().to_oklch()
    }

    pub fn to_cielab(&self) -> Cielab {
        self.to_linear().to_cielab()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LinearRgb {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl LinearRgb {
    pub fn from_oklch(oklch: &Oklch) -> Self {
        oklch.to_oklab().to_linear_rgb()
    }

    pub fn to_srgb(&self) -> Rgb {
        let r = (linear_to_srgb(self.r) * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = (linear_to_srgb(self.g) * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = (linear_to_srgb(self.b) * 255.0).round().clamp(0.0, 255.0) as u8;
        Rgb {
            r,
            g,
            b,
            a: self.a,
        }
    }

    pub fn to_oklab(&self) -> Oklab {
        let l = 0.4122214708 * self.r + 0.5363325363 * self.g + 0.0514459929 * self.b;
        let m = 0.2119034982 * self.r + 0.6806995451 * self.g + 0.1073969566 * self.b;
        let s = 0.0883024619 * self.r + 0.2817188376 * self.g + 0.6299787005 * self.b;

        let l_ = cbrt_signed(l);
        let m_ = cbrt_signed(m);
        let s_ = cbrt_signed(s);

        Oklab {
            l: 0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_,
            a: 1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_,
            b: 0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_,
            alpha: self.a,
        }
    }

    pub fn to_cielab(&self) -> Cielab {
        let x = 0.4124564 * self.r + 0.3575761 * self.g + 0.1804375 * self.b;
        let y = 0.2126729 * self.r + 0.7151522 * self.g + 0.0721750 * self.b;
        let z = 0.0193339 * self.r + 0.1191920 * self.g + 0.9503041 * self.b;

        let xn = 0.950489;
        let yn = 1.000000;
        let zn = 1.088840;

        let fx = lab_f(x / xn);
        let fy = lab_f(y / yn);
        let fz = lab_f(z / zn);

        Cielab {
            l: (116.0 * fy - 16.0).clamp(0.0, 100.0),
            a: 500.0 * (fx - fy),
            b: 200.0 * (fy - fz),
            alpha: self.a,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Oklab {
    pub l: f64,
    pub a: f64,
    pub b: f64,
    pub alpha: f64,
}

impl Oklab {
    pub fn new(l: f64, a: f64, b: f64) -> Self {
        Self { l, a, b, alpha: 1.0 }
    }

    pub fn to_linear_rgb(&self) -> LinearRgb {
        let l_ = self.l + 0.3963377774 * self.a + 0.2158037573 * self.b;
        let m_ = self.l - 0.1055613458 * self.a - 0.0638541728 * self.b;
        let s_ = self.l - 0.0894841775 * self.a - 1.2914855480 * self.b;

        let l = l_ * l_ * l_;
        let m = m_ * m_ * m_;
        let s = s_ * s_ * s_;

        LinearRgb {
            r: 4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
            g: -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
            b: -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s,
            a: self.alpha,
        }
    }

    pub fn to_rgb(&self) -> Rgb {
        self.to_linear_rgb().to_srgb()
    }

    pub fn to_oklch(&self) -> Oklch {
        let c = (self.a * self.a + self.b * self.b).sqrt();
        let mut h = self.b.atan2(self.a).to_degrees();
        if h < 0.0 {
            h += 360.0;
        }
        Oklch {
            l: self.l,
            c,
            h,
            alpha: self.alpha,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Oklch {
    pub l: f64,
    pub c: f64,
    pub h: f64,
    pub alpha: f64,
}

impl Oklch {
    pub fn new(l: f64, c: f64, h: f64) -> Self {
        Self {
            l: l.clamp(0.0, 1.0),
            c: c.max(0.0),
            h: (h % 360.0 + 360.0) % 360.0,
            alpha: 1.0,
        }
    }

    pub fn with_alpha(l: f64, c: f64, h: f64, alpha: f64) -> Self {
        Self {
            l: l.clamp(0.0, 1.0),
            c: c.max(0.0),
            h: (h % 360.0 + 360.0) % 360.0,
            alpha: alpha.clamp(0.0, 1.0),
        }
    }

    pub fn to_oklab(&self) -> Oklab {
        let h_rad = self.h.to_radians();
        Oklab {
            l: self.l,
            a: self.c * h_rad.cos(),
            b: self.c * h_rad.sin(),
            alpha: self.alpha,
        }
    }

    pub fn to_rgb(&self) -> Rgb {
        self.to_oklab().to_rgb()
    }

    pub fn to_hex(&self) -> String {
        self.to_rgb().to_hex()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Cielab {
    pub l: f64,
    pub a: f64,
    pub b: f64,
    pub alpha: f64,
}

impl Cielab {
    pub fn to_rgb(&self) -> Rgb {
        let fy = (self.l + 16.0) / 116.0;
        let fx = self.a / 500.0 + fy;
        let fz = fy - self.b / 200.0;

        let xn = 0.950489;
        let yn = 1.000000;
        let zn = 1.088840;

        let x = xn * lab_f_inv(fx);
        let y = yn * lab_f_inv(fy);
        let z = zn * lab_f_inv(fz);

        let r_lin = 3.2404542 * x - 1.5371385 * y - 0.4985314 * z;
        let g_lin = -0.9692660 * x + 1.8760108 * y + 0.0415560 * z;
        let b_lin = 0.0556434 * x - 0.2040259 * y + 1.0572252 * z;

        LinearRgb {
            r: r_lin,
            g: g_lin,
            b: b_lin,
            a: self.alpha,
        }
        .to_srgb()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Hsl {
    pub h: f64,
    pub s: f64,
    pub l: f64,
    pub a: f64,
}

impl Hsl {
    pub fn to_rgb(&self) -> Rgb {
        let h = (self.h % 360.0 + 360.0) % 360.0;
        let s = (self.s / 100.0).clamp(0.0, 1.0);
        let l = (self.l / 100.0).clamp(0.0, 1.0);

        if s == 0.0 {
            let val = (l * 255.0).round() as u8;
            return Rgb::with_alpha(val, val, val, self.a);
        }

        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };
        let p = 2.0 * l - q;
        let hk = h / 360.0;

        let tr = (hk + 1.0 / 3.0) % 1.0;
        let tg = hk;
        let tb = (hk - 1.0 / 3.0 + 1.0) % 1.0;

        Rgb::with_alpha(
            (hue_to_rgb(p, q, tr) * 255.0).round() as u8,
            (hue_to_rgb(p, q, tg) * 255.0).round() as u8,
            (hue_to_rgb(p, q, tb) * 255.0).round() as u8,
            self.a,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Hsv {
    pub h: f64,
    pub s: f64,
    pub v: f64,
    pub a: f64,
}

impl Hsv {
    pub fn to_rgb(&self) -> Rgb {
        let h = ((self.h % 360.0 + 360.0) % 360.0) / 60.0;
        let s = (self.s / 100.0).clamp(0.0, 1.0);
        let v = (self.v / 100.0).clamp(0.0, 1.0);

        let i = h.floor() as usize;
        let f = h - i as f64;
        let p = v * (1.0 - s);
        let q = v * (1.0 - s * f);
        let t = v * (1.0 - s * (1.0 - f));

        let (r, g, b) = match i % 6 {
            0 => (v, t, p),
            1 => (q, v, p),
            2 => (p, v, t),
            3 => (p, q, v),
            4 => (t, p, v),
            _ => (v, p, q),
        };

        Rgb::with_alpha(
            (r * 255.0).round() as u8,
            (g * 255.0).round() as u8,
            (b * 255.0).round() as u8,
            self.a,
        )
    }
}

#[inline]
fn srgb_to_linear(c: f64) -> f64 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

#[inline]
fn linear_to_srgb(c: f64) -> f64 {
    if c <= 0.0031308 {
        c * 12.92
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

#[inline]
fn cbrt_signed(x: f64) -> f64 {
    x.signum() * x.abs().cbrt()
}

#[inline]
fn lab_f(t: f64) -> f64 {
    let delta = 6.0 / 29.0;
    if t > delta * delta * delta {
        t.cbrt()
    } else {
        t / (3.0 * delta * delta) + 4.0 / 29.0
    }
}

#[inline]
fn lab_f_inv(t: f64) -> f64 {
    let delta = 6.0 / 29.0;
    if t > delta {
        t * t * t
    } else {
        3.0 * delta * delta * (t - 4.0 / 29.0)
    }
}

#[inline]
fn hue_to_rgb(p: f64, q: f64, mut t: f64) -> f64 {
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }
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
