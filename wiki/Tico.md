# Tico

TICO is the TontooOS icon container. A `.tico` file is a TICO container as defined by ArchiveKit (same indexed single-file engine as `.app` containers, with its own `TICO`/`TICF` magic, a central directory and a CRC-checked footer) that is only ever named `*.tico` (never `*.tico.zip`). It stores an app icon as recolorable layers, so CoreIcon can render a high-res icon in any color and size from a few kilobytes of data. There is no preview image inside.

## Layout

```text
my-icon.tico            # TICO container with .tico extension
├── manifest.fico       # format version, name, background, layer table (Fish Config)
├── layer/00.tlyr       # custom layer files (never .png)
└── layer/01.tlyr
```

A `.tlyr` file is a tiny custom format: magic `TLYR`, one version byte, width and height (`u32` LE), PNG-coded RGBA bytes. Layers are stored at full 1024px. Flat builder artwork (shapes, symbols) compresses to a few KB per layer; whole icons land in the 1-100KB budget. Photo backgrounds do not fit that budget and should stay outside `.tico`.

Listing names or reading one layer only touches the footer plus the central directory (or the single entry). Packing, indexing and structural validation live in ArchiveKit (`TicoBuilder`, `TicoReader`, `validate_tico`); see [ArchiveKit Tico](https://github.com/TontooOS/ArchiveKit) for the container details.

## Module

```rust
pub mod tico;
```

```rust
pub struct Tico;
pub struct TicoIcon;
pub enum TicoError { Io, Container, Image, Format }
```

`TicoError::Container` covers every ArchiveKit failure (bad magic, corrupt central directory, missing entry, CRC mismatch). `TicoError::Format` covers corrupt `.tlyr` data, bad colors and bad gradient directions.

## Export

```rust
pub fn export(canvas: &IconCanvas, name: &str, path: impl AsRef<Path>) -> Result<(), TicoError>
pub fn export_bytes(canvas: &IconCanvas, name: &str) -> Result<Vec<u8>, TicoError>
```

Rasterizes every layer of an [`IconCanvas`](Generator.md) at 1024px into one `layer/NN.tlyr` file. Solid and gradient backgrounds are stored as hex colors in the manifest (no raster); image backgrounds are embedded as `layer/background.tlyr`. Shape, symbol and text layers are flagged `recolorable` with their fill as `default_color`; image layers are stored as-is and never recolored.

Returns `Err` when a background image cannot be opened, a layer cannot be rasterized, the container has no layers, or the output file cannot be written.

> **Note:** ArchiveKit validates the container and rejects an empty layer
> table (`tico has no layers`). A canvas with only a background and no
> layer therefore cannot be exported. Wrap flat artwork in one
> `LayerContent::image` layer instead; image layers are stored
> non-recolorable and keep their colors.

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

## Flat PNG to tico

`examples/tico_from_png` converts an existing app-icon PNG into the
container, which is what app bundles ship:

```bash
cargo run --release --manifest-path examples/tico_from_png/Cargo.toml -- \
    Resources/app_icon.png Weather Weather.app/App/icon.tico preview.png
```

- The artwork becomes exactly one full-bleed `LayerContent::image`
  layer over a transparent background, so the container has a layer
  table and validates.
- The layer is flagged non-recolorable, so `render` never tints it; the
  Apple app icon finish is added by `TicoIcon::render`, not baked into
  the stored layer.
- Layers are stored at 1024px, so a smaller source is upscaled by the
  rasterizer. A raster app icon lands around 3-4 MB per layer.
- Prints the container size and layer count, then reloads the file and
  writes a default render plus an accent-tinted render next to
  `preview.png` (`preview-tinted.png`).
- Exits with code `2` and a usage line when fewer than three arguments
  are given, and returns an error when the input PNG does not exist.

## Load

```rust
pub fn load(path: impl AsRef<Path>) -> Result<TicoIcon, TicoError>
pub fn load_bytes(bytes: &[u8]) -> Result<TicoIcon, TicoError>
```

Reads `manifest.fico` through the ArchiveKit container index, validates the format and version, then decodes every `.tlyr` file (magic, version and dimensions are checked).

Returns `Err` when the file is missing, is not a TICO container, has no manifest, has an unsupported version, or a layer file is corrupt.

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

`render` composites background plus layers at 1024px, applies `tint` to recolorable layers with a luminance-graded shade (all artwork shares one hue, shading and overlaps survive), adds the Apple app-icon finish and scales to `size` (`16`–`4096` clamped, `1024` default). `render_default` renders at 1024px with the stored layer colors.

Returns `Err` when the manifest holds an invalid hex color or a raster background is missing.

```rust
use CoreIcon::Color;
use coreimage::{ImageFormat, TiImage};

let red = Color::from_hex("#FF3B30").unwrap();
let big = icon.render(1024, Some(red))?;
let small = icon.render(256, Some(red))?;
TiImage::from_rgba(big).save("icon-red-1024.png", ImageFormat::Png, 100)?;
```

A runnable version lives in `examples/tico_demo` (`demo.tico`, two layers, rendered in default/red/blue). Layer decoding goes through CoreImage PNG; see `tests/tico_container.rs` for the container roundtrip tests.

## Usage / Example

```rust
use CoreIcon::generator::*;
use CoreIcon::tico::Tico;
use CoreIcon::Color;
use coreimage::{ImageFormat, TiImage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let icon = IconCanvas::new()
        .background(Background::color(Color::from_hex("#1d1d1d").unwrap()))
        .layer(
            Layer::new(LayerContent::circle(560.0))
                .position(232.0, 140.0)
                .tint(Color::WHITE),
        );

    // Save as demo.tico (TICO container, .tico extension, no PNG files).
    Tico::export(&icon, "demo", "demo.tico")?;

    // Load and render a red 1024px icon plus a blue 256px one.
    let tico = Tico::load("demo.tico")?;
    let red = Color::from_hex("#FF3B30").unwrap();
    TiImage::from_rgba(tico.render(1024, Some(red))?).save("tico-red.png", ImageFormat::Png, 100)?;
    TiImage::from_rgba(tico.render(256, Some(Color::ACCENT))?).save("tico-blue-256.png", ImageFormat::Png, 100)?;
    Ok(())
}
```

## Cross References

- [Generator.md](Generator.md) - `IconCanvas`, layers and the app-icon finish used by export and render
- [Color.md](Color.md) - hex colors used in the manifest and tints
- [AppIcon.md](AppIcon.md) - one-call raster app icons (no layers kept)
