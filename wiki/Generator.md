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
| `corner_radius` | `corner_radius(r: f32) -> Self` | Round the canvas edges (`0` = square) |
| `frosted` | `frosted(opacity: f32) -> Self` | White glass wash, `0.0`–`1.0` |
| `specular` | `specular(opacity: f32) -> Self` | Glossy top-left highlight |
| `inner_depth` | `inner_depth(blur: f32, opacity: f32) -> Self` | Bottom-right inner shadow |
| `edge_highlight` | `edge_highlight(width: f32, opacity: f32) -> Self` | Bright edge line inside the round rect |
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
3. Post-processing: frosted, specular, inner depth, edge highlight
4. Corner radius mask (everything outside the rounded rect becomes transparent)

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
| `Icon(SFSymbol)` | Loads the PNG from `ASSETS_DIR`, resizes to the layer size |
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

Shadows are drawn behind their layer element, respecting its shape (rect or
circle). For `Icon`, `Image` and `Text` layers, the shadow is drawn as a
blurred rectangle matching the layer bounds.

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
| Specular | `opacity` | Bright gradient from the top-left corner; simulates glossy glass |
| Inner Depth | `blur`, `opacity` | Darkens the inside of the canvas toward the bottom-right |
| Edge Highlight | `width`, `opacity` | Bright line along the inside edge of the rounded rect |

For the iOS 26 Liquid Glass look, combine all four:

```rust
let icon = IconCanvas::new()
    .corner_radius(256.0)
    .frosted(0.18)
    .specular(0.30)
    .inner_depth(40.0, 0.35)
    .edge_highlight(8.0, 0.45)
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

## Add Depth to Existing Images

Two static methods on `IconCanvas` let you apply depth effects to any existing
image file (JPG, PNG, etc.) without building layers manually.

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

Loads the source image, resizes it to fit the 1024x1024 canvas (centered),
then applies all requested depth effects. Returns the processed `RgbaImage`.

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
`RgbaImage`.

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
    Dark,   // background -> black
    Light,  // background -> white
}
```

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

Loads the source image, samples the edge color to detect the background, then
replaces all pixels close to that color with black (`Dark`) or white (`Light`).
Foreground pixels are kept as-is. Depth effects are applied on top.

The `tolerance` for background detection is `0.18` (Euclidean distance in RGB).

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

- [SFSymbol.md](SFSymbol.md) – symbol constants rendered via `LayerContent::Icon`
- [Color.md](Color.md) – fills, tints and shadows
- [Gradient.md](Gradient.md) – background and layer gradients
- [SFSymbolView.md](SFSymbolView.md) – styled symbol views
