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
| Icon Generator | [Generator.md](Generator.md) | `IconCanvas`, `Layer`, `Shadow`, `Background` PNG generation |

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

- 2026-08-12: Initial wiki created with 6 feature pages.
