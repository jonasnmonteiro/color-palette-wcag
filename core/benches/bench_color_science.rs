use criterion::{black_box, criterion_group, criterion_main, Criterion};
use colorust_core::{
    apca_contrast, audit_cvd_contrast, ciede2000, generate_harmony,
    map_to_gamut_binary_search, quantize_image_oklab, relative_luminance,
    simulate_cvd, wcag21_contrast, CvdType, DesignSystemTokens, HarmonyMode,
    Oklch, Rgb, TargetGamut,
};

fn bench_color_conversions(c: &mut Criterion) {
    let rgb = Rgb::new(59, 130, 246);
    let oklch = Oklch::new(0.60, 0.22, 250.0);

    c.bench_function("rgb_to_oklch", |b| {
        b.iter(|| black_box(&rgb).to_oklch())
    });

    c.bench_function("oklch_to_rgb", |b| {
        b.iter(|| Rgb::from_oklch(black_box(&oklch)))
    });

    c.bench_function("relative_luminance", |b| {
        b.iter(|| relative_luminance(black_box(&rgb)))
    });
}

fn bench_contrast_audits(c: &mut Criterion) {
    let fg = Rgb::new(59, 130, 246);
    let bg = Rgb::new(255, 255, 255);

    c.bench_function("wcag21_contrast", |b| {
        b.iter(|| wcag21_contrast(black_box(&fg), black_box(&bg)))
    });

    c.bench_function("apca_contrast", |b| {
        b.iter(|| apca_contrast(black_box(&fg), black_box(&bg)))
    });

    c.bench_function("ciede2000_distance", |b| {
        let lab1 = fg.to_cielab();
        let lab2 = bg.to_cielab();
        b.iter(|| ciede2000(black_box(&lab1), black_box(&lab2)))
    });
}

fn bench_cvd_simulation(c: &mut Criterion) {
    let rgb = Rgb::new(239, 68, 68);

    c.bench_function("simulate_protanopia", |b| {
        b.iter(|| simulate_cvd(black_box(&rgb), CvdType::Protanopia, 1.0))
    });

    c.bench_function("simulate_deuteranopia", |b| {
        b.iter(|| simulate_cvd(black_box(&rgb), CvdType::Deuteranopia, 1.0))
    });

    c.bench_function("audit_cvd_contrast", |b| {
        let fg = Rgb::new(239, 68, 68);
        let bg = Rgb::new(34, 197, 94);
        b.iter(|| audit_cvd_contrast(black_box(&fg), black_box(&bg), CvdType::Protanopia, 1.0))
    });
}

fn bench_gamut_mapping(c: &mut Criterion) {
    let out_color = Oklch::new(0.65, 0.40, 150.0);

    c.bench_function("gamut_clipping_binary_search", |b| {
        b.iter(|| map_to_gamut_binary_search(black_box(&out_color), TargetGamut::Srgb, 1e-4))
    });
}

fn bench_harmonies_and_tokens(c: &mut Criterion) {
    let base = Oklch::new(0.60, 0.20, 250.0);

    c.bench_function("generate_harmony_triadic", |b| {
        b.iter(|| generate_harmony(black_box(&base), HarmonyMode::Triadic, 5))
    });

    c.bench_function("generate_design_tokens", |b| {
        b.iter(|| DesignSystemTokens::from_seed_hex(black_box("#3B82F6")))
    });
}

fn bench_image_quantization(c: &mut Criterion) {
    let mut pixels = Vec::with_capacity(40_000);
    for i in 0..10_000 {
        let r = (i * 7 % 256) as u8;
        let g = (i * 13 % 256) as u8;
        let b = (i * 29 % 256) as u8;
        pixels.extend_from_slice(&[r, g, b, 255]);
    }

    c.bench_function("quantize_image_10k_pixels", |b| {
        b.iter(|| quantize_image_oklab(black_box(&pixels), 5, 10))
    });
}

criterion_group!(
    benches,
    bench_color_conversions,
    bench_contrast_audits,
    bench_cvd_simulation,
    bench_gamut_mapping,
    bench_harmonies_and_tokens,
    bench_image_quantization
);
criterion_main!(benches);
