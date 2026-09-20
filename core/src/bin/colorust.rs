use clap::{Parser, Subcommand};
use colored::*;
use colorust_core::{
    apca_contrast, audit_cvd_contrast, generate_harmony, is_in_gamut,
    map_to_gamut_binary_search, relative_luminance, wcag21_contrast, CvdType,
    ExportFormat, HarmonyMode, Oklch, Rgb, TargetGamut, TokenExporter,
};

#[derive(Parser)]
#[command(name = "colorust")]
#[command(about = "Colorust Suite — Perceptual Color Science & Accessibility Contrast Engine", long_about = None)]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Audit {
        #[arg(help = "Foreground hex color, e.g. #3B82F6")]
        fg: String,
        #[arg(help = "Background hex color, e.g. #FFFFFF")]
        bg: String,
    },
    Harmony {
        #[arg(help = "Base seed hex color, e.g. #3B82F6")]
        base: String,
        #[arg(short, long, default_value = "analogous", help = "analogous, complementary, split-complementary, triadic, tetradic, square, monochromatic, custom")]
        mode: String,
        #[arg(short, long, default_value_t = 5, help = "Number of swatches")]
        count: usize,
    },
    Tokens {
        #[arg(help = "Brand seed hex color, e.g. #3B82F6")]
        seed: String,
        #[arg(short, long, default_value = "css", help = "css, tailwind-v3, tailwind-v4, figma, style-dictionary, swiftui, android")]
        format: String,
    },
    Gamut {
        #[arg(help = "Input hex color or OKLCH, e.g. #00FF88")]
        color: String,
        #[arg(short, long, default_value = "srgb", help = "srgb, p3, rec2020")]
        target: String,
    },
    Convert {
        #[arg(help = "Hex color string, e.g. #3B82F6")]
        hex: String,
    },
}

fn print_color_chip(rgb: &Rgb) -> String {
    format!(
        " {} ",
        "      ".on_truecolor(rgb.r, rgb.g, rgb.b)
    )
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Audit { fg, bg } => {
            let fg_rgb = match Rgb::from_hex(&fg) {
                Some(c) => c,
                None => {
                    eprintln!("{} Invalid foreground hex: {}", "Error:".red().bold(), fg);
                    std::process::exit(1);
                }
            };
            let bg_rgb = match Rgb::from_hex(&bg) {
                Some(c) => c,
                None => {
                    eprintln!("{} Invalid background hex: {}", "Error:".red().bold(), bg);
                    std::process::exit(1);
                }
            };

            let wcag = wcag21_contrast(&fg_rgb, &bg_rgb);
            let apca = apca_contrast(&fg_rgb, &bg_rgb);

            println!("\n{}", "=== Colorust Accessibility Contrast Audit ===".cyan().bold());
            println!(
                "Foreground: {} {} (Lum: {:.4})",
                fg.bold(),
                print_color_chip(&fg_rgb),
                relative_luminance(&fg_rgb)
            );
            println!(
                "Background: {} {} (Lum: {:.4})\n",
                bg.bold(),
                print_color_chip(&bg_rgb),
                relative_luminance(&bg_rgb)
            );

            println!("{}", "--- WCAG 2.1 Standard ---".yellow().bold());
            println!("Contrast Ratio: {:.2}:1", wcag.ratio);
            println!(
                "Normal Text AA (4.5:1):  {}",
                if wcag.normal_aa { "PASS".green().bold() } else { "FAIL".red().bold() }
            );
            println!(
                "Normal Text AAA (7.0:1): {}",
                if wcag.normal_aaa { "PASS".green().bold() } else { "FAIL".red().bold() }
            );
            println!(
                "Large Text AA (3.0:1):   {}",
                if wcag.large_aa { "PASS".green().bold() } else { "FAIL".red().bold() }
            );
            println!(
                "Large Text AAA (4.5:1):  {}\n",
                if wcag.large_aaa { "PASS".green().bold() } else { "FAIL".red().bold() }
            );

            println!("{}", "--- WCAG 3.0 APCA (Lightness Contrast) ---".yellow().bold());
            println!("Lightness Contrast (Lc): {:.1}", apca.lc);
            println!(
                "Polarity: {}",
                if apca.is_dark_on_light { "Dark on Light (Positive)" } else { "Light on Dark (Negative)" }
            );
            println!(
                "Body Text (|Lc| >= 75):  {}",
                if apca.body_text { "PASS".green().bold() } else { "FAIL".red().bold() }
            );
            println!(
                "Fluent Text (|Lc| >= 60): {}",
                if apca.fluent_text { "PASS".green().bold() } else { "FAIL".red().bold() }
            );
            println!(
                "Large Text (|Lc| >= 45):  {}",
                if apca.large_text { "PASS".green().bold() } else { "FAIL".red().bold() }
            );
            println!(
                "Spot Text (|Lc| >= 30):   {}\n",
                if apca.spot_text { "PASS".green().bold() } else { "FAIL".red().bold() }
            );

            println!("{}", "--- CVD (Color Vision Deficiency) Simulation ---".yellow().bold());
            for cvd in &[
                CvdType::Protanopia,
                CvdType::Deuteranopia,
                CvdType::Tritanopia,
                CvdType::Achromatopsia,
            ] {
                let res = audit_cvd_contrast(&fg_rgb, &bg_rgb, *cvd, 1.0);
                let cvd_name = format!("{:?}", cvd);
                println!(
                    "{:<14} | FG: {} {} | BG: {} {} | WCAG: {:.2}:1 ({}) | APCA: Lc {:.1}",
                    cvd_name.cyan(),
                    res.simulated_fg.to_hex(),
                    print_color_chip(&res.simulated_fg),
                    res.simulated_bg.to_hex(),
                    print_color_chip(&res.simulated_bg),
                    res.wcag.ratio,
                    if res.wcag.normal_aa { "AA Pass".green() } else { "AA Fail".red() },
                    res.apca.lc
                );
            }
            println!();
        }

        Commands::Harmony { base, mode, count } => {
            let base_rgb = match Rgb::from_hex(&base) {
                Some(c) => c,
                None => {
                    eprintln!("{} Invalid seed hex: {}", "Error:".red().bold(), base);
                    std::process::exit(1);
                }
            };
            let harmony_mode = match HarmonyMode::from_str(&mode) {
                Some(m) => m,
                None => {
                    eprintln!("{} Unknown harmony mode: {}", "Error:".red().bold(), mode);
                    std::process::exit(1);
                }
            };

            let base_ok = base_rgb.to_oklch();
            let swatches = generate_harmony(&base_ok, harmony_mode, count);

            println!("\n{}", format!("=== Colorust Harmony: {} (Count: {}) ===", mode.to_uppercase(), count).cyan().bold());
            println!("Base: {} {}\n", base.bold(), print_color_chip(&base_rgb));

            for (idx, ok) in swatches.iter().enumerate() {
                let hex = ok.to_hex();
                let rgb = Rgb::from_oklch(ok);
                println!(
                    "Swatch {:02}: {} {} | L: {:.3}, C: {:.3}, H: {:5.1}°",
                    idx + 1,
                    hex.bold(),
                    print_color_chip(&rgb),
                    ok.l,
                    ok.c,
                    ok.h
                );
            }
            println!();
        }

        Commands::Tokens { seed, format } => {
            let exp_fmt = match TokenExporter::parse_format(&format) {
                Some(f) => f,
                None => {
                    eprintln!("{} Unknown format: {}", "Error:".red().bold(), format);
                    std::process::exit(1);
                }
            };

            let output = match TokenExporter::export(&seed, exp_fmt) {
                Some(out) => out,
                None => {
                    eprintln!("{} Failed to generate tokens for hex: {}", "Error:".red().bold(), seed);
                    std::process::exit(1);
                }
            };

            println!("{}", output);
        }

        Commands::Gamut { color, target } => {
            let rgb = match Rgb::from_hex(&color) {
                Some(c) => c,
                None => {
                    eprintln!("{} Invalid color hex: {}", "Error:".red().bold(), color);
                    std::process::exit(1);
                }
            };
            let oklch = rgb.to_oklch();
            let target_gamut = match target.to_lowercase().as_str() {
                "p3" | "display-p3" => TargetGamut::DisplayP3,
                "rec2020" => TargetGamut::Rec2020,
                _ => TargetGamut::Srgb,
            };

            let in_gamut = is_in_gamut(&oklch, target_gamut);
            let result = map_to_gamut_binary_search(&oklch, target_gamut, 1e-4);

            println!("\n{}", "=== Colorust Gamut Mapping ===".cyan().bold());
            println!("Input: {} {}", color.bold(), print_color_chip(&rgb));
            println!("OKLCH: L: {:.4}, C: {:.4}, H: {:.1}°", oklch.l, oklch.c, oklch.h);
            println!("Target Gamut: {:?}", target_gamut);
            println!(
                "In Gamut: {}",
                if in_gamut { "YES (Native)".green().bold() } else { "NO (Out of Bounds)".red().bold() }
            );

            if !in_gamut {
                let mapped_rgb = Rgb::from_oklch(&result.mapped);
                println!(
                    "Clipped Mapped: {} {} (Iterations: {})",
                    result.mapped.to_hex().bold(),
                    print_color_chip(&mapped_rgb),
                    result.iterations
                );
                println!(
                    "Mapped OKLCH:  L: {:.4}, C: {:.4}, H: {:.1}°\n",
                    result.mapped.l, result.mapped.c, result.mapped.h
                );
            } else {
                println!();
            }
        }

        Commands::Convert { hex } => {
            let rgb = match Rgb::from_hex(&hex) {
                Some(c) => c,
                None => {
                    eprintln!("{} Invalid hex: {}", "Error:".red().bold(), hex);
                    std::process::exit(1);
                }
            };

            let oklch = rgb.to_oklch();
            let oklab = rgb.to_oklab();
            let cielab = rgb.to_cielab();
            let hsl = rgb.to_hsl();
            let hsv = rgb.to_hsv();

            println!("\n{}", format!("=== Color Conversions for {} ===", hex).cyan().bold());
            println!("Preview: {}\n", print_color_chip(&rgb));
            println!("sRGB:    rgb({}, {}, {})", rgb.r, rgb.g, rgb.b);
            println!("Hex:     {}", hex.to_uppercase());
            println!("OKLCH:   oklch({:.3}% {:.3} {:.1})", oklch.l * 100.0, oklch.c, oklch.h);
            println!("OKLab:   oklab({:.3}% {:.4} {:.4})", oklab.l * 100.0, oklab.a, oklab.b);
            println!("CIELAB:  lab({:.2}% {:.2} {:.2})", cielab.l, cielab.a, cielab.b);
            println!("HSL:     hsl({:.1}deg, {:.1}%, {:.1}%)", hsl.h, hsl.s * 100.0, hsl.l * 100.0);
            println!("HSV:     hsv({:.1}deg, {:.1}%, {:.1}%)\n", hsv.h, hsv.s * 100.0, hsv.v * 100.0);
        }
    }
}
