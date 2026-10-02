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
pub const APPLE_SQUIRCLE_EXPONENT: f32 = 5.0;
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

Takes any flat image, trims it to its opaque bounding box, scales it to
completely fill 1024x1024 (full-bleed, no transparent margin), rounds it into
the Apple squircle (`APPLE_CORNER_RADIUS 232`, exponent
`APPLE_SQUIRCLE_EXPONENT 5`) and adds the Apple Liquid Glass finish: dual drop
shadow (ambient + key), raised-relief emboss on the artwork, vibrancy pop, wide
inner bevel, rim specular, top gloss, bottom shade and an anti-aliased corner
mask. Colors are left completely untouched.

Before scaling, the source is despeckled and prefiltered at its own resolution.
A small logo carries one-to-three-pixel compression grain and a one-pixel
staircase along its diagonals; a sharp upscale spreads both into ringing blobs
that the vibrancy pass turns into visible speckle along every glyph edge. See
[Generator.md](Generator.md#source-cleanup).

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
| `from_file(..)` | Original | Original colors + raised-relief emboss + vibrancy pop + full Apple glass finish |
| `.. .dark()` | TontooOS dark `#1d1d1d` | Original colors preserved (a blue VS Code logo stays blue); small white interior cutouts follow the background (size-gated remap, tolerance `0.25`) |
| `.. .tint(c)` (Light) | Takes the tint hue too | Luminance-graded `Shaded` replacement toward `c`: all planes share one hue while overlaps stay darker |
| `.. .dark().tint(c)` | TontooOS dark `#1d1d1d` | Luminance-graded `Shaded` replacement toward `c`; interior cutouts follow the background |

> **Note:** Light + `.tint()` recolors the **whole tile**, background included,
> the way a tinted Home Screen icon does. Earlier versions protected the
> background, so `.tint()` only ever touched the artwork and the icon never
> read as recolored as a whole.

The Apple glass finish (`gloss 0.20`, `vibrancy 0.24`, `specular 0.55`,
`inner_depth(64, 0.42)`, `edge(3, 0.62)`, `shade 0.24`, `artwork_emboss 0.60`)
is always applied. For a custom radius with the same finish, use
`apple_liquid_glass(radius)` with `process_file` / `process_image`.

> **Note:** `.light()` is the default appearance and only matters to undo a
> previous `.dark()` in a builder chain.

Returns `Err` when the source file cannot be opened or decoded.

### How the tint keeps its shape

`RecolorMode::Shaded` is a three-tone ramp rather than a multiply:

1. The source lightness is normalized against the tint's own lightness, so the
   brightest plane maps to the full tint.
2. A `0.46` shadow floor keeps shadows at a fraction of the tint instead of
   driving them to black, so they stay the same hue. This is what makes
   recolored artwork read as one material instead of a stencil.
3. Highlights above `0.82` lightness roll off toward white, so the brightest
   planes get a specular lift instead of clipping flat at the tint value.

Semi-transparent pixels are un-premultiplied before recoloring and
re-premultiplied after. Recoloring the stored value reads the anti-aliased
fringe as a washed-out mid tone, which shows up as a halo of the old hue around
every glyph.

### How the dark mode keeps colors

`Appearance::Dark` first tries the standard flood-fill background swap to
`DARK_BACKGROUND` plus a size-gated `remap(white -> dark background)` limited
by `DARK_REMAP_MAX_FRACTION` (`0.10`, tolerance `0.25` so AA fringe around
holes follows too). Small white holes / cutouts therefore blend into the
background instead of glowing, while large white foreground shapes (glyphs,
bubbles) and every other pixel keep their original hue. The background is
only swapped when the flood fraction looks sane (`0.05`–`0.90`); for
monochrome icons where artwork and background share hues (gray Settings gear
on a gray gradient) it falls back to a brightness-seed artwork mask instead.

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
