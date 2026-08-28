//! TontooOS octopus branding icons.
//!
//! The assets live in `assets/TontooOS/` (copied from `TontooIconAssets`).
//! This module provides the `use_octopus` API: pick a PNG variant and a
//! `Color`, get a tinted `RgbaImage` back.

use crate::generator::{RecolorMode, RecolorOptions};
use crate::{Color, generator::IconCanvas};
use image::RgbaImage;
use std::path::{Path, PathBuf};

/// Base directory for the TontooOS branding assets (relative to crate root / runtime CWD).
pub const TONTOO_OS_ASSETS_DIR: &str = "assets/TontooOS";

/// All known octopus/logo variants shipped in `assets/TontooOS/`.
///
/// The filenames use mixed casing; this enum normalises them so callers do not
/// have to remember exact casing or the `tuerkis`/`türkís` spelling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum OctopusVariant {
    Black,
    DarkBlue,
    DefaultIco,
    Default,
    LightGreen,
    Purple,
    Turkis,
    White,
}

impl OctopusVariant {
    /// Canonical file name on disk for this variant.
    pub fn file_name(self) -> &'static str {
        match self {
            Self::Black => "Tontoo_Black.png",
            Self::DarkBlue => "tontoo_dark_blue.png",
            Self::DefaultIco => "tontoo_default.ico",
            Self::Default => "Tontoo_Default.png",
            Self::LightGreen => "tontoo_light_green.png",
            Self::Purple => "tontoo_purple.png",
            Self::Turkis => "tontoo_türkis.png",
            Self::White => "Tontoo_White.png",
        }
    }

    /// Human readable label (without extension).
    pub fn label(self) -> &'static str {
        match self {
            Self::Black => "Tontoo_Black",
            Self::DarkBlue => "tontoo_dark_blue",
            Self::DefaultIco => "tontoo_default",
            Self::Default => "Tontoo_Default",
            Self::LightGreen => "tontoo_light_green",
            Self::Purple => "tontoo_purple",
            Self::Turkis => "tontoo_türkis",
            Self::White => "Tontoo_White",
        }
    }

    /// Absolute (runtime-relative) path to the file.
    pub fn path(self) -> String {
        format!("{}/{}", TONTOO_OS_ASSETS_DIR, self.file_name())
    }

    /// Try to parse a file name or label (case-insensitive, with or without
    /// extension) into a variant. Accepts `tuerkis`, `turkis`, `türkís`, etc.
    pub fn from_name(name: &str) -> Option<Self> {
        let n = name.trim().to_ascii_lowercase();
        // strip extension
        let n = n.strip_suffix(".png").unwrap_or(&n);
        let n = n.strip_suffix(".ico").unwrap_or(n);
        match n {
            "tontoo_black" | "black" => Some(Self::Black),
            "tontoo_dark_blue" | "dark_blue" | "darkblue" => Some(Self::DarkBlue),
            "tontoo_default" | "default" => Some(Self::Default),
            "tontoo_default_ico" | "default_ico" => Some(Self::DefaultIco),
            "tontoo_light_green" | "light_green" | "lightgreen" => Some(Self::LightGreen),
            "tontoo_purple" | "purple" => Some(Self::Purple),
            "tontoo_turkis" | "tuerkis" | "türkís" | "turkis" | "tontoo_tuerkis" => Some(Self::Turkis),
            "tontoo_white" | "white" => Some(Self::White),
            _ => {
                // also try exact turkis with umlaut lowercased
                if n.contains("rkis") { return Some(Self::Turkis); }
                None
            }
        }
    }

    /// All variants in stable order.
    pub fn all() -> &'static [OctopusVariant] {
        &[
            Self::Black,
            Self::DarkBlue,
            Self::Default,
            Self::DefaultIco,
            Self::LightGreen,
            Self::Purple,
            Self::Turkis,
            Self::White,
        ]
    }
}

impl std::fmt::Display for OctopusVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.file_name())
    }
}

/// Builder for loading and tinting an octopus icon.
///
/// # Example
/// ```no_run
/// use coreicon::{Color, octopus::{OctopusIcon, OctopusVariant}};
///
/// let img = OctopusIcon::new(OctopusVariant::Default)
///     .tint(Color::from_hex("#FF6B2B").unwrap())
///     .load_tinted()
///     .unwrap();
/// // img.save("octopus-blue.png").unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct OctopusIcon {
    variant: OctopusVariant,
    tint: Option<Color>,
}

impl OctopusIcon {
    pub fn new(variant: OctopusVariant) -> Self {
        Self { variant, tint: None }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        OctopusVariant::from_name(name).map(Self::new)
    }

    pub fn tint(mut self, color: Color) -> Self {
        self.tint = Some(color);
        self
    }

    pub fn no_tint(mut self) -> Self {
        self.tint = None;
        self
    }

    pub fn variant(&self) -> OctopusVariant { self.variant }
    pub fn path(&self) -> String { self.variant.path() }
    pub fn file_name(&self) -> &'static str { self.variant.file_name() }

    /// Load the original image without any recoloring.
    pub fn load(&self) -> Result<RgbaImage, Box<dyn std::error::Error>> {
        let path = self.path();
        Ok(image::open(&path)?.to_rgba8())
    }

    /// Load and tint with the configured color. If no tint is set, the
    /// original image is returned unmodified.
    pub fn load_tinted(&self) -> Result<RgbaImage, Box<dyn std::error::Error>> {
        match self.tint {
            None => self.load(),
            Some(c) => load_tinted_path(&self.path(), c),
        }
    }

    /// Save the (optionally tinted) image to `out`.
    pub fn save(&self, out: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
        let img = self.load_tinted()?;
        img.save(out.as_ref())?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Free functions - the exact names requested in the task: `use_octopus`
// ---------------------------------------------------------------------------

/// Load an octopus PNG and tint it to `color`.
///
/// `file` may be any of the variants with or without extension and casing is
/// ignored, e.g. `"Tontoo_Default.png"`, `"tontoo_dark_blue"` or `"turkis"`.
///
/// The recolor uses `RecolorMode::Shaded` so the black outline and shading are
/// preserved (yellows map to the full tint, blacks stay black, midtones shade
/// proportionally).
///
/// Returns the tinted `RgbaImage`. To save directly, use
/// [`use_octopus_and_save`].
pub fn use_octopus(file: &str, color: Color) -> Result<RgbaImage, Box<dyn std::error::Error>> {
    let variant = OctopusVariant::from_name(file)
        .ok_or_else(|| format!("unknown octopus variant '{}'. Expected one of: {}", file, available_variants_hint()))?;
    use_octopus_variant(variant, color)
}

/// Typed variant of [`use_octopus`].
pub fn use_octopus_variant(variant: OctopusVariant, color: Color) -> Result<RgbaImage, Box<dyn std::error::Error>> {
    load_tinted_path(&variant.path(), color)
}

/// Load without tinting - original colors.
pub fn use_octopus_original(file: &str) -> Result<RgbaImage, Box<dyn std::error::Error>> {
    let variant = OctopusVariant::from_name(file)
        .ok_or_else(|| format!("unknown octopus variant '{}'", file))?;
    Ok(image::open(variant.path())?.to_rgba8())
}

/// Convenience: load, tint and save to `output`.
pub fn use_octopus_and_save(
    file: &str,
    color: Color,
    output: impl AsRef<Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let img = use_octopus(file, color)?;
    img.save(output.as_ref())?;
    Ok(())
}

/// List all available octopus file names.
pub fn available_variants() -> Vec<&'static str> {
    OctopusVariant::all().iter().map(|v| v.file_name()).collect()
}

fn available_variants_hint() -> String {
    OctopusVariant::all().iter().map(|v| v.file_name()).collect::<Vec<_>>().join(", ")
}

fn load_tinted_path(path: &str, color: Color) -> Result<RgbaImage, Box<dyn std::error::Error>> {
    let img = image::open(path)?.to_rgba8();
    // Shaded mode: full luminance-graded replacement. Preserves outlines.
    let opts = RecolorOptions::new(color, 1.0).mode(RecolorMode::Shaded);
    Ok(IconCanvas::recolor_image(&img, &opts))
}

// ---------------------------------------------------------------------------
// Helpers for runtime discovery (scans the assets folder on disk)
// ---------------------------------------------------------------------------

/// Scan `assets/TontooOS/` and return file names that are present on disk.
pub fn available_on_disk() -> Vec<String> {
    let dir = PathBuf::from(TONTOO_OS_ASSETS_DIR);
    let Ok(entries) = std::fs::read_dir(&dir) else { return Vec::new() };
    let mut out = Vec::new();
    for e in entries.flatten() {
        if let Some(name) = e.file_name().to_str() {
            if name.ends_with(".png") || name.ends_with(".ico") {
                out.push(name.to_owned());
            }
        }
    }
    out.sort();
    out
}
