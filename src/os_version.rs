//! OS version specific assets (`assets/TontooOS/OSVersionAssets/<version>/`).
//!
//! Provides the `use_osversionicons` API: give a version string (e.g. `"26.1.0"`)
//! and a file name (e.g. `"TontooOS_Icon.png"`, `"seal.png"`, `"ocean.jpg"`)
//! and get the decoded `RgbaImage` back.

use image::RgbaImage;
use std::path::{Path, PathBuf};

/// Base directory for versioned assets.
pub const OS_VERSION_ASSETS_DIR: &str = "assets/TontooOS/OSVersionAssets";

/// Resolve the versioned assets base dir at runtime.
///
/// Priority: `$COREICON_OS_VERSION_DIR` override, LiveOS sidecar
/// (`/Library/System/coreicon.resources/assets/TontooOS/OSVersionAssets`),
/// staged sources, then the relative crate dir.
pub fn resolve_os_version_base() -> PathBuf {
    let mut candidates = Vec::new();
    if let Ok(env) = std::env::var("COREICON_OS_VERSION_DIR") {
        let p = PathBuf::from(env);
        if !p.as_os_str().is_empty() {
            candidates.push(p);
        }
    }
    candidates.push(PathBuf::from(format!(
        "{}/assets/TontooOS/OSVersionAssets",
        crate::SYSTEM_RESOURCES_DIR
    )));
    candidates.push(PathBuf::from(format!(
        "{}/assets/TontooOS/OSVersionAssets",
        crate::SYSTEM_SOURCE_DIR
    )));
    candidates.push(PathBuf::from(OS_VERSION_ASSETS_DIR));
    for c in &candidates {
        if c.exists() {
            return c.clone();
        }
    }
    PathBuf::from(OS_VERSION_ASSETS_DIR)
}

/// Build the on-disk path for a versioned asset.
///
/// Example: `os_version_path("26.1.0", "seal.png")`
/// -> `"assets/TontooOS/OSVersionAssets/26.1.0/seal.png"`
/// (absolute under `/Library/System/...` on LiveOS when staged).
pub fn os_version_path(version: &str, name: &str) -> String {
    // Normalise `name`: allow with or without extension already.
    let rel = format!("{}/{}/{}", OS_VERSION_ASSETS_DIR, version.trim(), name.trim());
    let abs = resolve_os_version_base().join(version.trim()).join(name.trim());
    if abs.exists() {
        abs.to_string_lossy().into_owned()
    } else {
        rel
    }
}

/// List all version folders that exist under `OSVersionAssets`.
pub fn available_versions() -> Vec<String> {
    let base = resolve_os_version_base();
    let Ok(entries) = std::fs::read_dir(&base) else { return Vec::new() };
    let mut out = Vec::new();
    for e in entries.flatten() {
        if let Ok(ft) = e.file_type() {
            if ft.is_dir() {
                if let Some(n) = e.file_name().to_str() {
                    out.push(n.to_owned());
                }
            }
        }
    }
    out.sort();
    out
}

/// List asset file names for a given version (non-recursive, files only).
pub fn available_icons(version: &str) -> Vec<String> {
    let dir = resolve_os_version_base().join(version);
    let Ok(entries) = std::fs::read_dir(&dir) else { return Vec::new() };
    let mut out = Vec::new();
    for e in entries.flatten() {
        if let Ok(ft) = e.file_type() {
            if ft.is_file() {
                if let Some(n) = e.file_name().to_str() {
                    // skip metadata like list.txt if caller wants only images, but we expose everything
                    out.push(n.to_owned());
                }
            }
        }
    }
    out.sort();
    out
}

/// Builder for a versioned icon - mirrors `OctopusIcon` style.
#[derive(Debug, Clone)]
pub struct OsVersionIcon {
    version: String,
    name: String,
}

impl OsVersionIcon {
    pub fn new(version: impl Into<String>, name: impl Into<String>) -> Self {
        Self { version: version.into(), name: name.into() }
    }

    pub fn version(&self) -> &str { &self.version }
    pub fn name(&self) -> &str { &self.name }
    pub fn path(&self) -> String { os_version_path(&self.version, &self.name) }

    /// Load the image (supports PNG, JPG, etc. via `image::open`).
    pub fn load(&self) -> Result<RgbaImage, Box<dyn std::error::Error>> {
        // Resolve again at load time so a sidecar staged after path()
        // was called is still found.
        let abs = resolve_os_version_base().join(self.version.trim()).join(self.name.trim());
        let p = if abs.exists() {
            abs.to_string_lossy().into_owned()
        } else {
            self.path()
        };
        Ok(image::open(&p)?.to_rgba8())
    }

    pub fn save(&self, out: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
        let img = self.load()?;
        img.save(out.as_ref())?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Free functions - exact names requested: `use_osversionicons`
// ---------------------------------------------------------------------------

/// Load an OS version asset.
///
/// `version` is a folder under `OSVersionAssets` (e.g. `"26.1.0"`),
/// `name` is the file inside that folder (e.g. `"TontooOS_Icon.png"`,
/// `"seal.png"`, `"ocean.jpg"`). Extension is required and matched
/// case-insensitively on fallback.
///
/// Returns an `RgbaImage`. For a fallible path string, see [`os_version_path`].
pub fn use_osversionicons(version: &str, name: &str) -> Result<RgbaImage, Box<dyn std::error::Error>> {
    let p = os_version_path(version, name);
    if Path::new(&p).exists() {
        return Ok(image::open(&p)?.to_rgba8());
    }
    // case-insensitive fallback: scan directory for case-insensitive match
    let dir = resolve_os_version_base().join(version);
    if let Ok(entries) = std::fs::read_dir(&dir) {
        let target = name.to_ascii_lowercase();
        for e in entries.flatten() {
            if let Some(n) = e.file_name().to_str() {
                if n.to_ascii_lowercase() == target {
                    let found = dir.join(n);
                    return Ok(image::open(&found)?.to_rgba8());
                }
            }
        }
    }
    Err(format!("OS version asset not found: version='{}' name='{}' (tried '{}')", version, name, p).into())
}

/// Alias with underscore-free naming for ergonomics.
pub fn use_os_version_icon(version: &str, name: &str) -> Result<RgbaImage, Box<dyn std::error::Error>> {
    use_osversionicons(version, name)
}

/// Convenience: load and save to `output`.
pub fn use_osversionicons_and_save(
    version: &str,
    name: &str,
    output: impl AsRef<Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let img = use_osversionicons(version, name)?;
    img.save(output.as_ref())?;
    Ok(())
}
