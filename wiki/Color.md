# Color

`Color` is the universal color type used across CoreIcon for backgrounds, layer
fills, tints, shadows and gradients. It stores normalized floating point
components (`0.0`–`1.0`) and is `Copy`, so it can be passed by value everywhere.

## Type

```rust
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
```

All channels are normalized to the `0.0`–`1.0` range. The `a` channel is the
alpha (opacity) and defaults to `1.0` unless set otherwise.

## Constructors

| Function | Signature | Description |
|---|---|---|
| `new` | `const fn new(r: f32, g: f32, b: f32, a: f32) -> Self` | Raw normalized components |
| `from_rgb` | `const fn from_rgb(r: u8, g: u8, b: u8) -> Self` | 8-bit channels, alpha `1.0` |
| `from_rgba` | `const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self` | 8-bit channels plus alpha |
| `from_hex` | `fn from_hex(hex: &str) -> Option<Self>` | Hex string, optional `#` prefix |

### `from_hex`

Parses `#RRGGBB` (6 digits) or `#RRGGBBAA` (8 digits). The leading `#` is
optional. Returns `None` for any other length or when a pair is not valid hex.

```rust
let orange = Color::from_hex("#FF6B2B").unwrap();
let dark   = Color::from_hex("1d1d1d").unwrap();
let blue50 = Color::from_hex("#007AFF80").unwrap(); // alpha 0x80
```

## Presets

| Constant | Value |
|---|---|
| `Color::TRANSPARENT` | `(0,0,0,0)` |
| `Color::WHITE` | `(1,1,1,1)` |
| `Color::BLACK` | `(0,0,0,1)` |
| `Color::RED` | `(1,0,0,1)` |
| `Color::GREEN` | `(0,1,0,1)` |
| `Color::BLUE` | `(0,0,1,1)` |
| `Color::YELLOW` | `(1,1,0,1)` |
| `Color::CYAN` | `(0,1,1,1)` |
| `Color::ORANGE` | `(1,0.5,0,1)` |
| `Color::ACCENT` | `(0.047,0.522,0.937,1)` system blue accent |
| `Color::TONTOO_ACCENT` | `(1,0.42,0.17,1)` TontooOS orange |

## Methods

```rust
pub const fn with_alpha(self, a: f32) -> Self
pub const fn is_transparent(&self) -> bool
pub const fn is_opaque(&self) -> bool
```

- `with_alpha` replaces the alpha channel, keeping RGB.
- `is_transparent` returns `true` when `a <= 0.0`.
- `is_opaque` returns `true` when `a >= 1.0`.

`Color` derives `Debug`, `Clone`, `Copy`, `PartialEq`, `Serialize`,
`Deserialize`, so it can be stored, compared and serialized.

## Usage

```rust
use CoreIcon::Color;

let accent = Color::from_hex("#FF6B2B").unwrap();
let semi   = accent.with_alpha(0.6);
assert!(!semi.is_opaque());
assert!(semi.is_transparent() == false);
```

## Cross References

- [Gradient.md](Gradient.md) – gradient stops are made of `Color`
- [Generator.md](Generator.md) – backgrounds, layer tints and shadows take `Color`
- [SFSymbolView.md](SFSymbolView.md) – symbol tinting uses `Color`
