# Tontoo CoreIcon – Wiki

Tontoo CoreIcon is a Rust library for making app icons and using SF Icons in UIs.
It ships a large set of SF Symbols as PNG assets plus a raster generator that
renders 1024x1024 icon PNGs with colors, gradients, transparency, shadows,
Liquid Glass post-processing and text.

- Repository: https://github.com/TontooOS/CoreIcon
- License: MIT
- Version: 26.1.0

## Feature Index

| Feature | File | Description |
|---|---|---|
| Main index | [MAIN.md](MAIN.md) | This page |
| Rules | [RULE.md](RULE.md) | Development and usage rules |
| Color | [Color.md](Color.md) | `Color` type, hex/rgb constructors and presets |
| Gradient | [Gradient.md](Gradient.md) | `Gradient`, `GradientStop`, `GradientDirection` |
| SFSymbol | [SFSymbol.md](SFSymbol.md) | The generated SF Symbols registry and asset paths |
| SFSymbolView | [SFSymbolView.md](SFSymbolView.md) | Styled symbol views for use in UIs |
| Icon Generator | [Generator.md](Generator.md) | `IconCanvas`, `Layer`, `Shadow`, `Background`, image processing pipeline, PNG generation |
| TintMatrix | [TintMatrix.md](TintMatrix.md) | 4x5 color-matrix recoloring (Apple-style tint rows) |
| AppIcon | [AppIcon.md](AppIcon.md) | High-level APIs: PNG to 3D app icon, dark mode + color options |
| Octopus | [Octopus.md](Octopus.md) | TontooOS octopus branding icons: `use_octopus` with PNG variant + `Color` tint |
| OsVersion | [OsVersion.md](OsVersion.md) | OS version assets: `use_osversionicons` with `version` + `name` under `OSVersionAssets/` |

## Quick Start

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
CoreIcon = { path = "/Library/System/coreicon.library" }
```

Point the generator at the SF Symbol assets, then render an icon:

```rust
use CoreIcon::prelude::*;
use CoreIcon::generator::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe { CoreIcon::generator::ASSETS_DIR = "assets/icons"; }

    let icon = IconCanvas::new()
        .background(Background::color(Color::from_hex("#1d1d1d").unwrap()))
        .layer(
            Layer::new(LayerContent::icon(SFSymbol::HOUSE))
                .position(312.0, 312.0)
                .size(400.0, 400.0)
                .tint(Color::WHITE)
                .shadow(Shadow::new().offset(0.0, 10.0).blur(20.0))
        );

    icon.save("my-icon.png")?;
    Ok(())
}
```

See [Generator.md](Generator.md), [Color.md](Color.md) and
[SFSymbol.md](SFSymbol.md) for details.

## Changelog

- 2026-09-09: Halo-free swaps + glyph depth: `replace_background`
  decontaminates chromatic fringe over the new color (no light-blue halo
  around kept glyphs); new `DepthOptions::artwork_shadow` (file pipeline)
  casts the flood-mask foreground onto the background, wired into
  `default_app_icon_depth()` and `apple_liquid_glass()`.
- 2026-09-09: Fixed dark-mode white remap eating foreground glyphs:
  new `RecolorOptions::remap_max_fraction` with connected-component gating;
  `AppIcon` dark paths use `DARK_REMAP_MAX_FRACTION` (`0.10`) so small
  cutouts (VS Code triangle ~6.5%) follow the dark background while large
  white artwork (speech bubble ~17.9%) survives.
- 2026-09-09: Apple-strong Liquid Glass for app icons: new
  `DepthOptions` / `IconCanvas` effects `gloss`, `vibrancy` and `shade`;
  `APPLE_CORNER_RADIUS` (`232.0`) plus `apple_liquid_glass()` preset;
  `default_app_icon_depth()` and `.glass()` retuned to Apple levels (dual
  ambient + key shadow, top gloss + diagonal sheen, gradient edge stroke,
  bottom shade, AA corner mask); `DARK_BACKGROUND` aligned to TontooOS dark
  `#1d1d1d`.
- 2026-08-28: Added `Octopus` module (`use_octopus`, `OctopusVariant`, `OctopusIcon`) with `assets/TontooOS` branding PNGs and `Shaded` tint; added `OsVersion` module (`use_osversionicons`, `OsVersionIcon`) for `OSVersionAssets/<version>/<name>` (shipped `26.1.0`: `TontooOS_Icon.png`, `seal.png`, `ocean.jpg`) plus `available_versions`/`available_icons` discovery helpers.
- 2026-08-25: `Colorize` with `neutral_threshold(0)` now tints pure grays;
  added `ProcessOptions::protect_background` so Light+tint recolors monochrome
  artwork while only the flood-filled background is protected.

- 2026-08-25: Added the two high-level app icon APIs: `IconCanvas::png_to_3d_icon` (API 1) and the
  `AppIcon` builder with `dark()` / `tint(color)` options (API 2), plus `examples/app_icon_demo`.
- 2026-08-25: Background flood-fill hardened (border-reference check +
  anti-halo dilation); added `RecolorMode::Shaded` and
  `RecolorOptions::protect` / `protect_tolerance` so artwork interiors
  (e.g. a white wedge) recolor fully while swapped backgrounds stay intact.

- 2026-08-25: Added `TintMatrix` module (Apple-style 4x5 color matrices:
  solid, multiply, luma_tint, hue_rotate, saturate, brightness, contrast,
  invert, fade, composition) with `Layer.tint_matrix` support.
- 2026-08-25: Added unified image processing core: `process_file`,
  `process_image`, `recolor_image`, `set_background_color`, `RecolorOptions`
  (`Colorize` / `AccentLuma` / `Replace`, configurable neutral threshold) and
  the `DepthOptions` builder. Legacy entry points now delegate to it; recolor
  is fringe-free via straight-alpha rounding.
- 2026-08-25: Generator upgrades: `.glass()` Liquid Glass preset,
  `light_direction(x, y)` rim lighting for specular / inner depth / edge
  highlight, glyph-shaped distance-transform shadows for icons (`O(n)` instead
  of stamp blur), aspect-fit icon placement for non-square symbols.
- 2026-08-14: Wiki synced with current implementation (flood-fill background
  detection in `dark_light_mode`, replacement colors per `IconMode`, HSL
  colorization in `change_color`).
- 2026-08-14: Added `dark_light_mode` / `dark_light_mode_and_save` to Generator.
- 2026-08-14: Added `change_color` / `change_color_and_save` to Generator (tint + depth).
- 2026-08-14: Added `add_depth_to_image` / `add_depth_to_image_and_save` to Generator.
- 2026-08-12: Initial wiki created with 6 feature pages.
