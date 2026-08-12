# Gradient

Gradients let colors blend across a shape or symbol instead of a flat fill.
CoreIcon defines a `Gradient` made of `GradientStop` entries and a
`GradientDirection` that controls the blend axis. Gradients work for icon
canvases, layer fills, symbol tints and text.

## Types

### GradientStop

```rust
pub struct GradientStop {
    pub color: Color,
    pub position: f32,
}

impl GradientStop {
    pub const fn new(color: Color, position: f32) -> Self;
}
```

`position` is a normalized value `0.0` (start) to `1.0` (end).

### GradientDirection

```rust
pub enum GradientDirection {
    TopToBottom,
    BottomToTop,
    LeftToRight,
    RightToLeft,
    TopLeadingToBottomTrailing,
    TopTrailingToBottomLeading,
    CenterRadial,
}
```

The `Default` implementation returns `TopToBottom`. The generator currently
interpolates `TopToBottom`, `BottomToTop`, `LeftToRight` and `RightToLeft`;
the diagonal and radial variants fall back to `TopToBottom` sampling in the
raster generator.

### Gradient

```rust
pub struct Gradient {
    pub stops: Vec<GradientStop>,
    pub direction: GradientDirection,
}
```

`Gradient` derives `Clone` and `PartialEq` and can be serialized. It is not
`Copy` because it owns a `Vec`.

## Constructors

| Function | Signature | Description |
|---|---|---|
| `new` | `const fn new(direction, stops) -> Self` | Full control over stops + direction |
| `linear_two` | `fn linear_two(from: Color, to: Color) -> Self` | Two-stop vertical gradient |
| `linear_three` | `fn linear_three(c1, c2, c3) -> Self` | Three-stop vertical gradient (mid at `0.5`) |
| `with_direction` | `fn with_direction(self, d: GradientDirection) -> Self` | Builder to change direction |

## Usage

```rust
use CoreIcon::{Color, Gradient, GradientDirection, GradientStop};

let gold  = Color::from_hex("#FFD700").unwrap();
let orange = Color::from_hex("#FF6B2B").unwrap();

// Two stops, top-to-bottom
let g1 = Gradient::linear_two(orange, gold);

// Custom stops and direction
let g2 = Gradient::new(
    GradientDirection::LeftToRight,
    vec![
        GradientStop::new(Color::RED, 0.0),
        GradientStop::new(Color::YELLOW, 0.5),
        GradientStop::new(Color::GREEN, 1.0),
    ],
);

// Two-stop gradient, horizontal instead of vertical
let g3 = Gradient::linear_two(orange, gold).with_direction(GradientDirection::LeftToRight);
```

## Sampling behavior

When the raster generator samples a gradient at position `t`, it interpolates
linearly between the two stops surrounding `t`. Edge cases:

- No stops: samples as `Color::WHITE`.
- One stop: returns that stop's color for every `t`.
- `t` outside the stop range: clamped to `[0.0, 1.0]` and blended with the
  nearest pair.

## Cross References

- [Color.md](Color.md) – stops are composed of `Color`
- [Generator.md](Generator.md) – backgrounds and layer fills accept gradients
- [SFSymbolView.md](SFSymbolView.md) – `gradient()` tinting of symbols
