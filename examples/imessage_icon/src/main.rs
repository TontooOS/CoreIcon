use CoreIcon::generator::*;
use CoreIcon::{Color, Gradient, GradientDirection, GradientStop, MESSAGE_FILL};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe { CoreIcon::generator::ASSETS_DIR = "../../assets/icons"; }

    let icon = IconCanvas::new()
        .corner_radius(256.0)
        // Liquid Glass post-processing
        .frosted(0.72)
        .specular(0.18)
        .inner_depth(30.0, 0.22)
        .edge_highlight(3.0, 0.12)
        // Background: light mint green (frosted overlay will wash it white)
        .background(Background::gradient(
            Gradient::new(
                GradientDirection::TopToBottom,
                vec![
                    GradientStop::new(Color::from_hex("#B4E8B4").unwrap(), 0.0),
                    GradientStop::new(Color::from_hex("#8BD88B").unwrap(), 0.35),
                    GradientStop::new(Color::from_hex("#5CC85C").unwrap(), 0.7),
                    GradientStop::new(Color::from_hex("#3DA83D").unwrap(), 1.0),
                ],
            )
        ))
        // Bottom shadow overlay
        .layer(
            Layer::new(LayerContent::rect(1024.0, 180.0, 0.0))
                .position(0.0, 844.0)
                .gradient(
                    Gradient::new(
                        GradientDirection::TopToBottom,
                        vec![
                            GradientStop::new(Color::new(0.0, 0.0, 0.0, 0.0), 0.0),
                            GradientStop::new(Color::new(0.0, 0.0, 0.0, 0.18), 1.0),
                        ],
                    )
                )
        )
        // Speech bubble with glossy glass gradient + inner edge shadow
        .layer(
            Layer::new(LayerContent::icon(MESSAGE_FILL))
                .position(162.0, 202.0)
                .size(700.0, 640.0)
                .gradient(
                    Gradient::new(
                        GradientDirection::TopToBottom,
                        vec![
                            GradientStop::new(Color::from_hex("#F0F0F0").unwrap(), 0.0),
                            GradientStop::new(Color::WHITE, 0.12),
                            GradientStop::new(Color::WHITE, 0.45),
                            GradientStop::new(Color::from_hex("#F5F5F5").unwrap(), 0.75),
                            GradientStop::new(Color::from_hex("#D8D8D8").unwrap(), 1.0),
                        ],
                    )
                )
                .shadow(
                    Shadow::new()
                        .offset(0.0, 14.0)
                        .blur(35.0)
                        .opacity(0.30)
                )
                .inner_shadow(
                    Shadow::new()
                        .blur(28.0)
                        .color(Color::new(0.0, 0.0, 0.0, 1.0))
                        .opacity(0.12)
                )
        );

    icon.save("imessage.png")?;
    println!("Saved: imessage.png");
    Ok(())
}
