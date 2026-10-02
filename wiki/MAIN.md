# Tontoo CoreIcon – Wiki

Tontoo CoreIcon is a Rust library for making app icons and using SF Icons in UIs.
It ships a large set of SF Symbols as PNG assets plus a raster generator that
renders 1024x1024 icon PNGs with colors, gradients, transparency, shadows,
Liquid Glass post-processing and text.

- Repository: https://github.com/TontooOS/CoreIcon
- License: TCL
- Version: 27.0.0

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
| Tico | [Tico.md](Tico.md) | `.tico` icon container: ArchiveKit-based layer storage with high-res tinted rendering |
| Octopus | [Octopus.md](Octopus.md) | TontooOS octopus branding icons: `use_octopus` with PNG variant + `Color` tint |
| OsVersion | [OsVersion.md](OsVersion.md) | OS version assets: `use_osversionicons` with `version` + `name` under `OSVersionAssets/` |
| RuntimePaths | [RuntimePaths.md](RuntimePaths.md) | LiveOS asset lookup: sidecar-first resolvers for icons, branding and versioned assets |

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
use CoreIcon::HOUSE;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe { CoreIcon::generator::ASSETS_DIR = "assets/icons"; }

    let icon = IconCanvas::new()
        .background(Background::color(Color::from_hex("#1d1d1d").unwrap()))
        .layer(
            Layer::new(LayerContent::icon(HOUSE))
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

- 2026-10-02: App-icon look overhaul, driven by a comparison against Apple
  reference renders. New `DepthOptions::squircle_exponent` /
  `IconCanvas::squircle_exponent` plus `APPLE_SQUIRCLE_EXPONENT` (`5.0`): the
  corner is now a real superellipse with continuous curvature instead of a
  circular arc, and every depth band (specular, gloss, inner depth, edge
  stroke, corner mask) follows the new signed distance field. `n = 2.0`
  reproduces the old circular silhouette exactly. New
  `DepthOptions::artwork_emboss` / `IconCanvas::artwork_emboss` (`0.0` off,
  `0.60` in the Apple preset): a raised-relief bevel on the artwork silhouette
  driven by a signed distance field, so glyphs read as extruded from the tile
  rather than printed on it. Speckle removed at its source: new source
  despeckle (`5x5` conditional median, `GRAIN_CEILING` `0.30`) and an upscale
  prefilter (binomial, half the scale factor) both run at the **source**
  resolution, because a sharp upscale spreads one-pixel grain and staircase
  aliasing into ringing blobs that vibrancy then amplifies; the artwork
  silhouette is also morphologically smoothed and disc-rounded before the drop
  shadow, and the shadow's distance field is box-smoothed to drop chamfer
  banding. The narrow diagonal gloss sheen is replaced by one broad
  off-center reflection lobe. `AppIcon` Light + `.tint()` now recolors the whole
  tile instead of protecting the background, `Shaded` became a three-tone ramp
  with a hue-preserving shadow floor and a highlight roll-off, and
  `recolor_pixels` un-premultiplies before recoloring to kill tinted halos.
  Background replacement only decontaminates semi-transparent texels, so opaque
  edge pixels are no longer pulled toward unrelated neighbors. Apple preset
  retuned: shadow `24/48/0.38` -> `30/58/0.42`, artwork shadow `18/30/0.30` ->
  `22/34/0.34`, inner depth `52/0.38` -> `64/0.42`, specular `0.50` -> `0.55`,
  gloss `0.24` -> `0.20`, vibrancy `0.22` -> `0.24`, shade `0.20` -> `0.24`.
  The previously failing `tests/zz_xcode_check` now passes.
  See [AppIcon.md](AppIcon.md), [Generator.md](Generator.md).

- 2026-10-02: New `examples/tico_from_png` converts a flat app-icon
  PNG into a valid layered `.tico`: the artwork becomes one
  non-recolorable image layer over a transparent background, then
  exports, reloads and renders previews. Written for the new format, where
  ArchiveKit rejects a container with an empty layer table (a bare
  `Background::image` canvas, which is what TBuild used to build, no
  longer validates). See [Tico.md](Tico.md).

- 2026-09-29: `.tico` moved from ZIP to the ArchiveKit TICO container (same
  indexed engine as `.app`, own `TICO`/`TICF` magic, `manifest.fico`
  FishFile manifest, `layer/*.tlyr` entries): new `Tico::export_bytes` /
  `Tico::load_bytes`, `TicoError::Zip`/`Json` replaced by
  `TicoError::Container`, `zip`/`serde_json` dependencies removed in favor
  of `archivekit`; `examples/tico_demo/demo.tico` regenerated (2 layers);
  container roundtrips covered by `tests/tico_container.rs`.
  See [Tico.md](Tico.md).
- 2026-09-28: Raster backend moved from the third-party `image` crate to CoreImage:
  file loading (`TiImage::load`), PNG layer coding (`codecs::png`), Lanczos scaling
  (`resize` / aspect-fit / fill-crop) and saving (`TiImage::save`, format from file
  extension) in `generator`, `tico`, `tint`, `octopus` and `os_version` go through a
  new crate-internal `src/img.rs` helper module. The `image` dependency is removed;
  `Rgba` / `RgbaImage` now come from `coreimage` (same buffer type, zero-copy via
  `TiImage::from_rgba` / `into_rgba`). Callers save raw buffers with
  `coreimage::TiImage::from_rgba(buf).save(path, format, quality)`. Fixed doctests
  (`coreicon::` paths, `Result` mains) and the `SFSymbol::HOUSE` examples (symbol
  constants live at the crate root, e.g. `coreicon::HOUSE_FILL`).
  See [Generator.md](Generator.md), [Tico.md](Tico.md), [Octopus.md](Octopus.md).
- 2026-09-25: New `Layer.shaded` flag (default `false`) with `.shaded(b)`
  builder: `shaded: true` tints `Icon`/`Image` layers with the source
  brightness as mask (white maps to the full tint, darker pixels shade toward
  black) so authored gradients in recolorable artwork survive; `false` keeps
  the flat alpha-mask replace used by black SF Symbol masks. Covered by
  `tests/shaded_tint.rs`. See [Generator.md](Generator.md).
- 2026-09-24: Added TICO (`.tico` icon container): plain ZIP named `*.tico`
  with `manifest.json` + `layer/NN.tlyr` custom layer files (no preview, no
  PNG files); `Tico::export` rasterizes `IconCanvas` layers at 1024px,
  `Tico::load` + `TicoIcon::render(size, tint)` re-renders high-res icons in
  any color with the Apple finish. Demo: 2 layers in 6.5KB.
  See [Tico.md](Tico.md).
- 2026-09-24: AppIcon tint + background-mask overhaul: Light+tint now uses
  luminance-graded `Shaded` (uniform hue, overlaps stay darker) instead of
  `Colorize` (which washed yellows pale and greens dark); Dark uses the
  standard flood swap with a brightness-seed fallback for gray-on-gray gears;
  flood is alpha-aware (inset opaque palette, pass-through transparent ring,
  feathered texels always protected) with reference `0.25` / chain `0.14`;
  corner mask is pixel-correct 1px (no more translucent edge ring); Dark
  white-remap tolerance `0.25` absorbs hole fringe; border palette uses
  corner squares only (up to 6 entries) so edge-touching artwork never
  poisons it. Verified on the Photos
  flower: uniform tints, dark mode keeps colors, no fringe.
- 2026-09-11: LiveOS runtime paths: new resolvers `resolve_icon_dir`,
  `resolve_icon_path`, `octopus::resolve_octopus_dir`,
  `octopus::resolve_octopus_path` and `os_version::resolve_os_version_base`
  (env override, `/Library/System/coreicon.resources/` sidecar, staged
  sources, relative dev dir); `SFSymbol::path`, `OctopusVariant::path`,
  `os_version_path` and the generator sprite loader use them, so SF Symbols
  and branding load on LiveOS without manual `ASSETS_DIR` setup.
  See [RuntimePaths.md](RuntimePaths.md).
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
- 2026-08-28: Added `Octopus` module (`use_octopus`, `OctopusVariant`, `OctopusIcon`) with `assets/TontooOS` branding PNGs and `Shaded` tint; added `OsVersion` module (`use_osversionicons`, `OsVersionIcon`) for `OSVersionAssets/<version>/<name>` (shipped `27.0.0`: `TontooOS_Icon.png`, `seal.png`, `ocean.jpg`) plus `available_versions`/`available_icons` discovery helpers.
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
