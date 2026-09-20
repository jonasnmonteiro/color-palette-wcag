use colorust_core::quantization::quantize_image_oklab;

#[test]
fn test_quantize_empty_buffer() {
    let empty: [u8; 0] = [];
    let res = quantize_image_oklab(&empty, 5, 10);
    assert!(res.is_empty());
}

#[test]
fn test_quantize_solid_color() {
    let mut pixels = Vec::new();
    for _ in 0..100 {
        pixels.extend_from_slice(&[255, 0, 0, 255]);
    }
    let res = quantize_image_oklab(&pixels, 3, 10);
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].rgb.r, 255);
    assert_eq!(res[0].rgb.g, 0);
    assert_eq!(res[0].rgb.b, 0);
    assert!((res[0].weight - 1.0).abs() < 1e-6);
}

#[test]
fn test_quantize_two_distinct_colors() {
    let mut pixels = Vec::new();
    for _ in 0..80 {
        pixels.extend_from_slice(&[0, 0, 255, 255]);
    }
    for _ in 0..20 {
        pixels.extend_from_slice(&[255, 255, 0, 255]);
    }
    let res = quantize_image_oklab(&pixels, 2, 20);
    assert_eq!(res.len(), 2);
    assert!(res[0].weight > res[1].weight);
    assert!((res[0].weight - 0.8).abs() < 0.05);
}
