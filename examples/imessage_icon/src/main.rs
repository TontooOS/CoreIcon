use CoreIcon::generator::*;
use CoreIcon::{Color, Gradient, GradientDirection, GradientStop, MESSAGE_FILL};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe { CoreIcon::generator::ASSETS_DIR = "../../assets/icons"; }

    let icon = IconCanvas::new()
        .corner_radius(256.0)
        // Liquid Glass post-processing
        .frosted(0.18)
        .specular(0.30)
        .inner_depth(40.0, 0.35)
        .edge_highlight(8.0, 0.45)
        // Background: extremely pale mint — almost white, vertical gradient
        .background(Background::gradient(
            Gradient::new(
                GradientDirection::TopToBottom,
                vec![
                    GradientStop::new(Color::from_hex("#F4FBF4").unwrap(), 0.0),
                    GradientStop::new(Color::from_hex("#E8F5E8").unwrap(), 0.4),
                    GradientStop::new(Color::from_hex("#DCF0DC").unwrap(), 0.8),
                    GradientStop::new(Color::from_hex("#D0EBD0").unwrap(), 1.0),
                ],
            )
        ))
        // Subtle bottom shadow (very light)
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
        // Speech bubble: pure white, soft subtle glow
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
