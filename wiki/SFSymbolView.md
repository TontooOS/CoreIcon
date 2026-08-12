# SFSymbolView

`SFSymbolView` wraps an `SFSymbol` with presentation properties: size, tint
mode and opacity. It is the type used when a symbol needs to appear styled in a
UI context rather than as raw PNG asset.

## Type

```rust
pub struct SFSymbolView {
    pub symbol: SFSymbol,
    pub width: f32,
    pub height: f32,
    pub tint: TintMode,
    pub opacity: f32,
}
```

The default view is 24x24, untinted (`TintMode::Original`) and fully opaque.

## TintMode

```rust
pub enum TintMode {
    Original,
    Tint(Color),
    Gradient(Gradient),
}
```

| Variant | Meaning |
|---|---|
| `Original` | Use the asset's original colors |
| `Tint(Color)` | Replace RGB with the tint color, using alpha as mask |
| `Gradient(Gradient)` | Apply a gradient tint sampled over the symbol height |

`Default` is `Original`. `TintMode` is `Clone` and `PartialEq`.

## Constructors

```rust
pub fn new(symbol: SFSymbol) -> Self
```

Creates a 24x24, `Original`, fully opaque view of the given symbol.

## Builder methods

All builders return `self` and can be chained.

| Function | Signature | Behavior |
|---|---|---|
| `size` | `size(w: f32, h: f32) -> Self` | Set both dimensions |
| `with_width` | `with_width(w: f32) -> Self` | Set width only |
| `with_height` | `with_height(h: f32) -> Self` | Set height only |
| `tint` | `tint(color: Color) -> Self` | Solid color tint |
| `gradient` | `gradient(gradient: Gradient) -> Self` | Gradient tint |
| `original` | `original() -> Self` | Reset to `Original` tint |
| `opacity` | `opacity(o: f32) -> Self` | Clamped to `[0.0, 1.0]` |
| `hidden` | `hidden() -> Self` | Shorthand for `opacity(0.0)` |
| `semi_transparent` | `semi_transparent() -> Self` | Shorthand for `opacity(0.5)` |
| `path` | `path(&self) -> String` | Asset path of the wrapped symbol |

## Usage

```rust
use CoreIcon::{Color, Gradient, SFSymbol, SFSymbolView};

let view = SFSymbolView::new(SFSymbol::HOUSE_FILL)
    .size(48.0, 48.0)
    .tint(Color::TONTOO_ACCENT)
    .semi_transparent();

let gradient_view = SFSymbolView::new(SFSymbol::STAR_FILL)
    .with_height(64.0)
    .gradient(Gradient::linear_two(Color::RED, Color::YELLOW));

let hidden = SFSymbolView::new(SFSymbol::AIRPLANE).hidden();
```

## Cross References

- [SFSymbol.md](SFSymbol.md) – the wrapped symbol type
- [Gradient.md](Gradient.md) – gradient tinting
- [Color.md](Color.md) – solid tint colors
- [Generator.md](Generator.md) – layer icons apply tint + gradient fills
