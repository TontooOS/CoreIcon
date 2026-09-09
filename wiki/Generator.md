# Icon Generator

The icon generator renders 1024x1024 PNG icons using a builder-based API. It
supports layered composition of SF Symbols, shapes, text and raster images with
per-layer gradient or solid fill, transparency, shadows, inner shadows and a set
of Liquid Glass post-processing effects (frosted, specular, inner depth, edge
highlight).

## Module

```rust
pub mod generator;
```

Import everything with:

```rust
use CoreIcon::generator::*;
```

## Constants and globals

| Symbol | Value | Description |
|---|---|---|
| `CANVAS_SIZE` | `1024` | Rendered image dimension |
| `ASSETS_DIR` | `static mut &str = "assets/icons"` | Runtime path to SF Symbol PNGs |

> **Note:** `ASSETS_DIR` is `unsafe` to modify. Point it at the real assets
> folder at startup before rendering any icon layers.

## IconCanvas

`IconCanvas` is the top-level builder. All state is owned; the API is
builder-pattern and the final call is `save` or `render`.

```rust
pub struct IconCanvas { /* ... */ }

impl IconCanvas {
    pub fn new() -> Self;
}
```

The default background is a solid dark color `(0.11, 0.11, 0.118)`. All
post-processing effects start at zero.

### Builder methods

| Method | Signature | Description |
|---|---|---|
| `background` | `background(bg: Background) -> Self` | Fill the entire canvas |
| `corner_radius` | `corner_radius(r: f32) -> Self` | Round the canvas edges (`0` = square, `APPLE_CORNER_RADIUS` = `232.0` for the Apple squircle) |
| `frosted` | `frosted(opacity: f32) -> Self` | White glass wash, `0.0`–`1.0` |
| `specular` | `specular(opacity: f32) -> Self` | Glossy rim highlight on light-facing edges |
| `gloss` | `gloss(opacity: f32) -> Self` | Full-surface top gloss + diagonal sheen (Apple Liquid Glass) |
| `vibrancy` | `vibrancy(v: f32) -> Self` | Saturation + contrast pop for flat artwork |
| `shade` | `shade(opacity: f32) -> Self` | Bottom inner shade that grounds the icon |
| `inner_depth` | `inner_depth(blur: f32, opacity: f32) -> Self` | Inner shadow on the side away from the light |
| `edge_highlight` | `edge_highlight(width: f32, opacity: f32) -> Self` | Gradient edge stroke, bright on the light side |
| `light_direction` | `light_direction(x: f32, y: f32) -> Self` | Light source for `specular`, `inner_depth`, `edge_highlight`, `gloss`; default top-left (`-0.45, -0.89`) |
| `glass` | `glass() -> Self` | One-call Apple Liquid Glass preset (see below) |
| `layer` | `layer(layer: Layer) -> Self` | Append a layer (drawn in order) |

### Rendering

```rust
pub fn save(&self, path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>>;
pub fn render(&self) -> RgbaImage;
```

- `save` writes a PNG to disk. Returns `Err` when the image crate fails.
- `render` returns the raw `RgbaImage` for further programmatic use.

The render order is:

1. Background fill
2. All layers (in order: `shadow`, then element content)
3. Frosted wash (if enabled)
4. Light-steered glass effects: vibrancy, specular rim, top gloss + sheen,
   inner depth, bottom shade, gradient edge stroke
5. Anti-aliased corner mask (1.5px feather, everything outside the rounded
   rect becomes transparent)

### Liquid Glass preset

`.glass()` applies the Apple Tahoe / iOS 26 combination in one call:

```rust
let icon = IconCanvas::new()
    .glass()          // corner 232, frosted 0.08, specular 0.50,
    // ... layers ... // inner_depth(52, 0.38), edge(3, 0.60),
                      // gloss 0.24, vibrancy 0.18, shade 0.20
    .background(Background::color(Color::from_hex("#2255AA").unwrap()))
;
```

Every value can be overridden by calling the individual builders afterwards.

## Background

```rust
pub enum Background {
    Color(Color),
    Gradient(Gradient),
    Image { path: String, tint: Option<Color> },
}
```

Constructors:

| Function | Description |
|---|---|
| `Background::color(c)` | Solid color fill |
| `Background::gradient(g)` | Gradient fill |
| `Background::image(path)` | Raster image stretched to 1024x1024 |
| `Background::image_tinted(path, tint)` | Image with a color tint applied |

## Layer

```rust
pub struct Layer {
    pub content: LayerContent,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub fill: Option<Color>,
    pub gradient: Option<Gradient>,
    pub tint_matrix: Option<TintMatrix>,
    pub opacity: f32,
    pub shadow: Option<Shadow>,
    pub inner_shadow: Option<Shadow>,
}
```

### Constructors and builders

| Method | Description |
|---|---|
| `Layer::new(content)` | Defaults: position `(0,0)`, size `100x100`, no fill, opacity `1.0` |
| `.position(x, y)` | Offset on the 1024px canvas |
| `.size(w, h)` | Element dimensions |
| `.tint(c)` | Solid color fill |
| `.gradient(g)` | Gradient fill (overrides tint) |
| `.tint_matrix(m)` | Color-matrix recolor applied after fill/gradient (see [TintMatrix.md](TintMatrix.md)) |
| `.opacity(o)` | Clamped to `[0.0, 1.0]` |
| `.shadow(s)` | Shadow drawn before the element |
| `.inner_shadow(s)` | Shadow drawn inside the element's alpha |

> **Note:** `gradient` and `tint` are mutually exclusive on a single layer. If
> both are set, `gradient` takes precedence during rasterization.

## LayerContent

```rust
pub enum LayerContent {
    Icon(SFSymbol),
    Rect { width: f32, height: f32, corner_radius: f32 },
    Circle { diameter: f32 },
    Image { path: String },
    Text { content: String, font_size: f32 },
}
```

| Variant | Behavior |
|---|---|
| `Icon(SFSymbol)` | Loads the PNG from `ASSETS_DIR`, aspect-fit (contain) into the padded layer box, centered - non-square symbols are no longer stretched |
| `Rect` | Fills a rounded rectangle; `corner_radius` is the rect's own radius |
| `Circle` | Fills a circle of the given diameter |
| `Image` | Loads a raster image from the given file path |
| `Text` | Renders text using SF Pro (Linux: `/usr/share/fonts/OTF/` or `TTF/`); font size in px |

### Constructors

```rust
LayerContent::icon(symbol)
LayerContent::rect(w, h, corner_radius)
LayerContent::circle(diameter)
LayerContent::image(path)
LayerContent::text(content, font_size)
```

## Shadow

```rust
pub struct Shadow {
    pub offset_x: f32,
    pub offset_y: f32,
    pub blur: f32,
    pub color: Color,
    pub opacity: f32,
}
```

| Method | Description |
|---|---|
| `Shadow::new()` | `offset(0, 8)`, `blur(16)`, black at `opacity(0.3)` |
| `.offset(x, y)` | Shadow shift in pixels |
| `.blur(b)` | Gaussian-like blur radius in pixels |
| `.color(c)` | Shadow color |
| `.opacity(o)` | Clamped to `[0.0, 1.0]` |

Shadows are drawn behind their layer element. For `Rect` and `Circle` layers
the shadow follows the shape; for `Icon` layers it is glyph-shaped - computed
from a chamfer distance transform of the actual symbol silhouette, so the
shadow matches the artwork instead of a bounding box. The transform is `O(n)`
regardless of blur radius.

## Inner Shadow

An inner shadow darkens the inside edge of an element. It is computed using a
two-pass distance transform on the element's alpha mask. The shadow is strongest
at the edge and fades inward over `blur` pixels.

## Post-Processing Effects

These are applied to the entire canvas as a single pass after all layers are
drawn, before the corner mask. They are only active when their opacity / blur
values are greater than zero.

| Effect | Parameters | Description |
|---|---|---|
| Frosted | `opacity` | Blends white over every opaque pixel; simulates frosted glass |
| Vibrancy | `v` | Saturation + contrast pop; near-grays (whites/blacks) are skipped |
| Specular | `opacity` | Bright rim on the edges facing the light (rim lighting via the rounded-rect surface normal), fading inward over ~3.5% of the canvas |
| Gloss | `opacity` | Soft white gradient over the top ~45% plus a diagonal sheen band (Apple Liquid Glass) |
| Inner Depth | `blur`, `opacity` | Darkens the inside edge on the side away from the light |
| Bottom Shade | `opacity` | Dark gradient over the bottom ~26% that grounds the icon |
| Edge Highlight | `width`, `opacity` | Gradient stroke along the inside edge: full strength on the light side, ~35% opposite; plus a thin dark outer rim on the shadow side |

All light-steered effects share `light_direction(x, y)`. The vector
points toward the light source; the default is top-left (`-0.45, -0.89`).
Because the specular band uses the surface normal of the rounded rect, the
sheen wraps around corners like real glass.

For the Apple Liquid Glass look, either combine them manually or use
`.glass()`.

```rust
let icon = IconCanvas::new()
    .corner_radius(232.0)
    .gloss(0.24)
    .vibrancy(0.18)
    .specular(0.50)
    .inner_depth(52.0, 0.38)
    .shade(0.20)
    .edge_highlight(3.0, 0.60)
    // ... layers ...
;
```

## Full Example

A complete app-icon generator:

```rust
use CoreIcon::generator::*;
use CoreIcon::{Color, Gradient, GradientDirection, GradientStop, SFSymbol};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe { CoreIcon::generator::ASSETS_DIR = "../../assets/icons"; }

    let dark  = Color::from_hex("#1d1d1d").unwrap();
    let white = Color::WHITE;

    let icon = IconCanvas::new()
        .corner_radius(256.0)
        .frosted(0.18)
        .specular(0.30)
        .inner_depth(40.0, 0.35)
        .edge_highlight(8.0, 0.45)
        .background(Background::gradient(
            Gradient::new(
                GradientDirection::TopToBottom,
                vec![
                    GradientStop::new(dark, 0.0),
                    GradientStop::new(Color::new(0.08, 0.08, 0.12, 1.0), 1.0),
                ],
            )
        ))
        .layer(
            Layer::new(LayerContent::icon(SFSymbol::HOUSE_FILL))
                .position(212.0, 212.0)
                .size(600.0, 600.0)
                .tint(white)
                .shadow(Shadow::new().offset(0.0, 10.0).blur(20.0))
        );

    icon.save("app-icon.png")?;
    Ok(())
}
```

## Image Processing Core

All file-based entry points (`add_depth_to_image`, `change_color`,
`dark_light_mode`, `set_background_color`) are thin wrappers over one
pipeline. The pipeline is also available directly:

### `process_file` / `process_image`

```rust
pub fn process_file(
    input_path: impl AsRef<Path>,
    options: &ProcessOptions,
) -> Result<RgbaImage, Box<dyn std::error::Error>>
pub fn process_image(src: &RgbaImage, options: &ProcessOptions) -> RgbaImage
```

`process_file` loads and scales the source to exactly 1024x1024, then runs the
same pipeline as `process_image`. Pipeline order:

1. Background replacement (flood fill from the edges)
2. Recolor (`RecolorOptions`)
3. Dual shadow (distance transform) -> content -> vibrancy -> gloss +
   sheen -> specular -> inner depth -> bottom shade -> gradient edge stroke
4. Anti-aliased corner mask

```rust
pub struct ProcessOptions {
    pub recolor: Option<RecolorOptions>,
    pub background_replace: Option<Color>,
    pub depth: DepthOptions,
    pub protect_background: bool,
}
```

`protect_background: true` excludes the flood-filled background region from
recoloring (computed with strict thresholds so gray artwork is still tinted).
This is how `AppIcon` Light+tint colors monochrome artwork while keeping the
original background.

### `DepthOptions`

Builder for all depth effects; defaults switch every effect off.

| Method | Description |
|---|---|
| `DepthOptions::new(corner_radius)` | Start with a corner radius, effects off |
| `.shadow(s)` | Dual drop shadow: large soft ambient + tight key (`Shadow`); key is synthesized when `blur > 12` |
| `.artwork_shadow(s)` | Glyph drop shadow onto the background (file pipeline only; foreground = inverse of the border flood mask) |
| `.inner_depth(blur, opacity)` | Inner bevel away from the light |
| `.specular(opacity)` | Rim highlight toward the light |
| `.gloss(opacity)` | Top gloss + diagonal sheen |
| `.vibrancy(v)` | Saturation + contrast pop |
| `.shade(opacity)` | Bottom grounding shade |
| `.edge_highlight(width, opacity)` | Gradient edge stroke + dark outer rim on the shadow side |
| `.light_direction(x, y)` | Light source vector, default top-left (`-0.45, -0.89`) |

Apple presets:

```rust
pub const APPLE_CORNER_RADIUS: f32 = 232.0;
pub fn default_app_icon_depth() -> DepthOptions; // Apple-strong AppIcon finish
pub fn apple_liquid_glass(corner_radius: f32) -> DepthOptions; // same finish, custom radius
```

### `recolor_image`

```rust
pub fn recolor_image(src: &RgbaImage, options: &RecolorOptions) -> RgbaImage
```

Pure recoloring without depth effects or canvas compositing.

```rust
pub enum RecolorMode {
    Colorize,
    AccentLuma,
    Replace,
    Shaded,
}

pub struct RecolorOptions {
    pub tint: Color,
    pub intensity: f32,
    pub mode: RecolorMode,
    pub neutral_threshold: f32,
    pub protect: Option<Color>,
    pub protect_tolerance: f32,
    pub remap_from: Option<Color>,
    pub remap_to: Option<Color>,
    pub remap_tolerance: f32,
}
```

| Mode | Algorithm |
|---|---|
| `Colorize` | HSL colorize: hue + saturation from `tint`, lightness preserved per pixel |
| `AccentLuma` | Apple accent tint: `out.rgb = tint.rgb * (0.2*luma + 0.8*value)` |
| `Replace` | Flat template replacement: RGB becomes `tint`, alpha kept |
| `Shaded` | Luminance-graded replacement: pixel becomes `tint * (lightness / tint_lightness)` clamped to 1.0 - whites map to the full tint, blacks stay black |

`neutral_threshold` only applies to `Colorize`; pure white/black carry no hue
information and can never be tinted by HSL colorize - use `Shaded` when such
pixels must be recolored too. `protect(color)` skips pixels within
`protect_tolerance` (Euclidean RGB, 0.0-1.0 space, default `0.12`) of the
given color - typically the flat background color swapped earlier in the same
`ProcessOptions` run.

`remap(from, to)` replaces pixels within `remap_tolerance` of `from` with `to`
outright, before the mode is applied. Use it for interior cutouts that should
follow the swapped background color instead of the artwork color.
`remap_max_fraction(f)` restricts the remap to connected components smaller
than `f` of the image area, so small holes follow the background while large
foreground shapes (white glyphs, bubbles) survive. `None` (default) remaps
every matching pixel.

`intensity` blends linearly between original and recolored (`0.0` = original).
In `Colorize` mode pixels with saturation at or below `neutral_threshold`
(default `0.05`) keep their original color, protecting grays / white / black.
The conversion works on straight alpha with proper rounding, so anti-aliased
edges no longer produce dark fringes.

## Add Depth to Existing Images

Two static methods on `IconCanvas` let you apply depth effects to any existing
image file (JPG, PNG, etc.) without building layers manually. Both delegate to
the processing core described above; shadows use a distance transform of the
image silhouette.

### `add_depth_to_image`

```rust
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
) -> Result<RgbaImage, Box<dyn std::error::Error>>
```

Loads the source image, scales it to exactly 1024x1024, then applies all
requested depth effects. Returns the processed `RgbaImage`.

Parameters:

| Parameter | Description |
|---|---|
| `input_path` | Path to the source image (any format supported by the `image` crate) |
| `corner_radius` | Round the canvas edges; `0` = square, `220` = iOS-style icon |
| `shadow_offset_x` | Horizontal shadow shift in pixels |
| `shadow_offset_y` | Vertical shadow shift in pixels |
| `shadow_blur` | Shadow blur radius in pixels |
| `shadow_opacity` | Shadow strength, `0.0`–`1.0` |
| `inner_depth_blur` | Inner bevel blur radius; `0` to disable |
| `inner_depth_opacity` | Inner bevel strength, `0.0`–`1.0` |
| `specular_opacity` | Glossy highlight strength, `0.0`–`1.0` |
| `edge_highlight_width` | Edge highlight thickness in pixels; `0` to disable |
| `edge_highlight_opacity` | Edge highlight strength, `0.0`–`1.0` |

Pass `None` for any effect you want to skip.

### `add_depth_to_image_and_save`

```rust
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
) -> Result<(), Box<dyn std::error::Error>>
```

Convenience wrapper that calls `add_depth_to_image` and saves the result.

### Example: Add Depth to an Existing Icon

```rust
use CoreIcon::generator::IconCanvas;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Convert JPG to PNG and apply depth in one go
    let result = IconCanvas::add_depth_to_image(
        "my-app-icon.jpg",
        220.0,              // corner_radius
        Some(0.0),          // shadow_offset_x
        Some(10.0),         // shadow_offset_y
        Some(20.0),         // shadow_blur
        Some(0.35),         // shadow_opacity
        Some(14.0),         // inner_depth_blur
        Some(0.3),          // inner_depth_opacity
        Some(0.18),         // specular_opacity
        Some(5.0),          // edge_highlight_width
        Some(0.25),         // edge_highlight_opacity
    )?;
    result.save("my-app-icon-depth.png")?;
    Ok(())
}
```

## Change Color

Tint an existing icon to a target color with configurable intensity, then
apply depth effects. The depth effects (shadow, specular, inner depth,
edge highlight) keep their original colors and are not tinted.

### `change_color`

```rust
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
) -> Result<RgbaImage, Box<dyn std::error::Error>>
```

Loads the source image, blends each pixel toward `tint_color` based on
`intensity`, then applies depth effects on top. Returns the processed
`RgbaImage`. This is a convenience wrapper over
`process_file` with `RecolorMode::Colorize` and the neutral threshold fixed at
`0.05`; use [`recolor_image`](#recolor_image) or [`ProcessOptions`] to pick a
different mode or threshold.

Colorization happens in HSL space: hue and saturation are taken from
`tint_color` while each pixel's lightness is preserved. Pixels with near-zero
saturation (grays, white, black) keep their original color instead of being
tinted. `intensity` then blends linearly between the original and the
colorized color.

Parameters:

| Parameter | Description |
|---|---|
| `input_path` | Path to the source image |
| `tint_color` | Target color to blend toward |
| `intensity` | Blend factor, `0.0` = original, `1.0` = fully tinted |
| `corner_radius` | Round the canvas edges; `0` = square |
| `shadow_offset_x` | Horizontal shadow shift in pixels |
| `shadow_offset_y` | Vertical shadow shift in pixels |
| `shadow_blur` | Shadow blur radius in pixels |
| `shadow_opacity` | Shadow strength, `0.0`–`1.0` |
| `inner_depth_blur` | Inner bevel blur radius; `0` to disable |
| `inner_depth_opacity` | Inner bevel strength, `0.0`–`1.0` |
| `specular_opacity` | Glossy highlight strength, `0.0`–`1.0` |
| `edge_highlight_width` | Edge highlight thickness in pixels; `0` to disable |
| `edge_highlight_opacity` | Edge highlight strength, `0.0`–`1.0` |

### `change_color_and_save`

```rust
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
) -> Result<(), Box<dyn std::error::Error>>
```

Convenience wrapper that calls `change_color` and saves the result.

### Example: Tint an Icon to Orange

```rust
use CoreIcon::generator::IconCanvas;
use CoreIcon::Color;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let orange = Color::from_hex("#FF6B2B").unwrap();
    let result = IconCanvas::change_color(
        "my-icon.png",
        orange,             // tint to TontooOS orange
        0.8,                // 80% intensity
        220.0,              // corner_radius
        Some(0.0),          // shadow_offset_x
        Some(10.0),         // shadow_offset_y
        Some(20.0),         // shadow_blur
        Some(0.35),         // shadow_opacity
        Some(14.0),         // inner_depth_blur
        Some(0.3),          // inner_depth_opacity
        Some(0.18),         // specular_opacity
        Some(5.0),          // edge_highlight_width
        Some(0.25),         // edge_highlight_opacity
    )?;
    result.save("orange-icon.png")?;
    Ok(())
}
```

## Dark / Light Mode

Switch an icon between dark and light mode by detecting the background color
and replacing it. The foreground (logo, text, icons) stays unchanged.

### `IconMode`

```rust
pub enum IconMode {
    Dark,   // background -> TontooOS dark #1d1d1d (DARK_BACKGROUND)
    Light,  // background -> white (1.0, 1.0, 1.0)
}
```

| Variant | Background replacement |
|---|---|
| `Dark` | `#1d1d1d` TontooOS dark (`DARK_BACKGROUND`) |
| `Light` | `(1.0, 1.0, 1.0)` white |

### `dark_light_mode`

```rust
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
) -> Result<RgbaImage, Box<dyn std::error::Error>>
```

Loads the source image, detects the background region with a flood fill seeded
from the image edges, then replaces every background pixel with a solid color
(see `IconMode`). Foreground pixels are kept as-is. Depth effects are applied
on top. Implemented via [`set_background_color`](#set_background_color).

The flood fill starts from all four border edges against a 2px-border
reference color. A 4-connected neighbor joins the background when it is close
to the reference (`0.32`) and close to its chain neighbor (`0.25`), which
keeps the fill from leaking through soft logo edges. One dilation pass
absorbs low-chroma halo pixels so the swapped background has no bright
fringe. Any pixel not reached by the fill is treated as foreground.

### `set_background_color`

```rust
pub fn set_background_color(
    input_path: impl AsRef<Path>,
    target: Color,
    depth: DepthOptions,
) -> Result<RgbaImage, Box<dyn std::error::Error>>
```

Generalization of `dark_light_mode`: replaces the flood-filled background with
any `Color` instead of the two fixed presets. Chromatic fringe pixels around
kept artwork are decontaminated (re-blended over the new color), so no halo
of the old background survives.

```rust
use CoreIcon::generator::{DepthOptions, IconCanvas};
use CoreIcon::Color;

let out = IconCanvas::set_background_color(
    "icon.png",
    Color::from_hex("#22FF88").unwrap(),
    DepthOptions::new(220.0),
)?;
```

### `dark_light_mode_and_save`

```rust
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
) -> Result<(), Box<dyn std::error::Error>>
```

Convenience wrapper that calls `dark_light_mode` and saves the result.

### Example: VS Code Dark Mode

```rust
use CoreIcon::generator::{IconCanvas, IconMode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = IconCanvas::dark_light_mode(
        "vscode-icon.png",
        IconMode::Dark,
        220.0,
        Some(0.0), Some(10.0), Some(20.0), Some(0.35),
        Some(14.0), Some(0.3),
        Some(0.18),
        Some(5.0), Some(0.25),
    )?;
    result.save("vscode-dark.png")?;
    Ok(())
}
```

## Cross References

- [TintMatrix.md](TintMatrix.md) - color-matrix recoloring used by `Layer.tint_matrix` and `RecolorOptions`
- [SFSymbol.md](SFSymbol.md) – symbol constants rendered via `LayerContent::Icon`
- [Color.md](Color.md) – fills, tints and shadows
- [Gradient.md](Gradient.md) – background and layer gradients
- [SFSymbolView.md](SFSymbolView.md) – styled symbol views
