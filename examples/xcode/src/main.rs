use CoreIcon::generator::{AppIcon, Background, IconCanvas, Layer, LayerContent};
use CoreIcon::tico::Tico;
use CoreIcon::Color;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Light normal: original colors kept, app-icon depth applied.
    AppIcon::from_file("xcode.png").light().save("xcode_light.png")?;
    println!("saved xcode_light.png");

    // Dark normal: background swapped to TontooOS dark, artwork kept.
    AppIcon::from_file("xcode.png").dark().save("xcode_dark.png")?;
    println!("saved xcode_dark.png");

    // Light red: color filter over the artwork, background untouched.
    let red = Color::from_hex("#FF3B30").unwrap();
    AppIcon::from_file("xcode.png").light().tint(red).save("xcode_red.png")?;
    println!("saved xcode_red.png");

    // Dark red: same filter on the dark background.
    AppIcon::from_file("xcode.png").dark().tint(red).save("xcode_red_dark.png")?;
    println!("saved xcode_red_dark.png");

    // Light green: same filter in green.
    let green = Color::from_hex("#34C759").unwrap();
    AppIcon::from_file("xcode.png").light().tint(green).save("xcode_green.png")?;
    println!("saved xcode_green.png");

    // Dark green: same filter on the dark background.
    AppIcon::from_file("xcode.png").dark().tint(green).save("xcode_green_dark.png")?;
    println!("saved xcode_green_dark.png");

    // App Icon (.tico): embed the finished light icon as raster artwork.
    // TontooOS only accepts `.tico` as app icon (LaunchPad/CoreWindows probe
    // `App/icon.tico`; the Fish Config `manifest.fico` lives inside it).
    // The PNG already carries the full-bleed artwork + glass finish, so one
    // full-size image layer over a white backstop reproduces it exactly;
    // the squircle corner mask is re-applied on render.
    let canvas = IconCanvas::new()
        .background(Background::color(Color::WHITE))
        .layer(
            Layer::new(LayerContent::image("xcode_light.png"))
                .position(0.0, 0.0)
                .size(1024.0, 1024.0),
        );
    Tico::export(&canvas, "xcode", "xcode.tico")?;
    println!("saved xcode.tico ({} bytes)", std::fs::metadata("xcode.tico")?.len());

    // Round-trip check: load back and render at 1024px.
    let loaded = Tico::load("xcode.tico")?;
    assert_eq!(loaded.layer_count(), 1);
    let rendered = loaded.render_default()?;
    assert_eq!((rendered.width(), rendered.height()), (1024, 1024));
    println!("verified xcode.tico renders 1024x1024");
    Ok(())
}
