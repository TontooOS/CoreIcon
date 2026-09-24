// TICO - TontooOS icon container.
//
// A `.tico` file is a plain ZIP archive that is only ever named `*.tico`
// (never `*.tico.zip`). It contains no preview image and no `.png` files:
//
//   manifest.json      metadata, background, layer table
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

use crate::generator::{Background, IconCanvas, LayerContent, CANVAS_SIZE};
use crate::{Color, GradientDirection};
use image::{Rgba, RgbaImage};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
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
    Zip(String),
    Json(String),
    Image(String),
    Format(String),
}

impl std::fmt::Display for TicoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "tico io: {}", e),
            Self::Zip(e) => write!(f, "tico zip: {}", e),
            Self::Json(e) => write!(f, "tico manifest: {}", e),
            Self::Image(e) => write!(f, "tico image: {}", e),
            Self::Format(e) => write!(f, "tico format: {}", e),
        }
    }
}

impl std::error::Error for TicoError {}

impl From<std::io::Error> for TicoError {
    fn from(e: std::io::Error) -> Self { Self::Io(e) }
}
impl From<serde_json::Error> for TicoError {
    fn from(e: serde_json::Error) -> Self { Self::Json(e.to_string()) }
}
impl From<image::ImageError> for TicoError {
    fn from(e: image::ImageError) -> Self { Self::Image(e.to_string()) }
}
impl From<zip::result::ZipError> for TicoError {
    fn from(e: zip::result::ZipError) -> Self { Self::Zip(e.to_string()) }
}

// ── Manifest ────────────────────────────────────────────────────────

/// Background stored in `manifest.json` (no raster needed for flat icons).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TicoBackground {
    Color { color: String },
    Gradient { colors: Vec<String>, positions: Vec<f32>, direction: GradientDirection },
    Raster { file: String },
}

/// One entry of the manifest layer table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicoLayerMeta {
    pub file: String,
    pub opacity: f32,
    pub recolorable: bool,
    pub default_color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Manifest {
    format: String,
    version: u32,
    name: String,
    canvas: u32,
    background: TicoBackground,
    layers: Vec<TicoLayerMeta>,
}

// ── TLYR layer files ────────────────────────────────────────────────

fn encode_tlyr(img: &RgbaImage) -> Result<Vec<u8>, TicoError> {
    let mut png = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut png);
    image::DynamicImage::ImageRgba8(img.clone())
        .write_to(&mut cursor, image::ImageFormat::Png)?;
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
    let img = image::load_from_memory(&bytes[18..18 + len])?.to_rgba8();
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

/// Write an [`IconCanvas`] as `*.tico` (ZIP container, no PNG files inside).
///
/// Every layer is rasterized at full 1024px into one `layer/NN.tlyr` file.
/// Flat builder artwork compresses to a few KB per layer (1-100KB total).
/// Photo (`Image`) backgrounds are embedded as their own raster layer.
pub struct Tico;

impl Tico {
    pub fn export(canvas: &IconCanvas, name: &str, path: impl AsRef<Path>) -> Result<(), TicoError> {
        let mut files: Vec<(String, Vec<u8>)> = Vec::new();

        let bg = match canvas.tico_background() {
            Background::Color(c) => TicoBackground::Color { color: color_to_hex(c) },
            Background::Gradient(g) => TicoBackground::Gradient {
                colors: g.stops.iter().map(|s| color_to_hex(s.color)).collect(),
                positions: g.stops.iter().map(|s| s.position).collect(),
                direction: g.direction,
            },
            Background::Image { path, .. } => {
                let img = image::open(&path)
                    .map_err(|e| TicoError::Format(format!("background image: {}", e)))?
                    .resize_to_fill(CANVAS_SIZE, CANVAS_SIZE, image::imageops::FilterType::Lanczos3)
                    .to_rgba8();
                files.push(("layer/background.tlyr".into(), encode_tlyr(&img)?));
                TicoBackground::Raster { file: "layer/background.tlyr".into() }
            }
        };

        let mut layers_meta = Vec::new();
        for i in 0..canvas.tico_layer_count() {
            let layer = canvas
                .tico_layer(i)
                .ok_or_else(|| TicoError::Format(format!("missing layer {}", i)))?;
            let raster = canvas
                .tico_rasterize_layer(i)
                .ok_or_else(|| TicoError::Format(format!("cannot rasterize layer {}", i)))?;
            let file = format!("layer/{:02}.tlyr", layers_meta.len());
            files.push((file.clone(), encode_tlyr(&raster)?));
            layers_meta.push(TicoLayerMeta {
                file,
                opacity: layer.opacity,
                recolorable: !matches!(layer.content, LayerContent::Image { .. }),
                default_color: color_to_hex(layer.fill.unwrap_or(Color::WHITE)),
            });
        }

        let manifest = Manifest {
            format: "tico".into(),
            version: TICO_VERSION,
            name: name.into(),
            canvas: CANVAS_SIZE,
            background: bg,
            layers: layers_meta,
        };
        let manifest_json = serde_json::to_string_pretty(&manifest)?;

        let out = std::fs::File::create(path.as_ref())?;
        let mut zip = zip::ZipWriter::new(out);
        let opts = zip::write::SimpleFileOptions::default();
        zip.start_file("manifest.json", opts)?;
        zip.write_all(manifest_json.as_bytes())?;
        for (name, bytes) in &files {
            zip.start_file(name, opts)?;
            zip.write_all(bytes)?;
        }
        zip.finish()?;
        Ok(())
    }

    /// Load a `*.tico` file for rendering.
    pub fn load(path: impl AsRef<Path>) -> Result<TicoIcon, TicoError> {
        let file = std::fs::File::open(path.as_ref())?;
        let mut zip = zip::ZipArchive::new(file)?;
        let mut raw = String::new();
        zip.by_name("manifest.json")?.read_to_string(&mut raw)?;
        let manifest: Manifest = serde_json::from_str(&raw)?;
        if manifest.format != "tico" {
            return Err(TicoError::Format("not a tico file".into()));
        }
        if manifest.version != TICO_VERSION {
            return Err(TicoError::Format(format!("unsupported tico v{}", manifest.version)));
        }

        let mut read_layer = |file: &str| -> Result<RgbaImage, TicoError> {
            let mut buf = Vec::new();
            zip.by_name(file)?.read_to_end(&mut buf)?;
            decode_tlyr(&buf)
        };

        // Decode raster background (if any) and all layers.
        let bg_image = match &manifest.background {
            TicoBackground::Raster { file } => Some(read_layer(file)?),
            _ => None,
        };
        let mut layers = Vec::new();
        for meta in &manifest.layers {
            let image = read_layer(&meta.file)?;
            layers.push(LoadedLayer { meta: meta.clone(), image });
        }

        Ok(TicoIcon {
            name: manifest.name,
            background: manifest.background,
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
            Ok(image::imageops::resize(&base, size, size, image::imageops::FilterType::Lanczos3))
        }
    }

    /// Render at 1024px with default layer colors.
    pub fn render_default(&self) -> Result<RgbaImage, TicoError> {
        self.render(CANVAS_SIZE, None)
    }
}
