use CoreIcon::generator::*;
use CoreIcon::Color;

/// The two color types applied on top of plain Light / Dark.
const COLOR_TYPES: &[(&str, Color)] = &[
    ("tontoo", Color::TONTOO_ACCENT), // orange
    ("accent", Color::ACCENT),        // blue
];

const ICONS: &[&str] = &["fox", "gomp", "settings"];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for icon in ICONS {
        let src = format!("{}/images.jpg", icon);

        // Plain Light / Dark (original colors)
        AppIcon::from_file(&src).save(format!("{}/{}_light.png", icon, icon))?;
        AppIcon::from_file(&src).dark().save(format!("{}/{}_dark.png", icon, icon))?;

        // Light / Dark with each color type
        for (cname, color) in COLOR_TYPES {
            AppIcon::from_file(&src)
                .tint(*color)
                .save(format!("{}/{}_light_{}.png", icon, icon, cname))?;
            AppIcon::from_file(&src)
                .dark()
                .tint(*color)
                .save(format!("{}/{}_dark_{}.png", icon, icon, cname))?;
        }
        println!("{}: 6 icons generated", icon);
    }
    Ok(())
}
