# AppIcon

The high-level app icon APIs turn any flat image into a finished 1024x1024
app icon. Two entry points exist:

1. `IconCanvas::png_to_3d_icon` - one call, same colors, 3D finish.
2. `AppIcon` - the builder behind it, with dark-mode and color options.

Both run the full pipeline (background handling, recolor, Liquid Glass depth,
corner mask) with sensible defaults, so no `DepthOptions` wiring is needed.

## Module

```rust
pub mod generator;
```

Both APIs live in `CoreIcon::generator`:

```rust
pub enum Appearance { Light, Dark }
pub const DARK_BACKGROUND: Color; // TontooOS dark #1d1d1d
pub const APPLE_CORNER_RADIUS: f32 = 232.0;
pub fn default_app_icon_depth() -> DepthOptions;
pub fn apple_liquid_glass(corner_radius: f32) -> DepthOptions;

pub struct AppIcon { /* ... */ }
impl IconCanvas {
    pub fn png_to_3d_icon(input_path: impl AsRef<Path>) -> Result<RgbaImage, Box<dyn std::error::Error>>;
}
```

## API 1: `png_to_3d_icon`

```rust
let icon = IconCanvas::png_to_3d_icon("logo.png")?;
icon.save("app-icon.png")?;
```

Takes any flat image, scales it to exactly 1024x1024, rounds it into the
Apple squircle (`APPLE_CORNER_RADIUS 232`) and adds the Apple-strong Liquid
Glass finish: dual drop shadow (ambient + key), vibrancy pop, top gloss +
diagonal sheen, specular rim, wide inner bevel, bottom shade, gradient edge
stroke and an anti-aliased corner mask. Colors are left completely untouched.

## API 2: `AppIcon`

```rust
pub struct AppIcon { /* ... */ }

impl AppIcon {
    pub fn from_file(path: impl AsRef<Path>) -> Self;
    pub fn from_image(image: &RgbaImage) -> Self;
    pub fn appearance(appearance: Appearance) -> Self;
    pub fn dark(self) -> Self;
    pub fn light(self) -> Self;
    pub fn tint(self, color: Color) -> Self;
    pub fn no_tint(self) -> Self;
    pub fn process(&self) -> Result<RgbaImage, Box<dyn std::error::Error>>;
    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>>;
}
```

Default equals API 1. Options combine freely:

| Call chain | Background | Artwork |
|---|---|---|
| `from_file(..)` | Original | Original colors + vibrancy pop + full Apple glass finish |
| `.. .dark()` | TontooOS dark `#1d1d1d` | Original colors preserved (a blue VS Code logo stays blue); white interior cutouts follow the background |
| `.. .tint(c)` (Light) | Original (untouched) | HSL colorize toward `c`; grays / whites / blacks are protected |
| `.. .dark().tint(c)` | TontooOS dark `#1d1d1d` | Luminance-graded `Shaded` replacement toward `c`; interior cutouts follow the background |

The Apple glass finish (`gloss 0.24`, `vibrancy 0.22`, `specular 0.50`,
`inner_depth(52, 0.38)`, `edge(3, 0.60)`, `shade 0.20`) is always applied.
For a custom radius with the same finish, use
`apple_liquid_glass(radius)` with `process_file` / `process_image`.

> **Note:** `.light()` is the default appearance and only matters to undo a
> previous `.dark()` in a builder chain.

Returns `Err` when the source file cannot be opened or decoded.

### How the dark mode keeps colors

`Appearance::Dark` swaps the flood-filled background to `DARK_BACKGROUND`
and then runs a pass-through recolor (`intensity 0`) whose only active rule
is a size-gated `remap(white -> dark background)` limited by
`DARK_REMAP_MAX_FRACTION` (`0.10`). Small white holes / cutouts therefore
blend into the background instead of glowing, while large white foreground
shapes (glyphs, bubbles) and every other pixel keep their original hue.

## Usage / Example

```rust
use CoreIcon::{Color, generator::{AppIcon, IconCanvas}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // API 1: flat PNG -> 3D app icon, same colors.
    let icon = IconCanvas::png_to_3d_icon("vscode.png")?;
    icon.save("vscode-app.png")?;

    // API 2: dark mode, VS Code stays blue.
    AppIcon::from_file("vscode.png").dark().save("vscode-dark.png")?;

    // API 2: red artwork on the dark background.
    let red = Color::from_hex("#FF3B30").unwrap();
    AppIcon::from_file("vscode.png").dark().tint(red).save("vscode-red-dark.png")?;

    Ok(())
}
```

A runnable version lives in `examples/app_icon_demo`.

## Cross References

- [Generator.md](Generator.md) - the underlying processing core
  (`process_file`, `RecolorOptions`, `DepthOptions`)
- [TintMatrix.md](TintMatrix.md) - low-level color matrices used by the modes
