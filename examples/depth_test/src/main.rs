use CoreIcon::generator::IconCanvas;
use CoreIcon::generator::IconMode;
use CoreIcon::Color;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = "../images.jpg";
    let png_output = "vscode_converted.png";
    let depth_output = "vscode_with_depth.png";
    let orange_output = "vscode_orange.png";
    let dark_output = "vscode_dark.png";

    // 1. Convert JPG to PNG
    println!("Converting {} to PNG...", input);
    let img = image::open(input)?;
    img.save(png_output)?;
    println!("Saved: {} ({}x{})", png_output, img.width(), img.height());

    // 2. Apply depth effects
    println!("\nApplying depth effects...");
    let result = IconCanvas::add_depth_to_image(
        input,
        220.0,             // corner_radius (iOS-style rounded icon)
        Some(0.0),         // shadow_offset_x
        Some(8.0),         // shadow_offset_y
        Some(12.0),        // shadow_blur
        Some(0.3),         // shadow_opacity
        Some(10.0),        // inner_depth_blur
        Some(0.25),        // inner_depth_opacity
        Some(0.15),        // specular_opacity
        Some(4.0),         // edge_highlight_width
        Some(0.2),         // edge_highlight_opacity
    )?;
    result.save(depth_output)?;
    println!("Saved: {}", depth_output);

    // 3. Change color to orange (TontooOS accent) with depth
    println!("\nChanging color to orange...");
    let orange = Color::from_hex("#FF6B2B").unwrap();
    let result = IconCanvas::change_color(
        input,
        orange,
        0.8,               // intensity (80% tinted)
        220.0,             // corner_radius
        Some(0.0),         // shadow_offset_x
        Some(8.0),         // shadow_offset_y
        Some(12.0),        // shadow_blur
        Some(0.3),         // shadow_opacity
        Some(10.0),        // inner_depth_blur
        Some(0.25),        // inner_depth_opacity
        Some(0.15),        // specular_opacity
        Some(4.0),         // edge_highlight_width
        Some(0.2),         // edge_highlight_opacity
    )?;
    result.save(orange_output)?;
    println!("Saved: {}", orange_output);

    // 4. Dark mode: background -> black, logo stays white
    println!("\nSwitching to dark mode...");
    let result = IconCanvas::dark_light_mode(
        input,
        IconMode::Dark,
        220.0,             // corner_radius
        Some(0.0),         // shadow_offset_x
        Some(8.0),         // shadow_offset_y
        Some(12.0),        // shadow_blur
        Some(0.3),         // shadow_opacity
        Some(10.0),        // inner_depth_blur
        Some(0.25),        // inner_depth_opacity
        Some(0.15),        // specular_opacity
        Some(4.0),         // edge_highlight_width
        Some(0.2),         // edge_highlight_opacity
    )?;
    result.save(dark_output)?;
    println!("Saved: {}", dark_output);

    println!("\nDone! Check the output files.");
    Ok(())
}
