# Octopus

TontooOS octopus branding icons from `assets/TontooOS/`. The folder is a copy of `TontooIconAssets` and contains the mascot PNGs plus `OSVersionAssets/`. This page documents the `use_octopus` API: pick a PNG variant and a `Color`, get a tinted `RgbaImage` back.

## Module

```rust
pub mod octopus;
```

Import with:

```rust
use coreicon::octopus::{OctopusVariant, OctopusIcon, use_octopus};
use coreicon::Color;
```

## Constants

| Symbol | Value | Description |
|---|---|---|
| `TONTOO_OS_ASSETS_DIR` | `"assets/TontooOS"` | Base directory for branding assets (relative to runtime CWD) |

## OctopusVariant

Enum over the shipped files. Casing and extension are normalised so callers do not need to remember exact spelling.

```rust
pub enum OctopusVariant {
    Black,       // Tontoo_Black.png
    DarkBlue,    // tontoo_dark_blue.png
    Default,     // Tontoo_Default.png
    DefaultIco,  // tontoo_default.ico
    LightGreen,  // tontoo_light_green.png
    Purple,      // tontoo_purple.png
    Turkis,      // tontoo_türkis.png
    White,       // Tontoo_White.png
}
```

### Constructors and helpers

```rust
impl OctopusVariant {
    pub fn file_name(self) -> &'static str
    pub fn label(self) -> &'static str
    pub fn path(self) -> String
    pub fn from_name(name: &str) -> Option<Self>
    pub fn all() -> &'static [OctopusVariant]
}
```

| Method | Description |
|---|---|
| `file_name` | Canonical file name on disk |
| `label` | Human readable label without extension |
| `path` | Runtime-relative path `assets/TontooOS/<file>` |
| `from_name` | Parse case-insensitively, with or without `.png`/`.ico`; accepts `tuerkis`, `turkis`, `türkís` for the turquoise variant |
| `all` | All variants in stable order |

Returns `None` from `from_name` when the string does not match any variant.

## OctopusIcon

Builder that mirrors `SFSymbolView` but for octopus logos. Tint is optional.

```rust
pub struct OctopusIcon {
    variant: OctopusVariant,
    tint: Option<Color>,
}

impl OctopusIcon {
    pub fn new(variant: OctopusVariant) -> Self
    pub fn from_name(name: &str) -> Option<Self>
    pub fn tint(self, color: Color) -> Self
    pub fn no_tint(self) -> Self
    pub fn variant(&self) -> OctopusVariant
    pub fn path(&self) -> String
    pub fn file_name(&self) -> &'static str
    pub fn load(&self) -> Result<RgbaImage, Box<dyn std::error::Error>>
    pub fn load_tinted(&self) -> Result<RgbaImage, Box<dyn std::error::Error>>
    pub fn save(&self, out: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>>
}
```

| Method | Description |
|---|---|
| `new` | Create for a typed variant |
| `from_name` | Create by parsing a file-name-like string |
| `tint` | Apply a `Color` (used by `load_tinted`) |
| `no_tint` | Remove previously set tint |
| `load` | Load original `RgbaImage` without recoloring |
| `load_tinted` | Load and tint via `RecolorMode::Shaded` when a tint is set, otherwise original |
| `save` | `load_tinted` then `image::save` to `out` |

Returns `Err` when `image::open` fails (file not found or invalid format).

## Free functions

### `use_octopus`

```rust
pub fn use_octopus(file: &str, color: Color) -> Result<RgbaImage, Box<dyn std::error::Error>>
```

Load a variant by name (case-insensitive, extension optional) and tint it to `color` using `RecolorMode::Shaded`. Preserves the black outline and shading: yellows map to the full tint, blacks stay black, midtones shade proportionally.

Returns `Err` when the variant name is unknown or the underlying file cannot be opened.

### `use_octopus_variant`

```rust
pub fn use_octopus_variant(variant: OctopusVariant, color: Color) -> Result<RgbaImage, Box<dyn std::error::Error>>
```

Typed variant of `use_octopus`.

### `use_octopus_original`

```rust
pub fn use_octopus_original(file: &str) -> Result<RgbaImage, Box<dyn std::error::Error>>
```

Load without tinting.

### `use_octopus_and_save`

```rust
pub fn use_octopus_and_save(file: &str, color: Color, output: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>>
```

Tint and save to `output`.

### Discovery helpers

```rust
pub fn available_variants() -> Vec<&'static str>
pub fn available_on_disk() -> Vec<String>
```

| Function | Description |
|---|---|
| `available_variants` | Canonical file names from the enum |
| `available_on_disk` | Scan `assets/TontooOS/` on disk for `*.png`/`*.ico` files, sorted |

## Usage / Example

```rust
use coreicon::{Color, octopus::{OctopusVariant, OctopusIcon, use_octopus}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Builder style
    let blue = Color::from_hex("#007AFF").unwrap();
    OctopusIcon::new(OctopusVariant::Default)
        .tint(blue)
        .save("octopus-blue.png")?;

    // Free function: file name + color
    let orange = Color::from_hex("#FF6B2B").unwrap();
    let img = use_octopus("tontoo_purple.png", orange)?;
    img.save("octopus-orange.png")?;

    // Without extension, case-insensitive, turkis alias
    let img2 = use_octopus("tuerkis", Color::CYAN)?;
    img2.save("octopus-cyan.png")?;

    Ok(())
}
```

Recolor details: `use_octopus` uses `IconCanvas::recolor_image` with `RecolorOptions::new(color, 1.0).mode(RecolorMode::Shaded)`. The `Shaded` mode computes `k = lightness / tint_lightness` per pixel (HSL) and scales `tint.rgb * k`. White maps to full tint, black stays black, which keeps the octopus outline intact. For a flat replacement with no shading, use `RecolorMode::Replace` directly via `IconCanvas::recolor_image`.

## Cross References

- [Color.md](Color.md) - `Color` constructors and presets used as tint
- [Generator.md](Generator.md) - `RecolorOptions`, `RecolorMode::Shaded`, `IconCanvas::recolor_image`
- [OsVersion.md](OsVersion.md) - versioned OS assets under the same `assets/TontooOS/` tree
