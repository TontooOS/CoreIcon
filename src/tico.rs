// TICO - TontooOS icon container.
//
// A `.tico` file is a TICO container as defined by ArchiveKit
// (`ArchiveKit/src/tico.rs`): the same indexed single-file engine as `.app`
// (TAPP) containers with its own `TICO`/`TICF` magic, a central directory
// and a CRC-checked footer. It is only ever named `*.tico` (never
// `*.tico.zip`). It contains no preview image and no `.png` files:
//
//   manifest.fico      metadata, background, layer table (Fish Config syntax)
//   layer/00.tlyr      one custom layer file per entry
//   layer/01.tlyr      ...
//
// A `.tlyr` file is a tiny custom format: magic `TLYR`, version, dimensions
// and PNG-coded RGBA bytes. Layers are stored at full 1024px so re-renders
// stay sharp; flat builder artwork (shapes, symbols) compresses to a few KB
// per layer, keeping whole icons in the 1-100KB budget.
//
// Rendering (`TicoIcon::render`) composites the layers at 1024px, applies an
// optional tint color (luminance-graded, overlaps stay darker) and the Apple
// app-icon finish, then scales to the requested size.
//
// Listing names or reading one layer only touches the footer plus the
// central directory (or the single entry), so icons open in milliseconds
// without scanning the whole file.

use crate::generator::{Background, IconCanvas, LayerContent, CANVAS_SIZE};
use crate::img::{load_rgba, resize_fill};
use crate::{Color, GradientDirection};
use coreimage::{Rgba, RgbaImage};
use std::path::Path;

/// TICO manifest version.
pub const TICO_VERSION: u32 = 1;
/// Magic bytes at the start of every `.tlyr` file.
pub const TLYR_MAGIC: &[u8; 4] = b"TLYR";
/// `.tlyr` format version.
pub const TLYR_VERSION: u8 = 1;

// ── Errors ──────────────────────────────────────────────────────────

/// Everything that can go wrong while writing, reading or rendering TICO.
#[derive(Debug)]
pub enum TicoError {
    Io(std::io::Error),
    Container(String),
    Image(String),
    Format(String),
}

impl std::fmt::Display for TicoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "tico io: {}", e),
            Self::Container(e) => write!(f, "tico container: {}", e),
            Self::Image(e) => write!(f, "tico image: {}", e),
            Self::Format(e) => write!(f, "tico format: {}", e),
        }
    }
}

impl std::error::Error for TicoError {}

impl From<std::io::Error> for TicoError {
    fn from(e: std::io::Error) -> Self { Self::Io(e) }
}
impl From<coreimage::ImageError> for TicoError {
    fn from(e: coreimage::ImageError) -> Self { Self::Image(e.to_string()) }
}
impl From<archivekit::ArchiveError> for TicoError {
    fn from(e: archivekit::ArchiveError) -> Self { Self::Container(e.to_string()) }
}

// ── Manifest ────────────────────────────────────────────────────────

/// Background of a `.tico` icon (no raster needed for flat icons).
#[derive(Debug, Clone)]
pub enum TicoBackground {
    Color { color: String },
    Gradient { colors: Vec<String>, positions: Vec<f32>, direction: GradientDirection },
    Raster { file: String },
}

/// One entry of the manifest layer table.
#[derive(Debug, Clone)]
pub struct TicoLayerMeta {
    pub file: String,
    pub opacity: f32,
    pub recolorable: bool,
    pub default_color: String,
}

fn direction_to_string(dir: GradientDirection) -> String {
    match dir {
        GradientDirection::TopToBottom => "TopToBottom",
        GradientDirection::BottomToTop => "BottomToTop",
        GradientDirection::LeftToRight => "LeftToRight",
        GradientDirection::RightToLeft => "RightToLeft",
        GradientDirection::TopLeadingToBottomTrailing => "TopLeadingToBottomTrailing",
        GradientDirection::TopTrailingToBottomLeading => "TopTrailingToBottomLeading",
        GradientDirection::CenterRadial => "CenterRadial",
    }
    .to_string()
}

fn direction_from_string(s: &str) -> Result<GradientDirection, TicoError> {
    match s {
        "TopToBottom" => Ok(GradientDirection::TopToBottom),
        "BottomToTop" => Ok(GradientDirection::BottomToTop),
        "LeftToRight" => Ok(GradientDirection::LeftToRight),
        "RightToLeft" => Ok(GradientDirection::RightToLeft),
        "TopLeadingToBottomTrailing" => Ok(GradientDirection::TopLeadingToBottomTrailing),
        "TopTrailingToBottomLeading" => Ok(GradientDirection::TopTrailingToBottomLeading),
        "CenterRadial" => Ok(GradientDirection::CenterRadial),
        _ => Err(TicoError::Format(format!("unknown gradient direction '{s}'"))),
    }
}

// ── TLYR layer files ────────────────────────────────────────────────

fn encode_tlyr(img: &RgbaImage) -> Result<Vec<u8>, TicoError> {
    let png = coreimage::codecs::png::encode(img.width(), img.height(), img.as_raw())
        .map_err(|e| TicoError::Image(e.to_string()))?;
    let mut out = Vec::with_capacity(16 + png.len());
    out.extend_from_slice(TLYR_MAGIC);
    out.push(TLYR_VERSION);
    out.push(1); // kind 1 = RGBA
    out.extend_from_slice(&(img.width()).to_le_bytes());
    out.extend_from_slice(&(img.height()).to_le_bytes());
    out.extend_from_slice(&(png.len() as u32).to_le_bytes());
    out.extend_from_slice(&png);
    Ok(out)
}

fn decode_tlyr(bytes: &[u8]) -> Result<RgbaImage, TicoError> {
    if bytes.len() < 16 {
        return Err(TicoError::Format("tlyr too short".into()));
    }
    if &bytes[0..4] != TLYR_MAGIC {
        return Err(TicoError::Format("bad tlyr magic".into()));
    }
    if bytes[4] != TLYR_VERSION {
        return Err(TicoError::Format(format!("unsupported tlyr v{}", bytes[4])));
    }
    let w = u32::from_le_bytes([bytes[6], bytes[7], bytes[8], bytes[9]]);
    let h = u32::from_le_bytes([bytes[10], bytes[11], bytes[12], bytes[13]]);
    let len = u32::from_le_bytes([bytes[14], bytes[15], bytes[16], bytes[17]]) as usize;
    if bytes.len() < 18 + len {
        return Err(TicoError::Format("tlyr truncated".into()));
    }
    let img = coreimage::TiImage::from_bytes(&bytes[18..18 + len])
        .map_err(|e| TicoError::Image(e.to_string()))?
        .into_rgba();
    if img.width() != w || img.height() != h {
        return Err(TicoError::Format("tlyr dimension mismatch".into()));
    }
    Ok(img)
}

// ── Color helpers ───────────────────────────────────────────────────

fn color_to_hex(c: Color) -> String {
    format!(
        "#{:02X}{:02X}{:02X}",
        (c.r * 255.0).round().clamp(0.0, 255.0) as u8,
        (c.g * 255.0).round().clamp(0.0, 255.0) as u8,
        (c.b * 255.0).round().clamp(0.0, 255.0) as u8
    )
}

fn rgb_to_hsl(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;
    if max - min < 0.0001 {
        return (0.0, 0.0, l);
    }
    let d = max - min;
    let s = if l > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };
    let h = if max == r {
        ((g - b) / d + if g < b { 6.0 } else { 0.0 }) / 6.0
    } else if max == g {
        ((b - r) / d + 2.0) / 6.0
    } else {
        ((r - g) / d + 4.0) / 6.0
    };
    (h, s, l)
}

/// Luminance-graded tint: every pixel becomes `tint * (l / tint_l)`, so all
/// artwork shares one hue while shading and overlaps survive.
fn shaded_apply(img: &RgbaImage, tint: Color) -> RgbaImage {
    let (_, _, tgt_l) = rgb_to_hsl(tint.r, tint.g, tint.b);
    let mut out = RgbaImage::new(img.width(), img.height());
    for (x, y, p) in img.enumerate_pixels() {
        if p[3] < 3 {
            out.put_pixel(x, y, *p);
            continue;
        }
        let r = p[0] as f32 / 255.0;
        let g = p[1] as f32 / 255.0;
        let b = p[2] as f32 / 255.0;
        let (_, _, l) = rgb_to_hsl(r, g, b);
        let k = if tgt_l <= 0.001 { l } else { (l / tgt_l).min(1.0) };
        out.put_pixel(x, y, Rgba([
            (tint.r * k * 255.0).round().clamp(0.0, 255.0) as u8,
            (tint.g * k * 255.0).round().clamp(0.0, 255.0) as u8,
            (tint.b * k * 255.0).round().clamp(0.0, 255.0) as u8,
            p[3],
        ]));
    }
    out
}

fn blend_over(dst: &mut RgbaImage, src: &RgbaImage) {
    debug_assert_eq!((dst.width(), dst.height()), (src.width(), src.height()));
    for (x, y, s) in src.enumerate_pixels() {
        let sa = s[3] as f32 / 255.0;
        if sa <= 0.001 {
            continue;
        }
        if sa >= 0.999 {
            dst.put_pixel(x, y, *s);
            continue;
        }
        let d = *dst.get_pixel(x, y);
        let da = d[3] as f32 / 255.0;
        let out_a = sa + da * (1.0 - sa);
        if out_a < 0.001 {
            continue;
        }
        dst.put_pixel(x, y, Rgba([
            ((s[0] as f32 * sa + d[0] as f32 * da * (1.0 - sa)) / out_a).round() as u8,
            ((s[1] as f32 * sa + d[1] as f32 * da * (1.0 - sa)) / out_a).round() as u8,
            ((s[2] as f32 * sa + d[2] as f32 * da * (1.0 - sa)) / out_a).round() as u8,
            (out_a * 255.0).round() as u8,
        ]));
    }
}

fn sample_tico_gradient(colors: &[Color], positions: &[f32], dir: GradientDirection, x: u32, y: u32, size: u32) -> Color {
    if colors.is_empty() {
        return Color::WHITE;
    }
    if colors.len() == 1 || positions.len() != colors.len() {
        return colors[0];
    }
    let (w, h) = (size as f32, size as f32);
    let t = match dir {
        GradientDirection::TopToBottom => y as f32 / h,
        GradientDirection::BottomToTop => 1.0 - y as f32 / h,
        GradientDirection::LeftToRight => x as f32 / w,
        GradientDirection::RightToLeft => 1.0 - x as f32 / w,
        _ => y as f32 / h,
    }
    .clamp(0.0, 1.0);
    let mut lo = 0usize;
    for i in 0..positions.len() - 1 {
        if t >= positions[i] && t <= positions[i + 1] {
            lo = i;
            break;
        }
        if t > positions[i + 1] {
            lo = i + 1;
        }
    }
    let hi = (lo + 1).min(colors.len() - 1);
    let range = (positions[hi] - positions[lo]).max(0.001);
    let f = ((t - positions[lo]) / range).clamp(0.0, 1.0);
    let (a, b) = (colors[lo], colors[hi]);
    Color::new(
        a.r + (b.r - a.r) * f,
        a.g + (b.g - a.g) * f,
        a.b + (b.b - a.b) * f,
        a.a + (b.a - a.a) * f,
    )
}

// ── Export ──────────────────────────────────────────────────────────

/// Write an [`IconCanvas`] as `*.tico` (TICO container, no PNG files inside).
///
/// Every layer is rasterized at full 1024px into one `layer/NN.tlyr` file.
/// Flat builder artwork compresses to a few KB per layer (1-100KB total).
/// Photo (`Image`) backgrounds are embedded as their own raster layer.
pub struct Tico;

impl Tico {
    fn build_manifest(
        canvas: &IconCanvas,
        name: &str,
    ) -> Result<(archivekit::TicoManifest, Vec<(String, Vec<u8>)>), TicoError> {
        let mut files: Vec<(String, Vec<u8>)> = Vec::new();

        let bg = match canvas.tico_background() {
            Background::Color(c) => archivekit::TicoBackground::Color { color: color_to_hex(c) },
            Background::Gradient(g) => archivekit::TicoBackground::Gradient {
                colors: g.stops.iter().map(|s| color_to_hex(s.color)).collect(),
                positions: g.stops.iter().map(|s| s.position).collect(),
                direction: direction_to_string(g.direction),
            },
            Background::Image { path, .. } => {
                let img = load_rgba(&path)
                    .map_err(|e| TicoError::Format(format!("background image: {}", e)))?;
                let img = resize_fill(&img, CANVAS_SIZE, CANVAS_SIZE)
                    .map_err(|e| TicoError::Format(format!("background image: {}", e)))?;
                files.push(("layer/background.tlyr".into(), encode_tlyr(&img)?));
                archivekit::TicoBackground::Raster { file: "layer/background.tlyr".into() }
            }
        };

        let mut layers = Vec::new();
        for i in 0..canvas.tico_layer_count() {
            let layer = canvas
                .tico_layer(i)
                .ok_or_else(|| TicoError::Format(format!("missing layer {}", i)))?;
            let raster = canvas
                .tico_rasterize_layer(i)
                .ok_or_else(|| TicoError::Format(format!("cannot rasterize layer {}", i)))?;
            let file = format!("layer/{:02}.tlyr", layers.len());
            files.push((file.clone(), encode_tlyr(&raster)?));
            layers.push(archivekit::TicoLayerMeta {
                file,
                opacity: layer.opacity,
                recolorable: !matches!(layer.content, LayerContent::Image { .. }),
                default_color: color_to_hex(layer.fill.unwrap_or(Color::WHITE)),
            });
        }

        Ok((
            archivekit::TicoManifest {
                name: name.into(),
                canvas: CANVAS_SIZE,
                background: bg,
                layers,
            },
            files,
        ))
    }

    /// Pack an [`IconCanvas`] into `.tico` bytes (TICO container).
    pub fn export_bytes(canvas: &IconCanvas, name: &str) -> Result<Vec<u8>, TicoError> {
        let (manifest, files) = Self::build_manifest(canvas, name)?;
        let mut builder = archivekit::TicoBuilder::new();
        builder.set_manifest(manifest);
        for (path, bytes) in &files {
            if path.ends_with(".tlyr") {
                builder.add_layer(path, bytes.clone())?;
            } else {
                builder.add_file(path, bytes.clone())?;
            }
        }
        Ok(builder.finish()?)
    }

    /// Write an [`IconCanvas`] as `*.tico` (TICO container, no PNG files inside).
    ///
    /// Every layer is rasterized at full 1024px into one `layer/NN.tlyr` file.
    /// Flat builder artwork compresses to a few KB per layer (1-100KB total).
    /// Photo (`Image`) backgrounds are embedded as their own raster layer.
    pub fn export(canvas: &IconCanvas, name: &str, path: impl AsRef<Path>) -> Result<(), TicoError> {
        let bytes = Self::export_bytes(canvas, name)?;
        std::fs::write(path.as_ref(), bytes)?;
        Ok(())
    }

    /// Load a `*.tico` file for rendering.
    pub fn load(path: impl AsRef<Path>) -> Result<TicoIcon, TicoError> {
        let bytes = std::fs::read(path.as_ref())?;
        Self::load_bytes(&bytes)
    }

    /// Load `.tico` bytes for rendering.
    pub fn load_bytes(bytes: &[u8]) -> Result<TicoIcon, TicoError> {
        let mut reader = archivekit::TicoReader::from_bytes(bytes)?;
        let manifest = reader.read_manifest()?;

        let mut read_layer = |file: &str| -> Result<RgbaImage, TicoError> {
            let buf = reader.read_file(file)?;
            decode_tlyr(&buf)
        };

        // Decode raster background (if any) and all layers.
        let bg_image = match &manifest.background {
            archivekit::TicoBackground::Raster { file } => Some(read_layer(file)?),
            _ => None,
        };
        let background = match &manifest.background {
            archivekit::TicoBackground::Color { color } => TicoBackground::Color {
                color: color.clone(),
            },
            archivekit::TicoBackground::Gradient { colors, positions, direction } => {
                TicoBackground::Gradient {
                    colors: colors.clone(),
                    positions: positions.clone(),
                    direction: direction_from_string(direction)?,
                }
            }
            archivekit::TicoBackground::Raster { file } => TicoBackground::Raster {
                file: file.clone(),
            },
        };
        let mut layers = Vec::new();
        for meta in &manifest.layers {
            let image = read_layer(&meta.file)?;
            layers.push(LoadedLayer {
                meta: TicoLayerMeta {
                    file: meta.file.clone(),
                    opacity: meta.opacity,
                    recolorable: meta.recolorable,
                    default_color: meta.default_color.clone(),
                },
                image,
            });
        }

        Ok(TicoIcon {
            name: manifest.name,
            background,
            bg_image,
            layers,
        })
    }
}

/// A loaded `.tico` icon: high-res rendering in any color.
pub struct TicoIcon {
    name: String,
    background: TicoBackground,
    bg_image: Option<RgbaImage>,
    layers: Vec<LoadedLayer>,
}

struct LoadedLayer {
    meta: TicoLayerMeta,
    image: RgbaImage,
}

impl TicoIcon {
    pub fn name(&self) -> &str { &self.name }
    pub fn layer_count(&self) -> usize { self.layers.len() }

    /// Render at `size` px with an optional `tint` for recolorable layers.
    /// Always composites at 1024px with the Apple app-icon finish, then
    /// scales to `size` (`16`–`4096`, default `1024`).
    pub fn render(&self, size: u32, tint: Option<Color>) -> Result<RgbaImage, TicoError> {
        let size = size.clamp(16, 4096);
        let mut base = match &self.background {
            TicoBackground::Color { color } => {
                let c = Color::from_hex(color)
                    .ok_or_else(|| TicoError::Format(format!("bad color {}", color)))?;
                RgbaImage::from_pixel(
                    CANVAS_SIZE,
                    CANVAS_SIZE,
                    Rgba([
                        (c.r * 255.0).round() as u8,
                        (c.g * 255.0).round() as u8,
                        (c.b * 255.0).round() as u8,
                        255,
                    ]),
                )
            }
            TicoBackground::Gradient { colors, positions, direction } => {
                let cols: Vec<Color> = colors
                    .iter()
                    .map(|s| {
                        Color::from_hex(s)
                            .ok_or_else(|| TicoError::Format(format!("bad color {}", s)))
                    })
                    .collect::<Result<_, _>>()?;
                let mut img = RgbaImage::new(CANVAS_SIZE, CANVAS_SIZE);
                for y in 0..CANVAS_SIZE {
                    for x in 0..CANVAS_SIZE {
                        let c = sample_tico_gradient(&cols, positions, *direction, x, y, CANVAS_SIZE);
                        img.put_pixel(x, y, Rgba([
                            (c.r * 255.0).round().clamp(0.0, 255.0) as u8,
                            (c.g * 255.0).round().clamp(0.0, 255.0) as u8,
                            (c.b * 255.0).round().clamp(0.0, 255.0) as u8,
                            255,
                        ]));
                    }
                }
                img
            }
            TicoBackground::Raster { .. } => self.bg_image.clone().ok_or_else(|| {
                TicoError::Format("missing background raster".into())
            })?,
        };

        for layer in &self.layers {
            let mut img = layer.image.clone();
            if layer.meta.recolorable {
                if let Some(t) = tint {
                    img = shaded_apply(&img, t);
                }
            }
            blend_over(&mut base, &img);
        }

        IconCanvas::apply_app_icon_finish(&mut base);

        if size == CANVAS_SIZE {
            Ok(base)
        } else {
            Ok(coreimage::TiImage::from_rgba(base)
                .resize(size, size, coreimage::FilterType::Lanczos3)
                .into_rgba())
        }
    }

    /// Render at 1024px with default layer colors.
    pub fn render_default(&self) -> Result<RgbaImage, TicoError> {
        self.render(CANVAS_SIZE, None)
    }
}
