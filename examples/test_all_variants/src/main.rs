use CoreIcon::generator::*;

use CoreIcon::{Color, Gradient, GradientDirection, SFSymbol, HEART_FILL, BOLT_FILL, CHECKMARK, STAR_FILL, HAND_THUMBSUP_FILL};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Point to the library's assets folder
    unsafe { CoreIcon::generator::ASSETS_DIR = "../../assets/icons"; }

    println!("Generating test icon with all variants...");

    let orange = Color::from_hex("#FF6B2B").unwrap();
    let gold = Color::from_hex("#FFD700").unwrap();
    let dark = Color::from_hex("#1d1d1d").unwrap();
    let blue = Color::from_hex("#007AFF").unwrap();
    let green = Color::from_hex("#34C759").unwrap();
    let red = Color::from_hex("#FF3B30").unwrap();
    let purple = Color::from_hex("#AF52DE").unwrap();

    let icon = IconCanvas::new()
        // === BACKGROUND: Gradient (TopToBottom) ===
        .background(Background::gradient(
            Gradient::linear_two(dark, Color::new(0.08, 0.08, 0.12, 1.0))
        ))

        // === LAYER 1: Large rounded rect (background element) ===
        .layer(
            Layer::new(LayerContent::rect(600.0, 600.0, 80.0))
                .position(212.0, 150.0)
                .gradient(
                    Gradient::linear_two(orange, gold)
                        .with_direction(GradientDirection::LeftToRight)
                )
                .shadow(Shadow::new().offset(0.0, 15.0).blur(30.0).opacity(0.4))
        )

        // === LAYER 2: Circle (top-left badge) ===
        .layer(
            Layer::new(LayerContent::circle(120.0))
                .position(50.0, 50.0)
                .tint(red)
                .shadow(Shadow::new().offset(0.0, 6.0).blur(12.0))
        )

        // === LAYER 3: Icon inside circle (standalone icon) ===
        .layer(
            Layer::new(LayerContent::icon(HEART_FILL))
                .position(70.0, 70.0)
                .size(80.0, 80.0)
                .tint(Color::WHITE)
        )

        // === LAYER 4: Main icon (large, centered on gradient rect) ===
        .layer(
            Layer::new(LayerContent::icon(BOLT_FILL))
                .position(362.0, 250.0)
                .size(300.0, 300.0)
                .tint(Color::WHITE)
                .shadow(Shadow::new().offset(0.0, 10.0).blur(20.0).opacity(0.5))
        )

        // === LAYER 5: Small rect (bottom-right badge) ===
        .layer(
            Layer::new(LayerContent::rect(100.0, 100.0, 20.0))
                .position(850.0, 650.0)
                .tint(green)
                .shadow(Shadow::new().offset(0.0, 8.0).blur(16.0))
        )

        // === LAYER 6: Icon inside small rect ===
        .layer(
            Layer::new(LayerContent::icon(CHECKMARK))
                .position(870.0, 670.0)
                .size(60.0, 60.0)
                .tint(Color::WHITE)
        )

        // === LAYER 7: Semi-transparent circle (transparency demo) ===
        .layer(
            Layer::new(LayerContent::circle(80.0))
                .position(100.0, 850.0)
                .tint(blue.with_alpha(0.6))
                .shadow(Shadow::new().offset(0.0, 5.0).blur(10.0))
        )

        // === LAYER 8: Icon with gradient tint ===
        .layer(
            Layer::new(LayerContent::icon(STAR_FILL))
                .position(450.0, 800.0)
                .size(120.0, 120.0)
                .gradient(
                    Gradient::linear_three(red, gold, green)
                )
                .shadow(Shadow::new().offset(0.0, 8.0).blur(16.0))
        )

        // === LAYER 9: Overlapping rect (depth/layer demo) ===
        .layer(
            Layer::new(LayerContent::rect(200.0, 200.0, 40.0))
                .position(700.0, 200.0)
                .tint(purple.with_alpha(0.7))
                .shadow(Shadow::new().offset(0.0, 12.0).blur(24.0))
        )

        // === LAYER 10: Icon overlapping previous rect ===
        .layer(
            Layer::new(LayerContent::icon(HAND_THUMBSUP_FILL))
                .position(730.0, 230.0)
                .size(140.0, 140.0)
                .tint(Color::WHITE)
                .shadow(Shadow::new().offset(0.0, 6.0).blur(12.0))
        )

        // === LAYER 11: Text with solid color ===
        .layer(
            Layer::new(LayerContent::text("TontooOS", 72.0))
                .position(280.0, 770.0)
                .tint(Color::WHITE)
                .shadow(Shadow::new().offset(0.0, 4.0).blur(8.0).opacity(0.5))
        )

        // === LAYER 12: Text with gradient ===
        .layer(
            Layer::new(LayerContent::text("v1.0", 48.0))
                .position(650.0, 900.0)
                .gradient(Gradient::linear_two(gold, orange))
                .shadow(Shadow::new().offset(0.0, 3.0).blur(6.0).opacity(0.4))
        );

    icon.save("test_all_variants.png")?;
    println!("Saved: test_all_variants.png");
    println!("");
    println!("Variants demonstrated:");
    println!("  - Background: Gradient (dark)");
    println!("  - RoundedRect with Gradient fill (orange -> gold)");
    println!("  - Circle with solid color (red)");
    println!("  - Icon with solid tint (white heart)");
    println!("  - Icon with solid tint (white bolt, large)");
    println!("  - Rect with solid color (green)");
    println!("  - Circle with transparency (blue, 0.6 alpha)");
    println!("  - Icon with gradient fill (red -> gold -> green)");
    println!("  - Rect with transparency (purple, 0.7 alpha)");
    println!("  - Icon overlapping rect (depth demo)");
    println!("  - Shadows on every element (per-element config)");
    println!("  - Text with solid color ('TontooOS')");
    println!("  - Text with gradient ('v1.0')");

    Ok(())
}
