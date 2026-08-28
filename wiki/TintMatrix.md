# TintMatrix

`TintMatrix` is a 4x5 color transformation matrix for recoloring icons without
creating new assets. Artwork can stay neutral while the color lives in 16
coefficients plus per-row offsets - one matrix replaces any number of recolored
asset variants. This mirrors Apple's IconRendering `RB::Layer` tint rows
(`tintMatrixRow0..tintMatrixRow3`).

## Module

```rust
pub mod tint;
```

The type is also re-exported at the crate root and in the prelude:

```rust
pub use tint::TintMatrix;
```

## Definition

Each output channel is the dot product of one matrix row with the input vector
`[r, g, b, a, 1]`, clamped to `0.0..=1.0`:

```text
out = M * [r, g, b, a, 1]^T
```

Rows `0..2` map RGB, row `3` maps alpha, the fifth column is an additive
offset. All components use straight (non-premultiplied) alpha in `0.0..=1.0`.

## API

```rust
pub struct TintMatrix(pub [[f32; 5]; 4]);

impl TintMatrix {
    pub fn identity() -> Self;
    pub const fn from_rows(rows: [[f32; 5]; 4]) -> Self;
    pub fn solid(color: Color) -> Self;
    pub fn multiply(color: Color) -> Self;
    pub fn luma_tint(accent: Color, keep_value: f32) -> Self;
    pub fn grayscale(strength: f32) -> Self;
    pub fn saturate(s: f32) -> Self;
    pub fn hue_rotate(degrees: f32) -> Self;
    pub fn brightness(factor: f32) -> Self;
    pub fn contrast(factor: f32) -> Self;
    pub fn invert(strength: f32) -> Self;
    pub fn fade(factor: f32) -> Self;
    pub fn then(&self, other: &TintMatrix) -> TintMatrix;
    pub fn apply(&self, r: f32, g: f32, b: f32, a: f32) -> (f32, f32, f32, f32);
    pub fn apply_image(&self, src: &RgbaImage) -> RgbaImage;
}

impl Default for TintMatrix {
    fn default() -> Self { Self::identity() }
}
```

| Constructor | Effect |
|---|---|
| `identity()` | Pass-through |
| `from_rows(rows)` | Raw rows `[r, g, b, a, offset]` |
| `solid(color)` | Replace RGB with a constant color, keep source alpha (template tint) |
| `multiply(color)` | Modulate: `out.rgb = src.rgb * color.rgb` |
| `luma_tint(accent, k)` | Accent tint preserving shading (see below) |
| `grayscale(strength)` | Desaturate, `0.0` = original, `1.0` = gray |
| `saturate(s)` | Scale saturation around luminance, `1.0` = unchanged |
| `hue_rotate(degrees)` | Rotate hue, preserves luminance and alpha |
| `brightness(factor)` | Multiply RGB, `1.0` = unchanged |
| `contrast(factor)` | Contrast pivoted at mid-gray, `1.0` = unchanged |
| `invert(strength)` | Invert RGB, `0.0` = original, `1.0` = fully inverted |
| `fade(factor)` | Scale the alpha channel |

### luma_tint

Implements Apple's dock-icon accent formula:

```text
out.rgb = accent.rgb * (k * luma + (1 - k) * value)
```

`keep_value` is the value/luma split; Apple uses `k = 0.8`. Shading of the
artwork is preserved while every colored pixel is pulled toward the accent hue.

> **Note:** pure black pixels stay black under `luma_tint` because their value
> and luma are both `0`. For black-on-transparent template glyphs use
> `solid(color)` instead.

### Composition and application

- `a.then(&b)` returns the combined matrix that applies `a` first, then `b`.
- `apply(...)` transforms one pixel's components.
- `apply_image(&RgbaImage)` transforms a whole image into a new buffer. Fully
  transparent pixels pass through untouched; all outputs are rounded, not
  truncated.

## Using matrices on generator layers

Every `Layer` accepts an optional matrix via `.tint_matrix(m)`. It runs after
`fill` / `gradient`, so it composes with both:

```rust
use CoreIcon::{BOLT_FILL, Color};

Layer::new(LayerContent::icon(BOLT_FILL))
    .position(112.0, 112.0)
    .size(800.0, 800.0)
    .tint_matrix(TintMatrix::luma_tint(Color::ACCENT, 0.8))
```

Shapes (`Rect`, `Circle`) and `Text` apply the matrix to their resolved fill
color as well.

## Usage / Example

```rust
use CoreIcon::prelude::*;

let accent = Color::from_hex("#FF6B2B").unwrap();
// Warm shift, then pull toward the accent hue:
let m = TintMatrix::hue_rotate(-15.0).then(&TintMatrix::luma_tint(accent, 0.8));
let recolored = m.apply_image(&some_rgba_image);
```

## Cross References

- [Generator.md](Generator.md) - `Layer.tint_matrix` and the image processing
  entry points built on this module
- [Color.md](Color.md) - the `Color` type used by all constructors
