use CoreIcon::generator::*;
use CoreIcon::Color;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating VS Code icon variants...");

    // Apple systemRed - fits the TontooOS accent palette.
    let red = Color::from_hex("#FF3B30").unwrap();

    let depth = || {
        DepthOptions::new(220.0)
            .shadow(Shadow::new().offset(0.0, 10.0).blur(20.0).opacity(0.35))
            .inner_depth(12.0, 0.25)
            .specular(0.15)
            .edge_highlight(4.0, 0.2)
    };

    // ── Light: white background, whites in the artwork stay ──
    let recolor_light = RecolorOptions::new(red, 1.0)
        .mode(RecolorMode::Colorize)
        .neutral_threshold(0.08);
    let light = IconCanvas::process_file(
        "../vscode_converted.png",
        &ProcessOptions { recolor: Some(recolor_light), background_replace: None, depth: depth(), ..Default::default() },
    )?;
    light.save("vscode_red.png")?;
    println!("Saved: vscode_red.png");

    // ── Dark gray background ──
    // Shaded mode recolors the whole ribbon; remap(white -> bg) turns the
    // interior cutout (the wedge in the middle) into the background color
    // instead, and protect(bg) keeps the swapped background itself intact.
    let dark_gray = Color::new(0.13, 0.13, 0.15, 1.0);
    let recolor_dark = RecolorOptions::new(red, 1.0)
        .mode(RecolorMode::Shaded)
        .protect(dark_gray)
        .remap(Color::WHITE, dark_gray);
    let dark = IconCanvas::process_file(
        "../vscode_converted.png",
        &ProcessOptions {
            recolor: Some(recolor_dark),
            background_replace: Some(dark_gray),
            depth: depth(),
            ..Default::default()
        },
    )?;
    dark.save("vscode_red_dark.png")?;
    println!("Saved: vscode_red_dark.png");

    // ── Light gray background ──
    let light_gray = Color::from_hex("#E9E9EC").unwrap();
    let recolor_gray = RecolorOptions::new(red, 1.0)
        .mode(RecolorMode::Shaded)
        .protect(light_gray)
        .protect_tolerance(0.10)
        .remap(Color::WHITE, light_gray);
    let gray = IconCanvas::process_file(
        "../vscode_converted.png",
        &ProcessOptions {
            recolor: Some(recolor_gray),
            background_replace: Some(light_gray),
            depth: depth(),
            ..Default::default()
        },
    )?;
    gray.save("vscode_red_lightgray.png")?;
    println!("Saved: vscode_red_lightgray.png");

    Ok(())
}
