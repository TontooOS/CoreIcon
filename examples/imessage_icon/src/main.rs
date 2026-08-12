use CoreIcon::generator::*;
use CoreIcon::{Color, Gradient, GradientDirection, GradientStop, MESSAGE_FILL};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe { CoreIcon::generator::ASSETS_DIR = "../../assets/icons"; }

    let icon = IconCanvas::new()
        .corner_radius(256.0)
        // 3D raised-button depth (no frosted — the background is already pale)
        .specular(0.0)
        .inner_depth(50.0, 0.28)
        .edge_highlight(0.0, 0.0)
        // Background: clearly GREEN mint — saturated enough to contrast
        // with the white bubble, not washed out
        .background(Background::gradient(
            Gradient::new(
                GradientDirection::TopToBottom,
                vec![
                    GradientStop::new(Color::from_hex("#7CD07C").unwrap(), 0.0),
                    GradientStop::new(Color::from_hex("#5CC45C").unwrap(), 0.35),
                    GradientStop::new(Color::from_hex("#44B444").unwrap(), 0.7),
                    GradientStop::new(Color::from_hex("#30A030").unwrap(), 1.0),
                ],
            )
        ))
        // Subtle bottom gradient darkening
        .layer(
            Layer::new(LayerContent::rect(1024.0, 120.0, 0.0))
                .position(0.0, 904.0)
                .gradient(
                    Gradient::new(
                        GradientDirection::TopToBottom,
                        vec![
                            GradientStop::new(Color::new(0.0, 0.0, 0.0, 0.0), 0.0),
                            GradientStop::new(Color::new(0.0, 0.0, 0.0, 0.06), 1.0),
                        ],
                    )
                )
        )
        // Speech bubble: pure white with soft shadow
        .layer(
            Layer::new(LayerContent::icon(MESSAGE_FILL))
                .position(162.0, 202.0)
                .size(700.0, 640.0)
                .tint(Color::WHITE)
                .shadow(
                    Shadow::new()
                        .offset(0.0, 8.0)
                        .blur(40.0)
                        .opacity(0.10)
                        .color(Color::new(0.0, 0.0, 0.0, 1.0))
                )
                .inner_shadow(
                    Shadow::new()
                        .blur(16.0)
                        .color(Color::new(0.0, 0.0, 0.0, 1.0))
                        .opacity(0.06)
                )
        );

    icon.save("imessage.png")?;
    println!("Saved: imessage.png");
    Ok(())
}
