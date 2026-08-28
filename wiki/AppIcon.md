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
pub const DARK_BACKGROUND: Color;
pub fn default_app_icon_depth() -> DepthOptions;

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
app-icon squircle (`corner_radius 220`) and adds the default Liquid Glass
finish (shadow, inner depth, specular, edge highlight). Colors are left
completely untouched.

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
| `from_file(..)` | Original | Original colors |
| `.. .dark()` | Dark gray `(0.13, 0.13, 0.15)` | Original colors preserved (a blue VS Code logo stays blue); white interior cutouts follow the background |
| `.. .tint(c)` (Light) | Original (untouched) | HSL colorize toward `c`; grays / whites / blacks are protected |
| `.. .dark().tint(c)` | Dark gray | Luminance-graded `Shaded` replacement toward `c`; interior cutouts follow the background |

> **Note:** `.light()` is the default appearance and only matters to undo a
> previous `.dark()` in a builder chain.

Returns `Err` when the source file cannot be opened or decoded.

### How the dark mode keeps colors

`Appearance::Dark` swaps the flood-filled background to `DARK_BACKGROUND`
and then runs a pass-through recolor (`intensity 0`) whose only active rule
is `remap(white -> dark background)`. Interior white cutouts therefore blend
into the background instead of glowing, while every other pixel keeps its
original hue.

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
