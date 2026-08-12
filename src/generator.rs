use crate::{Color, Gradient, GradientDirection, SFSymbol};
use ab_glyph::{FontRef, PxScale, Font};
use image::{Rgba, RgbaImage};
use std::path::{Path, PathBuf};

/// Canvas size (1024x1024).
pub const CANVAS_SIZE: u32 = 1024;

/// Base directory for SF Symbol assets.
/// Set this to the path of the `assets/icons/` folder at runtime.
pub static mut ASSETS_DIR: &str = "assets/icons";

// ═══════════════════════════════════════════════════════════════
// Shadow
// ═══════════════════════════════════════════════════════════════

/// Shadow configuration for a layer element.
#[derive(Debug, Clone)]
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
            opacity: 1.0,
            shadow: None,
            inner_shadow: None,
        }
    }

    pub fn position(mut self, x: f32, y: f32) -> Self { self.x = x; self.y = y; self }
    pub fn size(mut self, w: f32, h: f32) -> Self { self.width = w; self.height = h; self }
    pub fn tint(mut self, c: Color) -> Self { self.fill = Some(c); self }
    pub fn gradient(mut self, g: Gradient) -> Self { self.gradient = Some(g); self }
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
    edge_highlight_width: f32,
    edge_highlight_opacity: f32,
    frosted_opacity: f32,
    inner_depth_blur: f32,
    inner_depth_opacity: f32,
    specular_opacity: f32,
}

impl IconCanvas {
    pub fn new() -> Self {
        Self {
            background: Background::color(Color::new(0.11, 0.11, 0.118, 1.0)),
            layers: Vec::new(),
            corner_radius: 0.0,
            edge_highlight_width: 0.0,
            edge_highlight_opacity: 0.0,
            frosted_opacity: 0.0,
            inner_depth_blur: 0.0,
            inner_depth_opacity: 0.0,
            specular_opacity: 0.0,
        }
    }

    /// Set the background.
    pub fn background(mut self, bg: Background) -> Self { self.background = bg; self }

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

    /// Add a layer (drawn in order — last = on top).
    pub fn layer(mut self, layer: Layer) -> Self { self.layers.push(layer); self }

    /// Generate the icon and save as PNG.
    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
        let img = self.render();
        img.save(path.as_ref())?;
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

        // 3. Post-processing (before corner mask)
        if self.frosted_opacity > 0.0 {
            self.draw_frosted(&mut img);
        }
        if self.specular_opacity > 0.0 && self.corner_radius > 0.0 {
            self.draw_specular(&mut img);
        }
        if self.inner_depth_blur > 0.0 && self.inner_depth_opacity > 0.0 && self.corner_radius > 0.0 {
            self.draw_inner_depth(&mut img);
        }
        if self.edge_highlight_width > 0.0 && self.corner_radius > 0.0 {
            self.draw_edge_highlight(&mut img);
        }

        // 4. Apply corner radius mask
        if self.corner_radius > 0.0 {
            self.apply_corner_radius(&mut img);
        }

        img
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
            LayerContent::Icon(_) => {
                self.draw_blurred_rect(img, sx, sy, layer.width, layer.height, shadow.blur, shadow_color);
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

    fn draw_icon(&self, img: &mut RgbaImage, layer: &Layer, symbol: &SFSymbol) {
        let name = symbol.name();
        let full = unsafe { PathBuf::from(ASSETS_DIR).join(format!("{}.png", name)) };
        if let Ok(icon_img) = image::open(&full) {
            let resized = icon_img.resize_to_fill(layer.width as u32, layer.height as u32, image::imageops::FilterType::Lanczos3);
            let rgba = resized.to_rgba8();
            let h = rgba.height();
            let overlay = if let Some(s) = &layer.inner_shadow {
                Some(Self::inner_shadow_overlay(&rgba, s))
            } else {
                None
            };
            for (px, py, pixel) in rgba.enumerate_pixels() {
                let dx = layer.x as u32 + px;
                let dy = layer.y as u32 + py;
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
                    p[3] = (p[3] as f32 * layer.opacity) as u8;
                    Self::blend_pixel(img, dx, dy, p);
                }
            }
            // Draw inner shadow on top of the icon fill.
            if let Some(ov) = &overlay {
                Self::blend_overlay(img, ov, layer.x as u32, layer.y as u32);
            }
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
                    p[3] = (p[3] as f32 * layer.opacity) as u8;
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
                            let pixel_color = if let Some(gradient) = &layer.gradient {
                                let t = (py - layer.y) / layer.height;
                                Self::sample_gradient(gradient, t.clamp(0.0, 1.0))
                            } else if let Some(color) = &layer.fill {
                                *color
                            } else {
                                Color::WHITE
                            };
                            let rgba = Rgba([
                                (pixel_color.r * 255.0) as u8,
                                (pixel_color.g * 255.0) as u8,
                                (pixel_color.b * 255.0) as u8,
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

    fn resolve_fill_pixel(&self, layer: &Layer, px: f32, py: f32, x: f32, y: f32, w: f32, h: f32) -> Rgba<u8> {
        if let Some(gradient) = &layer.gradient {
            let t = (py - y) / h;
            let c = Self::sample_gradient(gradient, t.clamp(0.0, 1.0));
            Self::color_to_rgba(Color::new(c.r, c.g, c.b, c.a * layer.opacity))
        } else if let Some(color) = &layer.fill {
            Self::color_to_rgba(Color::new(color.r, color.g, color.b, color.a * layer.opacity))
        } else {
            Rgba([255, 255, 255, (255.0 * layer.opacity) as u8])
        }
    }

    fn color_to_rgba(c: Color) -> Rgba<u8> {
        Rgba([
            (c.r * 255.0) as u8,
            (c.g * 255.0) as u8,
            (c.b * 255.0) as u8,
            (c.a * 255.0) as u8,
        ])
    }

    fn tint_pixel(pixel: Rgba<u8>, tint: Color) -> Rgba<u8> {
        // Use alpha channel as mask, replace RGB with tint color
        let alpha = pixel[3] as f32 / 255.0;
        if alpha < 0.01 {
            return Rgba([0, 0, 0, 0]);
        }
        Rgba([
            (tint.r * 255.0) as u8,
            (tint.g * 255.0) as u8,
            (tint.b * 255.0) as u8,
            (alpha * tint.a * 255.0) as u8,
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

    /// Mask the canvas to a rounded rectangle shape.
    fn apply_corner_radius(&self, img: &mut RgbaImage) {
        let r = self.corner_radius;
        let size = CANVAS_SIZE as f32;
        for py in 0..CANVAS_SIZE {
            for px in 0..CANVAS_SIZE {
                if !Self::is_in_rounded_rect(px as f32, py as f32, 0.0, 0.0, size, size, r) {
                    img.put_pixel(px, py, Rgba([0, 0, 0, 0]));
                }
            }
        }
    }

    /// Draw a subtle white highlight just inside the rounded-rect edge.
    /// Blends onto the existing pixels — no extra layer needed, follows
    /// the corner radius exactly.
    fn draw_edge_highlight(&self, img: &mut RgbaImage) {
        let w = self.edge_highlight_width;
        let opacity = self.edge_highlight_opacity;
        let size = CANVAS_SIZE as f32;
        let r = self.corner_radius;
        for py in 0..CANVAS_SIZE {
            for px in 0..CANVAS_SIZE {
                // Signed distance to the rounded-rect edge (negative = inside).
                let d = Self::rounded_rect_sdf(px as f32 + 0.5, py as f32 + 0.5, 0.0, 0.0, size, size, r);
                if d <= 0.0 && d >= -w {
                    // Stronger at the very edge, fading inward.
                    let t = (-d / w).clamp(0.0, 1.0);
                    let a = t * opacity;
                    if a > 0.01 {
                        Self::blend_pixel(img, px, py, Rgba([255, 255, 255, (a * 255.0) as u8]));
                    }
                }
            }
        }
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

    /// Glossy specular highlight: a bright sheen that is strongest at the
    /// top and fades to zero over ~40 % of the canvas height.
    fn draw_specular(&self, img: &mut RgbaImage) {
        let opacity = self.specular_opacity;
        let size = CANVAS_SIZE as f32;
        let r = self.corner_radius;
        let band = size * 0.42;
        for py in 0..CANVAS_SIZE {
            // Vertical fade: 1 at top, 0 at `band` px from top.
            let t = 1.0 - (py as f32 / band);
            let fade = t.clamp(0.0, 1.0);
            // Gaussian-ish ease for a natural light curve
            let fade = fade * fade;
            let a = fade * opacity;
            if a < 0.01 { continue; }
            let alpha = (a * 255.0) as u8;
            for px in 0..CANVAS_SIZE {
                if !Self::is_in_rounded_rect(px as f32, py as f32, 0.0, 0.0, size, size, r) {
                    continue;
                }
                Self::blend_pixel(img, px, py, Rgba([255, 255, 255, alpha]));
            }
        }
    }

    /// Inset depth: darken the edges so the tile looks like a raised glass
    /// bevel. Uses the rounded-rect SDF — the closer to the edge the
    /// stronger the darkening, with Gaussian falloff over `blur` px.
    fn draw_inner_depth(&self, img: &mut RgbaImage) {
        let blur = self.inner_depth_blur;
        let opacity = self.inner_depth_opacity;
        let size = CANVAS_SIZE as f32;
        let r = self.corner_radius;
        for py in 0..CANVAS_SIZE {
            for px in 0..CANVAS_SIZE {
                let d = Self::rounded_rect_sdf(px as f32 + 0.5, py as f32 + 0.5, 0.0, 0.0, size, size, r);
                // d < 0 → inside.  At edge d ≈ 0, deeper inside → more negative.
                if d >= 0.0 || d < -blur { continue; }
                let t = (-d / blur).clamp(0.0, 1.0);
                let a = (1.0 - t * t) * opacity;
                if a < 0.01 { continue; }
                Self::blend_pixel(img, px, py, Rgba([0, 0, 0, (a * 255.0) as u8]));
            }
        }
    }

    /// Signed distance from a point to a rounded rect (negative = inside).
    fn rounded_rect_sdf(px: f32, py: f32, x: f32, y: f32, w: f32, h: f32, r: f32) -> f32 {
        let r = r.min(w / 2.0).min(h / 2.0);
        let cx = (px - (x + r)).clamp(0.0, w - 2.0 * r) + x + r;
        let cy = (py - (y + r)).clamp(0.0, h - 2.0 * r) + y + r;
        let dx = px - cx;
        let dy = py - cy;
        (dx * dx + dy * dy).sqrt() - r
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
}

impl Default for IconCanvas {
    fn default() -> Self { Self::new() }
}
