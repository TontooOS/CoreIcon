# OsVersion

OS version specific assets from `assets/TontooOS/OSVersionAssets/<version>/`. This page documents the `use_osversionicons` API: give a version string and a file name, get a decoded `RgbaImage` back.

## Module

```rust
pub mod os_version;
```

Import with:

```rust
use coreicon::os_version::{use_osversionicons, OsVersionIcon, available_versions};
```

## Constants

| Symbol | Value | Description |
|---|---|---|
| `OS_VERSION_ASSETS_DIR` | `"assets/TontooOS/OSVersionAssets"` | Base directory for versioned assets |

## Path helper

```rust
pub fn os_version_path(version: &str, name: &str) -> String
```

Builds `"assets/TontooOS/OSVersionAssets/<version>/<name>"`. Both arguments are trimmed; `name` is expected to include its extension (e.g. `"seal.png"`, `"ocean.jpg"`).

## Discovery helpers

```rust
pub fn available_versions() -> Vec<String>
pub fn available_icons(version: &str) -> Vec<String>
```

| Function | Description |
|---|---|
| `available_versions` | List subdirectories under `OSVersionAssets` (e.g. `["26.1.0"]`), sorted |
| `available_icons` | List file names inside a given version folder, sorted; includes images and `list.txt` |

Returns an empty `Vec` when the base directory cannot be read.

## OsVersionIcon

Builder struct mirroring `OctopusIcon`.

```rust
pub struct OsVersionIcon {
    version: String,
    name: String,
}

impl OsVersionIcon {
    pub fn new(version: impl Into<String>, name: impl Into<String>) -> Self
    pub fn version(&self) -> &str
    pub fn name(&self) -> &str
    pub fn path(&self) -> String
    pub fn load(&self) -> Result<RgbaImage, Box<dyn std::error::Error>>
    pub fn save(&self, out: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>>
}
```

| Method | Description |
|---|---|
| `new` | Create for `version` folder + file `name` |
| `path` | Runtime-relative path via `os_version_path` |
| `load` | Decode with `image::open` to `RgbaImage`; works for PNG and JPG |
| `save` | `load` then save to `out` |

Returns `Err` when the file cannot be opened or decoded.

## Free functions

### `use_osversionicons`

```rust
pub fn use_osversionicons(version: &str, name: &str) -> Result<RgbaImage, Box<dyn std::error::Error>>
```

Load an asset. `version` is a folder under `OSVersionAssets` (e.g. `"26.1.0"`), `name` is the file inside it (e.g. `"TontooOS_Icon.png"`, `"seal.png"`, `"ocean.jpg"`). First tries the exact path; on miss falls back to a case-insensitive scan of the version folder. Returns `Err` with a `"not found"` message when neither matches.

### `use_os_version_icon`

```rust
pub fn use_os_version_icon(version: &str, name: &str) -> Result<RgbaImage, Box<dyn std::error::Error>>
```

Alias for `use_osversionicons` (underscore variant for ergonomics).

### `use_osversionicons_and_save`

```rust
pub fn use_osversionicons_and_save(version: &str, name: &str, output: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>>
```

Load via `use_osversionicons` and save to `output`.

## Asset layout

```
assets/TontooOS/
  Tontoo_Black.png
  tontoo_dark_blue.png
  ...
  OSVersionAssets/
    26.1.0/
      TontooOS_Icon.png
      seal.png
      ocean.jpg
      list.txt
```

`26.1.0` is currently the only shipped version. New versions appear as sibling folders (e.g. `27.0.0`). Callers should use `available_versions()` to enumerate rather than hardcoding.

## Usage / Example

```rust
use coreicon::os_version::{use_osversionicons, OsVersionIcon, available_versions, available_icons};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Free function
    let icon = use_osversionicons("26.1.0", "TontooOS_Icon.png")?;
    icon.save("tontoos-26-icon.png")?;

    // Case-insensitive fallback still works
    let seal = use_osversionicons("26.1.0", "SEAL.PNG")?;
    println!("seal {}x{}", seal.width(), seal.height());

    // Builder
    OsVersionIcon::new("26.1.0", "ocean.jpg").save("ocean-copy.jpg")?;

    // Discovery
    println!("versions: {:?}", available_versions());
    println!("26.1.0 files: {:?}", available_icons("26.1.0"));

    Ok(())
}
```

The function supports any `image` crate format (PNG, JPG, etc.). No tinting is applied; for icon processing (recolor, depth, 3D) pipe the result through `CoreIcon::generator::IconCanvas::process_image` or `AppIcon`.

## Cross References

- [Octopus.md](Octopus.md) - branding octopus icons under the same `assets/TontooOS/` tree
- [Generator.md](Generator.md) - `process_image`, `DepthOptions` for post-processing a loaded version icon
- [Color.md](Color.md) - color types if piping through generator recoloring
