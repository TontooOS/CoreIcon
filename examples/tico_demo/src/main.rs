use CoreIcon::generator::*;
use CoreIcon::tico::Tico;
use CoreIcon::{Color, Gradient, GradientDirection};
use coreimage::{ImageFormat, TiImage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Builder icon: dark background, white circle + shaded rect.
    let gray = Color::from_rgb(140, 140, 140);
    let icon = IconCanvas::new()
        .background(Background::color(Color::from_hex("#1d1d1d").unwrap()))
        .layer(
            Layer::new(LayerContent::circle(560.0))
                .position(232.0, 140.0)
                .tint(Color::WHITE),
        )
        .layer(
            Layer::new(LayerContent::rect(300.0, 300.0, 80.0))
                .position(362.0, 480.0)
                .gradient(Gradient::linear_two(Color::WHITE, gray)
                    .with_direction(GradientDirection::TopToBottom)),
        );

    Tico::export(&icon, "demo", "demo.tico")?;
    let bytes = std::fs::metadata("demo.tico")?.len();
    println!("demo.tico bytes: {}", bytes);

    let t = Tico::load("demo.tico")?;
    println!("layers: {}", t.layer_count());
    TiImage::from_rgba(t.render_default()?).save("tico_default.png", ImageFormat::Png, 100)?;
    println!("saved tico_default.png");
    let red = Color::from_hex("#FF3B30").unwrap();
    TiImage::from_rgba(t.render(1024, Some(red))?).save("tico_red.png", ImageFormat::Png, 100)?;
    println!("saved tico_red.png");
    TiImage::from_rgba(t.render(1024, Some(Color::ACCENT))?).save("tico_blue.png", ImageFormat::Png, 100)?;
    println!("saved tico_blue.png");
    TiImage::from_rgba(t.render(256, Some(Color::ACCENT))?).save("tico_blue_256.png", ImageFormat::Png, 100)?;
    println!("saved tico_blue_256.png");
    Ok(())
}
