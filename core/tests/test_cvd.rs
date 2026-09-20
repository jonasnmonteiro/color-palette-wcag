use colorust_core::cvd::{audit_cvd_contrast, simulate_cvd, CvdType};
use colorust_core::spaces::Rgb;

#[test]
fn test_protanopia_simulation() {
    let red = Rgb::new(255, 0, 0);
    let sim = simulate_cvd(&red, CvdType::Protanopia, 1.0);

    assert!(sim.r > 130 && sim.r < 160);
    assert!(sim.g > 130 && sim.g < 160);
    assert_eq!(sim.b, 0);
}

#[test]
fn test_deuteranopia_simulation() {
    let green = Rgb::new(0, 255, 0);
    let sim = simulate_cvd(&green, CvdType::Deuteranopia, 1.0);

    assert!(sim.r > 80 && sim.r < 110);
    assert!(sim.g > 60 && sim.g < 90);
    assert!(sim.b > 60 && sim.b < 90);
}

#[test]
fn test_tritanopia_simulation() {
    let blue = Rgb::new(0, 0, 255);
    let sim = simulate_cvd(&blue, CvdType::Tritanopia, 1.0);

    assert_eq!(sim.r, 0);
    assert!(sim.g > 130 && sim.g < 160);
    assert!(sim.b > 120 && sim.b < 150);
}

#[test]
fn test_achromatopsia_simulation() {
    let rgb = Rgb::new(200, 100, 50);
    let sim = simulate_cvd(&rgb, CvdType::Achromatopsia, 1.0);

    assert_eq!(sim.r, sim.g);
    assert_eq!(sim.g, sim.b);
}

#[test]
fn test_cvd_contrast_audit() {
    let fg = Rgb::new(255, 0, 0);
    let bg = Rgb::new(0, 255, 0);
    let audit = audit_cvd_contrast(&fg, &bg, CvdType::Deuteranopia, 1.0);

    assert_eq!(audit.cvd_type, CvdType::Deuteranopia);
    assert!(audit.wcag.ratio < 4.5);
}
