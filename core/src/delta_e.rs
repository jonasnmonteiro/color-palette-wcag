use crate::spaces::Cielab;

pub fn ciede2000(c1: &Cielab, c2: &Cielab) -> f64 {
    let k_l = 1.0;
    let k_c = 1.0;
    let k_h = 1.0;

    let c1_star = (c1.a * c1.a + c1.b * c1.b).sqrt();
    let c2_star = (c2.a * c2.a + c2.b * c2.b).sqrt();
    let c_bar = (c1_star + c2_star) / 2.0;

    let c_bar_7 = c_bar.powi(7);
    let g = 0.5 * (1.0 - (c_bar_7 / (c_bar_7 + 25.0_f64.powi(7))).sqrt());

    let a1_prime = (1.0 + g) * c1.a;
    let a2_prime = (1.0 + g) * c2.a;

    let c1_prime = (a1_prime * a1_prime + c1.b * c1.b).sqrt();
    let c2_prime = (a2_prime * a2_prime + c2.b * c2.b).sqrt();
    let c_bar_prime = (c1_prime + c2_prime) / 2.0;

    let mut h1_prime = c1.b.atan2(a1_prime).to_degrees();
    if h1_prime < 0.0 {
        h1_prime += 360.0;
    }
    let mut h2_prime = c2.b.atan2(a2_prime).to_degrees();
    if h2_prime < 0.0 {
        h2_prime += 360.0;
    }

    let delta_h_prime = if c1_prime == 0.0 || c2_prime == 0.0 {
        0.0
    } else if (h1_prime - h2_prime).abs() <= 180.0 {
        h2_prime - h1_prime
    } else if h2_prime <= h1_prime {
        h2_prime - h1_prime + 360.0
    } else {
        h2_prime - h1_prime - 360.0
    };

    let delta_big_h_prime =
        2.0 * (c1_prime * c2_prime).sqrt() * (delta_h_prime.to_radians() / 2.0).sin();

    let delta_l_prime = c2.l - c1.l;
    let delta_c_prime = c2_prime - c1_prime;

    let l_bar_prime = (c1.l + c2.l) / 2.0;

    let h_bar_prime = if c1_prime == 0.0 || c2_prime == 0.0 {
        h1_prime + h2_prime
    } else if (h1_prime - h2_prime).abs() <= 180.0 {
        (h1_prime + h2_prime) / 2.0
    } else if (h1_prime + h2_prime) < 360.0 {
        (h1_prime + h2_prime + 360.0) / 2.0
    } else {
        (h1_prime + h2_prime - 360.0) / 2.0
    };

    let t = 1.0 - 0.17 * (h_bar_prime - 30.0).to_radians().cos()
        + 0.24 * (2.0 * h_bar_prime).to_radians().cos()
        + 0.32 * (3.0 * h_bar_prime + 6.0).to_radians().cos()
        - 0.20 * (4.0 * h_bar_prime - 63.0).to_radians().cos();

    let delta_theta =
        30.0 * (-(((h_bar_prime - 275.0) / 25.0).powi(2))).exp();

    let c_bar_prime_7 = c_bar_prime.powi(7);
    let r_c = 2.0 * (c_bar_prime_7 / (c_bar_prime_7 + 25.0_f64.powi(7))).sqrt();

    let r_t = -r_c * (2.0 * delta_theta).to_radians().sin();

    let s_l = 1.0
        + (0.015 * (l_bar_prime - 50.0).powi(2))
            / (20.0 + (l_bar_prime - 50.0).powi(2)).sqrt();
    let s_c = 1.0 + 0.045 * c_bar_prime;
    let s_h = 1.0 + 0.015 * c_bar_prime * t;

    let term_l = delta_l_prime / (k_l * s_l);
    let term_c = delta_c_prime / (k_c * s_c);
    let term_h = delta_big_h_prime / (k_h * s_h);

    (term_l * term_l + term_c * term_c + term_h * term_h + r_t * term_c * term_h)
        .max(0.0)
        .sqrt()
}
