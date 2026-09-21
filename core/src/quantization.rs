use crate::spaces::{Oklab, Oklch, Rgb};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantizedColor {
    pub rgb: Rgb,
    pub oklch: Oklch,
    pub pixel_count: usize,
    pub weight: f64,
}

#[derive(Clone, Copy)]
struct LabPoint {
    l: f64,
    a: f64,
    b: f64,
}

impl LabPoint {
    fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        let lab = Rgb::new(r, g, b).to_oklab();
        Self {
            l: lab.l,
            a: lab.a,
            b: lab.b,
        }
    }

    fn dist_sq(&self, other: &LabPoint) -> f64 {
        let dl = self.l - other.l;
        let da = self.a - other.a;
        let db = self.b - other.b;
        dl * dl + da * da + db * db
    }

    fn to_rgb(&self) -> Rgb {
        Oklab::new(self.l, self.a, self.b).to_rgb()
    }
}

pub fn quantize_image_oklab(
    pixels_rgba: &[u8],
    k: usize,
    max_iterations: usize,
) -> Vec<QuantizedColor> {
    if pixels_rgba.len() < 4 || k == 0 {
        return Vec::new();
    }

    let mut points: Vec<LabPoint> = Vec::with_capacity(pixels_rgba.len() / 4);
    for chunk in pixels_rgba.chunks_exact(4) {
        let a = chunk[3];
        if a >= 128 {
            points.push(LabPoint::from_rgb(chunk[0], chunk[1], chunk[2]));
        }
    }

    if points.is_empty() {
        return Vec::new();
    }

    let requested_k = k.min(points.len());
    let mut centroids: Vec<LabPoint> = Vec::with_capacity(requested_k);

    // Farthest-point seeding: the first centroid is the first pixel, and each
    // next one is the pixel farthest from every centroid chosen so far. It is
    // the deterministic form of k-means++ and needs no random source.
    //
    // Sampling at a fixed interval, which is what this did before, put every
    // centroid inside the same colour whenever one colour filled the start of
    // the image: eighty blue pixels followed by twenty yellow ones seeded both
    // centroids in the blue, left the second cluster empty, and returned one
    // colour for a request of two.
    centroids.push(points[0]);
    while centroids.len() < requested_k {
        let mut best_idx = 0;
        let mut best_dist = -1.0;
        for (idx, p) in points.iter().enumerate() {
            let nearest = centroids
                .iter()
                .map(|c| p.dist_sq(c))
                .fold(f64::MAX, f64::min);
            if nearest > best_dist {
                best_dist = nearest;
                best_idx = idx;
            }
        }
        if best_dist <= 0.0 {
            break;
        }
        centroids.push(points[best_idx]);
    }

    let target_k = centroids.len();

    let mut assignments: Vec<usize> = vec![0; points.len()];

    for _ in 0..max_iterations {
        let mut changed = false;

        #[cfg(feature = "parallel")]
        {
            use rayon::prelude::*;
            let new_assignments: Vec<usize> = points
                .par_iter()
                .map(|p| {
                    let mut min_d = f64::MAX;
                    let mut best_k = 0;
                    for (c_idx, c) in centroids.iter().enumerate() {
                        let d = p.dist_sq(c);
                        if d < min_d {
                            min_d = d;
                            best_k = c_idx;
                        }
                    }
                    best_k
                })
                .collect();

            if new_assignments != assignments {
                assignments = new_assignments;
                changed = true;
            }
        }

        #[cfg(not(feature = "parallel"))]
        {
            for (idx, p) in points.iter().enumerate() {
                let mut min_d = f64::MAX;
                let mut best_k = 0;
                for (c_idx, c) in centroids.iter().enumerate() {
                    let d = p.dist_sq(c);
                    if d < min_d {
                        min_d = d;
                        best_k = c_idx;
                    }
                }
                if assignments[idx] != best_k {
                    assignments[idx] = best_k;
                    changed = true;
                }
            }
        }

        if !changed {
            break;
        }

        let mut sums = vec![(0.0, 0.0, 0.0, 0usize); target_k];
        for (idx, p) in points.iter().enumerate() {
            let k_idx = assignments[idx];
            sums[k_idx].0 += p.l;
            sums[k_idx].1 += p.a;
            sums[k_idx].2 += p.b;
            sums[k_idx].3 += 1;
        }

        for (c_idx, sum) in sums.iter().enumerate() {
            if sum.3 > 0 {
                let count = sum.3 as f64;
                centroids[c_idx] = LabPoint {
                    l: sum.0 / count,
                    a: sum.1 / count,
                    b: sum.2 / count,
                };
            }
        }
    }

    let mut counts = vec![0usize; target_k];
    for &k_idx in &assignments {
        counts[k_idx] += 1;
    }

    let total_pixels = points.len() as f64;
    let mut results: Vec<QuantizedColor> = centroids
        .into_iter()
        .enumerate()
        .filter(|(idx, _)| counts[*idx] > 0)
        .map(|(idx, c)| {
            let rgb = c.to_rgb();
            let oklch = rgb.to_oklch();
            let pixel_count = counts[idx];
            let weight = pixel_count as f64 / total_pixels;
            QuantizedColor {
                rgb,
                oklch,
                pixel_count,
                weight,
            }
        })
        .collect();

    results.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap_or(std::cmp::Ordering::Equal));
    results
}
