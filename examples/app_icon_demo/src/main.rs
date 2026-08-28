use CoreIcon::generator::*;
use CoreIcon::Color;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let src = "../vscode_converted.png";

    // ── API 1: PNG -> 3D App Icon (1024x1024, same colors, glass finish) ──
    IconCanvas::png_to_3d_icon(src)?.save("api_1_3d.png")?;
    println!("Saved: api_1_3d.png");

    // ── API 2: default == API 1 ──
    AppIcon::from_file(src).save("api_2_default.png")?;
    println!("Saved: api_2_default.png");

    // ── API 2: Dark type - background dark, colors kept (VS Code stays blue) ──
    AppIcon::from_file(src).dark().save("api_2_dark_blue.png")?;
    println!("Saved: api_2_dark_blue.png");

    // ── API 2: Color change + Dark type ──
    let red = Color::from_hex("#FF3B30").unwrap();
    AppIcon::from_file(src).dark().tint(red).save("api_2_red_dark.png")?;
    println!("Saved: api_2_red_dark.png");

    // ── API 2: Color change + Light type ──
    AppIcon::from_file(src).tint(red).save("api_2_red_light.png")?;
    println!("Saved: api_2_red_light.png");

    // ── More accent colors (dark type) ──
    let accents = [
        ("orange", Color::ORANGE),
        ("blue", Color::ACCENT),
        ("green", Color::GREEN),
        ("purple", Color::from_hex("#AF52DE").unwrap()),
        ("yellow", Color::YELLOW),
        ("pink", Color::from_hex("#FF2D55").unwrap()),
    ];
    for (name, color) in accents {
        let out = format!("api_2_{}_dark.png", name);
        AppIcon::from_file(src).dark().tint(color).save(&out)?;
        println!("Saved: {}", out);
    }

    Ok(())
}
