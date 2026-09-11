# RuntimePaths

Asset lookup on a TontooOS system. The staged sidecar
(`/Library/System/coreicon.resources/`, see BaseOS `Frameworks.md`) is found
automatically; no manual `ASSETS_DIR` setup is needed on device.

## Resolution Order

Every resolver below uses the same priority:

1. Environment override (`COREICON_ASSETS_DIR`, `COREICON_TONTOO_OS_DIR`,
   `COREICON_OS_VERSION_DIR`)
2. LiveOS sidecar (`/Library/System/coreicon.resources/...`)
3. Staged sources (`/Library/System/coreicon/...`)
4. Relative crate dir (`assets/...`; dev / `cargo run`)

When nothing exists, the relative default is returned so error messages stay
familiar. A runtime-customised `ASSETS_DIR` still wins when the file exists
there.

## Functions

### `resolve_icon_dir`

```rust
pub fn resolve_icon_dir() -> PathBuf
```

Returns the SF Symbol PNG folder. Backs `SFSymbol::path` and the generator.

### `resolve_icon_path`

```rust
pub fn resolve_icon_path(name: &str) -> PathBuf
```

Returns the PNG file for a symbol name without extension.

```rust
let path = coreicon::resolve_icon_path("alarm");
// LiveOS: /Library/System/coreicon.resources/assets/icons/alarm.png
// dev:    assets/icons/alarm.png
```

### `resolve_octopus_dir` / `resolve_octopus_path`

```rust
pub fn resolve_octopus_dir() -> PathBuf
pub fn resolve_octopus_path(file_name: &str) -> PathBuf
```

Back `OctopusVariant::path` and `available_on_disk`.

### `resolve_os_version_base`

```rust
pub fn resolve_os_version_base() -> PathBuf
```

Backs `os_version_path`, `available_versions` and `available_icons`.

## Generator

`generator::icon_sprite` loads through `icon_file`: the runtime
`generator::ASSETS_DIR` first (when the file exists there), then
`crate::resolve_icon_path`. Setting `ASSETS_DIR` explicitly keeps working,
including the existing `unsafe { ASSETS_DIR = ... }` override.

## Cross References

- [SFSymbol.md](SFSymbol.md) – symbol registry and `SFSymbol::path`
- [Generator.md](Generator.md) – `IconCanvas` and `ASSETS_DIR`
- [Octopus.md](Octopus.md) – branding icons
- [OsVersion.md](OsVersion.md) – versioned assets
