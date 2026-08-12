# SFSymbol

`SFSymbol` is a lightweight handle to one SF Symbol shipped with CoreIcon. The
registry is auto-generated: the `src/lib.rs` file is marked "Auto-generated - do
not edit manually" and contains one `pub const` per symbol plus a static `ALL`
array. Each symbol maps to a PNG file under the `assets/icons/` directory.

## Type

```rust
pub struct SFSymbol {
    name: &'static str,
}
```

The name is private; access it through `name()`. `SFSymbol` is `Copy` plus
`Eq`, `Hash`, `PartialOrd`, `Ord`, and can be serialized.

## Constants

Every SF Symbol ships as a constant named after the symbol, with dots replaced
by underscores and symbols starting with a digit prefixed by `_`. Examples:

| Constant | Asset file |
|---|---|
| `SFSymbol::HOUSE` | `house.png` |
| `SFSymbol::HOUSE_FILL` | `house.fill.png` |
| `SFSymbol::MESSAGE_FILL` | `message.fill.png` |
| `SFSymbol::HEART_FILL` | `heart.fill.png` |
| `SFSymbol::BOLT_FILL` | `bolt.fill.png` |
| `SFSymbol::CHECKMARK` | `checkmark.png` |
| `SFSymbol::STAR_FILL` | `star.fill.png` |
| `SFSymbol::HAND_THUMBSUP_FILL` | `hand.thumbsup.fill.png` |
| `SFSymbol::_0_CIRCLE` | `0.circle.png` |

## Methods

| Function | Signature | Description |
|---|---|---|
| `name` | `const fn name(self) -> &'static str` | The SF Symbol name, e.g. `"house.fill"` |
| `path` | `fn path(self) -> String` | `assets/icons/<name>.png` path |
| `from_name` | `fn from_name(name: &str) -> Option<Self>` | Look up by name |
| `all` | `fn all() -> &'static [SFSymbol]` | Full registry slice |
| `count` | `fn count() -> usize` | Number of registered symbols |
| `styled` | `fn styled(self) -> SFSymbolView` | Wrap into a styled view |

`SFSymbol` also implements `Display`, printing the raw symbol name.

### `from_name`

Performs a linear search over `ALL`. Returns `None` when the name is not
registered. The search is case-sensitive.

## Asset directory

The runtime asset directory is `ASSETS_DIR` (`assets/icons`) in the crate root
and an independent `generator::ASSETS_DIR` used by the renderer. Point the
generator's `ASSETS_DIR` at the actual assets folder at runtime before
rendering, otherwise icon layers are silently skipped.

```rust
use CoreIcon::SFSymbol;

let house = SFSymbol::HOUSE;
assert_eq!(house.name(), "house");

let found = SFSymbol::from_name("house.fill").unwrap();
assert_eq!(found, SFSymbol::HOUSE_FILL);

let total = SFSymbol::count();
let all   = SFSymbol::all();
```

## Cross References

- [SFSymbolView.md](SFSymbolView.md) – styled wrapper around a symbol
- [Generator.md](Generator.md) – `LayerContent::icon` renders a symbol
- [Color.md](Color.md) – tints applied to symbols
