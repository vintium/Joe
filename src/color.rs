pub struct RGB {
    pub r: u8, 
    pub g: u8,
    pub b: u8,
}

impl RGB {
    pub fn from(r: u8, g: u8, b: u8) -> RGB {
        RGB {
            r,
            g,
            b,
        }
    }
    pub fn from_HSV(hsv: &HSV) -> RGB {
        // implementation of an algorithm found at:
        // https://en.wikipedia.org/wiki/HSL_and_HSV#HSV_to_RGB
        let chroma = hsv.v * hsv.s;
        let hue_section = (hsv.h as f64) / 60.0;
        let intermediate = chroma * (
            (1.0 - ((hue_section % 2.0) - 1.0).abs()) as f64
        );
        let pre_rgb: (f64, f64, f64) = if hue_section >= 0.0 &&
            hue_section <= 1.0 {
            (chroma, intermediate, 0.0)
        } else if hue_section > 1.0 && hue_section <= 2.0 {
            (intermediate, chroma, 0.0)
        } else if hue_section > 2.0 && hue_section <= 3.0 {
            (0.0, chroma, intermediate)
        } else if hue_section > 3.0 && hue_section <= 4.0 {
            (0.0, intermediate, chroma)
        } else if hue_section > 4.0 && hue_section <= 5.0 {
            (intermediate, 0.0, chroma)
        } else if hue_section > 5.0 && hue_section <= 6.0 {
            (chroma, 0.0, intermediate)
        } else {
            (0.0, 0.0, 0.0) 
        };
        let m = hsv.v - chroma;
        let rgb = (pre_rgb.0 + m, pre_rgb.1 + m, pre_rgb.2 + m);
        RGB {
            r: ((rgb.0 * 255.0) as u8),
            g: ((rgb.1 * 255.0) as u8),
            b: ((rgb.2 * 255.0) as u8)
        }
    }
}

pub struct HSV {
    pub h: u16, // hue 0 to 360
    pub s: f64, // saturation 0 to 1
    pub v: f64, // value 0 to 1
}


impl HSV {
    pub fn from(h: u16,s: f64,v: f64) -> HSV {
        // make values valid
        let hue = if h < 360 { h } else { 360 };
        let saturation = if s < 0.0 { 0.0 } else if s > 1.0 { 1.0 } else { s };
        let value = if v < 0.0 { 0.0 } else if v > 1.0 { 1.0 } else { v };
        HSV {
            h: hue,
            s: saturation,
            v: value,
        }
    }
}
