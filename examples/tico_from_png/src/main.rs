//! Convert a flat app-icon PNG into the layered `.tico` container.
//!
//! The new TICO format stores an icon as recolorable layers, and
//! ArchiveKit rejects a container with an empty layer table. A bare
//! `Background::image` canvas (what TBuild used to build) has zero
//! layers and no longer validates, so the artwork goes in as exactly one
//! image layer instead: the container gets one `layer/00.tlyr`, the layer
//! is flagged non-recolorable (image layers keep their colors), and the
//! Apple app icon finish is applied later by `TicoIcon::render`.
//!
//! Layers are stored at 1024px, so a smaller source is upscaled by the
//! tico rasterizer. A raster app icon lands around 3-4 MB per layer,
//! which is what the app bundles carry.
//!
//! Usage:
//!
//! ```text
//! tico-from-png <input.png> <name> <out.tico> [preview.png]
//! ```

use CoreIcon::generator::{Background, IconCanvas, Layer, LayerContent, CANVAS_SIZE};
use CoreIcon::tico::Tico;
use CoreIcon::Color;
use coreimage::{ImageFormat, TiImage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        eprintln!("usage: tico-from-png <input.png> <name> <out.tico> [preview.png]");
        eprintln!("example: tico-from-png Resources/app_icon.png Weather Weather.app/App/icon.tico");
        std::process::exit(2);
    }
    let input = &args[1];
    let name = &args[2];
    let out = &args[3];
    let preview = args.get(4);

    if !std::path::Path::new(input).is_file() {
        return Err(format!("input not found: {input}").into());
    }

    // One full-bleed image layer over an empty background: the artwork
    // itself, no 3D finish (render adds it) and no tint (keeps colors).
    let canvas = IconCanvas::new()
        .background(Background::color(Color::new(0.0, 0.0, 0.0, 0.0)))
        .layer(
            Layer::new(LayerContent::image(input.clone()))
                .position(0.0, 0.0)
                .size(CANVAS_SIZE as f32, CANVAS_SIZE as f32),
        );

    Tico::export(&canvas, name, out)?;

    // Round-trip check: the container must load and render again.
    let icon = Tico::load(out)?;
    let rendered = icon.render_default()?;
    println!(
        "{out}: {} bytes, {} layer(s), {}x{} render",
        std::fs::metadata(out)?.len(),
        icon.layer_count(),
        rendered.width(),
        rendered.height(),
    );
    for (label, image) in [
        ("default", rendered),
        ("tinted", icon.render(512, Some(Color::ACCENT))?),
    ] {
        // One preview path: the default render there, the tint beside it.
        let path = match preview {
            Some(path) if label == "default" => path.clone(),
            Some(path) => insert_suffix(path, label),
            None => format!("{out}.preview-{label}.png"),
        };
        TiImage::from_rgba(image).save(&path, ImageFormat::Png, 100)?;
        println!("wrote {path}");
    }
    Ok(())
}

/// Insert `-suffix` before the extension of `path`.
fn insert_suffix(path: &str, suffix: &str) -> String {
    match path.rsplit_once('.') {
        Some((stem, ext)) => format!("{stem}-{suffix}.{ext}"),
        None => format!("{path}-{suffix}.png"),
    }
}