use crate::tint::TintMatrix;
use crate::{Color, Gradient, GradientDirection, SFSymbol};
use ab_glyph::{FontRef, PxScale, Font};
use image::{Rgba, RgbaImage};
use std::path::{Path, PathBuf};

/// Canvas size (1024x1024).
pub const CANVAS_SIZE: u32 = 1024;

/// Base directory for SF Symbol assets.
/// Set this to the path of the `assets/icons/` folder at runtime.
pub static mut ASSETS_DIR: &str = "assets/icons";

/// Icon color mode for `dark_light_mode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconMode {
    /// Make background dark (black), keep foreground as-is.
    Dark,
    /// Make background light (white), keep foreground as-is.
    Light,
}

/// Algorithm used when recoloring an existing image.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RecolorMode {
    /// HSL colorize: take hue + saturation from the tint color, keep each
    /// pixel's lightness. Best for full-color artwork. Pure white/black carry
    /// no hue information and are left alone (see `neutral_threshold`).
    Colorize,
    /// Apple accent tint: `out.rgb = tint.rgb * (0.2*luma + 0.8*value)`.
    /// Keeps shading, pulls every colored pixel toward the accent hue.
    /// Best for grayscale artwork.
    AccentLuma,
    /// Flat template replacement: RGB becomes the tint color, alpha is kept.
    Replace,
    /// Luminance-graded replacement: every pixel becomes the tint color
    /// scaled by `pixel_lightness / tint_lightness` (clamped to 1.0).
    /// Whites map to the full tint, blacks stay black, midtones shade
    /// proportionally - unlike `Colorize`, pure white IS recolored.
    Shaded,
}

/// Recolor settings for `change_color` / `process_image`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RecolorOptions {
    pub tint: Color,
    pub intensity: f32,
    pub mode: RecolorMode,
    /// Pixels with saturation below this keep their original color
    /// (protects grays / white / black). Only used by [`RecolorMode::Colorize`].
    pub neutral_threshold: f32,
    /// Pixels within [`Self::protect_tolerance`] of this color are never
    /// recolored - e.g. a background that was already swapped to a flat
    /// color in the same pipeline run.
    pub protect: Option<Color>,
    /// Euclidean RGB distance (0.0–1.0 space) for [`Self::protect`].
    pub protect_tolerance: f32,
    /// Pixels within [`Self::remap_tolerance`] of this color are replaced
    /// outright by [`Self::remap_to`] before any mode is applied - e.g. an
    /// interior cutout that should follow the swapped background color.
    pub remap_from: Option<Color>,
    /// Replacement target for [`Self::remap_from`].
    pub remap_to: Option<Color>,
    /// Euclidean RGB distance (0.0–1.0 space) for the remap rule.
    pub remap_tolerance: f32,
}

impl RecolorOptions {
    pub fn new(tint: Color, intensity: f32) -> Self {
        Self {
            tint,
            intensity: intensity.clamp(0.0, 1.0),
            mode: RecolorMode::Colorize,
            neutral_threshold: 0.05,
            protect: None,
            protect_tolerance: 0.12,
            remap_from: None,
            remap_to: None,
            remap_tolerance: 0.12,
        }
    }
    pub fn mode(mut self, mode: RecolorMode) -> Self { self.mode = mode; self }
    pub fn neutral_threshold(mut self, t: f32) -> Self { self.neutral_threshold = t.clamp(0.0, 1.0); self }
    /// Never recolor pixels within [`Self::protect_tolerance`] of `color`.
    pub fn protect(mut self, color: Color) -> Self { self.protect = Some(color); self }
    pub fn protect_tolerance(mut self, t: f32) -> Self { self.protect_tolerance = t.clamp(0.001, 1.0); self }
    /// Replace pixels within [`Self::remap_tolerance`] of `from` with `to`.
    pub fn remap(mut self, from: Color, to: Color) -> Self {
        self.remap_from = Some(from);
        self.remap_to = Some(to);
        self
    }
    pub fn remap_tolerance(mut self, t: f32) -> Self { self.remap_tolerance = t.clamp(0.001, 1.0); self }
}

/// Depth-effect settings shared by the file-processing entry points.
///
/// Mirrors the positional parameters of the legacy functions; defaults are
/// "effect off". Light direction points toward the light source and steers
/// specular + inner depth (default: top-left, like macOS dock glass).
#[derive(Debug, Clone)]
pub struct DepthOptions {
    pub corner_radius: f32,
    pub shadow: Option<Shadow>,
    pub inner_depth_blur: f32,
    pub inner_depth_opacity: f32,
    pub specular_opacity: f32,
    pub edge_highlight_width: f32,
    pub edge_highlight_opacity: f32,
    pub light_x: f32,
    pub light_y: f32,
}

impl Default for DepthOptions {
    fn default() -> Self {
        Self {
            corner_radius: 0.0,
            shadow: None,
            inner_depth_blur: 0.0,
            inner_depth_opacity: 0.25,
            specular_opacity: 0.0,
            edge_highlight_width: 0.0,
            edge_highlight_opacity: 0.2,
            light_x: -0.6,
            light_y: -0.8,
        }
    }
}

impl DepthOptions {
    pub fn new(corner_radius: f32) -> Self { Self { corner_radius, ..Self::default() } }
    pub fn shadow(mut self, s: Shadow) -> Self { self.shadow = Some(s); self }
    pub fn inner_depth(mut self, blur: f32, opacity: f32) -> Self {
        self.inner_depth_blur = blur.max(0.0);
        self.inner_depth_opacity = opacity.clamp(0.0, 1.0);
        self
    }
    pub fn specular(mut self, opacity: f32) -> Self { self.specular_opacity = opacity.clamp(0.0, 1.0); self }
    pub fn edge_highlight(mut self, width: f32, opacity: f32) -> Self {
        self.edge_highlight_width = width.max(0.0);
        self.edge_highlight_opacity = opacity.clamp(0.0, 1.0);
        self
    }
    pub fn light_direction(mut self, x: f32, y: f32) -> Self {
        let len = (x * x + y * y).sqrt();
        if len > 0.0001 { self.light_x = x / len; self.light_y = y / len; }
        self
    }

    // Legacy positional-argument bridge.
    #[allow(clippy::too_many_arguments)]
    fn from_legacy(
        corner_radius: f32,
        shadow_offset_x: Option<f32>, shadow_offset_y: Option<f32>,
        shadow_blur: Option<f32>, shadow_opacity: Option<f32>,
        inner_depth_blur: Option<f32>, inner_depth_opacity: Option<f32>,
        specular_opacity: Option<f32>,
        edge_highlight_width: Option<f32>, edge_highlight_opacity: Option<f32>,
    ) -> Self {
        let mut opts = Self::new(corner_radius);
        if shadow_offset_y.is_some() || shadow_offset_x.is_some() || shadow_blur.is_some() || shadow_opacity.is_some() {
            opts.shadow = Some(Shadow {
                offset_x: shadow_offset_x.unwrap_or(0.0),
                offset_y: shadow_offset_y.unwrap_or(8.0),
                blur: shadow_blur.unwrap_or(16.0),
                color: Color::BLACK,
                opacity: shadow_opacity.unwrap_or(0.3),
            });
        }
        if let Some(b) = inner_depth_blur { opts.inner_depth_blur = b.max(0.0); }
        if let Some(o) = inner_depth_opacity { opts.inner_depth_opacity = o.clamp(0.0, 1.0); }
        if let Some(o) = specular_opacity { opts.specular_opacity = o.clamp(0.0, 1.0); }
        if let Some(w) = edge_highlight_width { opts.edge_highlight_width = w.max(0.0); }
        if let Some(o) = edge_highlight_opacity { opts.edge_highlight_opacity = o.clamp(0.0, 1.0); }
        opts
    }
}

/// Everything `process_image` applies to one icon, in order:
/// background replacement -> recolor -> depth effects.
#[derive(Debug, Clone, Default)]
pub struct ProcessOptions {
    pub recolor: Option<RecolorOptions>,
    pub background_replace: Option<Color>,
    pub depth: DepthOptions,
    /// When true, the flood-filled background region is excluded from
    /// recoloring - lets `Colorize` with `neutral_threshold(0)` tint gray
    /// artwork while the original background stays untouched.
    pub protect_background: bool,
}

// ═══════════════════════════════════════════════════════════════
// High-level app icon APIs
// ═══════════════════════════════════════════════════════════════

/// Background appearance for [`AppIcon`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Appearance {
    /// Keep the original background.
    Light,
    /// Swap the background to the dark preset `(0.13, 0.13, 0.15)`.
    /// Artwork colors are preserved (a blue VS Code logo stays blue);
    /// pure-white interior cutouts follow the background color.
    Dark,
}

/// Dark background preset used by [`Appearance::Dark`].
pub const DARK_BACKGROUND: Color = Color::new(0.13, 0.13, 0.15, 1.0);

/// Default Liquid Glass finish for app icons.
pub fn default_app_icon_depth() -> DepthOptions {
    DepthOptions::new(220.0)
        .shadow(Shadow::new().offset(0.0, 10.0).blur(20.0).opacity(0.35))
        .inner_depth(12.0, 0.25)
        .specular(0.15)
        .edge_highlight(4.0, 0.2)
}

impl IconCanvas {
    /// API 1 - PNG to 3D app icon.
    ///
    /// Takes any flat image, scales it to 1024x1024, rounds it into the
    /// app-icon squircle and adds the Liquid Glass depth finish. Colors are
    /// left completely untouched.
    ///
    /// # Example
    /// ```no_run
    /// let icon = CoreIcon::generator::IconCanvas::png_to_3d_icon("logo.png")?;
    /// icon.save("app-icon.png")?;
    /// ```
    pub fn png_to_3d_icon(input_path: impl AsRef<Path>) -> Result<RgbaImage, Box<dyn std::error::Error>> {
        AppIcon::from_file(input_path).process()
    }
}

/// API 2 - turn any icon into an app icon with appearance / color options.
///
/// Default equals [`IconCanvas::png_to_3d_icon`] (same colors, rounded,
/// 1024x1024, glass depth). From there:
///
/// - `.dark()` puts the icon into dark mode: the background becomes dark
///   gray while all other colors survive (VS Code stays blue) and white
///   interior cutouts follow the background.
/// - `.tint(color)` recolors the artwork. With `Light` this uses HSL
///   colorize (the original background is untouched); with `Dark` it uses
///   luminance-graded `Shaded` replacement on top of the dark background.
///   Combine freely: `.dark().tint(red)`.
///
/// # Example
/// ```no_run
/// use CoreIcon::{Color, generator::AppIcon};
///
/// // Default (like API 1):
/// AppIcon::from_file("vscode.png").save("vscode-app.png")?;
///
/// // Dark mode, original colors kept:
/// AppIcon::from_file("vscode.png").dark().save("vscode-dark.png")?;
///
/// // Red artwork on the dark background:
/// let red = Color::from_hex("#FF3B30").unwrap();
/// AppIcon::from_file("vscode.png").dark().tint(red).save("vscode-red-dark.png")?;
/// ```
#[derive(Debug, Clone)]
pub struct AppIcon {
    source: AppIconSource,
    appearance: Appearance,
    tint: Option<Color>,
}

#[derive(Debug, Clone)]
enum AppIconSource {
    File(PathBuf),
    Image(RgbaImage),
}

impl AppIcon {
    /// Build from any image file (PNG, JPG, ...).
    pub fn from_file(path: impl AsRef<Path>) -> Self {
        Self {
            source: AppIconSource::File(path.as_ref().to_path_buf()),
            appearance: Appearance::Light,
            tint: None,
        }
    }

    /// Build from an in-memory image.
    pub fn from_image(image: &RgbaImage) -> Self {
        Self {
            source: AppIconSource::Image(image.clone()),
            appearance: Appearance::Light,
            tint: None,
        }
    }

    /// Set the background appearance explicitly.
    pub fn appearance(mut self, appearance: Appearance) -> Self { self.appearance = appearance; self }

    /// Dark mode: dark background, artwork colors preserved.
    pub fn dark(mut self) -> Self { self.appearance = Appearance::Dark; self }

    /// Light mode: original background is kept (default).
    pub fn light(mut self) -> Self { self.appearance = Appearance::Light; self }

    /// Recolor the artwork to `color` (luminance graded `Shaded` mode).
    pub fn tint(mut self, color: Color) -> Self { self.tint = Some(color); self }

    /// Drop a previously set tint (back to original colors).
    pub fn no_tint(mut self) -> Self { self.tint = None; self }

    /// Run the pipeline and return the finished 1024x1024 icon.
    pub fn process(&self) -> Result<RgbaImage, Box<dyn std::error::Error>> {
        let src = match &self.source {
            AppIconSource::File(path) => image::open(path)?.resize(
                CANVAS_SIZE, CANVAS_SIZE, image::imageops::FilterType::Lanczos3).to_rgba8(),
            AppIconSource::Image(img) => image::imageops::resize(
                img, CANVAS_SIZE, CANVAS_SIZE, image::imageops::FilterType::Lanczos3),
        };

        let mut options = ProcessOptions {
            recolor: None,
            background_replace: None,
            depth: default_app_icon_depth(),
            protect_background: false,
        };

        match self.appearance {
            Appearance::Light => {
                // Keep the original background; tint the whole artwork -
                // including gray artwork - while only the flood-filled
                // background region is protected.
                if let Some(tint) = self.tint {
                    options.protect_background = true;
                    options.recolor = Some(
                        RecolorOptions::new(tint, 1.0).mode(RecolorMode::Colorize).neutral_threshold(0.0),
                    );
                }
            }
            Appearance::Dark => {
                options.background_replace = Some(DARK_BACKGROUND);
                options.recolor = Some(match self.tint {
                    Some(tint) => RecolorOptions::new(tint, 1.0)
                        .mode(RecolorMode::Shaded)
                        .protect(DARK_BACKGROUND)
                        .remap(Color::WHITE, DARK_BACKGROUND),
                    None =>
                        // intensity 0 = pass-through recolor; only the
                        // white->background remap takes effect.
                        RecolorOptions::new(Color::WHITE, 0.0)
                            .remap(Color::WHITE, DARK_BACKGROUND),
                });
            }
        }

        Ok(IconCanvas::process_image(&src, &options))
    }

    /// Process and write a PNG to `path`.
    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
        let img = self.process()?;
        img.save(path.as_ref())?;
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════
// Shadow
// ═══════════════════════════════════════════════════════════════

/// Shadow configuration for a layer element.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shadow {
    pub offset_x: f32,
    pub offset_y: f32,
    pub blur: f32,
    pub color: Color,
    pub opacity: f32,
}

impl Shadow {
    pub fn new() -> Self {
        Self { offset_x: 0.0, offset_y: 8.0, blur: 16.0, color: Color::new(0.0, 0.0, 0.0, 0.3), opacity: 0.3 }
    }

    pub fn offset(mut self, x: f32, y: f32) -> Self { self.offset_x = x; self.offset_y = y; self }
    pub fn blur(mut self, b: f32) -> Self { self.blur = b; self }
    pub fn color(mut self, c: Color) -> Self { self.color = c; self }
    pub fn opacity(mut self, o: f32) -> Self { self.opacity = o.clamp(0.0, 1.0); self }
}

impl Default for Shadow {
    fn default() -> Self { Self::new() }
}

// ═══════════════════════════════════════════════════════════════
// Background
// ═══════════════════════════════════════════════════════════════

/// Background fill for the icon canvas.
#[derive(Debug, Clone)]
pub enum Background {
    /// Solid colour.
    Color(Color),
    /// Gradient fill.
    Gradient(Gradient),
    /// Image as background (path + optional tint).
    Image { path: String, tint: Option<Color> },
}

impl Background {
    pub fn color(c: Color) -> Self { Self::Color(c) }
    pub fn gradient(g: Gradient) -> Self { Self::Gradient(g) }
    pub fn image(path: impl Into<String>) -> Self { Self::Image { path: path.into(), tint: None } }
    pub fn image_tinted(path: impl Into<String>, tint: Color) -> Self { Self::Image { path: path.into(), tint: Some(tint) } }
}

// ═══════════════════════════════════════════════════════════════
// LayerContent — what to draw
// ═══════════════════════════════════════════════════════════════

/// The visual content of a layer.
#[derive(Debug, Clone)]
pub enum LayerContent {
    /// An SF Symbol icon.
    Icon(SFSymbol),
    /// A solid rectangle.
    Rect { width: f32, height: f32, corner_radius: f32 },
    /// A filled circle.
    Circle { diameter: f32 },
    /// A raster image from file.
    Image { path: String },
    /// Text with font size.
    Text { content: String, font_size: f32 },
}

impl LayerContent {
    pub fn icon(symbol: SFSymbol) -> Self { Self::Icon(symbol) }
    pub fn rect(w: f32, h: f32, r: f32) -> Self { Self::Rect { width: w, height: h, corner_radius: r } }
    pub fn circle(d: f32) -> Self { Self::Circle { diameter: d } }
    pub fn image(path: impl Into<String>) -> Self { Self::Image { path: path.into() } }
    pub fn text(content: impl Into<String>, font_size: f32) -> Self { Self::Text { content: content.into(), font_size } }
}

// ═══════════════════════════════════════════════════════════════
// Layer — one element on the canvas
// ═══════════════════════════════════════════════════════════════

/// A single layer element placed on the icon canvas.
#[derive(Debug, Clone)]
pub struct Layer {
    pub content: LayerContent,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub fill: Option<Color>,
    pub gradient: Option<Gradient>,
    /// Color-matrix recolor applied after fill/gradient. Composes with
    /// `fill`/`gradient` (matrix runs last). See [`TintMatrix`].
    pub tint_matrix: Option<TintMatrix>,
    pub opacity: f32,
    pub shadow: Option<Shadow>,
    pub inner_shadow: Option<Shadow>,
}

impl Layer {
    pub fn new(content: LayerContent) -> Self {
        Self {
            content,
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
            fill: None,
            gradient: None,
            tint_matrix: None,
            opacity: 1.0,
            shadow: None,
            inner_shadow: None,
        }
    }

    pub fn position(mut self, x: f32, y: f32) -> Self { self.x = x; self.y = y; self }
    pub fn size(mut self, w: f32, h: f32) -> Self { self.width = w; self.height = h; self }
    pub fn tint(mut self, c: Color) -> Self { self.fill = Some(c); self }
    pub fn gradient(mut self, g: Gradient) -> Self { self.gradient = Some(g); self }
    pub fn tint_matrix(mut self, m: TintMatrix) -> Self { self.tint_matrix = Some(m); self }
    pub fn opacity(mut self, o: f32) -> Self { self.opacity = o.clamp(0.0, 1.0); self }
    pub fn shadow(mut self, s: Shadow) -> Self { self.shadow = Some(s); self }
    pub fn inner_shadow(mut self, s: Shadow) -> Self { self.inner_shadow = Some(s); self }
}

// ═══════════════════════════════════════════════════════════════
// IconCanvas — main builder
// ═══════════════════════════════════════════════════════════════

/// Builder for generating 1024x1024 icon PNGs.
///
/// # Example
/// ```no_run
/// use CoreIcon::prelude::*;
/// use CoreIcon::generator::*;
///
/// let icon = IconCanvas::new()
///     .background(Background::color(Color::from_hex("#1d1d1d").unwrap()))
///     .layer(
///         Layer::new(LayerContent::icon(SFSymbol::HOUSE))
///             .position(312.0, 312.0)
///             .size(400.0, 400.0)
///             .tint(Color::WHITE)
///             .shadow(Shadow::new().offset(0.0, 10.0).blur(20.0))
///     )
///     .layer(
///         Layer::new(LayerContent::rect(200.0, 200.0, 40.0))
///             .position(412.0, 600.0)
///             .tint(Color::from_hex("#FF6B2B").unwrap())
///     );
///
/// icon.save("my-icon.png").unwrap();
/// ```
pub struct IconCanvas {
    background: Background,
    layers: Vec<Layer>,
    corner_radius: f32,
    padding: f32,
    edge_highlight_width: f32,
    edge_highlight_opacity: f32,
    frosted_opacity: f32,
    inner_depth_blur: f32,
    inner_depth_opacity: f32,
    specular_opacity: f32,
    light_x: f32,
    light_y: f32,
}

impl IconCanvas {
    pub fn new() -> Self {
        Self {
            background: Background::color(Color::new(0.11, 0.11, 0.118, 1.0)),
            layers: Vec::new(),
            corner_radius: 0.0,
            padding: 0.0,
            edge_highlight_width: 0.0,
            edge_highlight_opacity: 0.0,
            frosted_opacity: 0.0,
            inner_depth_blur: 0.0,
            inner_depth_opacity: 0.0,
            specular_opacity: 0.0,
            light_x: -0.6,
            light_y: -0.8,
        }
    }

    /// Set the background.
    pub fn background(mut self, bg: Background) -> Self { self.background = bg; self }

    /// Set padding (inset from canvas edges) for icon layers.
    /// When set, `LayerContent::icon` layers are automatically inset by this amount.
    pub fn padding(mut self, p: f32) -> Self { self.padding = p; self }

    /// Set corner radius for the entire canvas (rounded rect shape).
    /// Use 256.0 for 25% of 1024px (iOS-style icon).
    pub fn corner_radius(mut self, r: f32) -> Self { self.corner_radius = r; self }

    /// Add a subtle highlight along the rounded-rect edge of the canvas.
    /// `width` is the highlight thickness in pixels (e.g. 4.0),
    /// `opacity` controls its strength (0.0–1.0).
    /// Drawn as post-processing, so it never needs extra layers.
    pub fn edge_highlight(mut self, width: f32, opacity: f32) -> Self {
        self.edge_highlight_width = width.max(0.0);
        self.edge_highlight_opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Frosted-glass overlay: washes the whole canvas toward white so
    /// colours look like they glow through frosted glass (iOS 26 style).
    /// `opacity` 0.0–1.0 (e.g. 0.5 for a strong frost, 0.2 subtle).
    pub fn frosted(mut self, opacity: f32) -> Self {
        self.frosted_opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Inset depth along the rounded-rect edge (inner shadow that gives
    /// the tile a 3D, beveled-glass look). `blur` in px, `opacity` 0.0–1.0.
    pub fn inner_depth(mut self, blur: f32, opacity: f32) -> Self {
        self.inner_depth_blur = blur.max(0.0);
        self.inner_depth_opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Glossy specular highlight: a bright sheen that fades from the top
    /// edge downward, like light hitting the glass. `opacity` 0.0–1.0.
    pub fn specular(mut self, opacity: f32) -> Self {
        self.specular_opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Direction of the light source, used by `specular`, `inner_depth` and
    /// `edge_highlight`. The vector points toward the light; the default is
    /// top-left (`-0.6, -0.8`). Values are normalized internally.
    pub fn light_direction(mut self, x: f32, y: f32) -> Self {
        let len = (x * x + y * y).sqrt();
        if len > 0.0001 { self.light_x = x / len; self.light_y = y / len; }
        self
    }

    /// One-call Liquid Glass look: iOS-style corner radius plus tuned
    /// frosted / specular / inner-depth / edge-highlight values (defaults
    /// derived from the macOS Tahoe dock glass parameters).
    pub fn glass(mut self) -> Self {
        self.corner_radius = 256.0;
        self.frosted_opacity = 0.16;
        self.specular_opacity = 0.30;
        self.inner_depth_blur = 36.0;
        self.inner_depth_opacity = 0.32;
        self.edge_highlight_width = 6.0;
        self.edge_highlight_opacity = 0.40;
        self
    }

    /// Add a layer (drawn in order — last = on top).
    pub fn layer(mut self, layer: Layer) -> Self { self.layers.push(layer); self }

    /// Generate the icon and save as PNG.
    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
        let img = self.render();
        img.save(path.as_ref())?;
        Ok(())
    }

    // ═══════════════════════════════════════════════════════════════
    // Image processing core — recolor / background swap / depth effects
    // ═══════════════════════════════════════════════════════════════

    /// Load an image file and run [`ProcessOptions`] on it.
    ///
    /// The source is scaled to exactly 1024x1024 before processing.
    /// Returns the processed `RgbaImage`.
    pub fn process_file(
        input_path: impl AsRef<Path>,
        options: &ProcessOptions,
    ) -> Result<RgbaImage, Box<dyn std::error::Error>> {
        let src = image::open(input_path)?;
        let src = src.resize(CANVAS_SIZE, CANVAS_SIZE, image::imageops::FilterType::Lanczos3);
        Ok(Self::process_image(&src.to_rgba8(), options))
    }

    /// Apply recoloring, background replacement and depth effects to an
    /// in-memory image.
    ///
    /// Pipeline order:
    /// 1. Background replacement (flood fill from the edges)
    /// 2. Recolor ([`RecolorOptions`])
    /// 3. Shadow -> content -> specular -> inner depth -> edge highlight
    /// 4. Corner radius mask
    pub fn process_image(src: &RgbaImage, options: &ProcessOptions) -> RgbaImage {
        let mut work = src.clone();

        if let Some(target) = options.background_replace {
            work = Self::replace_background(&work, target.r, target.g, target.b);
        }
        if let Some(recolor) = &options.recolor {
            let mask = if options.protect_background {
                // Strict thresholds: this mask exists to *skip* pixels, so a
                // false positive would silently drop artwork from the tint.
                Some(Self::background_mask_with(&work, 0.16, 0.14))
            } else {
                None
            };
            work = Self::recolor_pixels(&work, recolor, mask.as_deref());
        }

        let mut canvas = RgbaImage::from_pixel(CANVAS_SIZE, CANVAS_SIZE, Rgba([0, 0, 0, 0]));
        let d = &options.depth;
        let ox = ((CANVAS_SIZE - work.width().min(CANVAS_SIZE)) / 2) as i64;
        let oy = ((CANVAS_SIZE - work.height().min(CANVAS_SIZE)) / 2) as i64;

        // 1. Shadow FIRST (behind the content), glyph-shaped via distance field.
        if let Some(shadow) = &d.shadow {
            let color = Color::new(shadow.color.r, shadow.color.g, shadow.color.b, shadow.opacity);
            Self::paint_distance_shadow(
                &mut canvas, &work,
                ox + shadow.offset_x.round() as i64,
                oy + shadow.offset_y.round() as i64,
                shadow.blur, color,
            );
        }

        // 2. Content on top of the shadow.
        for (px, py, pixel) in work.enumerate_pixels() {
            let x = ox + px as i64;
            let y = oy + py as i64;
            if x >= 0 && y >= 0 && (x as u32) < CANVAS_SIZE && (y as u32) < CANVAS_SIZE {
                canvas.put_pixel(x as u32, y as u32, *pixel);
            }
        }

        // 3./4. Glass effects + corner mask.
        Self::apply_depth_effects(&mut canvas, d);

        canvas
    }

    /// Recolor an image without any depth effects or canvas compositing.
    pub fn recolor_image(src: &RgbaImage, options: &RecolorOptions) -> RgbaImage {
        Self::recolor_pixels(src, options, None)
    }

    /// Replace the flood-filled background region with an arbitrary color.
    /// Generalization of `dark_light_mode` (which maps `IconMode` presets).
    pub fn set_background_color(
        input_path: impl AsRef<Path>,
        target: Color,
        depth: DepthOptions,
    ) -> Result<RgbaImage, Box<dyn std::error::Error>> {
        Self::process_file(
            input_path,
            &ProcessOptions { recolor: None, background_replace: Some(target), depth, ..Default::default() },
        )
    }

    // ── Processing internals ──────────────────────────────────

    /// Flood-fill the border-connected background and repaint it `target`.
    fn replace_background(rgba: &RgbaImage, tr: f32, tg: f32, tb: f32) -> RgbaImage {
        let mask = Self::background_mask(rgba);
        let mut out = RgbaImage::from_pixel(rgba.width(), rgba.height(), Rgba([0, 0, 0, 0]));
        for (px, py, pixel) in rgba.enumerate_pixels() {
            let a = pixel[3];
            if a < 3 { continue; }
            if mask[py as usize * rgba.width() as usize + px as usize] {
                out.put_pixel(px, py, Rgba([
                    (tr * 255.0).round() as u8,
                    (tg * 255.0).round() as u8,
                    (tb * 255.0).round() as u8,
                    a,
                ]));
            } else {
                out.put_pixel(px, py, *pixel);
            }
        }
        out
    }

    /// Compute the border-connected background mask (true = background),
    /// with the standard (swap-oriented) thresholds.
    fn background_mask(rgba: &RgbaImage) -> Vec<bool> {
        Self::background_mask_with(rgba, 0.32, 0.25)
    }

    /// Compute the border-connected background mask (true = background).
    ///
    /// Robust against soft logo edges: a pixel joins the background only when
    /// it is close to the border reference color AND close to its neighbour -
    /// a pure neighbour chain would leak through anti-aliased gradients into
    /// the artwork. One dilation pass pulls low-saturation halo pixels into
    /// the mask so the swapped background has no bright fringe.
    fn background_mask_with(rgba: &RgbaImage, ref_threshold: f32, chain_threshold: f32) -> Vec<bool> {
        let w = rgba.width() as usize;
        let h = rgba.height() as usize;
        let mut is_bg = vec![false; w * h];
        let mut queue: std::collections::VecDeque<(usize, usize)> = std::collections::VecDeque::new();

        // Reference colour: average of the outermost 2px border.
        let mut rr = 0u64;
        let mut rg = 0u64;
        let mut rb = 0u64;
        let mut n = 0u64;
        let mut acc = |p: &Rgba<u8>| {
            rr += p[0] as u64;
            rg += p[1] as u64;
            rb += p[2] as u64;
            n += 1;
        };
        for x in 0..w {
            acc(rgba.get_pixel(x as u32, 0));
            acc(rgba.get_pixel(x as u32, h as u32 - 1));
        }
        for y in 0..h {
            acc(rgba.get_pixel(0, y as u32));
            acc(rgba.get_pixel(w as u32 - 1, y as u32));
        }
        let (ref_r, ref_g, ref_b) = (
            rr as f32 / 255.0 / n as f32,
            rg as f32 / 255.0 / n as f32,
            rb as f32 / 255.0 / n as f32,
        );

        let dist = |a: (f32, f32, f32), b: (f32, f32, f32)| -> f32 {
            ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2) + (a.2 - b.2).powi(2)).sqrt()
        };
        let color_at = |x: usize, y: usize| -> (f32, f32, f32) {
            let p = rgba.get_pixel(x as u32, y as u32);
            (p[0] as f32 / 255.0, p[1] as f32 / 255.0, p[2] as f32 / 255.0)
        };

        for x in 0..w {
            queue.push_back((x, 0));
            queue.push_back((x, h - 1));
        }
        for y in 1..h - 1 {
            queue.push_back((0, y));
            queue.push_back((w - 1, y));
        }

        while let Some((x, y)) = queue.pop_front() {
            let idx = y * w + x;
            if is_bg[idx] { continue; }
            if dist(color_at(x, y), (ref_r, ref_g, ref_b)) >= ref_threshold { continue; }
            is_bg[idx] = true;

            for (dx, dy) in [(-1i64, 0i64), (1, 0), (0, -1), (0, 1)] {
                let nx = x as i64 + dx;
                let ny = y as i64 + dy;
                if nx < 0 || ny < 0 || nx >= w as i64 || ny >= h as i64 { continue; }
                let (nx, ny) = (nx as usize, ny as usize);
                let nidx = ny * w + nx;
                if is_bg[nidx] { continue; }
                if dist(color_at(nx, ny), color_at(x, y)) < chain_threshold {
                    queue.push_back((nx, ny));
                }
            }
        }

        // Dilation: absorb anti-aliased fringe pixels (near-reference, low
        // chroma) that touch the background on at least two sides.
        let mut halo = vec![false; w * h];
        for y in 1..h - 1 {
            for x in 1..w - 1 {
                let idx = y * w + x;
                if is_bg[idx] { continue; }
                let c = color_at(x, y);
                if dist(c, (ref_r, ref_g, ref_b)) >= 0.60 { continue; }
                let chroma = c.0.max(c.1).max(c.2) - c.0.min(c.1).min(c.2);
                if chroma > 0.18 { continue; }
                let mut bg_neighbors = 0;
                for (dx, dy) in [(-1i64, 0i64), (1, 0), (0, -1), (0, 1)] {
                    let ni = (y as i64 + dy) as usize * w + (x as i64 + dx) as usize;
                    if is_bg[ni] { bg_neighbors += 1; }
                }
                if bg_neighbors >= 2 { halo[idx] = true; }
            }
        }

        for i in 0..is_bg.len() { is_bg[i] = is_bg[i] || halo[i]; }
        is_bg
    }

    /// Improved recolor pass.
    ///
    /// Works on straight-alpha values with proper rounding; anti-aliased edge
    /// pixels no longer produce dark fringes because transparent neighbors do
    /// not bleed into the conversion.
    fn recolor_pixels(rgba: &RgbaImage, o: &RecolorOptions, bg_mask: Option<&[bool]>) -> RgbaImage {
        let intensity = o.intensity.clamp(0.0, 1.0);
        let mut out = RgbaImage::from_pixel(rgba.width(), rgba.height(), Rgba([0, 0, 0, 0]));

        let (tgt_h, tgt_s, tgt_l) = Self::rgb_to_hsl(o.tint.r, o.tint.g, o.tint.b);
        let w = rgba.width() as usize;

        for (px, py, pixel) in rgba.enumerate_pixels() {
            if let Some(mask) = bg_mask {
                if mask[py as usize * w + px as usize] {
                    out.put_pixel(px, py, *pixel);
                    continue;
                }
            }
            let a = pixel[3] as f32 / 255.0;
            if a < 0.01 {
                out.put_pixel(px, py, *pixel);
                continue;
            }
            let r = pixel[0] as f32 / 255.0;
            let g = pixel[1] as f32 / 255.0;
            let b = pixel[2] as f32 / 255.0;

            if let (Some(from), Some(to)) = (o.remap_from, o.remap_to) {
                let d = ((r - from.r).powi(2) + (g - from.g).powi(2) + (b - from.b).powi(2)).sqrt();
                if d < o.remap_tolerance {
                    out.put_pixel(px, py, Rgba([
                        (to.r * 255.0).round() as u8,
                        (to.g * 255.0).round() as u8,
                        (to.b * 255.0).round() as u8,
                        pixel[3],
                    ]));
                    continue;
                }
            }

            if let Some(protect) = o.protect {
                let d = ((r - protect.r).powi(2) + (g - protect.g).powi(2) + (b - protect.b).powi(2)).sqrt();
                if d < o.protect_tolerance {
                    out.put_pixel(px, py, *pixel);
                    continue;
                }
            }

            let (nr, ng, nb) = match o.mode {
                RecolorMode::Replace => (o.tint.r, o.tint.g, o.tint.b),
                RecolorMode::AccentLuma => {
                    let luma = Self::luma(r, g, b);
                    (
                        o.tint.r * (0.2 * luma + 0.8 * r),
                        o.tint.g * (0.2 * luma + 0.8 * g),
                        o.tint.b * (0.2 * luma + 0.8 * b),
                    )
                }
                RecolorMode::Shaded => {
                    let (_, _, l) = Self::rgb_to_hsl(r, g, b);
                    let k = if tgt_l <= 0.001 { l } else { (l / tgt_l).min(1.0) };
                    (o.tint.r * k, o.tint.g * k, o.tint.b * k)
                }
                RecolorMode::Colorize => {
                    let (_, s, l) = Self::rgb_to_hsl(r, g, b);
                    // threshold > 0 protects near-neutral pixels; with
                    // threshold == 0 even pure grays are colorized.
                    let keep_neutral = o.neutral_threshold > 0.0 && s <= o.neutral_threshold;
                    if !keep_neutral {
                        Self::hsl_to_rgb(tgt_h, tgt_s.max(s), l)
                    } else {
                        (r, g, b)
                    }
                }
            };

            let fr = r + (nr - r) * intensity;
            let fg = g + (ng - g) * intensity;
            let fb = b + (nb - b) * intensity;

            out.put_pixel(px, py, Rgba([
                (fr * 255.0).round().clamp(0.0, 255.0) as u8,
                (fg * 255.0).round().clamp(0.0, 255.0) as u8,
                (fb * 255.0).round().clamp(0.0, 255.0) as u8,
                pixel[3],
            ]));
        }
        out
    }

    /// Glyph-shaped drop shadow: chamfer distance transform over the content
    /// silhouette (already placed at its final position), then a smoothstep
    /// falloff. O(n) regardless of blur radius — the previous stamp-blur was
    /// O(n * blur^2).
    fn paint_distance_shadow(
        canvas: &mut RgbaImage,
        content: &RgbaImage,
        cx: i64, cy: i64,
        blur: f32,
        color: Color,
    ) {
        let blur = blur.max(0.5);
        let w = canvas.width() as usize;
        let h = canvas.height() as usize;
        const INF: f32 = 1.0e9;
        let mut dist = vec![INF; w * h];

        for (px, py, pixel) in content.enumerate_pixels() {
            if pixel[3] < 10 { continue; }
            let x = cx + px as i64;
            let y = cy + py as i64;
            if x >= 0 && y >= 0 && (x as u32) < canvas.width() && (y as u32) < canvas.height() {
                dist[y as usize * w + x as usize] = 0.0;
            }
        }

        let (d1, d2) = (1.0f32, std::f32::consts::SQRT_2);
        // Forward pass (top-left origin).
        for y in 0..h {
            for x in 0..w {
                let i = y * w + x;
                let mut v = dist[i];
                if x > 0 { v = v.min(dist[i - 1] + d1); }
                if y > 0 {
                    v = v.min(dist[i - w] + d1);
                    if x > 0 { v = v.min(dist[i - w - 1] + d2); }
                    if x + 1 < w { v = v.min(dist[i - w + 1] + d2); }
                }
                dist[i] = v;
            }
        }
        // Backward pass (bottom-right origin).
        for y in (0..h).rev() {
            for x in (0..w).rev() {
                let i = y * w + x;
                let mut v = dist[i];
                if x + 1 < w { v = v.min(dist[i + 1] + d1); }
                if y + 1 < h {
                    v = v.min(dist[i + w] + d1);
                    if x + 1 < w { v = v.min(dist[i + w + 1] + d2); }
                    if x > 0 { v = v.min(dist[i + w - 1] + d2); }
                }
                dist[i] = v;
            }
        }

        for y in 0..h {
            for x in 0..w {
                let dd = dist[y * w + x];
                if dd >= blur { continue; }
                let t = 1.0 - dd / blur;
                let a = color.a * t * t * (3.0 - 2.0 * t);
                if a <= 0.004 { continue; }
                Self::blend_pixel(canvas, x as u32, y as u32, Rgba([
                    (color.r * 255.0).round() as u8,
                    (color.g * 255.0).round() as u8,
                    (color.b * 255.0).round() as u8,
                    (a * 255.0).round() as u8,
                ]));
            }
        }
    }

    /// Specular rim + inner depth + edge highlight + corner mask, all steered
    /// by one light direction. The specular band uses the rounded-rect surface
    /// normal (rim lighting), so the sheen wraps around corners like real
    /// glass instead of being a flat top gradient.
    fn apply_depth_effects(img: &mut RgbaImage, d: &DepthOptions) {
        if d.corner_radius <= 0.0 { return; }
        let size = CANVAS_SIZE as f32;
        let r = d.corner_radius.min(size / 2.0);
        let band = size * 0.035;

        if d.specular_opacity > 0.0 {
            for py in 0..CANVAS_SIZE {
                for px in 0..CANVAS_SIZE {
                    let (sdf, n) =
                        Self::rounded_rect_sdf_normal(px as f32 + 0.5, py as f32 + 0.5, size, size, r);
                    if sdf >= 0.0 || -sdf > band { continue; }
                    let rim = (n[0] * d.light_x + n[1] * d.light_y).max(0.0);
                    if rim <= 0.001 { continue; }
                    let edge_t = 1.0 + sdf / band;
                    let a = edge_t * edge_t * rim * d.specular_opacity;
                    if a < 0.01 { continue; }
                    Self::blend_pixel(img, px, py, Rgba([255, 255, 255, (a * 255.0).round() as u8]));
                }
            }
        }

        if d.inner_depth_blur > 0.0 && d.inner_depth_opacity > 0.0 {
            let blur = d.inner_depth_blur;
            for py in 0..CANVAS_SIZE {
                for px in 0..CANVAS_SIZE {
                    let (sdf, _) =
                        Self::rounded_rect_sdf_normal(px as f32 + 0.5, py as f32 + 0.5, size, size, r);
                    if sdf >= 0.0 || -sdf > blur { continue; }
                    // Darkness grows away from the light (planar gradient).
                    let darkness = (0.5
                        - ((px as f32 + 0.5) / size - 0.5) * d.light_x
                        - ((py as f32 + 0.5) / size - 0.5) * d.light_y)
                        .clamp(0.0, 1.0);
                    let t = (-sdf / blur).clamp(0.0, 1.0);
                    let a = (1.0 - t) * darkness * d.inner_depth_opacity;
                    if a < 0.01 { continue; }
                    Self::blend_pixel(img, px, py, Rgba([0, 0, 0, (a * 255.0).round() as u8]));
                }
            }
        }

        if d.edge_highlight_width > 0.0 {
            let ew = d.edge_highlight_width;
            for py in 0..CANVAS_SIZE {
                for px in 0..CANVAS_SIZE {
                    let (sdf, n) =
                        Self::rounded_rect_sdf_normal(px as f32 + 0.5, py as f32 + 0.5, size, size, r);
                    if sdf >= 0.0 || -sdf > ew { continue; }
                    let rim = (n[0] * d.light_x + n[1] * d.light_y).max(0.15);
                    let a = (-sdf / ew) * rim * d.edge_highlight_opacity;
                    if a < 0.01 { continue; }
                    Self::blend_pixel(img, px, py, Rgba([255, 255, 255, (a * 255.0).round() as u8]));
                }
            }
        }

        // Corner radius mask last.
        for py in 0..CANVAS_SIZE {
            for px in 0..CANVAS_SIZE {
                let (sdf, _) =
                    Self::rounded_rect_sdf_normal(px as f32 + 0.5, py as f32 + 0.5, size, size, r);
                if sdf >= 0.0 {
                    img.put_pixel(px, py, Rgba([0, 0, 0, 0]));
                }
            }
        }
    }

    /// Signed distance plus outward surface normal of a centered rounded rect
    /// (negative distance = inside).
    fn rounded_rect_sdf_normal(px: f32, py: f32, w: f32, h: f32, r: f32) -> (f32, [f32; 2]) {
        let half_w = w / 2.0;
        let half_h = h / 2.0;
        let r = r.min(half_w).min(half_h);
        let cx = half_w + (px - half_w).clamp(-half_w + r, half_w - r);
        let cy = half_h + (py - half_h).clamp(-half_h + r, half_h - r);
        let dx = px - cx;
        let dy = py - cy;
        let len = (dx * dx + dy * dy).sqrt();
        let sdf = len - r;
        let n = if len > 0.0001 {
            [dx / len, dy / len]
        } else {
            [0.0, -1.0]
        };
        (sdf, n)
    }

    fn luma(r: f32, g: f32, b: f32) -> f32 {
        crate::tint::LUMA_R * r + crate::tint::LUMA_G * g + crate::tint::LUMA_B * b
    }

    /// Load an existing image and apply depth effects (shadow, inner depth,
    /// specular highlight, edge highlight, corner radius).
    ///
    /// Returns the processed `RgbaImage`.
    ///
    /// # Example
    /// ```no_run
    /// use CoreIcon::generator::IconCanvas;
    ///
    /// let result = IconCanvas::add_depth_to_image(
    ///     "my-icon.png",
    ///     220.0, // corner_radius
    ///     Some(0.0),   // shadow_offset_x
    ///     Some(10.0),  // shadow_offset_y
    ///     Some(20.0),  // shadow_blur
    ///     Some(0.3),   // shadow_opacity
    ///     Some(12.0),  // inner_depth_blur
    ///     Some(0.25),  // inner_depth_opacity
    ///     Some(0.15),  // specular_opacity
    ///     Some(4.0),   // edge_highlight_width
    ///     Some(0.2),   // edge_highlight_opacity
    /// );
    /// result.unwrap().save("output.png").unwrap();
    /// ```
    pub fn add_depth_to_image(
        input_path: impl AsRef<Path>,
        corner_radius: f32,
        shadow_offset_x: Option<f32>,
        shadow_offset_y: Option<f32>,
        shadow_blur: Option<f32>,
        shadow_opacity: Option<f32>,
        inner_depth_blur: Option<f32>,
        inner_depth_opacity: Option<f32>,
        specular_opacity: Option<f32>,
        edge_highlight_width: Option<f32>,
        edge_highlight_opacity: Option<f32>,
    ) -> Result<RgbaImage, Box<dyn std::error::Error>> {
        Self::process_file(input_path, &ProcessOptions {
            recolor: None,
            background_replace: None,
            depth: DepthOptions::from_legacy(
                corner_radius,
                shadow_offset_x, shadow_offset_y, shadow_blur, shadow_opacity,
                inner_depth_blur, inner_depth_opacity,
                specular_opacity,
                edge_highlight_width, edge_highlight_opacity,
            ),
            ..Default::default()
        })
    }

    /// Convenience: load image, apply depth, save to output path.
    pub fn add_depth_to_image_and_save(
        input_path: impl AsRef<Path>,
        output_path: impl AsRef<Path>,
        corner_radius: f32,
        shadow_offset_x: Option<f32>,
        shadow_offset_y: Option<f32>,
        shadow_blur: Option<f32>,
        shadow_opacity: Option<f32>,
        inner_depth_blur: Option<f32>,
        inner_depth_opacity: Option<f32>,
        specular_opacity: Option<f32>,
        edge_highlight_width: Option<f32>,
        edge_highlight_opacity: Option<f32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let img = Self::add_depth_to_image(
            input_path, corner_radius,
            shadow_offset_x, shadow_offset_y, shadow_blur, shadow_opacity,
            inner_depth_blur, inner_depth_opacity,
            specular_opacity,
            edge_highlight_width, edge_highlight_opacity,
        )?;
        img.save(output_path.as_ref())?;
        Ok(())
    }

    // ═══════════════════════════════════════════════════════════════
    // change_color — tint an existing icon to a target color
    // ═══════════════════════════════════════════════════════════════

    /// Tint an existing icon to a target color with configurable intensity,
    /// then apply depth effects. The depth effects (shadow, specular,
    /// inner depth, edge highlight) keep their original colors.
    ///
    /// - `intensity`: `0.0` = original colors, `1.0` = fully tinted
    /// - `tint_color`: the target color to blend toward
    ///
    /// # Example
    /// ```no_run
    /// use CoreIcon::generator::IconCanvas;
    /// use CoreIcon::Color;
    ///
    /// let result = IconCanvas::change_color(
    ///     "my-icon.png",
    ///     Color::from_hex("#FF6B2B").unwrap(), // tint to orange
    ///     0.7,                                  // 70% intensity
    ///     220.0,                                // corner_radius
    ///     Some(0.0),   Some(8.0),  Some(12.0), Some(0.3),
    ///     Some(10.0),  Some(0.25),
    ///     Some(0.15),
    ///     Some(4.0),   Some(0.2),
    /// );
    /// result.unwrap().save("orange-icon.png").unwrap();
    /// ```
    pub fn change_color(
        input_path: impl AsRef<Path>,
        tint_color: Color,
        intensity: f32,
        corner_radius: f32,
        shadow_offset_x: Option<f32>,
        shadow_offset_y: Option<f32>,
        shadow_blur: Option<f32>,
        shadow_opacity: Option<f32>,
        inner_depth_blur: Option<f32>,
        inner_depth_opacity: Option<f32>,
        specular_opacity: Option<f32>,
        edge_highlight_width: Option<f32>,
        edge_highlight_opacity: Option<f32>,
    ) -> Result<RgbaImage, Box<dyn std::error::Error>> {
        Self::process_file(input_path, &ProcessOptions {
            recolor: Some(RecolorOptions::new(tint_color, intensity)),
            background_replace: None,
            depth: DepthOptions::from_legacy(
                corner_radius,
                shadow_offset_x, shadow_offset_y, shadow_blur, shadow_opacity,
                inner_depth_blur, inner_depth_opacity,
                specular_opacity,
                edge_highlight_width, edge_highlight_opacity,
            ),
            ..Default::default()
        })
    }

    /// Convenience: tint an icon and save to output path.
    pub fn change_color_and_save(
        input_path: impl AsRef<Path>,
        output_path: impl AsRef<Path>,
        tint_color: Color,
        intensity: f32,
        corner_radius: f32,
        shadow_offset_x: Option<f32>,
        shadow_offset_y: Option<f32>,
        shadow_blur: Option<f32>,
        shadow_opacity: Option<f32>,
        inner_depth_blur: Option<f32>,
        inner_depth_opacity: Option<f32>,
        specular_opacity: Option<f32>,
        edge_highlight_width: Option<f32>,
        edge_highlight_opacity: Option<f32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let img = Self::change_color(
            input_path, tint_color, intensity, corner_radius,
            shadow_offset_x, shadow_offset_y, shadow_blur, shadow_opacity,
            inner_depth_blur, inner_depth_opacity,
            specular_opacity,
            edge_highlight_width, edge_highlight_opacity,
        )?;
        img.save(output_path.as_ref())?;
        Ok(())
    }

    // ═══════════════════════════════════════════════════════════════
    // dark_light_mode — switch icon background between dark/light
    // ═══════════════════════════════════════════════════════════════

    /// Switch an icon between dark and light mode by detecting the
    /// background color and replacing it.
    ///
    /// - `Dark`: background becomes black, foreground unchanged
    /// - `Light`: background becomes white, foreground unchanged
    ///
    /// Depth effects are applied on top (untinted).
    pub fn dark_light_mode(
        input_path: impl AsRef<Path>,
        mode: IconMode,
        corner_radius: f32,
        shadow_offset_x: Option<f32>,
        shadow_offset_y: Option<f32>,
        shadow_blur: Option<f32>,
        shadow_opacity: Option<f32>,
        inner_depth_blur: Option<f32>,
        inner_depth_opacity: Option<f32>,
        specular_opacity: Option<f32>,
        edge_highlight_width: Option<f32>,
        edge_highlight_opacity: Option<f32>,
    ) -> Result<RgbaImage, Box<dyn std::error::Error>> {
        let (r, g, b) = match mode {
            IconMode::Dark => (0.13, 0.13, 0.15),
            IconMode::Light => (1.0, 1.0, 1.0),
        };
        Self::set_background_color(
            input_path,
            Color::new(r, g, b, 1.0),
            DepthOptions::from_legacy(
                corner_radius,
                shadow_offset_x, shadow_offset_y, shadow_blur, shadow_opacity,
                inner_depth_blur, inner_depth_opacity,
                specular_opacity,
                edge_highlight_width, edge_highlight_opacity,
            ),
        )
    }

    /// Convenience: switch icon mode and save.
    pub fn dark_light_mode_and_save(
        input_path: impl AsRef<Path>,
        output_path: impl AsRef<Path>,
        mode: IconMode,
        corner_radius: f32,
        shadow_offset_x: Option<f32>,
        shadow_offset_y: Option<f32>,
        shadow_blur: Option<f32>,
        shadow_opacity: Option<f32>,
        inner_depth_blur: Option<f32>,
        inner_depth_opacity: Option<f32>,
        specular_opacity: Option<f32>,
        edge_highlight_width: Option<f32>,
        edge_highlight_opacity: Option<f32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let img = Self::dark_light_mode(
            input_path, mode, corner_radius,
            shadow_offset_x, shadow_offset_y, shadow_blur, shadow_opacity,
            inner_depth_blur, inner_depth_opacity,
            specular_opacity,
            edge_highlight_width, edge_highlight_opacity,
        )?;
        img.save(output_path.as_ref())?;
        Ok(())
    }

    /// Render the icon and return the raw image buffer.
    pub fn render(&self) -> RgbaImage {
        let mut img = RgbaImage::from_pixel(CANVAS_SIZE, CANVAS_SIZE, Rgba([0, 0, 0, 0]));

        // 1. Draw background
        self.draw_background(&mut img);

        // 2. Draw layers in order
        for layer in &self.layers {
            self.draw_layer(&mut img, layer);
        }

        // 3./4. Post-processing: frosted wash, then light-steered glass
        // effects (specular rim, inner depth, edge highlight) and corner mask.
        if self.frosted_opacity > 0.0 {
            self.draw_frosted(&mut img);
        }
        Self::apply_depth_effects(&mut img, &self.depth_options());

        img
    }

    /// Collect the canvas-level effect state into [`DepthOptions`] so the
    /// builder and the file-processing pipeline share one implementation.
    fn depth_options(&self) -> DepthOptions {
        DepthOptions {
            corner_radius: self.corner_radius,
            shadow: None,
            inner_depth_blur: self.inner_depth_blur,
            inner_depth_opacity: if self.inner_depth_blur > 0.0 { self.inner_depth_opacity } else { 0.0 },
            specular_opacity: self.specular_opacity,
            edge_highlight_width: self.edge_highlight_width,
            edge_highlight_opacity: self.edge_highlight_opacity,
            light_x: self.light_x,
            light_y: self.light_y,
        }
    }

    // ── Background rendering ───────────────────────────────

    fn draw_background(&self, img: &mut RgbaImage) {
        match &self.background {
            Background::Color(color) => {
                let rgba = Self::color_to_rgba(*color);
                for pixel in img.pixels_mut() {
                    *pixel = rgba;
                }
            }
            Background::Gradient(gradient) => {
                Self::draw_gradient_rect(img, 0.0, 0.0, CANVAS_SIZE as f32, CANVAS_SIZE as f32, gradient, 1.0);
            }
            Background::Image { path, tint } => {
                if let Ok(bg_img) = image::open(path) {
                    let bg = bg_img.resize_to_fill(CANVAS_SIZE, CANVAS_SIZE, image::imageops::FilterType::Lanczos3);
                    for (x, y, pixel) in bg.to_rgba8().enumerate_pixels() {
                        if x < CANVAS_SIZE && y < CANVAS_SIZE {
                            let mut p = *pixel;
                            if let Some(t) = tint {
                                p = Self::tint_pixel(p, *t);
                            }
                            img.put_pixel(x, y, p);
                        }
                    }
                }
            }
        }
    }

    // ── Layer rendering ────────────────────────────────────

    fn draw_layer(&self, img: &mut RgbaImage, layer: &Layer) {
        // Draw shadow first (behind the element)
        if let Some(shadow) = &layer.shadow {
            self.draw_shadow(img, layer, shadow);
        }

        // Draw the element content
        match &layer.content {
            LayerContent::Icon(symbol) => {
                self.draw_icon(img, layer, symbol);
            }
            LayerContent::Rect { width, height, corner_radius } => {
                self.draw_rounded_rect(img, layer.x, layer.y, *width, *height, *corner_radius, layer);
            }
            LayerContent::Circle { diameter } => {
                self.draw_circle(img, layer.x, layer.y, *diameter, layer);
            }
            LayerContent::Image { path } => {
                self.draw_image_element(img, layer, path);
            }
            LayerContent::Text { content, font_size } => {
                self.draw_text(img, layer, content, *font_size);
            }
        }
    }

    // ── Shadow ─────────────────────────────────────────────

    fn draw_shadow(&self, img: &mut RgbaImage, layer: &Layer, shadow: &Shadow) {
        let shadow_color = Color::new(shadow.color.r, shadow.color.g, shadow.color.b, shadow.opacity);
        // Simple shadow: draw a blurred dark shape offset by shadow offset
        // For simplicity, we draw a semi-transparent version offset
        let sx = layer.x + shadow.offset_x;
        let sy = layer.y + shadow.offset_y;

        match &layer.content {
            LayerContent::Icon(symbol) => {
                // Glyph-shaped shadow: distance field of the actual symbol
                // silhouette instead of a blurred rectangle.
                if let Some((sprite, sx, sy)) = self.icon_sprite(layer, symbol) {
                    Self::paint_distance_shadow(
                        img,
                        &sprite,
                        sx as i64 + shadow.offset_x.round() as i64,
                        sy as i64 + shadow.offset_y.round() as i64,
                        shadow.blur,
                        shadow_color,
                    );
                }
            }
            LayerContent::Rect { width, height, .. } => {
                self.draw_blurred_rect(img, sx, sy, *width, *height, shadow.blur, shadow_color);
            }
            LayerContent::Circle { diameter } => {
                self.draw_blurred_circle(img, sx, sy, *diameter, shadow.blur, shadow_color);
            }
            LayerContent::Image { .. } => {
                self.draw_blurred_rect(img, sx, sy, layer.width, layer.height, shadow.blur, shadow_color);
            }
            LayerContent::Text { .. } => {
                self.draw_blurred_rect(img, sx, sy, layer.width, layer.height, shadow.blur, shadow_color);
            }
        }
    }

    fn draw_blurred_rect(&self, img: &mut RgbaImage, x: f32, y: f32, w: f32, h: f32, blur: f32, color: Color) {
        let blur_px = blur as i32;
        for py in (y as i32 - blur_px)..((y + h) as i32 + blur_px) {
            for px in (x as i32 - blur_px)..((x + w) as i32 + blur_px) {
                if px < 0 || py < 0 || px >= CANVAS_SIZE as i32 || py >= CANVAS_SIZE as i32 { continue; }
                // Calculate distance from rect edge
                let dx = (px as f32 - x).max(0.0).min(w) + x - px as f32 - w / 2.0;
                let dy = (py as f32 - y).max(0.0).min(h) + y - py as f32 - h / 2.0;
                let dist = (dx * dx + dy * dy).sqrt();
                let alpha = (1.0 - (dist / blur).min(1.0)).max(0.0) * color.a;
                if alpha > 0.01 {
                    let rgba = Self::color_to_rgba(Color::new(color.r, color.g, color.b, alpha));
                    Self::blend_pixel(img, px as u32, py as u32, rgba);
                }
            }
        }
    }

    fn draw_blurred_circle(&self, img: &mut RgbaImage, cx: f32, cy: f32, diameter: f32, blur: f32, color: Color) {
        let r = diameter / 2.0;
        let blur_px = blur as i32;
        for py in (cy as i32 - blur_px)..((cy + diameter) as i32 + blur_px) {
            for px in (cx as i32 - blur_px)..((cx + diameter) as i32 + blur_px) {
                if px < 0 || py < 0 || px >= CANVAS_SIZE as i32 || py >= CANVAS_SIZE as i32 { continue; }
                let dx = px as f32 - (cx + r);
                let dy = py as f32 - (cy + r);
                let dist = (dx * dx + dy * dy).sqrt() - r;
                let alpha = (1.0 - (dist / blur).min(1.0)).max(0.0) * color.a;
                if alpha > 0.01 {
                    let rgba = Self::color_to_rgba(Color::new(color.r, color.g, color.b, alpha));
                    Self::blend_pixel(img, px as u32, py as u32, rgba);
                }
            }
        }
    }

    // ── Shape drawing ──────────────────────────────────────

    fn draw_rounded_rect(&self, img: &mut RgbaImage, x: f32, y: f32, w: f32, h: f32, r: f32, layer: &Layer) {
        let (uw, uh) = (w.max(1.0) as u32, h.max(1.0) as u32);
        let overlay = if let Some(s) = &layer.inner_shadow {
            let mut cov = RgbaImage::from_pixel(uw, uh, Rgba([0, 0, 0, 0]));
            for py in 0..uh {
                for px in 0..uw {
                    if Self::is_in_rounded_rect(px as f32, py as f32, 0.0, 0.0, w, h, r) {
                        cov.put_pixel(px, py, Rgba([255, 255, 255, 255]));
                    }
                }
            }
            Some(Self::inner_shadow_overlay(&cov, s))
        } else {
            None
        };
        for py in y as u32..((y + h) as u32).min(CANVAS_SIZE) {
            for px in x as u32..((x + w) as u32).min(CANVAS_SIZE) {
                if Self::is_in_rounded_rect(px as f32, py as f32, x, y, w, h, r) {
                    let pixel = self.resolve_fill_pixel(layer, px as f32, py as f32, x, y, w, h);
                    Self::blend_pixel(img, px, py, pixel);
                }
            }
        }
        if let Some(ov) = &overlay {
            Self::blend_overlay(img, ov, x as u32, y as u32);
        }
    }

    fn draw_circle(&self, img: &mut RgbaImage, cx: f32, cy: f32, diameter: f32, layer: &Layer) {
        let r = diameter / 2.0;
        let overlay = if let Some(s) = &layer.inner_shadow {
            let d = diameter.max(1.0) as u32;
            let mut cov = RgbaImage::from_pixel(d, d, Rgba([0, 0, 0, 0]));
            for py in 0..d {
                for px in 0..d {
                    let dx = px as f32 + 0.5 - r;
                    let dy = py as f32 + 0.5 - r;
                    if dx * dx + dy * dy <= r * r {
                        cov.put_pixel(px, py, Rgba([255, 255, 255, 255]));
                    }
                }
            }
            Some(Self::inner_shadow_overlay(&cov, s))
        } else {
            None
        };
        for py in cy as u32..((cy + diameter) as u32).min(CANVAS_SIZE) {
            for px in cx as u32..((cx + diameter) as u32).min(CANVAS_SIZE) {
                let dx = px as f32 - (cx + r);
                let dy = py as f32 - (cy + r);
                if dx * dx + dy * dy <= r * r {
                    let pixel = self.resolve_fill_pixel(layer, px as f32, py as f32, cx, cy, diameter, diameter);
                    Self::blend_pixel(img, px, py, pixel);
                }
            }
        }
        if let Some(ov) = &overlay {
            Self::blend_overlay(img, ov, cx as u32, cy as u32);
        }
    }

    /// Load an SF Symbol PNG and fit it inside the padded layer box,
    /// preserving its aspect ratio (contain fit, centered). Returns the
    /// positioned sprite ready for compositing.
    fn icon_sprite(&self, layer: &Layer, symbol: &SFSymbol) -> Option<(RgbaImage, u32, u32)> {
        let full = unsafe { PathBuf::from(ASSETS_DIR).join(format!("{}.png", symbol.name())) };
        let icon_img = image::open(&full).ok()?;
        let p = self.padding;
        let box_w = (layer.width - p * 2.0).max(1.0);
        let box_h = (layer.height - p * 2.0).max(1.0);
        let iw = icon_img.width() as f32;
        let ih = icon_img.height() as f32;
        let scale = (box_w / iw).min(box_h / ih);
        let resized = icon_img.resize(
            ((iw * scale).round() as u32).max(1),
            ((ih * scale).round() as u32).max(1),
            image::imageops::FilterType::Lanczos3,
        );
        let x = (layer.x + p + (box_w - resized.width() as f32) / 2.0).round().max(0.0) as u32;
        let y = (layer.y + p + (box_h - resized.height() as f32) / 2.0).round().max(0.0) as u32;
        Some((resized.to_rgba8(), x, y))
    }

    fn draw_icon(&self, img: &mut RgbaImage, layer: &Layer, symbol: &SFSymbol) {
        let Some((rgba, ox, oy)) = self.icon_sprite(layer, symbol) else { return; };
        let h = rgba.height();
        let overlay = if let Some(s) = &layer.inner_shadow {
            Some(Self::inner_shadow_overlay(&rgba, s))
        } else {
            None
        };
        for (px, py, pixel) in rgba.enumerate_pixels() {
            let dx = ox + px;
            let dy = oy + py;
            if dx < CANVAS_SIZE && dy < CANVAS_SIZE {
                let mut p = *pixel;
                if let Some(tint) = &layer.fill {
                    p = Self::tint_pixel(p, *tint);
                }
                if let Some(gradient) = &layer.gradient {
                    let t = py as f32 / h as f32;
                    let c = Self::sample_gradient(gradient, t.clamp(0.0, 1.0));
                    p = Self::tint_pixel(p, c);
                }
                if let Some(m) = &layer.tint_matrix {
                    p = Self::apply_matrix_to_pixel(m, p);
                }
                p[3] = (p[3] as f32 * layer.opacity).round().clamp(0.0, 255.0) as u8;
                Self::blend_pixel(img, dx, dy, p);
            }
        }
        // Draw inner shadow on top of the icon fill.
        if let Some(ov) = &overlay {
            Self::blend_overlay(img, ov, ox, oy);
        }
    }

    fn draw_image_element(&self, img: &mut RgbaImage, layer: &Layer, path: &str) {
        if let Ok(element_img) = image::open(path) {
            let resized = element_img.resize_to_fill(layer.width as u32, layer.height as u32, image::imageops::FilterType::Lanczos3);
            for (px, py, pixel) in resized.to_rgba8().enumerate_pixels() {
                let dx = layer.x as u32 + px;
                let dy = layer.y as u32 + py;
                if dx < CANVAS_SIZE && dy < CANVAS_SIZE {
                    let mut p = *pixel;
                    if let Some(tint) = &layer.fill {
                        p = Self::tint_pixel(p, *tint);
                    }
                    if let Some(m) = &layer.tint_matrix {
                        p = Self::apply_matrix_to_pixel(m, p);
                    }
                    p[3] = (p[3] as f32 * layer.opacity).round().clamp(0.0, 255.0) as u8;
                    Self::blend_pixel(img, dx, dy, p);
                }
            }
        }
    }

    // ── Text ───────────────────────────────────────────────

    fn draw_text(&self, img: &mut RgbaImage, layer: &Layer, content: &str, font_size: f32) {
        // Try to load SF Pro from system paths, fallback to embedded default
        let font_data = Self::load_system_font();
        let font = FontRef::try_from_slice(&font_data).ok();
        if let Some(font) = font {
            let scale = PxScale::from(font_size);
            let mut cursor_x = layer.x;
            let baseline_y = layer.y + font_size * 0.8;

            for ch in content.chars() {
                let glyph_id = font.glyph_id(ch);
                let glyph = glyph_id.with_scale_and_position(scale, ab_glyph::point(cursor_x, baseline_y));
                if let Some(outlined) = font.outline_glyph(glyph) {
                    let bounds = outlined.px_bounds();
                    // Draw each pixel of the glyph
                    outlined.draw(|gx, gy, coverage| {
                        let px = bounds.min.x + gx as f32;
                        let py = bounds.min.y + gy as f32;
                        if px >= 0.0 && py >= 0.0 && px < CANVAS_SIZE as f32 && py < CANVAS_SIZE as f32 {
                            let alpha = coverage;
                            let mut pixel_color = if let Some(gradient) = &layer.gradient {
                                let t = (py - layer.y) / layer.height;
                                Self::sample_gradient(gradient, t.clamp(0.0, 1.0))
                            } else if let Some(color) = &layer.fill {
                                *color
                            } else {
                                Color::WHITE
                            };
                            if let Some(m) = &layer.tint_matrix {
                                let (r, g, b, _) = m.apply(pixel_color.r, pixel_color.g, pixel_color.b, 1.0);
                                pixel_color = Color::new(r, g, b, pixel_color.a);
                            }
                            let rgba = Rgba([
                                (pixel_color.r * 255.0).round() as u8,
                                (pixel_color.g * 255.0).round() as u8,
                                (pixel_color.b * 255.0).round() as u8,
                                (alpha * layer.opacity * 255.0) as u8,
                            ]);
                            Self::blend_pixel(img, px as u32, py as u32, rgba);
                        }
                    });
                }
                // Advance cursor
                let upm = font.units_per_em().unwrap_or(1000.0);
                let advance = font.h_advance_unscaled(glyph_id) * (font_size / upm);
                cursor_x += advance;
            }
        }
    }

    fn load_system_font() -> Vec<u8> {
        // Try common system font paths
        let paths = if cfg!(target_os = "windows") {
            vec![
                "C:/Windows/Fonts/segoeui.ttf",
                "C:/Windows/Fonts/arial.ttf",
            ]
        } else if cfg!(target_os = "macos") {
            vec![
                "/System/Library/Fonts/SFPro.ttf",
                "/System/Library/Fonts/SFNS.ttf",
                "/Library/Fonts/Arial.ttf",
            ]
        } else {
            // Linux
            vec![
                "/usr/share/fonts/OTF/SF-Pro-Display-Regular.otf",
                "/usr/share/fonts/TTF/SF-Pro-Display-Regular.otf",
                "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
                "/usr/share/fonts/TTF/dejavu/DejaVuSans.ttf",
            ]
        };
        for path in paths {
            if let Ok(data) = std::fs::read(path) {
                return data;
            }
        }
        // Return empty if no font found (text won't render)
        Vec::new()
    }

    // ── Gradient ───────────────────────────────────────────

    fn draw_gradient_rect(img: &mut RgbaImage, x: f32, y: f32, w: f32, h: f32, gradient: &Gradient, opacity: f32) {
        for py in y as u32..((y + h) as u32).min(CANVAS_SIZE) {
            for px in x as u32..((x + w) as u32).min(CANVAS_SIZE) {
                let t = match gradient.direction {
                    GradientDirection::TopToBottom => (py as f32 - y) / h,
                    GradientDirection::BottomToTop => 1.0 - (py as f32 - y) / h,
                    GradientDirection::LeftToRight => (px as f32 - x) / w,
                    GradientDirection::RightToLeft => 1.0 - (px as f32 - x) / w,
                    _ => (py as f32 - y) / h,
                };
                let color = Self::sample_gradient(gradient, t.clamp(0.0, 1.0));
                let rgba = Self::color_to_rgba(Color::new(color.r, color.g, color.b, color.a * opacity));
                Self::blend_pixel(img, px, py, rgba);
            }
        }
    }

    fn sample_gradient(gradient: &Gradient, t: f32) -> Color {
        if gradient.stops.is_empty() { return Color::WHITE; }
        if gradient.stops.len() == 1 { return gradient.stops[0].color; }

        let mut lower = &gradient.stops[0];
        let mut upper = &gradient.stops[gradient.stops.len() - 1];

        for i in 0..gradient.stops.len() - 1 {
            if t >= gradient.stops[i].position && t <= gradient.stops[i + 1].position {
                lower = &gradient.stops[i];
                upper = &gradient.stops[i + 1];
                break;
            }
        }

        let range = (upper.position - lower.position).max(0.001);
        let local_t = (t - lower.position) / range;
        Color::new(
            lower.color.r + (upper.color.r - lower.color.r) * local_t,
            lower.color.g + (upper.color.g - lower.color.g) * local_t,
            lower.color.b + (upper.color.b - lower.color.b) * local_t,
            lower.color.a + (upper.color.a - lower.color.a) * local_t,
        )
    }

    // ── Helpers ────────────────────────────────────────────

    fn resolve_fill_pixel(&self, layer: &Layer, _px: f32, py: f32, _x: f32, y: f32, _w: f32, h: f32) -> Rgba<u8> {
        let mut pixel = if let Some(gradient) = &layer.gradient {
            let t = (py - y) / h;
            let c = Self::sample_gradient(gradient, t.clamp(0.0, 1.0));
            Self::color_to_rgba(Color::new(c.r, c.g, c.b, c.a * layer.opacity))
        } else if let Some(color) = &layer.fill {
            Self::color_to_rgba(Color::new(color.r, color.g, color.b, color.a * layer.opacity))
        } else {
            Rgba([255, 255, 255, (255.0 * layer.opacity).round() as u8])
        };
        if let Some(m) = &layer.tint_matrix {
            pixel = Self::apply_matrix_to_pixel(m, pixel);
        }
        pixel
    }

    fn color_to_rgba(c: Color) -> Rgba<u8> {
        Rgba([
            (c.r * 255.0) as u8,
            (c.g * 255.0) as u8,
            (c.b * 255.0) as u8,
            (c.a * 255.0) as u8,
        ])
    }

    /// Run a [`TintMatrix`] over one straight-alpha pixel.
    fn apply_matrix_to_pixel(m: &TintMatrix, p: Rgba<u8>) -> Rgba<u8> {
        if p[3] == 0 { return p; }
        let (r, g, b, a) = m.apply(
            p[0] as f32 / 255.0,
            p[1] as f32 / 255.0,
            p[2] as f32 / 255.0,
            p[3] as f32 / 255.0,
        );
        Rgba([
            (r * 255.0).round() as u8,
            (g * 255.0).round() as u8,
            (b * 255.0).round() as u8,
            (a * 255.0).round() as u8,
        ])
    }

    fn tint_pixel(pixel: Rgba<u8>, tint: Color) -> Rgba<u8> {
        // Use alpha channel as mask, replace RGB with tint color
        let alpha = pixel[3] as f32 / 255.0;
        if alpha < 0.01 {
            return Rgba([0, 0, 0, 0]);
        }
        Rgba([
            (tint.r * 255.0).round() as u8,
            (tint.g * 255.0).round() as u8,
            (tint.b * 255.0).round() as u8,
            (alpha * tint.a * 255.0).round() as u8,
        ])
    }

    /// Compute an inner-shadow overlay for a shape, using its alpha
    /// channel as coverage mask. Edge pixels of the shape get darkened,
    /// fading inward over `shadow.blur` pixels.
    fn inner_shadow_overlay(shape: &RgbaImage, shadow: &Shadow) -> RgbaImage {
        let (w, h) = (shape.width() as usize, shape.height() as usize);
        let total = w * h;
        let blur = shadow.blur.max(1.0);
        let big = blur + 4.0;
        let mut dist = vec![big; total];

        // Initialise: transparent = 0 (outside), opaque = large (inside).
        for (x, y, pixel) in shape.enumerate_pixels() {
            if pixel[3] > 0 {
                dist[y as usize * w + x as usize] = big;
            } else {
                dist[y as usize * w + x as usize] = 0.0;
            }
        }

        // Forward pass (top-left to bottom-right).
        for y in 0..h {
            for x in 0..w {
                let idx = y * w + x;
                if x > 0 { dist[idx] = dist[idx].min(dist[idx - 1] + 1.0); }
                if y > 0 { dist[idx] = dist[idx].min(dist[idx - w] + 1.0); }
            }
        }
        // Backward pass (bottom-right to top-left).
        for y in (0..h).rev() {
            for x in (0..w).rev() {
                let idx = y * w + x;
                if x + 1 < w { dist[idx] = dist[idx].min(dist[idx + 1] + 1.0); }
                if y + 1 < h { dist[idx] = dist[idx].min(dist[idx + w] + 1.0); }
            }
        }

        // Build overlay: closer to edge -> stronger shadow.
        let mut overlay = RgbaImage::new(shape.width(), shape.height());
        for y in 0..h {
            for x in 0..w {
                let idx = y * w + x;
                let inside = shape.get_pixel(x as u32, y as u32)[3] > 0;
                if !inside { continue; }
                let d = dist[idx];
                let falloff = (1.0 - (d / blur)).clamp(0.0, 1.0);
                let a = falloff * shadow.opacity;
                if a > 0.01 {
                    overlay.put_pixel(x as u32, y as u32, Rgba([
                        (shadow.color.r * 255.0) as u8,
                        (shadow.color.g * 255.0) as u8,
                        (shadow.color.b * 255.0) as u8,
                        (a * 255.0) as u8,
                    ]));
                }
            }
        }
        overlay
    }

    /// Blend an overlay image onto the canvas at the given offset.
    fn blend_overlay(img: &mut RgbaImage, overlay: &RgbaImage, ox: u32, oy: u32) {
        for (px, py, pixel) in overlay.enumerate_pixels() {
            let dx = ox + px;
            let dy = oy + py;
            if dx < CANVAS_SIZE && dy < CANVAS_SIZE {
                Self::blend_pixel(img, dx, dy, *pixel);
            }
        }
    }

    fn blend_pixel(img: &mut RgbaImage, x: u32, y: u32, new: Rgba<u8>) {
        if x >= img.width() || y >= img.height() { return; }
        let old = img.get_pixel(x, y);
        let src_a = new[3] as f32 / 255.0;
        let dst_a = old[3] as f32 / 255.0;
        let out_a = src_a + dst_a * (1.0 - src_a);
        if out_a < 0.001 { return; }
        let r = ((new[0] as f32 * src_a + old[0] as f32 * dst_a * (1.0 - src_a)) / out_a) as u8;
        let g = ((new[1] as f32 * src_a + old[1] as f32 * dst_a * (1.0 - src_a)) / out_a) as u8;
        let b = ((new[2] as f32 * src_a + old[2] as f32 * dst_a * (1.0 - src_a)) / out_a) as u8;
        img.put_pixel(x, y, Rgba([r, g, b, (out_a * 255.0) as u8]));
    }

    fn is_in_rounded_rect(px: f32, py: f32, x: f32, y: f32, w: f32, h: f32, r: f32) -> bool {
        if px < x || px > x + w || py < y || py > y + h { return false; }
        let r = r.min(w / 2.0).min(h / 2.0);
        // Check corners
        let corners = [
            (x + r, y + r),
            (x + w - r, y + r),
            (x + w - r, y + h - r),
            (x + r, y + h - r),
        ];
        let in_corners = [
            px < x + r && py < y + r,
            px > x + w - r && py < y + r,
            px > x + w - r && py > y + h - r,
            px < x + r && py > y + h - r,
        ];
        for (i, &(cx, cy)) in corners.iter().enumerate() {
            if in_corners[i] {
                let dx = px - cx;
                let dy = py - cy;
                if dx * dx + dy * dy > r * r { return false; }
            }
        }
        true
    }

    /// Frosted-glass overlay — blends white over every opaque pixel so
    /// colours look lighter, as if viewed through frosted glass.
    fn draw_frosted(&self, img: &mut RgbaImage) {
        let a = self.frosted_opacity;
        if a <= 0.0 { return; }
        let white_a = (a * 255.0) as u8;
        for pixel in img.pixels_mut() {
            if pixel[3] > 0 {
                Self::blend_pixel_in_place(pixel, Rgba([255, 255, 255, white_a]));
            }
        }
    }

    /// Blend src onto dst in-place (src-over compositing).
    fn blend_pixel_in_place(dst: &mut Rgba<u8>, src: Rgba<u8>) {
        let sa = src[3] as f32 / 255.0;
        let da = dst[3] as f32 / 255.0;
        let out_a = sa + da * (1.0 - sa);
        if out_a < 0.001 { return; }
        dst[0] = ((src[0] as f32 * sa + dst[0] as f32 * da * (1.0 - sa)) / out_a) as u8;
        dst[1] = ((src[1] as f32 * sa + dst[1] as f32 * da * (1.0 - sa)) / out_a) as u8;
        dst[2] = ((src[2] as f32 * sa + dst[2] as f32 * da * (1.0 - sa)) / out_a) as u8;
        dst[3] = (out_a * 255.0) as u8;
    }

    // ── HSL conversion (for change_color) ────────────────────────

    fn rgb_to_hsl(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let l = (max + min) / 2.0;

        if max - min < 0.0001 {
            return (0.0, 0.0, l); // achromatic
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

    fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
        if s < 0.0001 {
            return (l, l, l);
        }

        let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
        let p = 2.0 * l - q;
        let h = h.fract();

        fn hue_to_rgb(p: f32, q: f32, t: f32) -> f32 {
            let t = if t < 0.0 { t + 1.0 } else if t > 1.0 { t - 1.0 } else { t };
            if t < 1.0 / 6.0 { p + (q - p) * 6.0 * t }
            else if t < 1.0 / 2.0 { q }
            else if t < 2.0 / 3.0 { p + (q - p) * (2.0 / 3.0 - t) * 6.0 }
            else { p }
        }

        let r = hue_to_rgb(p, q, h + 1.0 / 3.0);
        let g = hue_to_rgb(p, q, h);
        let b = hue_to_rgb(p, q, h - 1.0 / 3.0);
        (r, g, b)
    }
}

impl Default for IconCanvas {
    fn default() -> Self { Self::new() }
}
