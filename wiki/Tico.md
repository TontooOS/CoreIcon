# Tico

TICO is the TontooOS icon container. A `.tico` file is a plain ZIP archive
that is only ever named `*.tico` (never `*.tico.zip`). It stores an app icon
as recolorable layers, so CoreIcon can render a high-res icon in any color
and size from a few kilobytes of data. There is no preview image inside.

## Layout

```text
my-icon.tico            # ZIP archive with .tico extension
├── manifest.json       # format version, name, background, layer table
├── layer/00.tlyr       # custom layer files (never .png)
└── layer/01.tlyr
```

A `.tlyr` file is a tiny custom format: magic `TLYR`, one version byte, width
and height (`u32` LE), PNG-coded RGBA bytes. Layers are stored at full
1024px. Flat builder artwork (shapes, symbols) compresses to a few KB per
layer; whole icons land in the 1-100KB budget. Photo backgrounds do not fit
that budget and should stay outside `.tico`.

## Module

```rust
pub mod tico;
```

```rust
pub struct Tico;
pub struct TicoIcon;
pub enum TicoError { Io, Zip, Json, Image, Format }
```

## Export

```rust
pub fn export(canvas: &IconCanvas, name: &str, path: impl AsRef<Path>) -> Result<(), TicoError>
```

Rasterizes every layer of an [`IconCanvas`](Generator.md) at 1024px into one
`layer/NN.tlyr` file. Solid and gradient backgrounds are stored as hex
colors in the manifest (no raster); image backgrounds are embedded as
`layer/background.tlyr`. Shape, symbol and text layers are flagged
`recolorable` with their fill as `default_color`; image layers are stored
as-is and never recolored.

Returns `Err` when a background image cannot be opened, a layer cannot be
rasterized, or the output file cannot be written.

```rust
use CoreIcon::generator::*;
use CoreIcon::tico::Tico;
use CoreIcon::Color;

let icon = IconCanvas::new()
    .background(Background::color(Color::from_hex("#1d1d1d").unwrap()))
    .layer(
        Layer::new(LayerContent::circle(560.0))
            .position(232.0, 140.0)
            .tint(Color::WHITE),
    );

Tico::export(&icon, "demo", "demo.tico")?;
```

## Load

```rust
pub fn load(path: impl AsRef<Path>) -> Result<TicoIcon, TicoError>
```

Reads `manifest.json`, validates `format == "tico"` and the version, then
decodes every `.tlyr` file (magic, version and dimensions are checked).

Returns `Err` when the file is missing, is not a ZIP, has no manifest, has
an unsupported version, or a layer file is corrupt.

```rust
let icon = Tico::load("demo.tico")?;
```

## Render

```rust
pub fn render(&self, size: u32, tint: Option<Color>) -> Result<RgbaImage, TicoError>
pub fn render_default(&self) -> Result<RgbaImage, TicoError>
pub fn name(&self) -> &str
pub fn layer_count(&self) -> usize
```

`render` composites background plus layers at 1024px, applies `tint` to
recolorable layers with a luminance-graded shade (all artwork shares one
hue, shading and overlaps survive), adds the Apple app-icon finish and
scales to `size` (`16`–`4096` clamped, `1024` default). `render_default`
renders at 1024px with the stored layer colors.

Returns `Err` when the manifest holds an invalid hex color or a raster
background is missing.

```rust
use CoreIcon::Color;

let red = Color::from_hex("#FF3B30").unwrap();
let big = icon.render(1024, Some(red))?;
let small = icon.render(256, Some(red))?;
big.save("icon-red-1024.png")?;
```

A runnable version lives in `examples/tico_demo` (`demo.tico`, 6.5KB for
two layers, rendered in default/red/blue).

## Usage / Example

```rust
use CoreIcon::generator::*;
use CoreIcon::tico::Tico;
use CoreIcon::Color;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let icon = IconCanvas::new()
        .background(Background::color(Color::from_hex("#1d1d1d").unwrap()))
        .layer(
            Layer::new(LayerContent::circle(560.0))
                .position(232.0, 140.0)
                .tint(Color::WHITE),
        );

    // Save as demo.tico (plain ZIP, .tico extension, no PNG files).
    Tico::export(&icon, "demo", "demo.tico")?;

    // Load and render a red 1024px icon plus a blue 256px one.
    let tico = Tico::load("demo.tico")?;
    let red = Color::from_hex("#FF3B30").unwrap();
    tico.render(1024, Some(red))?.save("tico-red.png")?;
    tico.render(256, Some(Color::ACCENT))?.save("tico-blue-256.png")?;
    Ok(())
}
```

## Cross References

- [Generator.md](Generator.md) - `IconCanvas`, layers and the app-icon finish
  used by export and render
- [Color.md](Color.md) - hex colors used in the manifest and tints
- [AppIcon.md](AppIcon.md) - one-call raster app icons (no layers kept)
