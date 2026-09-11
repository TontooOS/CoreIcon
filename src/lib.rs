// Auto-generated - do not edit manually.
// Contains all SF Symbols for TontooOS with Color, Gradient & Transparency.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub const ASSETS_DIR: &str = "assets/icons";

/// Sidecar resources of this library on a TontooOS system.
/// `stage-frameworks.sh` copies `assets/`, `lang/`, ... next to the
/// `coreicon.library` file into `/Library/System/coreicon.resources/`.
pub const SYSTEM_RESOURCES_DIR: &str = "/Library/System/coreicon.resources";
/// Legacy fallback: staged crate sources at `/Library/System/coreicon/`.
pub const SYSTEM_SOURCE_DIR: &str = "/Library/System/coreicon";

fn probe_first(candidates: &[PathBuf]) -> Option<PathBuf> {
    for c in candidates {
        if c.exists() {
            return Some(c.clone());
        }
    }
    None
}

fn candidate_icon_dirs() -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Ok(env) = std::env::var("COREICON_ASSETS_DIR") {
        let p = PathBuf::from(env);
        if !p.as_os_str().is_empty() {
            out.push(p);
        }
    }
    out.push(PathBuf::from(format!("{}/assets/icons", SYSTEM_RESOURCES_DIR)));
    out.push(PathBuf::from(format!("{}/assets/icons", SYSTEM_SOURCE_DIR)));
    out.push(PathBuf::from(ASSETS_DIR));
    out
}

/// Resolve the SF Symbol PNG folder at runtime.
///
/// Priority: `$COREICON_ASSETS_DIR` override, LiveOS sidecar
/// (`/Library/System/coreicon.resources/assets/icons`), staged sources
/// (`/Library/System/coreicon/assets/icons`), then the relative crate dir
/// (`assets/icons`, dev / `cargo run`). Falls back to the relative dir when
/// nothing exists so error messages stay familiar.
pub fn resolve_icon_dir() -> PathBuf {
    probe_first(&candidate_icon_dirs()).unwrap_or_else(|| PathBuf::from(ASSETS_DIR))
}

/// Resolve the PNG file for an SF Symbol name (without extension).
pub fn resolve_icon_path(name: &str) -> PathBuf {
    // A runtime-customised `ASSETS_DIR` wins when the file exists there.
    let custom = PathBuf::from(ASSETS_DIR).join(format!("{}.png", name));
    if custom.exists() {
        return custom;
    }
    resolve_icon_dir().join(format!("{}.png", name))
}

// Color

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color { pub r: f32, pub g: f32, pub b: f32, pub a: f32 }

impl Color {
    pub const TRANSPARENT: Self = Self { r:0.0, g:0.0, b:0.0, a:0.0 };
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self { Self { r, g, b, a } }
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self { Self { r: r as f32/255.0, g: g as f32/255.0, b: b as f32/255.0, a: 1.0 } }
    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self { Self { r: r as f32/255.0, g: g as f32/255.0, b: b as f32/255.0, a: a as f32/255.0 } }
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        match hex.len() {
            6 => { let r = u8::from_str_radix(&hex[0..2],16).ok()?; let g = u8::from_str_radix(&hex[2..4],16).ok()?; let b = u8::from_str_radix(&hex[4..6],16).ok()?; Some(Self::from_rgb(r,g,b)) }
            8 => { let r = u8::from_str_radix(&hex[0..2],16).ok()?; let g = u8::from_str_radix(&hex[2..4],16).ok()?; let b = u8::from_str_radix(&hex[4..6],16).ok()?; let a = u8::from_str_radix(&hex[6..8],16).ok()?; Some(Self::from_rgba(r,g,b,a)) }
            _ => None,
        }
    }
    pub const fn with_alpha(self, a: f32) -> Self { Self { a, ..self } }
    pub const fn is_transparent(&self) -> bool { self.a <= 0.0 }
    pub const fn is_opaque(&self) -> bool { self.a >= 1.0 }
}

impl Color {
    pub const WHITE: Self = Self::new(1.0,1.0,1.0,1.0);
    pub const BLACK: Self = Self::new(0.0,0.0,0.0,1.0);
    pub const RED: Self = Self::new(1.0,0.0,0.0,1.0);
    pub const GREEN: Self = Self::new(0.0,1.0,0.0,1.0);
    pub const BLUE: Self = Self::new(0.0,0.0,1.0,1.0);
    pub const YELLOW: Self = Self::new(1.0,1.0,0.0,1.0);
    pub const CYAN: Self = Self::new(0.0,1.0,1.0,1.0);
    pub const ORANGE: Self = Self::new(1.0,0.5,0.0,1.0);
    pub const ACCENT: Self = Self::new(0.047,0.522,0.937,1.0);
    pub const TONTOO_ACCENT: Self = Self::new(1.0,0.42,0.17,1.0);
}

// Gradient

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GradientStop { pub color: Color, pub position: f32 }
impl GradientStop { pub const fn new(color: Color, position: f32) -> Self { Self { color, position } } }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GradientDirection { TopToBottom, BottomToTop, LeftToRight, RightToLeft, TopLeadingToBottomTrailing, TopTrailingToBottomLeading, CenterRadial }
impl Default for GradientDirection { fn default() -> Self { Self::TopToBottom } }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gradient { pub stops: Vec<GradientStop>, pub direction: GradientDirection }
impl Gradient {
    pub const fn new(direction: GradientDirection, stops: Vec<GradientStop>) -> Self { Self { stops, direction } }
    pub fn linear_two(from: Color, to: Color) -> Self { Self { stops: vec![GradientStop::new(from,0.0),GradientStop::new(to,1.0)], direction: GradientDirection::TopToBottom } }
    pub fn linear_three(c1: Color, c2: Color, c3: Color) -> Self { Self { stops: vec![GradientStop::new(c1,0.0),GradientStop::new(c2,0.5),GradientStop::new(c3,1.0)], direction: GradientDirection::TopToBottom } }
    pub fn with_direction(mut self, d: GradientDirection) -> Self { self.direction = d; self }
}

// TintMode

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TintMode { Original, Tint(Color), Gradient(Gradient) }
impl Default for TintMode { fn default() -> Self { Self::Original } }

// SFSymbol

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SFSymbol { name: &'static str }

impl SFSymbol {
    pub const fn name(self) -> &'static str { self.name }
    pub fn path(self) -> String { resolve_icon_path(self.name).to_string_lossy().into_owned() }
    pub fn from_name(name: &str) -> Option<Self> { ALL.iter().find(|s| s.name == name).copied() }
    pub fn all() -> &'static [SFSymbol] { &ALL }
    pub fn count() -> usize { ALL.len() }
    pub fn styled(self) -> SFSymbolView { SFSymbolView::new(self) }
}

impl std::fmt::Display for SFSymbol { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.name) } }

// SFSymbolView

#[derive(Debug, Clone, PartialEq)]
pub struct SFSymbolView {
    pub symbol: SFSymbol,
    pub width: f32,
    pub height: f32,
    pub tint: TintMode,
    pub opacity: f32,
}

impl SFSymbolView {
    pub fn new(symbol: SFSymbol) -> Self { Self { symbol, width: 24.0, height: 24.0, tint: TintMode::Original, opacity: 1.0 } }
    pub fn size(mut self, w: f32, h: f32) -> Self { self.width = w; self.height = h; self }
    pub fn with_width(mut self, w: f32) -> Self { self.width = w; self }
    pub fn with_height(mut self, h: f32) -> Self { self.height = h; self }
    pub fn tint(mut self, color: Color) -> Self { self.tint = TintMode::Tint(color); self }
    pub fn gradient(mut self, gradient: Gradient) -> Self { self.tint = TintMode::Gradient(gradient); self }
    pub fn original(mut self) -> Self { self.tint = TintMode::Original; self }
    pub fn opacity(mut self, o: f32) -> Self { self.opacity = o.clamp(0.0, 1.0); self }
    pub fn hidden(self) -> Self { self.opacity(0.0) }
    pub fn semi_transparent(self) -> Self { self.opacity(0.5) }
    pub fn path(&self) -> String { self.symbol.path() }
}

// Prelude
pub mod prelude { pub use crate::{SFSymbol, SFSymbolView, Color, Gradient, GradientStop, GradientDirection, TintMode, TintMatrix, ASSETS_DIR}; }

// Icon generator
pub mod generator;

// Color matrix recoloring
pub mod tint;
pub use tint::TintMatrix;

// TontooOS octopus branding icons
pub mod octopus;

// OS version specific assets
pub mod os_version;

// Generated symbol constants.
pub const _0_CIRCLE: SFSymbol = SFSymbol { name: "0.circle" };
pub const _0_CIRCLE_FILL: SFSymbol = SFSymbol { name: "0.circle.fill" };
pub const _0_SQUARE: SFSymbol = SFSymbol { name: "0.square" };
pub const _0_SQUARE_FILL: SFSymbol = SFSymbol { name: "0.square.fill" };
pub const _00_CIRCLE: SFSymbol = SFSymbol { name: "00.circle" };
pub const _00_CIRCLE_FILL: SFSymbol = SFSymbol { name: "00.circle.fill" };
pub const _00_SQUARE: SFSymbol = SFSymbol { name: "00.square" };
pub const _00_SQUARE_FILL: SFSymbol = SFSymbol { name: "00.square.fill" };
pub const _01_CIRCLE: SFSymbol = SFSymbol { name: "01.circle" };
pub const _01_CIRCLE_FILL: SFSymbol = SFSymbol { name: "01.circle.fill" };
pub const _01_SQUARE: SFSymbol = SFSymbol { name: "01.square" };
pub const _01_SQUARE_FILL: SFSymbol = SFSymbol { name: "01.square.fill" };
pub const _02_CIRCLE: SFSymbol = SFSymbol { name: "02.circle" };
pub const _02_CIRCLE_FILL: SFSymbol = SFSymbol { name: "02.circle.fill" };
pub const _02_SQUARE: SFSymbol = SFSymbol { name: "02.square" };
pub const _02_SQUARE_FILL: SFSymbol = SFSymbol { name: "02.square.fill" };
pub const _03_CIRCLE: SFSymbol = SFSymbol { name: "03.circle" };
pub const _03_CIRCLE_FILL: SFSymbol = SFSymbol { name: "03.circle.fill" };
pub const _03_SQUARE: SFSymbol = SFSymbol { name: "03.square" };
pub const _03_SQUARE_FILL: SFSymbol = SFSymbol { name: "03.square.fill" };
pub const _04_CIRCLE: SFSymbol = SFSymbol { name: "04.circle" };
pub const _04_CIRCLE_FILL: SFSymbol = SFSymbol { name: "04.circle.fill" };
pub const _04_SQUARE: SFSymbol = SFSymbol { name: "04.square" };
pub const _04_SQUARE_FILL: SFSymbol = SFSymbol { name: "04.square.fill" };
pub const _05_CIRCLE: SFSymbol = SFSymbol { name: "05.circle" };
pub const _05_CIRCLE_FILL: SFSymbol = SFSymbol { name: "05.circle.fill" };
pub const _05_SQUARE: SFSymbol = SFSymbol { name: "05.square" };
pub const _05_SQUARE_FILL: SFSymbol = SFSymbol { name: "05.square.fill" };
pub const _06_CIRCLE: SFSymbol = SFSymbol { name: "06.circle" };
pub const _06_CIRCLE_FILL: SFSymbol = SFSymbol { name: "06.circle.fill" };
pub const _06_SQUARE: SFSymbol = SFSymbol { name: "06.square" };
pub const _06_SQUARE_FILL: SFSymbol = SFSymbol { name: "06.square.fill" };
pub const _07_CIRCLE: SFSymbol = SFSymbol { name: "07.circle" };
pub const _07_CIRCLE_FILL: SFSymbol = SFSymbol { name: "07.circle.fill" };
pub const _07_SQUARE: SFSymbol = SFSymbol { name: "07.square" };
pub const _07_SQUARE_FILL: SFSymbol = SFSymbol { name: "07.square.fill" };
pub const _08_CIRCLE: SFSymbol = SFSymbol { name: "08.circle" };
pub const _08_CIRCLE_FILL: SFSymbol = SFSymbol { name: "08.circle.fill" };
pub const _08_SQUARE: SFSymbol = SFSymbol { name: "08.square" };
pub const _08_SQUARE_FILL: SFSymbol = SFSymbol { name: "08.square.fill" };
pub const _09_CIRCLE: SFSymbol = SFSymbol { name: "09.circle" };
pub const _09_CIRCLE_FILL: SFSymbol = SFSymbol { name: "09.circle.fill" };
pub const _09_SQUARE: SFSymbol = SFSymbol { name: "09.square" };
pub const _09_SQUARE_FILL: SFSymbol = SFSymbol { name: "09.square.fill" };
pub const _1_CIRCLE: SFSymbol = SFSymbol { name: "1.circle" };
pub const _1_CIRCLE_FILL: SFSymbol = SFSymbol { name: "1.circle.fill" };
pub const _1_MAGNIFYINGGLASS: SFSymbol = SFSymbol { name: "1.magnifyingglass" };
pub const _1_SQUARE: SFSymbol = SFSymbol { name: "1.square" };
pub const _1_SQUARE_FILL: SFSymbol = SFSymbol { name: "1.square.fill" };
pub const _10_CIRCLE: SFSymbol = SFSymbol { name: "10.circle" };
pub const _10_CIRCLE_FILL: SFSymbol = SFSymbol { name: "10.circle.fill" };
pub const _10_SQUARE: SFSymbol = SFSymbol { name: "10.square" };
pub const _10_SQUARE_FILL: SFSymbol = SFSymbol { name: "10.square.fill" };
pub const _11_CIRCLE: SFSymbol = SFSymbol { name: "11.circle" };
pub const _11_CIRCLE_FILL: SFSymbol = SFSymbol { name: "11.circle.fill" };
pub const _11_SQUARE: SFSymbol = SFSymbol { name: "11.square" };
pub const _11_SQUARE_FILL: SFSymbol = SFSymbol { name: "11.square.fill" };
pub const _12_CIRCLE: SFSymbol = SFSymbol { name: "12.circle" };
pub const _12_CIRCLE_FILL: SFSymbol = SFSymbol { name: "12.circle.fill" };
pub const _12_SQUARE: SFSymbol = SFSymbol { name: "12.square" };
pub const _12_SQUARE_FILL: SFSymbol = SFSymbol { name: "12.square.fill" };
pub const _123_RECTANGLE: SFSymbol = SFSymbol { name: "123.rectangle" };
pub const _123_RECTANGLE_FILL: SFSymbol = SFSymbol { name: "123.rectangle.fill" };
pub const _13_CIRCLE: SFSymbol = SFSymbol { name: "13.circle" };
pub const _13_CIRCLE_FILL: SFSymbol = SFSymbol { name: "13.circle.fill" };
pub const _13_SQUARE: SFSymbol = SFSymbol { name: "13.square" };
pub const _13_SQUARE_FILL: SFSymbol = SFSymbol { name: "13.square.fill" };
pub const _14_CIRCLE: SFSymbol = SFSymbol { name: "14.circle" };
pub const _14_CIRCLE_FILL: SFSymbol = SFSymbol { name: "14.circle.fill" };
pub const _14_SQUARE: SFSymbol = SFSymbol { name: "14.square" };
pub const _14_SQUARE_FILL: SFSymbol = SFSymbol { name: "14.square.fill" };
pub const _15_CIRCLE: SFSymbol = SFSymbol { name: "15.circle" };
pub const _15_CIRCLE_FILL: SFSymbol = SFSymbol { name: "15.circle.fill" };
pub const _15_SQUARE: SFSymbol = SFSymbol { name: "15.square" };
pub const _15_SQUARE_FILL: SFSymbol = SFSymbol { name: "15.square.fill" };
pub const _16_CIRCLE: SFSymbol = SFSymbol { name: "16.circle" };
pub const _16_CIRCLE_FILL: SFSymbol = SFSymbol { name: "16.circle.fill" };
pub const _16_SQUARE: SFSymbol = SFSymbol { name: "16.square" };
pub const _16_SQUARE_FILL: SFSymbol = SFSymbol { name: "16.square.fill" };
pub const _17_CIRCLE: SFSymbol = SFSymbol { name: "17.circle" };
pub const _17_CIRCLE_FILL: SFSymbol = SFSymbol { name: "17.circle.fill" };
pub const _17_SQUARE: SFSymbol = SFSymbol { name: "17.square" };
pub const _17_SQUARE_FILL: SFSymbol = SFSymbol { name: "17.square.fill" };
pub const _18_CIRCLE: SFSymbol = SFSymbol { name: "18.circle" };
pub const _18_CIRCLE_FILL: SFSymbol = SFSymbol { name: "18.circle.fill" };
pub const _18_SQUARE: SFSymbol = SFSymbol { name: "18.square" };
pub const _18_SQUARE_FILL: SFSymbol = SFSymbol { name: "18.square.fill" };
pub const _19_CIRCLE: SFSymbol = SFSymbol { name: "19.circle" };
pub const _19_CIRCLE_FILL: SFSymbol = SFSymbol { name: "19.circle.fill" };
pub const _19_SQUARE: SFSymbol = SFSymbol { name: "19.square" };
pub const _19_SQUARE_FILL: SFSymbol = SFSymbol { name: "19.square.fill" };
pub const _2_CIRCLE: SFSymbol = SFSymbol { name: "2.circle" };
pub const _2_CIRCLE_FILL: SFSymbol = SFSymbol { name: "2.circle.fill" };
pub const _2_SQUARE: SFSymbol = SFSymbol { name: "2.square" };
pub const _2_SQUARE_FILL: SFSymbol = SFSymbol { name: "2.square.fill" };
pub const _20_CIRCLE: SFSymbol = SFSymbol { name: "20.circle" };
pub const _20_CIRCLE_FILL: SFSymbol = SFSymbol { name: "20.circle.fill" };
pub const _20_SQUARE: SFSymbol = SFSymbol { name: "20.square" };
pub const _20_SQUARE_FILL: SFSymbol = SFSymbol { name: "20.square.fill" };
pub const _21_CIRCLE: SFSymbol = SFSymbol { name: "21.circle" };
pub const _21_CIRCLE_FILL: SFSymbol = SFSymbol { name: "21.circle.fill" };
pub const _21_SQUARE: SFSymbol = SFSymbol { name: "21.square" };
pub const _21_SQUARE_FILL: SFSymbol = SFSymbol { name: "21.square.fill" };
pub const _22_CIRCLE: SFSymbol = SFSymbol { name: "22.circle" };
pub const _22_CIRCLE_FILL: SFSymbol = SFSymbol { name: "22.circle.fill" };
pub const _22_SQUARE: SFSymbol = SFSymbol { name: "22.square" };
pub const _22_SQUARE_FILL: SFSymbol = SFSymbol { name: "22.square.fill" };
pub const _23_CIRCLE: SFSymbol = SFSymbol { name: "23.circle" };
pub const _23_CIRCLE_FILL: SFSymbol = SFSymbol { name: "23.circle.fill" };
pub const _23_SQUARE: SFSymbol = SFSymbol { name: "23.square" };
pub const _23_SQUARE_FILL: SFSymbol = SFSymbol { name: "23.square.fill" };
pub const _24_CIRCLE: SFSymbol = SFSymbol { name: "24.circle" };
pub const _24_CIRCLE_FILL: SFSymbol = SFSymbol { name: "24.circle.fill" };
pub const _24_SQUARE: SFSymbol = SFSymbol { name: "24.square" };
pub const _24_SQUARE_FILL: SFSymbol = SFSymbol { name: "24.square.fill" };
pub const _25_CIRCLE: SFSymbol = SFSymbol { name: "25.circle" };
pub const _25_CIRCLE_FILL: SFSymbol = SFSymbol { name: "25.circle.fill" };
pub const _25_SQUARE: SFSymbol = SFSymbol { name: "25.square" };
pub const _25_SQUARE_FILL: SFSymbol = SFSymbol { name: "25.square.fill" };
pub const _26_CIRCLE: SFSymbol = SFSymbol { name: "26.circle" };
pub const _26_CIRCLE_FILL: SFSymbol = SFSymbol { name: "26.circle.fill" };
pub const _26_SQUARE: SFSymbol = SFSymbol { name: "26.square" };
pub const _26_SQUARE_FILL: SFSymbol = SFSymbol { name: "26.square.fill" };
pub const _27_CIRCLE: SFSymbol = SFSymbol { name: "27.circle" };
pub const _27_CIRCLE_FILL: SFSymbol = SFSymbol { name: "27.circle.fill" };
pub const _27_SQUARE: SFSymbol = SFSymbol { name: "27.square" };
pub const _27_SQUARE_FILL: SFSymbol = SFSymbol { name: "27.square.fill" };
pub const _28_CIRCLE: SFSymbol = SFSymbol { name: "28.circle" };
pub const _28_CIRCLE_FILL: SFSymbol = SFSymbol { name: "28.circle.fill" };
pub const _28_SQUARE: SFSymbol = SFSymbol { name: "28.square" };
pub const _28_SQUARE_FILL: SFSymbol = SFSymbol { name: "28.square.fill" };
pub const _29_CIRCLE: SFSymbol = SFSymbol { name: "29.circle" };
pub const _29_CIRCLE_FILL: SFSymbol = SFSymbol { name: "29.circle.fill" };
pub const _29_SQUARE: SFSymbol = SFSymbol { name: "29.square" };
pub const _29_SQUARE_FILL: SFSymbol = SFSymbol { name: "29.square.fill" };
pub const _3_CIRCLE: SFSymbol = SFSymbol { name: "3.circle" };
pub const _3_CIRCLE_FILL: SFSymbol = SFSymbol { name: "3.circle.fill" };
pub const _3_SQUARE: SFSymbol = SFSymbol { name: "3.square" };
pub const _3_SQUARE_FILL: SFSymbol = SFSymbol { name: "3.square.fill" };
pub const _30_CIRCLE: SFSymbol = SFSymbol { name: "30.circle" };
pub const _30_CIRCLE_FILL: SFSymbol = SFSymbol { name: "30.circle.fill" };
pub const _30_SQUARE: SFSymbol = SFSymbol { name: "30.square" };
pub const _30_SQUARE_FILL: SFSymbol = SFSymbol { name: "30.square.fill" };
pub const _31_CIRCLE: SFSymbol = SFSymbol { name: "31.circle" };
pub const _31_CIRCLE_FILL: SFSymbol = SFSymbol { name: "31.circle.fill" };
pub const _31_SQUARE: SFSymbol = SFSymbol { name: "31.square" };
pub const _31_SQUARE_FILL: SFSymbol = SFSymbol { name: "31.square.fill" };
pub const _32_CIRCLE: SFSymbol = SFSymbol { name: "32.circle" };
pub const _32_CIRCLE_FILL: SFSymbol = SFSymbol { name: "32.circle.fill" };
pub const _32_SQUARE: SFSymbol = SFSymbol { name: "32.square" };
pub const _32_SQUARE_FILL: SFSymbol = SFSymbol { name: "32.square.fill" };
pub const _33_CIRCLE: SFSymbol = SFSymbol { name: "33.circle" };
pub const _33_CIRCLE_FILL: SFSymbol = SFSymbol { name: "33.circle.fill" };
pub const _33_SQUARE: SFSymbol = SFSymbol { name: "33.square" };
pub const _33_SQUARE_FILL: SFSymbol = SFSymbol { name: "33.square.fill" };
pub const _34_CIRCLE: SFSymbol = SFSymbol { name: "34.circle" };
pub const _34_CIRCLE_FILL: SFSymbol = SFSymbol { name: "34.circle.fill" };
pub const _34_SQUARE: SFSymbol = SFSymbol { name: "34.square" };
pub const _34_SQUARE_FILL: SFSymbol = SFSymbol { name: "34.square.fill" };
pub const _35_CIRCLE: SFSymbol = SFSymbol { name: "35.circle" };
pub const _35_CIRCLE_FILL: SFSymbol = SFSymbol { name: "35.circle.fill" };
pub const _35_SQUARE: SFSymbol = SFSymbol { name: "35.square" };
pub const _35_SQUARE_FILL: SFSymbol = SFSymbol { name: "35.square.fill" };
pub const _36_CIRCLE: SFSymbol = SFSymbol { name: "36.circle" };
pub const _36_CIRCLE_FILL: SFSymbol = SFSymbol { name: "36.circle.fill" };
pub const _36_SQUARE: SFSymbol = SFSymbol { name: "36.square" };
pub const _36_SQUARE_FILL: SFSymbol = SFSymbol { name: "36.square.fill" };
pub const _37_CIRCLE: SFSymbol = SFSymbol { name: "37.circle" };
pub const _37_CIRCLE_FILL: SFSymbol = SFSymbol { name: "37.circle.fill" };
pub const _37_SQUARE: SFSymbol = SFSymbol { name: "37.square" };
pub const _37_SQUARE_FILL: SFSymbol = SFSymbol { name: "37.square.fill" };
pub const _38_CIRCLE: SFSymbol = SFSymbol { name: "38.circle" };
pub const _38_CIRCLE_FILL: SFSymbol = SFSymbol { name: "38.circle.fill" };
pub const _38_SQUARE: SFSymbol = SFSymbol { name: "38.square" };
pub const _38_SQUARE_FILL: SFSymbol = SFSymbol { name: "38.square.fill" };
pub const _39_CIRCLE: SFSymbol = SFSymbol { name: "39.circle" };
pub const _39_CIRCLE_FILL: SFSymbol = SFSymbol { name: "39.circle.fill" };
pub const _39_SQUARE: SFSymbol = SFSymbol { name: "39.square" };
pub const _39_SQUARE_FILL: SFSymbol = SFSymbol { name: "39.square.fill" };
pub const _4_ALT_CIRCLE: SFSymbol = SFSymbol { name: "4.alt.circle" };
pub const _4_ALT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "4.alt.circle.fill" };
pub const _4_ALT_SQUARE: SFSymbol = SFSymbol { name: "4.alt.square" };
pub const _4_ALT_SQUARE_FILL: SFSymbol = SFSymbol { name: "4.alt.square.fill" };
pub const _4_CIRCLE: SFSymbol = SFSymbol { name: "4.circle" };
pub const _4_CIRCLE_FILL: SFSymbol = SFSymbol { name: "4.circle.fill" };
pub const _4_SQUARE: SFSymbol = SFSymbol { name: "4.square" };
pub const _4_SQUARE_FILL: SFSymbol = SFSymbol { name: "4.square.fill" };
pub const _40_CIRCLE: SFSymbol = SFSymbol { name: "40.circle" };
pub const _40_CIRCLE_FILL: SFSymbol = SFSymbol { name: "40.circle.fill" };
pub const _40_SQUARE: SFSymbol = SFSymbol { name: "40.square" };
pub const _40_SQUARE_FILL: SFSymbol = SFSymbol { name: "40.square.fill" };
pub const _41_CIRCLE: SFSymbol = SFSymbol { name: "41.circle" };
pub const _41_CIRCLE_FILL: SFSymbol = SFSymbol { name: "41.circle.fill" };
pub const _41_SQUARE: SFSymbol = SFSymbol { name: "41.square" };
pub const _41_SQUARE_FILL: SFSymbol = SFSymbol { name: "41.square.fill" };
pub const _42_CIRCLE: SFSymbol = SFSymbol { name: "42.circle" };
pub const _42_CIRCLE_FILL: SFSymbol = SFSymbol { name: "42.circle.fill" };
pub const _42_SQUARE: SFSymbol = SFSymbol { name: "42.square" };
pub const _42_SQUARE_FILL: SFSymbol = SFSymbol { name: "42.square.fill" };
pub const _43_CIRCLE: SFSymbol = SFSymbol { name: "43.circle" };
pub const _43_CIRCLE_FILL: SFSymbol = SFSymbol { name: "43.circle.fill" };
pub const _43_SQUARE: SFSymbol = SFSymbol { name: "43.square" };
pub const _43_SQUARE_FILL: SFSymbol = SFSymbol { name: "43.square.fill" };
pub const _44_CIRCLE: SFSymbol = SFSymbol { name: "44.circle" };
pub const _44_CIRCLE_FILL: SFSymbol = SFSymbol { name: "44.circle.fill" };
pub const _44_SQUARE: SFSymbol = SFSymbol { name: "44.square" };
pub const _44_SQUARE_FILL: SFSymbol = SFSymbol { name: "44.square.fill" };
pub const _45_CIRCLE: SFSymbol = SFSymbol { name: "45.circle" };
pub const _45_CIRCLE_FILL: SFSymbol = SFSymbol { name: "45.circle.fill" };
pub const _45_SQUARE: SFSymbol = SFSymbol { name: "45.square" };
pub const _45_SQUARE_FILL: SFSymbol = SFSymbol { name: "45.square.fill" };
pub const _46_CIRCLE: SFSymbol = SFSymbol { name: "46.circle" };
pub const _46_CIRCLE_FILL: SFSymbol = SFSymbol { name: "46.circle.fill" };
pub const _46_SQUARE: SFSymbol = SFSymbol { name: "46.square" };
pub const _46_SQUARE_FILL: SFSymbol = SFSymbol { name: "46.square.fill" };
pub const _47_CIRCLE: SFSymbol = SFSymbol { name: "47.circle" };
pub const _47_CIRCLE_FILL: SFSymbol = SFSymbol { name: "47.circle.fill" };
pub const _47_SQUARE: SFSymbol = SFSymbol { name: "47.square" };
pub const _47_SQUARE_FILL: SFSymbol = SFSymbol { name: "47.square.fill" };
pub const _48_CIRCLE: SFSymbol = SFSymbol { name: "48.circle" };
pub const _48_CIRCLE_FILL: SFSymbol = SFSymbol { name: "48.circle.fill" };
pub const _48_SQUARE: SFSymbol = SFSymbol { name: "48.square" };
pub const _48_SQUARE_FILL: SFSymbol = SFSymbol { name: "48.square.fill" };
pub const _49_CIRCLE: SFSymbol = SFSymbol { name: "49.circle" };
pub const _49_CIRCLE_FILL: SFSymbol = SFSymbol { name: "49.circle.fill" };
pub const _49_SQUARE: SFSymbol = SFSymbol { name: "49.square" };
pub const _49_SQUARE_FILL: SFSymbol = SFSymbol { name: "49.square.fill" };
pub const _4K_TV: SFSymbol = SFSymbol { name: "4k.tv" };
pub const _4K_TV_FILL: SFSymbol = SFSymbol { name: "4k.tv.fill" };
pub const _5_CIRCLE: SFSymbol = SFSymbol { name: "5.circle" };
pub const _5_CIRCLE_FILL: SFSymbol = SFSymbol { name: "5.circle.fill" };
pub const _5_SQUARE: SFSymbol = SFSymbol { name: "5.square" };
pub const _5_SQUARE_FILL: SFSymbol = SFSymbol { name: "5.square.fill" };
pub const _50_CIRCLE: SFSymbol = SFSymbol { name: "50.circle" };
pub const _50_CIRCLE_FILL: SFSymbol = SFSymbol { name: "50.circle.fill" };
pub const _50_SQUARE: SFSymbol = SFSymbol { name: "50.square" };
pub const _50_SQUARE_FILL: SFSymbol = SFSymbol { name: "50.square.fill" };
pub const _6_ALT_CIRCLE: SFSymbol = SFSymbol { name: "6.alt.circle" };
pub const _6_ALT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "6.alt.circle.fill" };
pub const _6_ALT_SQUARE: SFSymbol = SFSymbol { name: "6.alt.square" };
pub const _6_ALT_SQUARE_FILL: SFSymbol = SFSymbol { name: "6.alt.square.fill" };
pub const _6_CIRCLE: SFSymbol = SFSymbol { name: "6.circle" };
pub const _6_CIRCLE_FILL: SFSymbol = SFSymbol { name: "6.circle.fill" };
pub const _6_SQUARE: SFSymbol = SFSymbol { name: "6.square" };
pub const _6_SQUARE_FILL: SFSymbol = SFSymbol { name: "6.square.fill" };
pub const _7_CIRCLE: SFSymbol = SFSymbol { name: "7.circle" };
pub const _7_CIRCLE_FILL: SFSymbol = SFSymbol { name: "7.circle.fill" };
pub const _7_SQUARE: SFSymbol = SFSymbol { name: "7.square" };
pub const _7_SQUARE_FILL: SFSymbol = SFSymbol { name: "7.square.fill" };
pub const _8_CIRCLE: SFSymbol = SFSymbol { name: "8.circle" };
pub const _8_CIRCLE_FILL: SFSymbol = SFSymbol { name: "8.circle.fill" };
pub const _8_SQUARE: SFSymbol = SFSymbol { name: "8.square" };
pub const _8_SQUARE_FILL: SFSymbol = SFSymbol { name: "8.square.fill" };
pub const _9_ALT_CIRCLE: SFSymbol = SFSymbol { name: "9.alt.circle" };
pub const _9_ALT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "9.alt.circle.fill" };
pub const _9_ALT_SQUARE: SFSymbol = SFSymbol { name: "9.alt.square" };
pub const _9_ALT_SQUARE_FILL: SFSymbol = SFSymbol { name: "9.alt.square.fill" };
pub const _9_CIRCLE: SFSymbol = SFSymbol { name: "9.circle" };
pub const _9_CIRCLE_FILL: SFSymbol = SFSymbol { name: "9.circle.fill" };
pub const _9_SQUARE: SFSymbol = SFSymbol { name: "9.square" };
pub const _9_SQUARE_FILL: SFSymbol = SFSymbol { name: "9.square.fill" };
pub const A_CIRCLE: SFSymbol = SFSymbol { name: "a.circle" };
pub const A_CIRCLE_FILL: SFSymbol = SFSymbol { name: "a.circle.fill" };
pub const A_MAGNIFY: SFSymbol = SFSymbol { name: "a.magnify" };
pub const A_SQUARE: SFSymbol = SFSymbol { name: "a.square" };
pub const A_SQUARE_FILL: SFSymbol = SFSymbol { name: "a.square.fill" };
pub const ABC: SFSymbol = SFSymbol { name: "abc" };
pub const AIR_CONDITIONER_HORIZONTAL: SFSymbol = SFSymbol { name: "air.conditioner.horizontal" };
pub const AIR_CONDITIONER_HORIZONTAL_FILL: SFSymbol = SFSymbol { name: "air.conditioner.horizontal.fill" };
pub const AIR_CONDITIONER_VERTICAL: SFSymbol = SFSymbol { name: "air.conditioner.vertical" };
pub const AIR_CONDITIONER_VERTICAL_FILL: SFSymbol = SFSymbol { name: "air.conditioner.vertical.fill" };
pub const AIR_PURIFIER: SFSymbol = SFSymbol { name: "air.purifier" };
pub const AIR_PURIFIER_FILL: SFSymbol = SFSymbol { name: "air.purifier.fill" };
pub const AIRPLANE: SFSymbol = SFSymbol { name: "airplane" };
pub const AIRPLANE_ARRIVAL: SFSymbol = SFSymbol { name: "airplane.arrival" };
pub const AIRPLANE_CIRCLE: SFSymbol = SFSymbol { name: "airplane.circle" };
pub const AIRPLANE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "airplane.circle.fill" };
pub const AIRPLANE_DEPARTURE: SFSymbol = SFSymbol { name: "airplane.departure" };
pub const AIRPLAYAUDIO: SFSymbol = SFSymbol { name: "airplayaudio" };
pub const AIRPLAYAUDIO_BADGE_EXCLAMATIONMARK: SFSymbol = SFSymbol { name: "airplayaudio.badge.exclamationmark" };
pub const AIRPLAYAUDIO_CIRCLE: SFSymbol = SFSymbol { name: "airplayaudio.circle" };
pub const AIRPLAYAUDIO_CIRCLE_FILL: SFSymbol = SFSymbol { name: "airplayaudio.circle.fill" };
pub const AIRPLAYVIDEO: SFSymbol = SFSymbol { name: "airplayvideo" };
pub const AIRPLAYVIDEO_BADGE_EXCLAMATIONMARK: SFSymbol = SFSymbol { name: "airplayvideo.badge.exclamationmark" };
pub const AIRPLAYVIDEO_CIRCLE: SFSymbol = SFSymbol { name: "airplayvideo.circle" };
pub const AIRPLAYVIDEO_CIRCLE_FILL: SFSymbol = SFSymbol { name: "airplayvideo.circle.fill" };
pub const AIRPOD_GEN3_LEFT: SFSymbol = SFSymbol { name: "airpod.gen3.left" };
pub const AIRPOD_GEN3_RIGHT: SFSymbol = SFSymbol { name: "airpod.gen3.right" };
pub const AIRPOD_LEFT: SFSymbol = SFSymbol { name: "airpod.left" };
pub const AIRPOD_RIGHT: SFSymbol = SFSymbol { name: "airpod.right" };
pub const AIRPODPRO_LEFT: SFSymbol = SFSymbol { name: "airpodpro.left" };
pub const AIRPODPRO_RIGHT: SFSymbol = SFSymbol { name: "airpodpro.right" };
pub const AIRPODS: SFSymbol = SFSymbol { name: "airpods" };
pub const AIRPODS_CHARGINGCASE: SFSymbol = SFSymbol { name: "airpods.chargingcase" };
pub const AIRPODS_CHARGINGCASE_FILL: SFSymbol = SFSymbol { name: "airpods.chargingcase.fill" };
pub const AIRPODS_CHARGINGCASE_WIRELESS: SFSymbol = SFSymbol { name: "airpods.chargingcase.wireless" };
pub const AIRPODS_CHARGINGCASE_WIRELESS_FILL: SFSymbol = SFSymbol { name: "airpods.chargingcase.wireless.fill" };
pub const AIRPODS_GEN3: SFSymbol = SFSymbol { name: "airpods.gen3" };
pub const AIRPODS_GEN3_CHARGINGCASE_WIRELESS: SFSymbol = SFSymbol { name: "airpods.gen3.chargingcase.wireless" };
pub const AIRPODS_GEN3_CHARGINGCASE_WIRELESS_FILL: SFSymbol = SFSymbol { name: "airpods.gen3.chargingcase.wireless.fill" };
pub const AIRPODSMAX: SFSymbol = SFSymbol { name: "airpodsmax" };
pub const AIRPODSPRO: SFSymbol = SFSymbol { name: "airpodspro" };
pub const AIRPODSPRO_CHARGINGCASE_WIRELESS: SFSymbol = SFSymbol { name: "airpodspro.chargingcase.wireless" };
pub const AIRPODSPRO_CHARGINGCASE_WIRELESS_FILL: SFSymbol = SFSymbol { name: "airpodspro.chargingcase.wireless.fill" };
pub const AIRPORT_EXPRESS: SFSymbol = SFSymbol { name: "airport.express" };
pub const AIRPORT_EXTREME: SFSymbol = SFSymbol { name: "airport.extreme" };
pub const AIRPORT_EXTREME_TOWER: SFSymbol = SFSymbol { name: "airport.extreme.tower" };
pub const AIRTAG: SFSymbol = SFSymbol { name: "airtag" };
pub const AIRTAG_FILL: SFSymbol = SFSymbol { name: "airtag.fill" };
pub const AIRTAG_RADIOWAVES_FORWARD: SFSymbol = SFSymbol { name: "airtag.radiowaves.forward" };
pub const AIRTAG_RADIOWAVES_FORWARD_FILL: SFSymbol = SFSymbol { name: "airtag.radiowaves.forward.fill" };
pub const ALARM: SFSymbol = SFSymbol { name: "alarm" };
pub const ALARM_FILL: SFSymbol = SFSymbol { name: "alarm.fill" };
pub const ALARM_WAVES_LEFT_AND_RIGHT: SFSymbol = SFSymbol { name: "alarm.waves.left.and.right" };
pub const ALARM_WAVES_LEFT_AND_RIGHT_FILL: SFSymbol = SFSymbol { name: "alarm.waves.left.and.right.fill" };
pub const ALIGN_HORIZONTAL_CENTER: SFSymbol = SFSymbol { name: "align.horizontal.center" };
pub const ALIGN_HORIZONTAL_CENTER_FILL: SFSymbol = SFSymbol { name: "align.horizontal.center.fill" };
pub const ALIGN_HORIZONTAL_LEFT: SFSymbol = SFSymbol { name: "align.horizontal.left" };
pub const ALIGN_HORIZONTAL_LEFT_FILL: SFSymbol = SFSymbol { name: "align.horizontal.left.fill" };
pub const ALIGN_HORIZONTAL_RIGHT: SFSymbol = SFSymbol { name: "align.horizontal.right" };
pub const ALIGN_HORIZONTAL_RIGHT_FILL: SFSymbol = SFSymbol { name: "align.horizontal.right.fill" };
pub const ALIGN_VERTICAL_BOTTOM: SFSymbol = SFSymbol { name: "align.vertical.bottom" };
pub const ALIGN_VERTICAL_BOTTOM_FILL: SFSymbol = SFSymbol { name: "align.vertical.bottom.fill" };
pub const ALIGN_VERTICAL_CENTER: SFSymbol = SFSymbol { name: "align.vertical.center" };
pub const ALIGN_VERTICAL_CENTER_FILL: SFSymbol = SFSymbol { name: "align.vertical.center.fill" };
pub const ALIGN_VERTICAL_TOP: SFSymbol = SFSymbol { name: "align.vertical.top" };
pub const ALIGN_VERTICAL_TOP_FILL: SFSymbol = SFSymbol { name: "align.vertical.top.fill" };
pub const ALLERGENS: SFSymbol = SFSymbol { name: "allergens" };
pub const ALLERGENS_FILL: SFSymbol = SFSymbol { name: "allergens.fill" };
pub const ALT: SFSymbol = SFSymbol { name: "alt" };
pub const ALTERNATINGCURRENT: SFSymbol = SFSymbol { name: "alternatingcurrent" };
pub const AMPLIFIER: SFSymbol = SFSymbol { name: "amplifier" };
pub const ANGLE: SFSymbol = SFSymbol { name: "angle" };
pub const ANT: SFSymbol = SFSymbol { name: "ant" };
pub const ANT_CIRCLE: SFSymbol = SFSymbol { name: "ant.circle" };
pub const ANT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "ant.circle.fill" };
pub const ANT_FILL: SFSymbol = SFSymbol { name: "ant.fill" };
pub const ANTENNA_RADIOWAVES_LEFT_AND_RIGHT: SFSymbol = SFSymbol { name: "antenna.radiowaves.left.and.right" };
pub const ANTENNA_RADIOWAVES_LEFT_AND_RIGHT_CIRCLE: SFSymbol = SFSymbol { name: "antenna.radiowaves.left.and.right.circle" };
pub const ANTENNA_RADIOWAVES_LEFT_AND_RIGHT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "antenna.radiowaves.left.and.right.circle.fill" };
pub const ANTENNA_RADIOWAVES_LEFT_AND_RIGHT_SLASH: SFSymbol = SFSymbol { name: "antenna.radiowaves.left.and.right.slash" };
pub const APP: SFSymbol = SFSymbol { name: "app" };
pub const APP_BADGE: SFSymbol = SFSymbol { name: "app.badge" };
pub const APP_BADGE_CHECKMARK: SFSymbol = SFSymbol { name: "app.badge.checkmark" };
pub const APP_BADGE_CHECKMARK_FILL: SFSymbol = SFSymbol { name: "app.badge.checkmark.fill" };
pub const APP_BADGE_FILL: SFSymbol = SFSymbol { name: "app.badge.fill" };
pub const APP_CONNECTED_TO_APP_BELOW_FILL: SFSymbol = SFSymbol { name: "app.connected.to.app.below.fill" };
pub const APP_DASHED: SFSymbol = SFSymbol { name: "app.dashed" };
pub const APP_FILL: SFSymbol = SFSymbol { name: "app.fill" };
pub const APP_GIFT: SFSymbol = SFSymbol { name: "app.gift" };
pub const APP_GIFT_FILL: SFSymbol = SFSymbol { name: "app.gift.fill" };
pub const APPCLIP: SFSymbol = SFSymbol { name: "appclip" };
pub const APPLE_LOGO: SFSymbol = SFSymbol { name: "apple.logo" };
pub const APPLELOGO: SFSymbol = SFSymbol { name: "applelogo" };
pub const APPLEPENCIL: SFSymbol = SFSymbol { name: "applepencil" };
pub const APPLESCRIPT: SFSymbol = SFSymbol { name: "applescript" };
pub const APPLESCRIPT_FILL: SFSymbol = SFSymbol { name: "applescript.fill" };
pub const APPLETV: SFSymbol = SFSymbol { name: "appletv" };
pub const APPLETV_FILL: SFSymbol = SFSymbol { name: "appletv.fill" };
pub const APPLETVREMOTE_GEN1: SFSymbol = SFSymbol { name: "appletvremote.gen1" };
pub const APPLETVREMOTE_GEN1_FILL: SFSymbol = SFSymbol { name: "appletvremote.gen1.fill" };
pub const APPLETVREMOTE_GEN2: SFSymbol = SFSymbol { name: "appletvremote.gen2" };
pub const APPLETVREMOTE_GEN2_FILL: SFSymbol = SFSymbol { name: "appletvremote.gen2.fill" };
pub const APPLETVREMOTE_GEN3: SFSymbol = SFSymbol { name: "appletvremote.gen3" };
pub const APPLETVREMOTE_GEN3_FILL: SFSymbol = SFSymbol { name: "appletvremote.gen3.fill" };
pub const APPLETVREMOTE_GEN4: SFSymbol = SFSymbol { name: "appletvremote.gen4" };
pub const APPLETVREMOTE_GEN4_FILL: SFSymbol = SFSymbol { name: "appletvremote.gen4.fill" };
pub const APPLEWATCH: SFSymbol = SFSymbol { name: "applewatch" };
pub const APPLEWATCH_CASE_INSET_FILLED: SFSymbol = SFSymbol { name: "applewatch.case.inset.filled" };
pub const APPLEWATCH_RADIOWAVES_LEFT_AND_RIGHT: SFSymbol = SFSymbol { name: "applewatch.radiowaves.left.and.right" };
pub const APPLEWATCH_SIDE_RIGHT: SFSymbol = SFSymbol { name: "applewatch.side.right" };
pub const APPLEWATCH_SLASH: SFSymbol = SFSymbol { name: "applewatch.slash" };
pub const APPLEWATCH_WATCHFACE: SFSymbol = SFSymbol { name: "applewatch.watchface" };
pub const APPS_IPAD: SFSymbol = SFSymbol { name: "apps.ipad" };
pub const APPS_IPAD_LANDSCAPE: SFSymbol = SFSymbol { name: "apps.ipad.landscape" };
pub const APPS_IPHONE: SFSymbol = SFSymbol { name: "apps.iphone" };
pub const APPS_IPHONE_BADGE_PLUS: SFSymbol = SFSymbol { name: "apps.iphone.badge.plus" };
pub const APPS_IPHONE_LANDSCAPE: SFSymbol = SFSymbol { name: "apps.iphone.landscape" };
pub const AQI_HIGH: SFSymbol = SFSymbol { name: "aqi.high" };
pub const AQI_LOW: SFSymbol = SFSymbol { name: "aqi.low" };
pub const AQI_MEDIUM: SFSymbol = SFSymbol { name: "aqi.medium" };
pub const ARCHIVEBOX: SFSymbol = SFSymbol { name: "archivebox" };
pub const ARCHIVEBOX_CIRCLE: SFSymbol = SFSymbol { name: "archivebox.circle" };
pub const ARCHIVEBOX_CIRCLE_FILL: SFSymbol = SFSymbol { name: "archivebox.circle.fill" };
pub const ARCHIVEBOX_FILL: SFSymbol = SFSymbol { name: "archivebox.fill" };
pub const ARKIT: SFSymbol = SFSymbol { name: "arkit" };
pub const ARKIT_BADGE_XMARK: SFSymbol = SFSymbol { name: "arkit.badge.xmark" };
pub const ARROW_2_SQUAREPATH: SFSymbol = SFSymbol { name: "arrow.2.squarepath" };
pub const ARROW_3_TRIANGLEPATH: SFSymbol = SFSymbol { name: "arrow.3.trianglepath" };
pub const ARROW_BACKWARD: SFSymbol = SFSymbol { name: "arrow.backward" };
pub const ARROW_BACKWARD_CIRCLE: SFSymbol = SFSymbol { name: "arrow.backward.circle" };
pub const ARROW_BACKWARD_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.backward.circle.fill" };
pub const ARROW_BACKWARD_SQUARE: SFSymbol = SFSymbol { name: "arrow.backward.square" };
pub const ARROW_BACKWARD_SQUARE_FILL: SFSymbol = SFSymbol { name: "arrow.backward.square.fill" };
pub const ARROW_BACKWARD_TO_LINE: SFSymbol = SFSymbol { name: "arrow.backward.to.line" };
pub const ARROW_BACKWARD_TO_LINE_CIRCLE: SFSymbol = SFSymbol { name: "arrow.backward.to.line.circle" };
pub const ARROW_BACKWARD_TO_LINE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.backward.to.line.circle.fill" };
pub const ARROW_CLOCKWISE: SFSymbol = SFSymbol { name: "arrow.clockwise" };
pub const ARROW_CLOCKWISE_CIRCLE: SFSymbol = SFSymbol { name: "arrow.clockwise.circle" };
pub const ARROW_CLOCKWISE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.clockwise.circle.fill" };
pub const ARROW_CLOCKWISE_HEART: SFSymbol = SFSymbol { name: "arrow.clockwise.heart" };
pub const ARROW_CLOCKWISE_HEART_FILL: SFSymbol = SFSymbol { name: "arrow.clockwise.heart.fill" };
pub const ARROW_CLOCKWISE_ICLOUD: SFSymbol = SFSymbol { name: "arrow.clockwise.icloud" };
pub const ARROW_CLOCKWISE_ICLOUD_FILL: SFSymbol = SFSymbol { name: "arrow.clockwise.icloud.fill" };
pub const ARROW_COUNTERCLOCKWISE: SFSymbol = SFSymbol { name: "arrow.counterclockwise" };
pub const ARROW_COUNTERCLOCKWISE_CIRCLE: SFSymbol = SFSymbol { name: "arrow.counterclockwise.circle" };
pub const ARROW_COUNTERCLOCKWISE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.counterclockwise.circle.fill" };
pub const ARROW_COUNTERCLOCKWISE_ICLOUD: SFSymbol = SFSymbol { name: "arrow.counterclockwise.icloud" };
pub const ARROW_COUNTERCLOCKWISE_ICLOUD_FILL: SFSymbol = SFSymbol { name: "arrow.counterclockwise.icloud.fill" };
pub const ARROW_DOWN: SFSymbol = SFSymbol { name: "arrow.down" };
pub const ARROW_DOWN_AND_LINE_HORIZONTAL_AND_ARROW_UP: SFSymbol = SFSymbol { name: "arrow.down.and.line.horizontal.and.arrow.up" };
pub const ARROW_DOWN_APP: SFSymbol = SFSymbol { name: "arrow.down.app" };
pub const ARROW_DOWN_APP_FILL: SFSymbol = SFSymbol { name: "arrow.down.app.fill" };
pub const ARROW_DOWN_BACKWARD: SFSymbol = SFSymbol { name: "arrow.down.backward" };
pub const ARROW_DOWN_BACKWARD_CIRCLE: SFSymbol = SFSymbol { name: "arrow.down.backward.circle" };
pub const ARROW_DOWN_BACKWARD_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.down.backward.circle.fill" };
pub const ARROW_DOWN_BACKWARD_SQUARE: SFSymbol = SFSymbol { name: "arrow.down.backward.square" };
pub const ARROW_DOWN_BACKWARD_SQUARE_FILL: SFSymbol = SFSymbol { name: "arrow.down.backward.square.fill" };
pub const ARROW_DOWN_CIRCLE: SFSymbol = SFSymbol { name: "arrow.down.circle" };
pub const ARROW_DOWN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.down.circle.fill" };
pub const ARROW_DOWN_DOC: SFSymbol = SFSymbol { name: "arrow.down.doc" };
pub const ARROW_DOWN_DOC_FILL: SFSymbol = SFSymbol { name: "arrow.down.doc.fill" };
pub const ARROW_DOWN_FORWARD: SFSymbol = SFSymbol { name: "arrow.down.forward" };
pub const ARROW_DOWN_FORWARD_AND_ARROW_UP_BACKWARD: SFSymbol = SFSymbol { name: "arrow.down.forward.and.arrow.up.backward" };
pub const ARROW_DOWN_FORWARD_AND_ARROW_UP_BACKWARD_CIRCLE: SFSymbol = SFSymbol { name: "arrow.down.forward.and.arrow.up.backward.circle" };
pub const ARROW_DOWN_FORWARD_AND_ARROW_UP_BACKWARD_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.down.forward.and.arrow.up.backward.circle.fill" };
pub const ARROW_DOWN_FORWARD_CIRCLE: SFSymbol = SFSymbol { name: "arrow.down.forward.circle" };
pub const ARROW_DOWN_FORWARD_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.down.forward.circle.fill" };
pub const ARROW_DOWN_FORWARD_SQUARE: SFSymbol = SFSymbol { name: "arrow.down.forward.square" };
pub const ARROW_DOWN_FORWARD_SQUARE_FILL: SFSymbol = SFSymbol { name: "arrow.down.forward.square.fill" };
pub const ARROW_DOWN_HEART: SFSymbol = SFSymbol { name: "arrow.down.heart" };
pub const ARROW_DOWN_HEART_FILL: SFSymbol = SFSymbol { name: "arrow.down.heart.fill" };
pub const ARROW_DOWN_LEFT: SFSymbol = SFSymbol { name: "arrow.down.left" };
pub const ARROW_DOWN_LEFT_CIRCLE: SFSymbol = SFSymbol { name: "arrow.down.left.circle" };
pub const ARROW_DOWN_LEFT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.down.left.circle.fill" };
pub const ARROW_DOWN_LEFT_SQUARE: SFSymbol = SFSymbol { name: "arrow.down.left.square" };
pub const ARROW_DOWN_LEFT_SQUARE_FILL: SFSymbol = SFSymbol { name: "arrow.down.left.square.fill" };
pub const ARROW_DOWN_LEFT_VIDEO: SFSymbol = SFSymbol { name: "arrow.down.left.video" };
pub const ARROW_DOWN_LEFT_VIDEO_FILL: SFSymbol = SFSymbol { name: "arrow.down.left.video.fill" };
pub const ARROW_DOWN_MESSAGE: SFSymbol = SFSymbol { name: "arrow.down.message" };
pub const ARROW_DOWN_MESSAGE_FILL: SFSymbol = SFSymbol { name: "arrow.down.message.fill" };
pub const ARROW_DOWN_RIGHT: SFSymbol = SFSymbol { name: "arrow.down.right" };
pub const ARROW_DOWN_RIGHT_AND_ARROW_UP_LEFT: SFSymbol = SFSymbol { name: "arrow.down.right.and.arrow.up.left" };
pub const ARROW_DOWN_RIGHT_AND_ARROW_UP_LEFT_CIRCLE: SFSymbol = SFSymbol { name: "arrow.down.right.and.arrow.up.left.circle" };
pub const ARROW_DOWN_RIGHT_AND_ARROW_UP_LEFT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.down.right.and.arrow.up.left.circle.fill" };
pub const ARROW_DOWN_RIGHT_CIRCLE: SFSymbol = SFSymbol { name: "arrow.down.right.circle" };
pub const ARROW_DOWN_RIGHT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.down.right.circle.fill" };
pub const ARROW_DOWN_RIGHT_SQUARE: SFSymbol = SFSymbol { name: "arrow.down.right.square" };
pub const ARROW_DOWN_RIGHT_SQUARE_FILL: SFSymbol = SFSymbol { name: "arrow.down.right.square.fill" };
pub const ARROW_DOWN_SQUARE: SFSymbol = SFSymbol { name: "arrow.down.square" };
pub const ARROW_DOWN_SQUARE_FILL: SFSymbol = SFSymbol { name: "arrow.down.square.fill" };
pub const ARROW_DOWN_TO_LINE: SFSymbol = SFSymbol { name: "arrow.down.to.line" };
pub const ARROW_DOWN_TO_LINE_CIRCLE: SFSymbol = SFSymbol { name: "arrow.down.to.line.circle" };
pub const ARROW_DOWN_TO_LINE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.down.to.line.circle.fill" };
pub const ARROW_DOWN_TO_LINE_COMPACT: SFSymbol = SFSymbol { name: "arrow.down.to.line.compact" };
pub const ARROW_FORWARD: SFSymbol = SFSymbol { name: "arrow.forward" };
pub const ARROW_FORWARD_CIRCLE: SFSymbol = SFSymbol { name: "arrow.forward.circle" };
pub const ARROW_FORWARD_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.forward.circle.fill" };
pub const ARROW_FORWARD_SQUARE: SFSymbol = SFSymbol { name: "arrow.forward.square" };
pub const ARROW_FORWARD_SQUARE_FILL: SFSymbol = SFSymbol { name: "arrow.forward.square.fill" };
pub const ARROW_FORWARD_TO_LINE: SFSymbol = SFSymbol { name: "arrow.forward.to.line" };
pub const ARROW_FORWARD_TO_LINE_CIRCLE: SFSymbol = SFSymbol { name: "arrow.forward.to.line.circle" };
pub const ARROW_FORWARD_TO_LINE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.forward.to.line.circle.fill" };
pub const ARROW_LEFT: SFSymbol = SFSymbol { name: "arrow.left" };
pub const ARROW_LEFT_AND_LINE_VERTICAL_AND_ARROW_RIGHT: SFSymbol = SFSymbol { name: "arrow.left.and.line.vertical.and.arrow.right" };
pub const ARROW_LEFT_AND_RIGHT: SFSymbol = SFSymbol { name: "arrow.left.and.right" };
pub const ARROW_LEFT_AND_RIGHT_CIRCLE: SFSymbol = SFSymbol { name: "arrow.left.and.right.circle" };
pub const ARROW_LEFT_AND_RIGHT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.left.and.right.circle.fill" };
pub const ARROW_LEFT_AND_RIGHT_RIGHTTRIANGLE_LEFT_RIGHTTRIANGLE_RIGHT: SFSymbol = SFSymbol { name: "arrow.left.and.right.righttriangle.left.righttriangle.right" };
pub const ARROW_LEFT_AND_RIGHT_RIGHTTRIANGLE_LEFT_RIGHTTRIANGLE_RIGHT_FILL: SFSymbol = SFSymbol { name: "arrow.left.and.right.righttriangle.left.righttriangle.right.fill" };
pub const ARROW_LEFT_AND_RIGHT_SQUARE: SFSymbol = SFSymbol { name: "arrow.left.and.right.square" };
pub const ARROW_LEFT_AND_RIGHT_SQUARE_FILL: SFSymbol = SFSymbol { name: "arrow.left.and.right.square.fill" };
pub const ARROW_LEFT_AND_RIGHT_TEXT_VERTICAL: SFSymbol = SFSymbol { name: "arrow.left.and.right.text.vertical" };
pub const ARROW_LEFT_ARROW_RIGHT: SFSymbol = SFSymbol { name: "arrow.left.arrow.right" };
pub const ARROW_LEFT_ARROW_RIGHT_CIRCLE: SFSymbol = SFSymbol { name: "arrow.left.arrow.right.circle" };
pub const ARROW_LEFT_ARROW_RIGHT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.left.arrow.right.circle.fill" };
pub const ARROW_LEFT_ARROW_RIGHT_SQUARE: SFSymbol = SFSymbol { name: "arrow.left.arrow.right.square" };
pub const ARROW_LEFT_ARROW_RIGHT_SQUARE_FILL: SFSymbol = SFSymbol { name: "arrow.left.arrow.right.square.fill" };
pub const ARROW_LEFT_CIRCLE: SFSymbol = SFSymbol { name: "arrow.left.circle" };
pub const ARROW_LEFT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.left.circle.fill" };
pub const ARROW_LEFT_SQUARE: SFSymbol = SFSymbol { name: "arrow.left.square" };
pub const ARROW_LEFT_SQUARE_FILL: SFSymbol = SFSymbol { name: "arrow.left.square.fill" };
pub const ARROW_LEFT_TO_LINE: SFSymbol = SFSymbol { name: "arrow.left.to.line" };
pub const ARROW_LEFT_TO_LINE_CIRCLE: SFSymbol = SFSymbol { name: "arrow.left.to.line.circle" };
pub const ARROW_LEFT_TO_LINE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.left.to.line.circle.fill" };
pub const ARROW_LEFT_TO_LINE_COMPACT: SFSymbol = SFSymbol { name: "arrow.left.to.line.compact" };
pub const ARROW_RECTANGLEPATH: SFSymbol = SFSymbol { name: "arrow.rectanglepath" };
pub const ARROW_RIGHT: SFSymbol = SFSymbol { name: "arrow.right" };
pub const ARROW_RIGHT_AND_LINE_VERTICAL_AND_ARROW_LEFT: SFSymbol = SFSymbol { name: "arrow.right.and.line.vertical.and.arrow.left" };
pub const ARROW_RIGHT_CIRCLE: SFSymbol = SFSymbol { name: "arrow.right.circle" };
pub const ARROW_RIGHT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.right.circle.fill" };
pub const ARROW_RIGHT_DOC_ON_CLIPBOARD: SFSymbol = SFSymbol { name: "arrow.right.doc.on.clipboard" };
pub const ARROW_RIGHT_SQUARE: SFSymbol = SFSymbol { name: "arrow.right.square" };
pub const ARROW_RIGHT_SQUARE_FILL: SFSymbol = SFSymbol { name: "arrow.right.square.fill" };
pub const ARROW_RIGHT_TO_LINE: SFSymbol = SFSymbol { name: "arrow.right.to.line" };
pub const ARROW_RIGHT_TO_LINE_CIRCLE: SFSymbol = SFSymbol { name: "arrow.right.to.line.circle" };
pub const ARROW_RIGHT_TO_LINE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.right.to.line.circle.fill" };
pub const ARROW_RIGHT_TO_LINE_COMPACT: SFSymbol = SFSymbol { name: "arrow.right.to.line.compact" };
pub const ARROW_TRIANGLE_2_CIRCLEPATH: SFSymbol = SFSymbol { name: "arrow.triangle.2.circlepath" };
pub const ARROW_TRIANGLE_2_CIRCLEPATH_CAMERA: SFSymbol = SFSymbol { name: "arrow.triangle.2.circlepath.camera" };
pub const ARROW_TRIANGLE_2_CIRCLEPATH_CAMERA_FILL: SFSymbol = SFSymbol { name: "arrow.triangle.2.circlepath.camera.fill" };
pub const ARROW_TRIANGLE_2_CIRCLEPATH_CIRCLE: SFSymbol = SFSymbol { name: "arrow.triangle.2.circlepath.circle" };
pub const ARROW_TRIANGLE_2_CIRCLEPATH_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.triangle.2.circlepath.circle.fill" };
pub const ARROW_TRIANGLE_2_CIRCLEPATH_DOC_ON_CLIPBOARD: SFSymbol = SFSymbol { name: "arrow.triangle.2.circlepath.doc.on.clipboard" };
pub const ARROW_TRIANGLE_BRANCH: SFSymbol = SFSymbol { name: "arrow.triangle.branch" };
pub const ARROW_TRIANGLE_CAPSULEPATH: SFSymbol = SFSymbol { name: "arrow.triangle.capsulepath" };
pub const ARROW_TRIANGLE_MERGE: SFSymbol = SFSymbol { name: "arrow.triangle.merge" };
pub const ARROW_TRIANGLE_PULL: SFSymbol = SFSymbol { name: "arrow.triangle.pull" };
pub const ARROW_TRIANGLE_SWAP: SFSymbol = SFSymbol { name: "arrow.triangle.swap" };
pub const ARROW_TRIANGLE_TURN_UP_RIGHT_CIRCLE: SFSymbol = SFSymbol { name: "arrow.triangle.turn.up.right.circle" };
pub const ARROW_TRIANGLE_TURN_UP_RIGHT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.triangle.turn.up.right.circle.fill" };
pub const ARROW_TRIANGLE_TURN_UP_RIGHT_DIAMOND: SFSymbol = SFSymbol { name: "arrow.triangle.turn.up.right.diamond" };
pub const ARROW_TRIANGLE_TURN_UP_RIGHT_DIAMOND_FILL: SFSymbol = SFSymbol { name: "arrow.triangle.turn.up.right.diamond.fill" };
pub const ARROW_TURN_DOWN_LEFT: SFSymbol = SFSymbol { name: "arrow.turn.down.left" };
pub const ARROW_TURN_DOWN_RIGHT: SFSymbol = SFSymbol { name: "arrow.turn.down.right" };
pub const ARROW_TURN_LEFT_DOWN: SFSymbol = SFSymbol { name: "arrow.turn.left.down" };
pub const ARROW_TURN_LEFT_UP: SFSymbol = SFSymbol { name: "arrow.turn.left.up" };
pub const ARROW_TURN_RIGHT_DOWN: SFSymbol = SFSymbol { name: "arrow.turn.right.down" };
pub const ARROW_TURN_RIGHT_UP: SFSymbol = SFSymbol { name: "arrow.turn.right.up" };
pub const ARROW_TURN_UP_FORWARD_IPHONE: SFSymbol = SFSymbol { name: "arrow.turn.up.forward.iphone" };
pub const ARROW_TURN_UP_FORWARD_IPHONE_FILL: SFSymbol = SFSymbol { name: "arrow.turn.up.forward.iphone.fill" };
pub const ARROW_TURN_UP_LEFT: SFSymbol = SFSymbol { name: "arrow.turn.up.left" };
pub const ARROW_TURN_UP_RIGHT: SFSymbol = SFSymbol { name: "arrow.turn.up.right" };
pub const ARROW_UP: SFSymbol = SFSymbol { name: "arrow.up" };
pub const ARROW_UP_AND_DOWN: SFSymbol = SFSymbol { name: "arrow.up.and.down" };
pub const ARROW_UP_AND_DOWN_AND_ARROW_LEFT_AND_RIGHT: SFSymbol = SFSymbol { name: "arrow.up.and.down.and.arrow.left.and.right" };
pub const ARROW_UP_AND_DOWN_AND_SPARKLES: SFSymbol = SFSymbol { name: "arrow.up.and.down.and.sparkles" };
pub const ARROW_UP_AND_DOWN_CIRCLE: SFSymbol = SFSymbol { name: "arrow.up.and.down.circle" };
pub const ARROW_UP_AND_DOWN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.up.and.down.circle.fill" };
pub const ARROW_UP_AND_DOWN_RIGHTTRIANGLE_UP_RIGHTTRIANGLE_DOWN: SFSymbol = SFSymbol { name: "arrow.up.and.down.righttriangle.up.righttriangle.down" };
pub const ARROW_UP_AND_DOWN_RIGHTTRIANGLE_UP_RIGHTTRIANGLE_DOWN_FILL: SFSymbol = SFSymbol { name: "arrow.up.and.down.righttriangle.up.righttriangle.down.fill" };
pub const ARROW_UP_AND_DOWN_SQUARE: SFSymbol = SFSymbol { name: "arrow.up.and.down.square" };
pub const ARROW_UP_AND_DOWN_SQUARE_FILL: SFSymbol = SFSymbol { name: "arrow.up.and.down.square.fill" };
pub const ARROW_UP_AND_DOWN_TEXT_HORIZONTAL: SFSymbol = SFSymbol { name: "arrow.up.and.down.text.horizontal" };
pub const ARROW_UP_AND_LINE_HORIZONTAL_AND_ARROW_DOWN: SFSymbol = SFSymbol { name: "arrow.up.and.line.horizontal.and.arrow.down" };
pub const ARROW_UP_AND_PERSON_RECTANGLE_PORTRAIT: SFSymbol = SFSymbol { name: "arrow.up.and.person.rectangle.portrait" };
pub const ARROW_UP_AND_PERSON_RECTANGLE_TURN_LEFT: SFSymbol = SFSymbol { name: "arrow.up.and.person.rectangle.turn.left" };
pub const ARROW_UP_AND_PERSON_RECTANGLE_TURN_RIGHT: SFSymbol = SFSymbol { name: "arrow.up.and.person.rectangle.turn.right" };
pub const ARROW_UP_ARROW_DOWN: SFSymbol = SFSymbol { name: "arrow.up.arrow.down" };
pub const ARROW_UP_ARROW_DOWN_CIRCLE: SFSymbol = SFSymbol { name: "arrow.up.arrow.down.circle" };
pub const ARROW_UP_ARROW_DOWN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.up.arrow.down.circle.fill" };
pub const ARROW_UP_ARROW_DOWN_SQUARE: SFSymbol = SFSymbol { name: "arrow.up.arrow.down.square" };
pub const ARROW_UP_ARROW_DOWN_SQUARE_FILL: SFSymbol = SFSymbol { name: "arrow.up.arrow.down.square.fill" };
pub const ARROW_UP_BACKWARD: SFSymbol = SFSymbol { name: "arrow.up.backward" };
pub const ARROW_UP_BACKWARD_AND_ARROW_DOWN_FORWARD: SFSymbol = SFSymbol { name: "arrow.up.backward.and.arrow.down.forward" };
pub const ARROW_UP_BACKWARD_AND_ARROW_DOWN_FORWARD_CIRCLE: SFSymbol = SFSymbol { name: "arrow.up.backward.and.arrow.down.forward.circle" };
pub const ARROW_UP_BACKWARD_AND_ARROW_DOWN_FORWARD_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.up.backward.and.arrow.down.forward.circle.fill" };
pub const ARROW_UP_BACKWARD_CIRCLE: SFSymbol = SFSymbol { name: "arrow.up.backward.circle" };
pub const ARROW_UP_BACKWARD_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.up.backward.circle.fill" };
pub const ARROW_UP_BACKWARD_SQUARE: SFSymbol = SFSymbol { name: "arrow.up.backward.square" };
pub const ARROW_UP_BACKWARD_SQUARE_FILL: SFSymbol = SFSymbol { name: "arrow.up.backward.square.fill" };
pub const ARROW_UP_BIN: SFSymbol = SFSymbol { name: "arrow.up.bin" };
pub const ARROW_UP_BIN_FILL: SFSymbol = SFSymbol { name: "arrow.up.bin.fill" };
pub const ARROW_UP_CIRCLE: SFSymbol = SFSymbol { name: "arrow.up.circle" };
pub const ARROW_UP_CIRCLE_BADGE_CLOCK: SFSymbol = SFSymbol { name: "arrow.up.circle.badge.clock" };
pub const ARROW_UP_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.up.circle.fill" };
pub const ARROW_UP_DOC: SFSymbol = SFSymbol { name: "arrow.up.doc" };
pub const ARROW_UP_DOC_FILL: SFSymbol = SFSymbol { name: "arrow.up.doc.fill" };
pub const ARROW_UP_DOC_ON_CLIPBOARD: SFSymbol = SFSymbol { name: "arrow.up.doc.on.clipboard" };
pub const ARROW_UP_FORWARD: SFSymbol = SFSymbol { name: "arrow.up.forward" };
pub const ARROW_UP_FORWARD_APP: SFSymbol = SFSymbol { name: "arrow.up.forward.app" };
pub const ARROW_UP_FORWARD_APP_FILL: SFSymbol = SFSymbol { name: "arrow.up.forward.app.fill" };
pub const ARROW_UP_FORWARD_CIRCLE: SFSymbol = SFSymbol { name: "arrow.up.forward.circle" };
pub const ARROW_UP_FORWARD_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.up.forward.circle.fill" };
pub const ARROW_UP_FORWARD_SQUARE: SFSymbol = SFSymbol { name: "arrow.up.forward.square" };
pub const ARROW_UP_FORWARD_SQUARE_FILL: SFSymbol = SFSymbol { name: "arrow.up.forward.square.fill" };
pub const ARROW_UP_HEART: SFSymbol = SFSymbol { name: "arrow.up.heart" };
pub const ARROW_UP_HEART_FILL: SFSymbol = SFSymbol { name: "arrow.up.heart.fill" };
pub const ARROW_UP_LEFT: SFSymbol = SFSymbol { name: "arrow.up.left" };
pub const ARROW_UP_LEFT_AND_ARROW_DOWN_RIGHT: SFSymbol = SFSymbol { name: "arrow.up.left.and.arrow.down.right" };
pub const ARROW_UP_LEFT_AND_ARROW_DOWN_RIGHT_CIRCLE: SFSymbol = SFSymbol { name: "arrow.up.left.and.arrow.down.right.circle" };
pub const ARROW_UP_LEFT_AND_ARROW_DOWN_RIGHT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.up.left.and.arrow.down.right.circle.fill" };
pub const ARROW_UP_LEFT_AND_DOWN_RIGHT_AND_ARROW_UP_RIGHT_AND_DOWN_LEFT: SFSymbol = SFSymbol { name: "arrow.up.left.and.down.right.and.arrow.up.right.and.down.left" };
pub const ARROW_UP_LEFT_AND_DOWN_RIGHT_MAGNIFYINGGLASS: SFSymbol = SFSymbol { name: "arrow.up.left.and.down.right.magnifyingglass" };
pub const ARROW_UP_LEFT_CIRCLE: SFSymbol = SFSymbol { name: "arrow.up.left.circle" };
pub const ARROW_UP_LEFT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.up.left.circle.fill" };
pub const ARROW_UP_LEFT_SQUARE: SFSymbol = SFSymbol { name: "arrow.up.left.square" };
pub const ARROW_UP_LEFT_SQUARE_FILL: SFSymbol = SFSymbol { name: "arrow.up.left.square.fill" };
pub const ARROW_UP_MESSAGE: SFSymbol = SFSymbol { name: "arrow.up.message" };
pub const ARROW_UP_MESSAGE_FILL: SFSymbol = SFSymbol { name: "arrow.up.message.fill" };
pub const ARROW_UP_RIGHT: SFSymbol = SFSymbol { name: "arrow.up.right" };
pub const ARROW_UP_RIGHT_AND_ARROW_DOWN_LEFT_RECTANGLE: SFSymbol = SFSymbol { name: "arrow.up.right.and.arrow.down.left.rectangle" };
pub const ARROW_UP_RIGHT_AND_ARROW_DOWN_LEFT_RECTANGLE_FILL: SFSymbol = SFSymbol { name: "arrow.up.right.and.arrow.down.left.rectangle.fill" };
pub const ARROW_UP_RIGHT_CIRCLE: SFSymbol = SFSymbol { name: "arrow.up.right.circle" };
pub const ARROW_UP_RIGHT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.up.right.circle.fill" };
pub const ARROW_UP_RIGHT_SQUARE: SFSymbol = SFSymbol { name: "arrow.up.right.square" };
pub const ARROW_UP_RIGHT_SQUARE_FILL: SFSymbol = SFSymbol { name: "arrow.up.right.square.fill" };
pub const ARROW_UP_RIGHT_VIDEO: SFSymbol = SFSymbol { name: "arrow.up.right.video" };
pub const ARROW_UP_RIGHT_VIDEO_FILL: SFSymbol = SFSymbol { name: "arrow.up.right.video.fill" };
pub const ARROW_UP_SQUARE: SFSymbol = SFSymbol { name: "arrow.up.square" };
pub const ARROW_UP_SQUARE_FILL: SFSymbol = SFSymbol { name: "arrow.up.square.fill" };
pub const ARROW_UP_TO_LINE: SFSymbol = SFSymbol { name: "arrow.up.to.line" };
pub const ARROW_UP_TO_LINE_CIRCLE: SFSymbol = SFSymbol { name: "arrow.up.to.line.circle" };
pub const ARROW_UP_TO_LINE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.up.to.line.circle.fill" };
pub const ARROW_UP_TO_LINE_COMPACT: SFSymbol = SFSymbol { name: "arrow.up.to.line.compact" };
pub const ARROW_UTURN_BACKWARD: SFSymbol = SFSymbol { name: "arrow.uturn.backward" };
pub const ARROW_UTURN_BACKWARD_CIRCLE: SFSymbol = SFSymbol { name: "arrow.uturn.backward.circle" };
pub const ARROW_UTURN_BACKWARD_CIRCLE_BADGE_ELLIPSIS: SFSymbol = SFSymbol { name: "arrow.uturn.backward.circle.badge.ellipsis" };
pub const ARROW_UTURN_BACKWARD_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.uturn.backward.circle.fill" };
pub const ARROW_UTURN_BACKWARD_SQUARE: SFSymbol = SFSymbol { name: "arrow.uturn.backward.square" };
pub const ARROW_UTURN_BACKWARD_SQUARE_FILL: SFSymbol = SFSymbol { name: "arrow.uturn.backward.square.fill" };
pub const ARROW_UTURN_DOWN: SFSymbol = SFSymbol { name: "arrow.uturn.down" };
pub const ARROW_UTURN_DOWN_CIRCLE: SFSymbol = SFSymbol { name: "arrow.uturn.down.circle" };
pub const ARROW_UTURN_DOWN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.uturn.down.circle.fill" };
pub const ARROW_UTURN_DOWN_SQUARE: SFSymbol = SFSymbol { name: "arrow.uturn.down.square" };
pub const ARROW_UTURN_DOWN_SQUARE_FILL: SFSymbol = SFSymbol { name: "arrow.uturn.down.square.fill" };
pub const ARROW_UTURN_FORWARD: SFSymbol = SFSymbol { name: "arrow.uturn.forward" };
pub const ARROW_UTURN_FORWARD_CIRCLE: SFSymbol = SFSymbol { name: "arrow.uturn.forward.circle" };
pub const ARROW_UTURN_FORWARD_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.uturn.forward.circle.fill" };
pub const ARROW_UTURN_FORWARD_SQUARE: SFSymbol = SFSymbol { name: "arrow.uturn.forward.square" };
pub const ARROW_UTURN_FORWARD_SQUARE_FILL: SFSymbol = SFSymbol { name: "arrow.uturn.forward.square.fill" };
pub const ARROW_UTURN_LEFT: SFSymbol = SFSymbol { name: "arrow.uturn.left" };
pub const ARROW_UTURN_LEFT_CIRCLE: SFSymbol = SFSymbol { name: "arrow.uturn.left.circle" };
pub const ARROW_UTURN_LEFT_CIRCLE_BADGE_ELLIPSIS: SFSymbol = SFSymbol { name: "arrow.uturn.left.circle.badge.ellipsis" };
pub const ARROW_UTURN_LEFT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.uturn.left.circle.fill" };
pub const ARROW_UTURN_LEFT_SQUARE: SFSymbol = SFSymbol { name: "arrow.uturn.left.square" };
pub const ARROW_UTURN_LEFT_SQUARE_FILL: SFSymbol = SFSymbol { name: "arrow.uturn.left.square.fill" };
pub const ARROW_UTURN_RIGHT: SFSymbol = SFSymbol { name: "arrow.uturn.right" };
pub const ARROW_UTURN_RIGHT_CIRCLE: SFSymbol = SFSymbol { name: "arrow.uturn.right.circle" };
pub const ARROW_UTURN_RIGHT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.uturn.right.circle.fill" };
pub const ARROW_UTURN_RIGHT_SQUARE: SFSymbol = SFSymbol { name: "arrow.uturn.right.square" };
pub const ARROW_UTURN_RIGHT_SQUARE_FILL: SFSymbol = SFSymbol { name: "arrow.uturn.right.square.fill" };
pub const ARROW_UTURN_UP: SFSymbol = SFSymbol { name: "arrow.uturn.up" };
pub const ARROW_UTURN_UP_CIRCLE: SFSymbol = SFSymbol { name: "arrow.uturn.up.circle" };
pub const ARROW_UTURN_UP_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrow.uturn.up.circle.fill" };
pub const ARROW_UTURN_UP_SQUARE: SFSymbol = SFSymbol { name: "arrow.uturn.up.square" };
pub const ARROW_UTURN_UP_SQUARE_FILL: SFSymbol = SFSymbol { name: "arrow.uturn.up.square.fill" };
pub const ARROWSHAPE_BACKWARD: SFSymbol = SFSymbol { name: "arrowshape.backward" };
pub const ARROWSHAPE_BACKWARD_FILL: SFSymbol = SFSymbol { name: "arrowshape.backward.fill" };
pub const ARROWSHAPE_BOUNCE_FORWARD: SFSymbol = SFSymbol { name: "arrowshape.bounce.forward" };
pub const ARROWSHAPE_BOUNCE_FORWARD_FILL: SFSymbol = SFSymbol { name: "arrowshape.bounce.forward.fill" };
pub const ARROWSHAPE_BOUNCE_RIGHT: SFSymbol = SFSymbol { name: "arrowshape.bounce.right" };
pub const ARROWSHAPE_BOUNCE_RIGHT_FILL: SFSymbol = SFSymbol { name: "arrowshape.bounce.right.fill" };
pub const ARROWSHAPE_FORWARD: SFSymbol = SFSymbol { name: "arrowshape.forward" };
pub const ARROWSHAPE_FORWARD_FILL: SFSymbol = SFSymbol { name: "arrowshape.forward.fill" };
pub const ARROWSHAPE_LEFT: SFSymbol = SFSymbol { name: "arrowshape.left" };
pub const ARROWSHAPE_LEFT_FILL: SFSymbol = SFSymbol { name: "arrowshape.left.fill" };
pub const ARROWSHAPE_RIGHT: SFSymbol = SFSymbol { name: "arrowshape.right" };
pub const ARROWSHAPE_RIGHT_FILL: SFSymbol = SFSymbol { name: "arrowshape.right.fill" };
pub const ARROWSHAPE_TURN_UP_BACKWARD: SFSymbol = SFSymbol { name: "arrowshape.turn.up.backward" };
pub const ARROWSHAPE_TURN_UP_BACKWARD_2: SFSymbol = SFSymbol { name: "arrowshape.turn.up.backward.2" };
pub const ARROWSHAPE_TURN_UP_BACKWARD_2_CIRCLE: SFSymbol = SFSymbol { name: "arrowshape.turn.up.backward.2.circle" };
pub const ARROWSHAPE_TURN_UP_BACKWARD_2_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrowshape.turn.up.backward.2.circle.fill" };
pub const ARROWSHAPE_TURN_UP_BACKWARD_2_FILL: SFSymbol = SFSymbol { name: "arrowshape.turn.up.backward.2.fill" };
pub const ARROWSHAPE_TURN_UP_BACKWARD_BADGE_CLOCK: SFSymbol = SFSymbol { name: "arrowshape.turn.up.backward.badge.clock" };
pub const ARROWSHAPE_TURN_UP_BACKWARD_BADGE_CLOCK_FILL: SFSymbol = SFSymbol { name: "arrowshape.turn.up.backward.badge.clock.fill" };
pub const ARROWSHAPE_TURN_UP_BACKWARD_CIRCLE: SFSymbol = SFSymbol { name: "arrowshape.turn.up.backward.circle" };
pub const ARROWSHAPE_TURN_UP_BACKWARD_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrowshape.turn.up.backward.circle.fill" };
pub const ARROWSHAPE_TURN_UP_BACKWARD_FILL: SFSymbol = SFSymbol { name: "arrowshape.turn.up.backward.fill" };
pub const ARROWSHAPE_TURN_UP_FORWARD: SFSymbol = SFSymbol { name: "arrowshape.turn.up.forward" };
pub const ARROWSHAPE_TURN_UP_FORWARD_CIRCLE: SFSymbol = SFSymbol { name: "arrowshape.turn.up.forward.circle" };
pub const ARROWSHAPE_TURN_UP_FORWARD_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrowshape.turn.up.forward.circle.fill" };
pub const ARROWSHAPE_TURN_UP_FORWARD_FILL: SFSymbol = SFSymbol { name: "arrowshape.turn.up.forward.fill" };
pub const ARROWSHAPE_TURN_UP_LEFT: SFSymbol = SFSymbol { name: "arrowshape.turn.up.left" };
pub const ARROWSHAPE_TURN_UP_LEFT_2: SFSymbol = SFSymbol { name: "arrowshape.turn.up.left.2" };
pub const ARROWSHAPE_TURN_UP_LEFT_2_CIRCLE: SFSymbol = SFSymbol { name: "arrowshape.turn.up.left.2.circle" };
pub const ARROWSHAPE_TURN_UP_LEFT_2_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrowshape.turn.up.left.2.circle.fill" };
pub const ARROWSHAPE_TURN_UP_LEFT_2_FILL: SFSymbol = SFSymbol { name: "arrowshape.turn.up.left.2.fill" };
pub const ARROWSHAPE_TURN_UP_LEFT_CIRCLE: SFSymbol = SFSymbol { name: "arrowshape.turn.up.left.circle" };
pub const ARROWSHAPE_TURN_UP_LEFT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrowshape.turn.up.left.circle.fill" };
pub const ARROWSHAPE_TURN_UP_LEFT_FILL: SFSymbol = SFSymbol { name: "arrowshape.turn.up.left.fill" };
pub const ARROWSHAPE_TURN_UP_RIGHT: SFSymbol = SFSymbol { name: "arrowshape.turn.up.right" };
pub const ARROWSHAPE_TURN_UP_RIGHT_CIRCLE: SFSymbol = SFSymbol { name: "arrowshape.turn.up.right.circle" };
pub const ARROWSHAPE_TURN_UP_RIGHT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrowshape.turn.up.right.circle.fill" };
pub const ARROWSHAPE_TURN_UP_RIGHT_FILL: SFSymbol = SFSymbol { name: "arrowshape.turn.up.right.fill" };
pub const ARROWSHAPE_ZIGZAG_FORWARD: SFSymbol = SFSymbol { name: "arrowshape.zigzag.forward" };
pub const ARROWSHAPE_ZIGZAG_FORWARD_FILL: SFSymbol = SFSymbol { name: "arrowshape.zigzag.forward.fill" };
pub const ARROWSHAPE_ZIGZAG_RIGHT: SFSymbol = SFSymbol { name: "arrowshape.zigzag.right" };
pub const ARROWSHAPE_ZIGZAG_RIGHT_FILL: SFSymbol = SFSymbol { name: "arrowshape.zigzag.right.fill" };
pub const ARROWTRIANGLE_BACKWARD: SFSymbol = SFSymbol { name: "arrowtriangle.backward" };
pub const ARROWTRIANGLE_BACKWARD_CIRCLE: SFSymbol = SFSymbol { name: "arrowtriangle.backward.circle" };
pub const ARROWTRIANGLE_BACKWARD_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrowtriangle.backward.circle.fill" };
pub const ARROWTRIANGLE_BACKWARD_FILL: SFSymbol = SFSymbol { name: "arrowtriangle.backward.fill" };
pub const ARROWTRIANGLE_BACKWARD_SQUARE: SFSymbol = SFSymbol { name: "arrowtriangle.backward.square" };
pub const ARROWTRIANGLE_BACKWARD_SQUARE_FILL: SFSymbol = SFSymbol { name: "arrowtriangle.backward.square.fill" };
pub const ARROWTRIANGLE_DOWN: SFSymbol = SFSymbol { name: "arrowtriangle.down" };
pub const ARROWTRIANGLE_DOWN_CIRCLE: SFSymbol = SFSymbol { name: "arrowtriangle.down.circle" };
pub const ARROWTRIANGLE_DOWN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrowtriangle.down.circle.fill" };
pub const ARROWTRIANGLE_DOWN_FILL: SFSymbol = SFSymbol { name: "arrowtriangle.down.fill" };
pub const ARROWTRIANGLE_DOWN_SQUARE: SFSymbol = SFSymbol { name: "arrowtriangle.down.square" };
pub const ARROWTRIANGLE_DOWN_SQUARE_FILL: SFSymbol = SFSymbol { name: "arrowtriangle.down.square.fill" };
pub const ARROWTRIANGLE_FORWARD: SFSymbol = SFSymbol { name: "arrowtriangle.forward" };
pub const ARROWTRIANGLE_FORWARD_CIRCLE: SFSymbol = SFSymbol { name: "arrowtriangle.forward.circle" };
pub const ARROWTRIANGLE_FORWARD_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrowtriangle.forward.circle.fill" };
pub const ARROWTRIANGLE_FORWARD_FILL: SFSymbol = SFSymbol { name: "arrowtriangle.forward.fill" };
pub const ARROWTRIANGLE_FORWARD_SQUARE: SFSymbol = SFSymbol { name: "arrowtriangle.forward.square" };
pub const ARROWTRIANGLE_FORWARD_SQUARE_FILL: SFSymbol = SFSymbol { name: "arrowtriangle.forward.square.fill" };
pub const ARROWTRIANGLE_LEFT: SFSymbol = SFSymbol { name: "arrowtriangle.left" };
pub const ARROWTRIANGLE_LEFT_AND_LINE_VERTICAL_AND_ARROWTRIANGLE_RIGHT: SFSymbol = SFSymbol { name: "arrowtriangle.left.and.line.vertical.and.arrowtriangle.right" };
pub const ARROWTRIANGLE_LEFT_AND_LINE_VERTICAL_AND_ARROWTRIANGLE_RIGHT_FILL: SFSymbol = SFSymbol { name: "arrowtriangle.left.and.line.vertical.and.arrowtriangle.right.fill" };
pub const ARROWTRIANGLE_LEFT_CIRCLE: SFSymbol = SFSymbol { name: "arrowtriangle.left.circle" };
pub const ARROWTRIANGLE_LEFT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrowtriangle.left.circle.fill" };
pub const ARROWTRIANGLE_LEFT_FILL: SFSymbol = SFSymbol { name: "arrowtriangle.left.fill" };
pub const ARROWTRIANGLE_LEFT_SQUARE: SFSymbol = SFSymbol { name: "arrowtriangle.left.square" };
pub const ARROWTRIANGLE_LEFT_SQUARE_FILL: SFSymbol = SFSymbol { name: "arrowtriangle.left.square.fill" };
pub const ARROWTRIANGLE_RIGHT: SFSymbol = SFSymbol { name: "arrowtriangle.right" };
pub const ARROWTRIANGLE_RIGHT_AND_LINE_VERTICAL_AND_ARROWTRIANGLE_LEFT: SFSymbol = SFSymbol { name: "arrowtriangle.right.and.line.vertical.and.arrowtriangle.left" };
pub const ARROWTRIANGLE_RIGHT_AND_LINE_VERTICAL_AND_ARROWTRIANGLE_LEFT_FILL: SFSymbol = SFSymbol { name: "arrowtriangle.right.and.line.vertical.and.arrowtriangle.left.fill" };
pub const ARROWTRIANGLE_RIGHT_CIRCLE: SFSymbol = SFSymbol { name: "arrowtriangle.right.circle" };
pub const ARROWTRIANGLE_RIGHT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrowtriangle.right.circle.fill" };
pub const ARROWTRIANGLE_RIGHT_FILL: SFSymbol = SFSymbol { name: "arrowtriangle.right.fill" };
pub const ARROWTRIANGLE_RIGHT_SQUARE: SFSymbol = SFSymbol { name: "arrowtriangle.right.square" };
pub const ARROWTRIANGLE_RIGHT_SQUARE_FILL: SFSymbol = SFSymbol { name: "arrowtriangle.right.square.fill" };
pub const ARROWTRIANGLE_UP: SFSymbol = SFSymbol { name: "arrowtriangle.up" };
pub const ARROWTRIANGLE_UP_CIRCLE: SFSymbol = SFSymbol { name: "arrowtriangle.up.circle" };
pub const ARROWTRIANGLE_UP_CIRCLE_FILL: SFSymbol = SFSymbol { name: "arrowtriangle.up.circle.fill" };
pub const ARROWTRIANGLE_UP_FILL: SFSymbol = SFSymbol { name: "arrowtriangle.up.fill" };
pub const ARROWTRIANGLE_UP_SQUARE: SFSymbol = SFSymbol { name: "arrowtriangle.up.square" };
pub const ARROWTRIANGLE_UP_SQUARE_FILL: SFSymbol = SFSymbol { name: "arrowtriangle.up.square.fill" };
pub const ASPECTRATIO: SFSymbol = SFSymbol { name: "aspectratio" };
pub const ASPECTRATIO_FILL: SFSymbol = SFSymbol { name: "aspectratio.fill" };
pub const ASTERISK: SFSymbol = SFSymbol { name: "asterisk" };
pub const ASTERISK_CIRCLE: SFSymbol = SFSymbol { name: "asterisk.circle" };
pub const ASTERISK_CIRCLE_FILL: SFSymbol = SFSymbol { name: "asterisk.circle.fill" };
pub const AT: SFSymbol = SFSymbol { name: "at" };
pub const AT_BADGE_MINUS: SFSymbol = SFSymbol { name: "at.badge.minus" };
pub const AT_BADGE_PLUS: SFSymbol = SFSymbol { name: "at.badge.plus" };
pub const AT_CIRCLE: SFSymbol = SFSymbol { name: "at.circle" };
pub const AT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "at.circle.fill" };
pub const ATOM: SFSymbol = SFSymbol { name: "atom" };
pub const AUSTRALSIGN: SFSymbol = SFSymbol { name: "australsign" };
pub const AUSTRALSIGN_CIRCLE: SFSymbol = SFSymbol { name: "australsign.circle" };
pub const AUSTRALSIGN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "australsign.circle.fill" };
pub const AUSTRALSIGN_SQUARE: SFSymbol = SFSymbol { name: "australsign.square" };
pub const AUSTRALSIGN_SQUARE_FILL: SFSymbol = SFSymbol { name: "australsign.square.fill" };
pub const AV_REMOTE: SFSymbol = SFSymbol { name: "av.remote" };
pub const AV_REMOTE_FILL: SFSymbol = SFSymbol { name: "av.remote.fill" };
pub const B_CIRCLE: SFSymbol = SFSymbol { name: "b.circle" };
pub const B_CIRCLE_FILL: SFSymbol = SFSymbol { name: "b.circle.fill" };
pub const B_SQUARE: SFSymbol = SFSymbol { name: "b.square" };
pub const B_SQUARE_FILL: SFSymbol = SFSymbol { name: "b.square.fill" };
pub const BACKPACK: SFSymbol = SFSymbol { name: "backpack" };
pub const BACKPACK_FILL: SFSymbol = SFSymbol { name: "backpack.fill" };
pub const BACKWARD: SFSymbol = SFSymbol { name: "backward" };
pub const BACKWARD_CIRCLE: SFSymbol = SFSymbol { name: "backward.circle" };
pub const BACKWARD_CIRCLE_FILL: SFSymbol = SFSymbol { name: "backward.circle.fill" };
pub const BACKWARD_END: SFSymbol = SFSymbol { name: "backward.end" };
pub const BACKWARD_END_ALT: SFSymbol = SFSymbol { name: "backward.end.alt" };
pub const BACKWARD_END_ALT_FILL: SFSymbol = SFSymbol { name: "backward.end.alt.fill" };
pub const BACKWARD_END_CIRCLE: SFSymbol = SFSymbol { name: "backward.end.circle" };
pub const BACKWARD_END_CIRCLE_FILL: SFSymbol = SFSymbol { name: "backward.end.circle.fill" };
pub const BACKWARD_END_FILL: SFSymbol = SFSymbol { name: "backward.end.fill" };
pub const BACKWARD_FILL: SFSymbol = SFSymbol { name: "backward.fill" };
pub const BACKWARD_FRAME: SFSymbol = SFSymbol { name: "backward.frame" };
pub const BACKWARD_FRAME_FILL: SFSymbol = SFSymbol { name: "backward.frame.fill" };
pub const BADGE_PLUS_RADIOWAVES_FORWARD: SFSymbol = SFSymbol { name: "badge.plus.radiowaves.forward" };
pub const BADGE_PLUS_RADIOWAVES_RIGHT: SFSymbol = SFSymbol { name: "badge.plus.radiowaves.right" };
pub const BAG: SFSymbol = SFSymbol { name: "bag" };
pub const BAG_BADGE_MINUS: SFSymbol = SFSymbol { name: "bag.badge.minus" };
pub const BAG_BADGE_PLUS: SFSymbol = SFSymbol { name: "bag.badge.plus" };
pub const BAG_BADGE_QUESTIONMARK: SFSymbol = SFSymbol { name: "bag.badge.questionmark" };
pub const BAG_CIRCLE: SFSymbol = SFSymbol { name: "bag.circle" };
pub const BAG_CIRCLE_FILL: SFSymbol = SFSymbol { name: "bag.circle.fill" };
pub const BAG_FILL: SFSymbol = SFSymbol { name: "bag.fill" };
pub const BAG_FILL_BADGE_MINUS: SFSymbol = SFSymbol { name: "bag.fill.badge.minus" };
pub const BAG_FILL_BADGE_PLUS: SFSymbol = SFSymbol { name: "bag.fill.badge.plus" };
pub const BAG_FILL_BADGE_QUESTIONMARK: SFSymbol = SFSymbol { name: "bag.fill.badge.questionmark" };
pub const BAHTSIGN: SFSymbol = SFSymbol { name: "bahtsign" };
pub const BAHTSIGN_CIRCLE: SFSymbol = SFSymbol { name: "bahtsign.circle" };
pub const BAHTSIGN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "bahtsign.circle.fill" };
pub const BAHTSIGN_SQUARE: SFSymbol = SFSymbol { name: "bahtsign.square" };
pub const BAHTSIGN_SQUARE_FILL: SFSymbol = SFSymbol { name: "bahtsign.square.fill" };
pub const BALLOON: SFSymbol = SFSymbol { name: "balloon" };
pub const BALLOON_2: SFSymbol = SFSymbol { name: "balloon.2" };
pub const BALLOON_2_FILL: SFSymbol = SFSymbol { name: "balloon.2.fill" };
pub const BALLOON_FILL: SFSymbol = SFSymbol { name: "balloon.fill" };
pub const BANDAGE: SFSymbol = SFSymbol { name: "bandage" };
pub const BANDAGE_FILL: SFSymbol = SFSymbol { name: "bandage.fill" };
pub const BANKNOTE: SFSymbol = SFSymbol { name: "banknote" };
pub const BANKNOTE_FILL: SFSymbol = SFSymbol { name: "banknote.fill" };
pub const BARCODE: SFSymbol = SFSymbol { name: "barcode" };
pub const BARCODE_VIEWFINDER: SFSymbol = SFSymbol { name: "barcode.viewfinder" };
pub const BAROMETER: SFSymbol = SFSymbol { name: "barometer" };
pub const BASEBALL: SFSymbol = SFSymbol { name: "baseball" };
pub const BASEBALL_CIRCLE: SFSymbol = SFSymbol { name: "baseball.circle" };
pub const BASEBALL_CIRCLE_FILL: SFSymbol = SFSymbol { name: "baseball.circle.fill" };
pub const BASEBALL_DIAMOND_BASES: SFSymbol = SFSymbol { name: "baseball.diamond.bases" };
pub const BASEBALL_FILL: SFSymbol = SFSymbol { name: "baseball.fill" };
pub const BASKET: SFSymbol = SFSymbol { name: "basket" };
pub const BASKET_FILL: SFSymbol = SFSymbol { name: "basket.fill" };
pub const BASKETBALL: SFSymbol = SFSymbol { name: "basketball" };
pub const BASKETBALL_CIRCLE: SFSymbol = SFSymbol { name: "basketball.circle" };
pub const BASKETBALL_CIRCLE_FILL: SFSymbol = SFSymbol { name: "basketball.circle.fill" };
pub const BASKETBALL_FILL: SFSymbol = SFSymbol { name: "basketball.fill" };
pub const BATHTUB: SFSymbol = SFSymbol { name: "bathtub" };
pub const BATHTUB_FILL: SFSymbol = SFSymbol { name: "bathtub.fill" };
pub const BATTERY_0: SFSymbol = SFSymbol { name: "battery.0" };
pub const BATTERY_100: SFSymbol = SFSymbol { name: "battery.100" };
pub const BATTERY_100_BOLT: SFSymbol = SFSymbol { name: "battery.100.bolt" };
pub const BATTERY_100_CIRCLE: SFSymbol = SFSymbol { name: "battery.100.circle" };
pub const BATTERY_100_CIRCLE_FILL: SFSymbol = SFSymbol { name: "battery.100.circle.fill" };
pub const BATTERY_25: SFSymbol = SFSymbol { name: "battery.25" };
pub const BATTERY_50: SFSymbol = SFSymbol { name: "battery.50" };
pub const BATTERY_75: SFSymbol = SFSymbol { name: "battery.75" };
pub const BEACH_UMBRELLA: SFSymbol = SFSymbol { name: "beach.umbrella" };
pub const BEACH_UMBRELLA_FILL: SFSymbol = SFSymbol { name: "beach.umbrella.fill" };
pub const BEATS_EARPHONES: SFSymbol = SFSymbol { name: "beats.earphones" };
pub const BEATS_FIT_PRO: SFSymbol = SFSymbol { name: "beats.fit.pro" };
pub const BEATS_FIT_PRO_CHARGINGCASE: SFSymbol = SFSymbol { name: "beats.fit.pro.chargingcase" };
pub const BEATS_FIT_PRO_CHARGINGCASE_FILL: SFSymbol = SFSymbol { name: "beats.fit.pro.chargingcase.fill" };
pub const BEATS_FIT_PRO_LEFT: SFSymbol = SFSymbol { name: "beats.fit.pro.left" };
pub const BEATS_FIT_PRO_RIGHT: SFSymbol = SFSymbol { name: "beats.fit.pro.right" };
pub const BEATS_HEADPHONES: SFSymbol = SFSymbol { name: "beats.headphones" };
pub const BEATS_POWERBEATS: SFSymbol = SFSymbol { name: "beats.powerbeats" };
pub const BEATS_POWERBEATS3: SFSymbol = SFSymbol { name: "beats.powerbeats3" };
pub const BEATS_POWERBEATSPRO: SFSymbol = SFSymbol { name: "beats.powerbeatspro" };
pub const BEATS_POWERBEATSPRO_CHARGINGCASE: SFSymbol = SFSymbol { name: "beats.powerbeatspro.chargingcase" };
pub const BEATS_POWERBEATSPRO_CHARGINGCASE_FILL: SFSymbol = SFSymbol { name: "beats.powerbeatspro.chargingcase.fill" };
pub const BEATS_POWERBEATSPRO_LEFT: SFSymbol = SFSymbol { name: "beats.powerbeatspro.left" };
pub const BEATS_POWERBEATSPRO_RIGHT: SFSymbol = SFSymbol { name: "beats.powerbeatspro.right" };
pub const BEATS_STUDIOBUD_LEFT: SFSymbol = SFSymbol { name: "beats.studiobud.left" };
pub const BEATS_STUDIOBUD_RIGHT: SFSymbol = SFSymbol { name: "beats.studiobud.right" };
pub const BEATS_STUDIOBUDS: SFSymbol = SFSymbol { name: "beats.studiobuds" };
pub const BEATS_STUDIOBUDS_CHARGINGCASE: SFSymbol = SFSymbol { name: "beats.studiobuds.chargingcase" };
pub const BEATS_STUDIOBUDS_CHARGINGCASE_FILL: SFSymbol = SFSymbol { name: "beats.studiobuds.chargingcase.fill" };
pub const BED_DOUBLE: SFSymbol = SFSymbol { name: "bed.double" };
pub const BED_DOUBLE_CIRCLE: SFSymbol = SFSymbol { name: "bed.double.circle" };
pub const BED_DOUBLE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "bed.double.circle.fill" };
pub const BED_DOUBLE_FILL: SFSymbol = SFSymbol { name: "bed.double.fill" };
pub const BELL: SFSymbol = SFSymbol { name: "bell" };
pub const BELL_AND_WAVEFORM: SFSymbol = SFSymbol { name: "bell.and.waveform" };
pub const BELL_AND_WAVEFORM_FILL: SFSymbol = SFSymbol { name: "bell.and.waveform.fill" };
pub const BELL_AND_WAVES_LEFT_AND_RIGHT: SFSymbol = SFSymbol { name: "bell.and.waves.left.and.right" };
pub const BELL_AND_WAVES_LEFT_AND_RIGHT_FILL: SFSymbol = SFSymbol { name: "bell.and.waves.left.and.right.fill" };
pub const BELL_BADGE: SFSymbol = SFSymbol { name: "bell.badge" };
pub const BELL_BADGE_CIRCLE: SFSymbol = SFSymbol { name: "bell.badge.circle" };
pub const BELL_BADGE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "bell.badge.circle.fill" };
pub const BELL_BADGE_FILL: SFSymbol = SFSymbol { name: "bell.badge.fill" };
pub const BELL_CIRCLE: SFSymbol = SFSymbol { name: "bell.circle" };
pub const BELL_CIRCLE_FILL: SFSymbol = SFSymbol { name: "bell.circle.fill" };
pub const BELL_FILL: SFSymbol = SFSymbol { name: "bell.fill" };
pub const BELL_SLASH: SFSymbol = SFSymbol { name: "bell.slash" };
pub const BELL_SLASH_CIRCLE: SFSymbol = SFSymbol { name: "bell.slash.circle" };
pub const BELL_SLASH_CIRCLE_FILL: SFSymbol = SFSymbol { name: "bell.slash.circle.fill" };
pub const BELL_SLASH_FILL: SFSymbol = SFSymbol { name: "bell.slash.fill" };
pub const BELL_SQUARE: SFSymbol = SFSymbol { name: "bell.square" };
pub const BELL_SQUARE_FILL: SFSymbol = SFSymbol { name: "bell.square.fill" };
pub const BICYCLE: SFSymbol = SFSymbol { name: "bicycle" };
pub const BICYCLE_CIRCLE: SFSymbol = SFSymbol { name: "bicycle.circle" };
pub const BICYCLE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "bicycle.circle.fill" };
pub const BINOCULARS: SFSymbol = SFSymbol { name: "binoculars" };
pub const BINOCULARS_FILL: SFSymbol = SFSymbol { name: "binoculars.fill" };
pub const BIRD: SFSymbol = SFSymbol { name: "bird" };
pub const BIRD_FILL: SFSymbol = SFSymbol { name: "bird.fill" };
pub const BIRTHDAY_CAKE: SFSymbol = SFSymbol { name: "birthday.cake" };
pub const BIRTHDAY_CAKE_FILL: SFSymbol = SFSymbol { name: "birthday.cake.fill" };
pub const BITCOINSIGN: SFSymbol = SFSymbol { name: "bitcoinsign" };
pub const BITCOINSIGN_CIRCLE: SFSymbol = SFSymbol { name: "bitcoinsign.circle" };
pub const BITCOINSIGN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "bitcoinsign.circle.fill" };
pub const BITCOINSIGN_SQUARE: SFSymbol = SFSymbol { name: "bitcoinsign.square" };
pub const BITCOINSIGN_SQUARE_FILL: SFSymbol = SFSymbol { name: "bitcoinsign.square.fill" };
pub const BLINDS_HORIZONTAL_CLOSED: SFSymbol = SFSymbol { name: "blinds.horizontal.closed" };
pub const BLINDS_HORIZONTAL_OPEN: SFSymbol = SFSymbol { name: "blinds.horizontal.open" };
pub const BLINDS_VERTICAL_CLOSED: SFSymbol = SFSymbol { name: "blinds.vertical.closed" };
pub const BLINDS_VERTICAL_OPEN: SFSymbol = SFSymbol { name: "blinds.vertical.open" };
pub const BOLD: SFSymbol = SFSymbol { name: "bold" };
pub const BOLD_ITALIC_UNDERLINE: SFSymbol = SFSymbol { name: "bold.italic.underline" };
pub const BOLD_UNDERLINE: SFSymbol = SFSymbol { name: "bold.underline" };
pub const BOLT: SFSymbol = SFSymbol { name: "bolt" };
pub const BOLT_BADGE_A: SFSymbol = SFSymbol { name: "bolt.badge.a" };
pub const BOLT_BADGE_A_FILL: SFSymbol = SFSymbol { name: "bolt.badge.a.fill" };
pub const BOLT_BADGE_CLOCK: SFSymbol = SFSymbol { name: "bolt.badge.clock" };
pub const BOLT_BADGE_CLOCK_FILL: SFSymbol = SFSymbol { name: "bolt.badge.clock.fill" };
pub const BOLT_BATTERYBLOCK: SFSymbol = SFSymbol { name: "bolt.batteryblock" };
pub const BOLT_BATTERYBLOCK_FILL: SFSymbol = SFSymbol { name: "bolt.batteryblock.fill" };
pub const BOLT_CAR: SFSymbol = SFSymbol { name: "bolt.car" };
pub const BOLT_CAR_CIRCLE: SFSymbol = SFSymbol { name: "bolt.car.circle" };
pub const BOLT_CAR_CIRCLE_FILL: SFSymbol = SFSymbol { name: "bolt.car.circle.fill" };
pub const BOLT_CAR_FILL: SFSymbol = SFSymbol { name: "bolt.car.fill" };
pub const BOLT_CIRCLE: SFSymbol = SFSymbol { name: "bolt.circle" };
pub const BOLT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "bolt.circle.fill" };
pub const BOLT_FILL: SFSymbol = SFSymbol { name: "bolt.fill" };
pub const BOLT_HEART: SFSymbol = SFSymbol { name: "bolt.heart" };
pub const BOLT_HEART_FILL: SFSymbol = SFSymbol { name: "bolt.heart.fill" };
pub const BOLT_HORIZONTAL: SFSymbol = SFSymbol { name: "bolt.horizontal" };
pub const BOLT_HORIZONTAL_CIRCLE: SFSymbol = SFSymbol { name: "bolt.horizontal.circle" };
pub const BOLT_HORIZONTAL_CIRCLE_FILL: SFSymbol = SFSymbol { name: "bolt.horizontal.circle.fill" };
pub const BOLT_HORIZONTAL_FILL: SFSymbol = SFSymbol { name: "bolt.horizontal.fill" };
pub const BOLT_HORIZONTAL_ICLOUD: SFSymbol = SFSymbol { name: "bolt.horizontal.icloud" };
pub const BOLT_HORIZONTAL_ICLOUD_FILL: SFSymbol = SFSymbol { name: "bolt.horizontal.icloud.fill" };
pub const BOLT_RING_CLOSED: SFSymbol = SFSymbol { name: "bolt.ring.closed" };
pub const BOLT_SHIELD: SFSymbol = SFSymbol { name: "bolt.shield" };
pub const BOLT_SHIELD_FILL: SFSymbol = SFSymbol { name: "bolt.shield.fill" };
pub const BOLT_SLASH: SFSymbol = SFSymbol { name: "bolt.slash" };
pub const BOLT_SLASH_CIRCLE: SFSymbol = SFSymbol { name: "bolt.slash.circle" };
pub const BOLT_SLASH_CIRCLE_FILL: SFSymbol = SFSymbol { name: "bolt.slash.circle.fill" };
pub const BOLT_SLASH_FILL: SFSymbol = SFSymbol { name: "bolt.slash.fill" };
pub const BOLT_SQUARE: SFSymbol = SFSymbol { name: "bolt.square" };
pub const BOLT_SQUARE_FILL: SFSymbol = SFSymbol { name: "bolt.square.fill" };
pub const BONJOUR: SFSymbol = SFSymbol { name: "bonjour" };
pub const BOOK: SFSymbol = SFSymbol { name: "book" };
pub const BOOK_CIRCLE: SFSymbol = SFSymbol { name: "book.circle" };
pub const BOOK_CIRCLE_FILL: SFSymbol = SFSymbol { name: "book.circle.fill" };
pub const BOOK_CLOSED: SFSymbol = SFSymbol { name: "book.closed" };
pub const BOOK_CLOSED_CIRCLE: SFSymbol = SFSymbol { name: "book.closed.circle" };
pub const BOOK_CLOSED_CIRCLE_FILL: SFSymbol = SFSymbol { name: "book.closed.circle.fill" };
pub const BOOK_CLOSED_FILL: SFSymbol = SFSymbol { name: "book.closed.fill" };
pub const BOOK_FILL: SFSymbol = SFSymbol { name: "book.fill" };
pub const BOOKMARK: SFSymbol = SFSymbol { name: "bookmark" };
pub const BOOKMARK_CIRCLE: SFSymbol = SFSymbol { name: "bookmark.circle" };
pub const BOOKMARK_CIRCLE_FILL: SFSymbol = SFSymbol { name: "bookmark.circle.fill" };
pub const BOOKMARK_FILL: SFSymbol = SFSymbol { name: "bookmark.fill" };
pub const BOOKMARK_SLASH: SFSymbol = SFSymbol { name: "bookmark.slash" };
pub const BOOKMARK_SLASH_FILL: SFSymbol = SFSymbol { name: "bookmark.slash.fill" };
pub const BOOKMARK_SQUARE: SFSymbol = SFSymbol { name: "bookmark.square" };
pub const BOOKMARK_SQUARE_FILL: SFSymbol = SFSymbol { name: "bookmark.square.fill" };
pub const BOOKS_VERTICAL: SFSymbol = SFSymbol { name: "books.vertical" };
pub const BOOKS_VERTICAL_CIRCLE: SFSymbol = SFSymbol { name: "books.vertical.circle" };
pub const BOOKS_VERTICAL_CIRCLE_FILL: SFSymbol = SFSymbol { name: "books.vertical.circle.fill" };
pub const BOOKS_VERTICAL_FILL: SFSymbol = SFSymbol { name: "books.vertical.fill" };
pub const BOX_TRUCK: SFSymbol = SFSymbol { name: "box.truck" };
pub const BOX_TRUCK_BADGE_CLOCK: SFSymbol = SFSymbol { name: "box.truck.badge.clock" };
pub const BOX_TRUCK_BADGE_CLOCK_FILL: SFSymbol = SFSymbol { name: "box.truck.badge.clock.fill" };
pub const BOX_TRUCK_FILL: SFSymbol = SFSymbol { name: "box.truck.fill" };
pub const BRAIN: SFSymbol = SFSymbol { name: "brain" };
pub const BRAIN_HEAD_PROFILE: SFSymbol = SFSymbol { name: "brain.head.profile" };
pub const BRAZILIANREALSIGN: SFSymbol = SFSymbol { name: "brazilianrealsign" };
pub const BRAZILIANREALSIGN_CIRCLE: SFSymbol = SFSymbol { name: "brazilianrealsign.circle" };
pub const BRAZILIANREALSIGN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "brazilianrealsign.circle.fill" };
pub const BRAZILIANREALSIGN_SQUARE: SFSymbol = SFSymbol { name: "brazilianrealsign.square" };
pub const BRAZILIANREALSIGN_SQUARE_FILL: SFSymbol = SFSymbol { name: "brazilianrealsign.square.fill" };
pub const BRIEFCASE: SFSymbol = SFSymbol { name: "briefcase" };
pub const BRIEFCASE_CIRCLE: SFSymbol = SFSymbol { name: "briefcase.circle" };
pub const BRIEFCASE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "briefcase.circle.fill" };
pub const BRIEFCASE_FILL: SFSymbol = SFSymbol { name: "briefcase.fill" };
pub const BUBBLE_LEFT: SFSymbol = SFSymbol { name: "bubble.left" };
pub const BUBBLE_LEFT_AND_BUBBLE_RIGHT: SFSymbol = SFSymbol { name: "bubble.left.and.bubble.right" };
pub const BUBBLE_LEFT_AND_BUBBLE_RIGHT_FILL: SFSymbol = SFSymbol { name: "bubble.left.and.bubble.right.fill" };
pub const BUBBLE_LEFT_AND_EXCLAMATIONMARK_BUBBLE_RIGHT: SFSymbol = SFSymbol { name: "bubble.left.and.exclamationmark.bubble.right" };
pub const BUBBLE_LEFT_AND_EXCLAMATIONMARK_BUBBLE_RIGHT_FILL: SFSymbol = SFSymbol { name: "bubble.left.and.exclamationmark.bubble.right.fill" };
pub const BUBBLE_LEFT_CIRCLE: SFSymbol = SFSymbol { name: "bubble.left.circle" };
pub const BUBBLE_LEFT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "bubble.left.circle.fill" };
pub const BUBBLE_LEFT_FILL: SFSymbol = SFSymbol { name: "bubble.left.fill" };
pub const BUBBLE_MIDDLE_BOTTOM: SFSymbol = SFSymbol { name: "bubble.middle.bottom" };
pub const BUBBLE_MIDDLE_BOTTOM_FILL: SFSymbol = SFSymbol { name: "bubble.middle.bottom.fill" };
pub const BUBBLE_MIDDLE_TOP: SFSymbol = SFSymbol { name: "bubble.middle.top" };
pub const BUBBLE_MIDDLE_TOP_FILL: SFSymbol = SFSymbol { name: "bubble.middle.top.fill" };
pub const BUBBLE_RIGHT: SFSymbol = SFSymbol { name: "bubble.right" };
pub const BUBBLE_RIGHT_CIRCLE: SFSymbol = SFSymbol { name: "bubble.right.circle" };
pub const BUBBLE_RIGHT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "bubble.right.circle.fill" };
pub const BUBBLE_RIGHT_FILL: SFSymbol = SFSymbol { name: "bubble.right.fill" };
pub const BUBBLES_AND_SPARKLES: SFSymbol = SFSymbol { name: "bubbles.and.sparkles" };
pub const BUBBLES_AND_SPARKLES_FILL: SFSymbol = SFSymbol { name: "bubbles.and.sparkles.fill" };
pub const BUILDING: SFSymbol = SFSymbol { name: "building" };
pub const BUILDING_2: SFSymbol = SFSymbol { name: "building.2" };
pub const BUILDING_2_CROP_CIRCLE: SFSymbol = SFSymbol { name: "building.2.crop.circle" };
pub const BUILDING_2_CROP_CIRCLE_FILL: SFSymbol = SFSymbol { name: "building.2.crop.circle.fill" };
pub const BUILDING_2_FILL: SFSymbol = SFSymbol { name: "building.2.fill" };
pub const BUILDING_COLUMNS: SFSymbol = SFSymbol { name: "building.columns" };
pub const BUILDING_COLUMNS_CIRCLE: SFSymbol = SFSymbol { name: "building.columns.circle" };
pub const BUILDING_COLUMNS_CIRCLE_FILL: SFSymbol = SFSymbol { name: "building.columns.circle.fill" };
pub const BUILDING_COLUMNS_FILL: SFSymbol = SFSymbol { name: "building.columns.fill" };
pub const BUILDING_FILL: SFSymbol = SFSymbol { name: "building.fill" };
pub const BURN: SFSymbol = SFSymbol { name: "burn" };
pub const BURST: SFSymbol = SFSymbol { name: "burst" };
pub const BURST_FILL: SFSymbol = SFSymbol { name: "burst.fill" };
pub const BUS: SFSymbol = SFSymbol { name: "bus" };
pub const BUS_DOUBLEDECKER: SFSymbol = SFSymbol { name: "bus.doubledecker" };
pub const BUS_DOUBLEDECKER_FILL: SFSymbol = SFSymbol { name: "bus.doubledecker.fill" };
pub const BUS_FILL: SFSymbol = SFSymbol { name: "bus.fill" };
pub const BUTTON_PROGRAMMABLE: SFSymbol = SFSymbol { name: "button.programmable" };
pub const BUTTON_PROGRAMMABLE_SQUARE: SFSymbol = SFSymbol { name: "button.programmable.square" };
pub const BUTTON_PROGRAMMABLE_SQUARE_FILL: SFSymbol = SFSymbol { name: "button.programmable.square.fill" };
pub const C_CIRCLE: SFSymbol = SFSymbol { name: "c.circle" };
pub const C_CIRCLE_FILL: SFSymbol = SFSymbol { name: "c.circle.fill" };
pub const C_SQUARE: SFSymbol = SFSymbol { name: "c.square" };
pub const C_SQUARE_FILL: SFSymbol = SFSymbol { name: "c.square.fill" };
pub const CABINET: SFSymbol = SFSymbol { name: "cabinet" };
pub const CABINET_FILL: SFSymbol = SFSymbol { name: "cabinet.fill" };
pub const CABLE_CONNECTOR: SFSymbol = SFSymbol { name: "cable.connector" };
pub const CABLE_CONNECTOR_HORIZONTAL: SFSymbol = SFSymbol { name: "cable.connector.horizontal" };
pub const CABLECAR: SFSymbol = SFSymbol { name: "cablecar" };
pub const CABLECAR_FILL: SFSymbol = SFSymbol { name: "cablecar.fill" };
pub const CALENDAR: SFSymbol = SFSymbol { name: "calendar" };
pub const CALENDAR_BADGE_CLOCK: SFSymbol = SFSymbol { name: "calendar.badge.clock" };
pub const CALENDAR_BADGE_EXCLAMATIONMARK: SFSymbol = SFSymbol { name: "calendar.badge.exclamationmark" };
pub const CALENDAR_BADGE_MINUS: SFSymbol = SFSymbol { name: "calendar.badge.minus" };
pub const CALENDAR_BADGE_PLUS: SFSymbol = SFSymbol { name: "calendar.badge.plus" };
pub const CALENDAR_CIRCLE: SFSymbol = SFSymbol { name: "calendar.circle" };
pub const CALENDAR_CIRCLE_FILL: SFSymbol = SFSymbol { name: "calendar.circle.fill" };
pub const CALENDAR_DAY_TIMELINE_LEADING: SFSymbol = SFSymbol { name: "calendar.day.timeline.leading" };
pub const CALENDAR_DAY_TIMELINE_LEFT: SFSymbol = SFSymbol { name: "calendar.day.timeline.left" };
pub const CALENDAR_DAY_TIMELINE_RIGHT: SFSymbol = SFSymbol { name: "calendar.day.timeline.right" };
pub const CALENDAR_DAY_TIMELINE_TRAILING: SFSymbol = SFSymbol { name: "calendar.day.timeline.trailing" };
pub const CAMERA: SFSymbol = SFSymbol { name: "camera" };
pub const CAMERA_APERTURE: SFSymbol = SFSymbol { name: "camera.aperture" };
pub const CAMERA_BADGE_ELLIPSIS: SFSymbol = SFSymbol { name: "camera.badge.ellipsis" };
pub const CAMERA_CIRCLE: SFSymbol = SFSymbol { name: "camera.circle" };
pub const CAMERA_CIRCLE_FILL: SFSymbol = SFSymbol { name: "camera.circle.fill" };
pub const CAMERA_FILL: SFSymbol = SFSymbol { name: "camera.fill" };
pub const CAMERA_FILL_BADGE_ELLIPSIS: SFSymbol = SFSymbol { name: "camera.fill.badge.ellipsis" };
pub const CAMERA_FILTERS: SFSymbol = SFSymbol { name: "camera.filters" };
pub const CAMERA_MACRO: SFSymbol = SFSymbol { name: "camera.macro" };
pub const CAMERA_MACRO_CIRCLE: SFSymbol = SFSymbol { name: "camera.macro.circle" };
pub const CAMERA_MACRO_CIRCLE_FILL: SFSymbol = SFSymbol { name: "camera.macro.circle.fill" };
pub const CAMERA_METERING_CENTER_WEIGHTED: SFSymbol = SFSymbol { name: "camera.metering.center.weighted" };
pub const CAMERA_METERING_CENTER_WEIGHTED_AVERAGE: SFSymbol = SFSymbol { name: "camera.metering.center.weighted.average" };
pub const CAMERA_METERING_MATRIX: SFSymbol = SFSymbol { name: "camera.metering.matrix" };
pub const CAMERA_METERING_MULTISPOT: SFSymbol = SFSymbol { name: "camera.metering.multispot" };
pub const CAMERA_METERING_NONE: SFSymbol = SFSymbol { name: "camera.metering.none" };
pub const CAMERA_METERING_PARTIAL: SFSymbol = SFSymbol { name: "camera.metering.partial" };
pub const CAMERA_METERING_SPOT: SFSymbol = SFSymbol { name: "camera.metering.spot" };
pub const CAMERA_METERING_UNKNOWN: SFSymbol = SFSymbol { name: "camera.metering.unknown" };
pub const CAMERA_ON_RECTANGLE: SFSymbol = SFSymbol { name: "camera.on.rectangle" };
pub const CAMERA_ON_RECTANGLE_FILL: SFSymbol = SFSymbol { name: "camera.on.rectangle.fill" };
pub const CAMERA_SHUTTER_BUTTON: SFSymbol = SFSymbol { name: "camera.shutter.button" };
pub const CAMERA_SHUTTER_BUTTON_FILL: SFSymbol = SFSymbol { name: "camera.shutter.button.fill" };
pub const CAMERA_VIEWFINDER: SFSymbol = SFSymbol { name: "camera.viewfinder" };
pub const CANDYBARPHONE: SFSymbol = SFSymbol { name: "candybarphone" };
pub const CAPSLOCK: SFSymbol = SFSymbol { name: "capslock" };
pub const CAPSLOCK_FILL: SFSymbol = SFSymbol { name: "capslock.fill" };
pub const CAPSULE: SFSymbol = SFSymbol { name: "capsule" };
pub const CAPSULE_BOTTOMHALF_FILLED: SFSymbol = SFSymbol { name: "capsule.bottomhalf.filled" };
pub const CAPSULE_FILL: SFSymbol = SFSymbol { name: "capsule.fill" };
pub const CAPSULE_INSET_FILLED: SFSymbol = SFSymbol { name: "capsule.inset.filled" };
pub const CAPSULE_LEFTHALF_FILLED: SFSymbol = SFSymbol { name: "capsule.lefthalf.filled" };
pub const CAPSULE_PORTRAIT: SFSymbol = SFSymbol { name: "capsule.portrait" };
pub const CAPSULE_PORTRAIT_BOTTOMHALF_FILLED: SFSymbol = SFSymbol { name: "capsule.portrait.bottomhalf.filled" };
pub const CAPSULE_PORTRAIT_FILL: SFSymbol = SFSymbol { name: "capsule.portrait.fill" };
pub const CAPSULE_PORTRAIT_INSET_FILLED: SFSymbol = SFSymbol { name: "capsule.portrait.inset.filled" };
pub const CAPSULE_PORTRAIT_LEFTHALF_FILLED: SFSymbol = SFSymbol { name: "capsule.portrait.lefthalf.filled" };
pub const CAPSULE_PORTRAIT_RIGHTHALF_FILLED: SFSymbol = SFSymbol { name: "capsule.portrait.righthalf.filled" };
pub const CAPSULE_PORTRAIT_TOPHALF_FILLED: SFSymbol = SFSymbol { name: "capsule.portrait.tophalf.filled" };
pub const CAPSULE_RIGHTHALF_FILLED: SFSymbol = SFSymbol { name: "capsule.righthalf.filled" };
pub const CAPSULE_TOPHALF_FILLED: SFSymbol = SFSymbol { name: "capsule.tophalf.filled" };
pub const CAPTIONS_BUBBLE: SFSymbol = SFSymbol { name: "captions.bubble" };
pub const CAPTIONS_BUBBLE_FILL: SFSymbol = SFSymbol { name: "captions.bubble.fill" };
pub const CAR: SFSymbol = SFSymbol { name: "car" };
pub const CAR_2: SFSymbol = SFSymbol { name: "car.2" };
pub const CAR_2_FILL: SFSymbol = SFSymbol { name: "car.2.fill" };
pub const CAR_CIRCLE: SFSymbol = SFSymbol { name: "car.circle" };
pub const CAR_CIRCLE_FILL: SFSymbol = SFSymbol { name: "car.circle.fill" };
pub const CAR_FERRY: SFSymbol = SFSymbol { name: "car.ferry" };
pub const CAR_FERRY_FILL: SFSymbol = SFSymbol { name: "car.ferry.fill" };
pub const CAR_FILL: SFSymbol = SFSymbol { name: "car.fill" };
pub const CARBON_DIOXIDE_CLOUD: SFSymbol = SFSymbol { name: "carbon.dioxide.cloud" };
pub const CARBON_DIOXIDE_CLOUD_FILL: SFSymbol = SFSymbol { name: "carbon.dioxide.cloud.fill" };
pub const CARBON_MONOXIDE_CLOUD: SFSymbol = SFSymbol { name: "carbon.monoxide.cloud" };
pub const CARBON_MONOXIDE_CLOUD_FILL: SFSymbol = SFSymbol { name: "carbon.monoxide.cloud.fill" };
pub const CARROT: SFSymbol = SFSymbol { name: "carrot" };
pub const CARROT_FILL: SFSymbol = SFSymbol { name: "carrot.fill" };
pub const CART: SFSymbol = SFSymbol { name: "cart" };
pub const CART_BADGE_MINUS: SFSymbol = SFSymbol { name: "cart.badge.minus" };
pub const CART_BADGE_PLUS: SFSymbol = SFSymbol { name: "cart.badge.plus" };
pub const CART_BADGE_QUESTIONMARK: SFSymbol = SFSymbol { name: "cart.badge.questionmark" };
pub const CART_CIRCLE: SFSymbol = SFSymbol { name: "cart.circle" };
pub const CART_CIRCLE_FILL: SFSymbol = SFSymbol { name: "cart.circle.fill" };
pub const CART_FILL: SFSymbol = SFSymbol { name: "cart.fill" };
pub const CART_FILL_BADGE_MINUS: SFSymbol = SFSymbol { name: "cart.fill.badge.minus" };
pub const CART_FILL_BADGE_PLUS: SFSymbol = SFSymbol { name: "cart.fill.badge.plus" };
pub const CART_FILL_BADGE_QUESTIONMARK: SFSymbol = SFSymbol { name: "cart.fill.badge.questionmark" };
pub const CASE: SFSymbol = SFSymbol { name: "case" };
pub const CASE_FILL: SFSymbol = SFSymbol { name: "case.fill" };
pub const CEDISIGN: SFSymbol = SFSymbol { name: "cedisign" };
pub const CEDISIGN_CIRCLE: SFSymbol = SFSymbol { name: "cedisign.circle" };
pub const CEDISIGN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "cedisign.circle.fill" };
pub const CEDISIGN_SQUARE: SFSymbol = SFSymbol { name: "cedisign.square" };
pub const CEDISIGN_SQUARE_FILL: SFSymbol = SFSymbol { name: "cedisign.square.fill" };
pub const CELLULARBARS: SFSymbol = SFSymbol { name: "cellularbars" };
pub const CENTSIGN: SFSymbol = SFSymbol { name: "centsign" };
pub const CENTSIGN_CIRCLE: SFSymbol = SFSymbol { name: "centsign.circle" };
pub const CENTSIGN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "centsign.circle.fill" };
pub const CENTSIGN_SQUARE: SFSymbol = SFSymbol { name: "centsign.square" };
pub const CENTSIGN_SQUARE_FILL: SFSymbol = SFSymbol { name: "centsign.square.fill" };
pub const CHAIR: SFSymbol = SFSymbol { name: "chair" };
pub const CHAIR_FILL: SFSymbol = SFSymbol { name: "chair.fill" };
pub const CHAIR_LOUNGE: SFSymbol = SFSymbol { name: "chair.lounge" };
pub const CHAIR_LOUNGE_FILL: SFSymbol = SFSymbol { name: "chair.lounge.fill" };
pub const CHANDELIER: SFSymbol = SFSymbol { name: "chandelier" };
pub const CHANDELIER_FILL: SFSymbol = SFSymbol { name: "chandelier.fill" };
pub const CHARACTER: SFSymbol = SFSymbol { name: "character" };
pub const CHARACTER_BOOK_CLOSED: SFSymbol = SFSymbol { name: "character.book.closed" };
pub const CHARACTER_BOOK_CLOSED_FILL: SFSymbol = SFSymbol { name: "character.book.closed.fill" };
pub const CHARACTER_BUBBLE: SFSymbol = SFSymbol { name: "character.bubble" };
pub const CHARACTER_BUBBLE_FILL: SFSymbol = SFSymbol { name: "character.bubble.fill" };
pub const CHARACTER_CURSOR_IBEAM: SFSymbol = SFSymbol { name: "character.cursor.ibeam" };
pub const CHARACTER_DUPLOYAN: SFSymbol = SFSymbol { name: "character.duployan" };
pub const CHARACTER_PHONETIC: SFSymbol = SFSymbol { name: "character.phonetic" };
pub const CHARACTER_SUTTON: SFSymbol = SFSymbol { name: "character.sutton" };
pub const CHARACTER_TEXTBOX: SFSymbol = SFSymbol { name: "character.textbox" };
pub const CHART_BAR: SFSymbol = SFSymbol { name: "chart.bar" };
pub const CHART_BAR_DOC_HORIZONTAL: SFSymbol = SFSymbol { name: "chart.bar.doc.horizontal" };
pub const CHART_BAR_DOC_HORIZONTAL_FILL: SFSymbol = SFSymbol { name: "chart.bar.doc.horizontal.fill" };
pub const CHART_BAR_FILL: SFSymbol = SFSymbol { name: "chart.bar.fill" };
pub const CHART_BAR_XAXIS: SFSymbol = SFSymbol { name: "chart.bar.xaxis" };
pub const CHART_LINE_DOWNTREND_XYAXIS: SFSymbol = SFSymbol { name: "chart.line.downtrend.xyaxis" };
pub const CHART_LINE_DOWNTREND_XYAXIS_CIRCLE: SFSymbol = SFSymbol { name: "chart.line.downtrend.xyaxis.circle" };
pub const CHART_LINE_DOWNTREND_XYAXIS_CIRCLE_FILL: SFSymbol = SFSymbol { name: "chart.line.downtrend.xyaxis.circle.fill" };
pub const CHART_LINE_FLATTREND_XYAXIS: SFSymbol = SFSymbol { name: "chart.line.flattrend.xyaxis" };
pub const CHART_LINE_FLATTREND_XYAXIS_CIRCLE: SFSymbol = SFSymbol { name: "chart.line.flattrend.xyaxis.circle" };
pub const CHART_LINE_FLATTREND_XYAXIS_CIRCLE_FILL: SFSymbol = SFSymbol { name: "chart.line.flattrend.xyaxis.circle.fill" };
pub const CHART_LINE_UPTREND_XYAXIS: SFSymbol = SFSymbol { name: "chart.line.uptrend.xyaxis" };
pub const CHART_LINE_UPTREND_XYAXIS_CIRCLE: SFSymbol = SFSymbol { name: "chart.line.uptrend.xyaxis.circle" };
pub const CHART_LINE_UPTREND_XYAXIS_CIRCLE_FILL: SFSymbol = SFSymbol { name: "chart.line.uptrend.xyaxis.circle.fill" };
pub const CHART_PIE: SFSymbol = SFSymbol { name: "chart.pie" };
pub const CHART_PIE_FILL: SFSymbol = SFSymbol { name: "chart.pie.fill" };
pub const CHART_XYAXIS_LINE: SFSymbol = SFSymbol { name: "chart.xyaxis.line" };
pub const CHECKERBOARD_RECTANGLE: SFSymbol = SFSymbol { name: "checkerboard.rectangle" };
pub const CHECKERBOARD_SHIELD: SFSymbol = SFSymbol { name: "checkerboard.shield" };
pub const CHECKLIST: SFSymbol = SFSymbol { name: "checklist" };
pub const CHECKLIST_CHECKED: SFSymbol = SFSymbol { name: "checklist.checked" };
pub const CHECKLIST_UNCHECKED: SFSymbol = SFSymbol { name: "checklist.unchecked" };
pub const CHECKMARK: SFSymbol = SFSymbol { name: "checkmark" };
pub const CHECKMARK_BUBBLE: SFSymbol = SFSymbol { name: "checkmark.bubble" };
pub const CHECKMARK_BUBBLE_FILL: SFSymbol = SFSymbol { name: "checkmark.bubble.fill" };
pub const CHECKMARK_CIRCLE: SFSymbol = SFSymbol { name: "checkmark.circle" };
pub const CHECKMARK_CIRCLE_BADGE_QUESTIONMARK: SFSymbol = SFSymbol { name: "checkmark.circle.badge.questionmark" };
pub const CHECKMARK_CIRCLE_BADGE_QUESTIONMARK_FILL: SFSymbol = SFSymbol { name: "checkmark.circle.badge.questionmark.fill" };
pub const CHECKMARK_CIRCLE_BADGE_XMARK: SFSymbol = SFSymbol { name: "checkmark.circle.badge.xmark" };
pub const CHECKMARK_CIRCLE_BADGE_XMARK_FILL: SFSymbol = SFSymbol { name: "checkmark.circle.badge.xmark.fill" };
pub const CHECKMARK_CIRCLE_FILL: SFSymbol = SFSymbol { name: "checkmark.circle.fill" };
pub const CHECKMARK_CIRCLE_TRIANGLEBADGE_EXCLAMATIONMARK: SFSymbol = SFSymbol { name: "checkmark.circle.trianglebadge.exclamationmark" };
pub const CHECKMARK_DIAMOND: SFSymbol = SFSymbol { name: "checkmark.diamond" };
pub const CHECKMARK_DIAMOND_FILL: SFSymbol = SFSymbol { name: "checkmark.diamond.fill" };
pub const CHECKMARK_ICLOUD: SFSymbol = SFSymbol { name: "checkmark.icloud" };
pub const CHECKMARK_ICLOUD_FILL: SFSymbol = SFSymbol { name: "checkmark.icloud.fill" };
pub const CHECKMARK_MESSAGE: SFSymbol = SFSymbol { name: "checkmark.message" };
pub const CHECKMARK_MESSAGE_FILL: SFSymbol = SFSymbol { name: "checkmark.message.fill" };
pub const CHECKMARK_RECTANGLE: SFSymbol = SFSymbol { name: "checkmark.rectangle" };
pub const CHECKMARK_RECTANGLE_FILL: SFSymbol = SFSymbol { name: "checkmark.rectangle.fill" };
pub const CHECKMARK_RECTANGLE_PORTRAIT: SFSymbol = SFSymbol { name: "checkmark.rectangle.portrait" };
pub const CHECKMARK_RECTANGLE_PORTRAIT_FILL: SFSymbol = SFSymbol { name: "checkmark.rectangle.portrait.fill" };
pub const CHECKMARK_SEAL: SFSymbol = SFSymbol { name: "checkmark.seal" };
pub const CHECKMARK_SEAL_FILL: SFSymbol = SFSymbol { name: "checkmark.seal.fill" };
pub const CHECKMARK_SHIELD: SFSymbol = SFSymbol { name: "checkmark.shield" };
pub const CHECKMARK_SHIELD_FILL: SFSymbol = SFSymbol { name: "checkmark.shield.fill" };
pub const CHECKMARK_SQUARE: SFSymbol = SFSymbol { name: "checkmark.square" };
pub const CHECKMARK_SQUARE_FILL: SFSymbol = SFSymbol { name: "checkmark.square.fill" };
pub const CHEVRON_BACKWARD: SFSymbol = SFSymbol { name: "chevron.backward" };
pub const CHEVRON_BACKWARD_2: SFSymbol = SFSymbol { name: "chevron.backward.2" };
pub const CHEVRON_BACKWARD_CIRCLE: SFSymbol = SFSymbol { name: "chevron.backward.circle" };
pub const CHEVRON_BACKWARD_CIRCLE_FILL: SFSymbol = SFSymbol { name: "chevron.backward.circle.fill" };
pub const CHEVRON_BACKWARD_SQUARE: SFSymbol = SFSymbol { name: "chevron.backward.square" };
pub const CHEVRON_BACKWARD_SQUARE_FILL: SFSymbol = SFSymbol { name: "chevron.backward.square.fill" };
pub const CHEVRON_BACKWARD_TO_LINE: SFSymbol = SFSymbol { name: "chevron.backward.to.line" };
pub const CHEVRON_COMPACT_DOWN: SFSymbol = SFSymbol { name: "chevron.compact.down" };
pub const CHEVRON_COMPACT_LEFT: SFSymbol = SFSymbol { name: "chevron.compact.left" };
pub const CHEVRON_COMPACT_RIGHT: SFSymbol = SFSymbol { name: "chevron.compact.right" };
pub const CHEVRON_COMPACT_UP: SFSymbol = SFSymbol { name: "chevron.compact.up" };
pub const CHEVRON_DOWN: SFSymbol = SFSymbol { name: "chevron.down" };
pub const CHEVRON_DOWN_CIRCLE: SFSymbol = SFSymbol { name: "chevron.down.circle" };
pub const CHEVRON_DOWN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "chevron.down.circle.fill" };
pub const CHEVRON_DOWN_SQUARE: SFSymbol = SFSymbol { name: "chevron.down.square" };
pub const CHEVRON_DOWN_SQUARE_FILL: SFSymbol = SFSymbol { name: "chevron.down.square.fill" };
pub const CHEVRON_FORWARD: SFSymbol = SFSymbol { name: "chevron.forward" };
pub const CHEVRON_FORWARD_2: SFSymbol = SFSymbol { name: "chevron.forward.2" };
pub const CHEVRON_FORWARD_CIRCLE: SFSymbol = SFSymbol { name: "chevron.forward.circle" };
pub const CHEVRON_FORWARD_CIRCLE_FILL: SFSymbol = SFSymbol { name: "chevron.forward.circle.fill" };
pub const CHEVRON_FORWARD_SQUARE: SFSymbol = SFSymbol { name: "chevron.forward.square" };
pub const CHEVRON_FORWARD_SQUARE_FILL: SFSymbol = SFSymbol { name: "chevron.forward.square.fill" };
pub const CHEVRON_FORWARD_TO_LINE: SFSymbol = SFSymbol { name: "chevron.forward.to.line" };
pub const CHEVRON_LEFT: SFSymbol = SFSymbol { name: "chevron.left" };
pub const CHEVRON_LEFT_2: SFSymbol = SFSymbol { name: "chevron.left.2" };
pub const CHEVRON_LEFT_CIRCLE: SFSymbol = SFSymbol { name: "chevron.left.circle" };
pub const CHEVRON_LEFT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "chevron.left.circle.fill" };
pub const CHEVRON_LEFT_FORWARDSLASH_CHEVRON_RIGHT: SFSymbol = SFSymbol { name: "chevron.left.forwardslash.chevron.right" };
pub const CHEVRON_LEFT_SQUARE: SFSymbol = SFSymbol { name: "chevron.left.square" };
pub const CHEVRON_LEFT_SQUARE_FILL: SFSymbol = SFSymbol { name: "chevron.left.square.fill" };
pub const CHEVRON_LEFT_TO_LINE: SFSymbol = SFSymbol { name: "chevron.left.to.line" };
pub const CHEVRON_RIGHT: SFSymbol = SFSymbol { name: "chevron.right" };
pub const CHEVRON_RIGHT_2: SFSymbol = SFSymbol { name: "chevron.right.2" };
pub const CHEVRON_RIGHT_CIRCLE: SFSymbol = SFSymbol { name: "chevron.right.circle" };
pub const CHEVRON_RIGHT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "chevron.right.circle.fill" };
pub const CHEVRON_RIGHT_SQUARE: SFSymbol = SFSymbol { name: "chevron.right.square" };
pub const CHEVRON_RIGHT_SQUARE_FILL: SFSymbol = SFSymbol { name: "chevron.right.square.fill" };
pub const CHEVRON_RIGHT_TO_LINE: SFSymbol = SFSymbol { name: "chevron.right.to.line" };
pub const CHEVRON_UP: SFSymbol = SFSymbol { name: "chevron.up" };
pub const CHEVRON_UP_CHEVRON_DOWN: SFSymbol = SFSymbol { name: "chevron.up.chevron.down" };
pub const CHEVRON_UP_CIRCLE: SFSymbol = SFSymbol { name: "chevron.up.circle" };
pub const CHEVRON_UP_CIRCLE_FILL: SFSymbol = SFSymbol { name: "chevron.up.circle.fill" };
pub const CHEVRON_UP_SQUARE: SFSymbol = SFSymbol { name: "chevron.up.square" };
pub const CHEVRON_UP_SQUARE_FILL: SFSymbol = SFSymbol { name: "chevron.up.square.fill" };
pub const CIRCLE: SFSymbol = SFSymbol { name: "circle" };
pub const CIRCLE_AND_LINE_HORIZONTAL: SFSymbol = SFSymbol { name: "circle.and.line.horizontal" };
pub const CIRCLE_AND_LINE_HORIZONTAL_FILL: SFSymbol = SFSymbol { name: "circle.and.line.horizontal.fill" };
pub const CIRCLE_BOTTOMHALF_FILLED: SFSymbol = SFSymbol { name: "circle.bottomhalf.filled" };
pub const CIRCLE_CIRCLE: SFSymbol = SFSymbol { name: "circle.circle" };
pub const CIRCLE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "circle.circle.fill" };
pub const CIRCLE_DASHED: SFSymbol = SFSymbol { name: "circle.dashed" };
pub const CIRCLE_DASHED_INSET_FILLED: SFSymbol = SFSymbol { name: "circle.dashed.inset.filled" };
pub const CIRCLE_DASHED_RECTANGLE: SFSymbol = SFSymbol { name: "circle.dashed.rectangle" };
pub const CIRCLE_DOTTED: SFSymbol = SFSymbol { name: "circle.dotted" };
pub const CIRCLE_FILL: SFSymbol = SFSymbol { name: "circle.fill" };
pub const CIRCLE_FILLED_PATTERN_DIAGONALLINE_RECTANGLE: SFSymbol = SFSymbol { name: "circle.filled.pattern.diagonalline.rectangle" };
pub const CIRCLE_GRID_2X1: SFSymbol = SFSymbol { name: "circle.grid.2x1" };
pub const CIRCLE_GRID_2X1_FILL: SFSymbol = SFSymbol { name: "circle.grid.2x1.fill" };
pub const CIRCLE_GRID_2X1_LEFT_FILLED: SFSymbol = SFSymbol { name: "circle.grid.2x1.left.filled" };
pub const CIRCLE_GRID_2X1_RIGHT_FILLED: SFSymbol = SFSymbol { name: "circle.grid.2x1.right.filled" };
pub const CIRCLE_GRID_2X2: SFSymbol = SFSymbol { name: "circle.grid.2x2" };
pub const CIRCLE_GRID_2X2_FILL: SFSymbol = SFSymbol { name: "circle.grid.2x2.fill" };
pub const CIRCLE_GRID_3X3: SFSymbol = SFSymbol { name: "circle.grid.3x3" };
pub const CIRCLE_GRID_3X3_CIRCLE: SFSymbol = SFSymbol { name: "circle.grid.3x3.circle" };
pub const CIRCLE_GRID_3X3_CIRCLE_FILL: SFSymbol = SFSymbol { name: "circle.grid.3x3.circle.fill" };
pub const CIRCLE_GRID_3X3_FILL: SFSymbol = SFSymbol { name: "circle.grid.3x3.fill" };
pub const CIRCLE_GRID_CROSS: SFSymbol = SFSymbol { name: "circle.grid.cross" };
pub const CIRCLE_GRID_CROSS_DOWN_FILLED: SFSymbol = SFSymbol { name: "circle.grid.cross.down.filled" };
pub const CIRCLE_GRID_CROSS_FILL: SFSymbol = SFSymbol { name: "circle.grid.cross.fill" };
pub const CIRCLE_GRID_CROSS_LEFT_FILLED: SFSymbol = SFSymbol { name: "circle.grid.cross.left.filled" };
pub const CIRCLE_GRID_CROSS_RIGHT_FILLED: SFSymbol = SFSymbol { name: "circle.grid.cross.right.filled" };
pub const CIRCLE_GRID_CROSS_UP_FILLED: SFSymbol = SFSymbol { name: "circle.grid.cross.up.filled" };
pub const CIRCLE_HEXAGONGRID: SFSymbol = SFSymbol { name: "circle.hexagongrid" };
pub const CIRCLE_HEXAGONGRID_CIRCLE: SFSymbol = SFSymbol { name: "circle.hexagongrid.circle" };
pub const CIRCLE_HEXAGONGRID_CIRCLE_FILL: SFSymbol = SFSymbol { name: "circle.hexagongrid.circle.fill" };
pub const CIRCLE_HEXAGONGRID_FILL: SFSymbol = SFSymbol { name: "circle.hexagongrid.fill" };
pub const CIRCLE_HEXAGONPATH: SFSymbol = SFSymbol { name: "circle.hexagonpath" };
pub const CIRCLE_HEXAGONPATH_FILL: SFSymbol = SFSymbol { name: "circle.hexagonpath.fill" };
pub const CIRCLE_INSET_FILLED: SFSymbol = SFSymbol { name: "circle.inset.filled" };
pub const CIRCLE_LEFTHALF_FILLED: SFSymbol = SFSymbol { name: "circle.lefthalf.filled" };
pub const CIRCLE_RECTANGLE_DASHED: SFSymbol = SFSymbol { name: "circle.rectangle.dashed" };
pub const CIRCLE_RECTANGLE_FILLED_PATTERN_DIAGONALLINE: SFSymbol = SFSymbol { name: "circle.rectangle.filled.pattern.diagonalline" };
pub const CIRCLE_RIGHTHALF_FILLED: SFSymbol = SFSymbol { name: "circle.righthalf.filled" };
pub const CIRCLE_SLASH: SFSymbol = SFSymbol { name: "circle.slash" };
pub const CIRCLE_SLASH_FILL: SFSymbol = SFSymbol { name: "circle.slash.fill" };
pub const CIRCLE_SQUARE: SFSymbol = SFSymbol { name: "circle.square" };
pub const CIRCLE_SQUARE_FILL: SFSymbol = SFSymbol { name: "circle.square.fill" };
pub const CIRCLE_TOPHALF_FILLED: SFSymbol = SFSymbol { name: "circle.tophalf.filled" };
pub const CIRCLEBADGE: SFSymbol = SFSymbol { name: "circlebadge" };
pub const CIRCLEBADGE_2: SFSymbol = SFSymbol { name: "circlebadge.2" };
pub const CIRCLEBADGE_2_FILL: SFSymbol = SFSymbol { name: "circlebadge.2.fill" };
pub const CIRCLEBADGE_FILL: SFSymbol = SFSymbol { name: "circlebadge.fill" };
pub const CLEAR: SFSymbol = SFSymbol { name: "clear" };
pub const CLEAR_FILL: SFSymbol = SFSymbol { name: "clear.fill" };
pub const CLIPBOARD: SFSymbol = SFSymbol { name: "clipboard" };
pub const CLIPBOARD_FILL: SFSymbol = SFSymbol { name: "clipboard.fill" };
pub const CLOCK: SFSymbol = SFSymbol { name: "clock" };
pub const CLOCK_ARROW_2_CIRCLEPATH: SFSymbol = SFSymbol { name: "clock.arrow.2.circlepath" };
pub const CLOCK_ARROW_CIRCLEPATH: SFSymbol = SFSymbol { name: "clock.arrow.circlepath" };
pub const CLOCK_BADGE: SFSymbol = SFSymbol { name: "clock.badge" };
pub const CLOCK_BADGE_CHECKMARK: SFSymbol = SFSymbol { name: "clock.badge.checkmark" };
pub const CLOCK_BADGE_CHECKMARK_FILL: SFSymbol = SFSymbol { name: "clock.badge.checkmark.fill" };
pub const CLOCK_BADGE_EXCLAMATIONMARK: SFSymbol = SFSymbol { name: "clock.badge.exclamationmark" };
pub const CLOCK_BADGE_EXCLAMATIONMARK_FILL: SFSymbol = SFSymbol { name: "clock.badge.exclamationmark.fill" };
pub const CLOCK_BADGE_FILL: SFSymbol = SFSymbol { name: "clock.badge.fill" };
pub const CLOCK_BADGE_QUESTIONMARK: SFSymbol = SFSymbol { name: "clock.badge.questionmark" };
pub const CLOCK_BADGE_QUESTIONMARK_FILL: SFSymbol = SFSymbol { name: "clock.badge.questionmark.fill" };
pub const CLOCK_BADGE_XMARK: SFSymbol = SFSymbol { name: "clock.badge.xmark" };
pub const CLOCK_BADGE_XMARK_FILL: SFSymbol = SFSymbol { name: "clock.badge.xmark.fill" };
pub const CLOCK_CIRCLE: SFSymbol = SFSymbol { name: "clock.circle" };
pub const CLOCK_CIRCLE_FILL: SFSymbol = SFSymbol { name: "clock.circle.fill" };
pub const CLOCK_FILL: SFSymbol = SFSymbol { name: "clock.fill" };
pub const CLOUD: SFSymbol = SFSymbol { name: "cloud" };
pub const CLOUD_BOLT: SFSymbol = SFSymbol { name: "cloud.bolt" };
pub const CLOUD_BOLT_CIRCLE: SFSymbol = SFSymbol { name: "cloud.bolt.circle" };
pub const CLOUD_BOLT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "cloud.bolt.circle.fill" };
pub const CLOUD_BOLT_FILL: SFSymbol = SFSymbol { name: "cloud.bolt.fill" };
pub const CLOUD_BOLT_RAIN: SFSymbol = SFSymbol { name: "cloud.bolt.rain" };
pub const CLOUD_BOLT_RAIN_CIRCLE: SFSymbol = SFSymbol { name: "cloud.bolt.rain.circle" };
pub const CLOUD_BOLT_RAIN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "cloud.bolt.rain.circle.fill" };
pub const CLOUD_BOLT_RAIN_FILL: SFSymbol = SFSymbol { name: "cloud.bolt.rain.fill" };
pub const CLOUD_CIRCLE: SFSymbol = SFSymbol { name: "cloud.circle" };
pub const CLOUD_CIRCLE_FILL: SFSymbol = SFSymbol { name: "cloud.circle.fill" };
pub const CLOUD_DRIZZLE: SFSymbol = SFSymbol { name: "cloud.drizzle" };
pub const CLOUD_DRIZZLE_CIRCLE: SFSymbol = SFSymbol { name: "cloud.drizzle.circle" };
pub const CLOUD_DRIZZLE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "cloud.drizzle.circle.fill" };
pub const CLOUD_DRIZZLE_FILL: SFSymbol = SFSymbol { name: "cloud.drizzle.fill" };
pub const CLOUD_FILL: SFSymbol = SFSymbol { name: "cloud.fill" };
pub const CLOUD_FOG: SFSymbol = SFSymbol { name: "cloud.fog" };
pub const CLOUD_FOG_CIRCLE: SFSymbol = SFSymbol { name: "cloud.fog.circle" };
pub const CLOUD_FOG_CIRCLE_FILL: SFSymbol = SFSymbol { name: "cloud.fog.circle.fill" };
pub const CLOUD_FOG_FILL: SFSymbol = SFSymbol { name: "cloud.fog.fill" };
pub const CLOUD_HAIL: SFSymbol = SFSymbol { name: "cloud.hail" };
pub const CLOUD_HAIL_CIRCLE: SFSymbol = SFSymbol { name: "cloud.hail.circle" };
pub const CLOUD_HAIL_CIRCLE_FILL: SFSymbol = SFSymbol { name: "cloud.hail.circle.fill" };
pub const CLOUD_HAIL_FILL: SFSymbol = SFSymbol { name: "cloud.hail.fill" };
pub const CLOUD_HEAVYRAIN: SFSymbol = SFSymbol { name: "cloud.heavyrain" };
pub const CLOUD_HEAVYRAIN_CIRCLE: SFSymbol = SFSymbol { name: "cloud.heavyrain.circle" };
pub const CLOUD_HEAVYRAIN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "cloud.heavyrain.circle.fill" };
pub const CLOUD_HEAVYRAIN_FILL: SFSymbol = SFSymbol { name: "cloud.heavyrain.fill" };
pub const CLOUD_MOON: SFSymbol = SFSymbol { name: "cloud.moon" };
pub const CLOUD_MOON_BOLT: SFSymbol = SFSymbol { name: "cloud.moon.bolt" };
pub const CLOUD_MOON_BOLT_CIRCLE: SFSymbol = SFSymbol { name: "cloud.moon.bolt.circle" };
pub const CLOUD_MOON_BOLT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "cloud.moon.bolt.circle.fill" };
pub const CLOUD_MOON_BOLT_FILL: SFSymbol = SFSymbol { name: "cloud.moon.bolt.fill" };
pub const CLOUD_MOON_CIRCLE: SFSymbol = SFSymbol { name: "cloud.moon.circle" };
pub const CLOUD_MOON_CIRCLE_FILL: SFSymbol = SFSymbol { name: "cloud.moon.circle.fill" };
pub const CLOUD_MOON_FILL: SFSymbol = SFSymbol { name: "cloud.moon.fill" };
pub const CLOUD_MOON_RAIN: SFSymbol = SFSymbol { name: "cloud.moon.rain" };
pub const CLOUD_MOON_RAIN_CIRCLE: SFSymbol = SFSymbol { name: "cloud.moon.rain.circle" };
pub const CLOUD_MOON_RAIN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "cloud.moon.rain.circle.fill" };
pub const CLOUD_MOON_RAIN_FILL: SFSymbol = SFSymbol { name: "cloud.moon.rain.fill" };
pub const CLOUD_RAIN: SFSymbol = SFSymbol { name: "cloud.rain" };
pub const CLOUD_RAIN_CIRCLE: SFSymbol = SFSymbol { name: "cloud.rain.circle" };
pub const CLOUD_RAIN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "cloud.rain.circle.fill" };
pub const CLOUD_RAIN_FILL: SFSymbol = SFSymbol { name: "cloud.rain.fill" };
pub const CLOUD_SLEET: SFSymbol = SFSymbol { name: "cloud.sleet" };
pub const CLOUD_SLEET_CIRCLE: SFSymbol = SFSymbol { name: "cloud.sleet.circle" };
pub const CLOUD_SLEET_CIRCLE_FILL: SFSymbol = SFSymbol { name: "cloud.sleet.circle.fill" };
pub const CLOUD_SLEET_FILL: SFSymbol = SFSymbol { name: "cloud.sleet.fill" };
pub const CLOUD_SNOW: SFSymbol = SFSymbol { name: "cloud.snow" };
pub const CLOUD_SNOW_CIRCLE: SFSymbol = SFSymbol { name: "cloud.snow.circle" };
pub const CLOUD_SNOW_CIRCLE_FILL: SFSymbol = SFSymbol { name: "cloud.snow.circle.fill" };
pub const CLOUD_SNOW_FILL: SFSymbol = SFSymbol { name: "cloud.snow.fill" };
pub const CLOUD_SUN: SFSymbol = SFSymbol { name: "cloud.sun" };
pub const CLOUD_SUN_BOLT: SFSymbol = SFSymbol { name: "cloud.sun.bolt" };
pub const CLOUD_SUN_BOLT_CIRCLE: SFSymbol = SFSymbol { name: "cloud.sun.bolt.circle" };
pub const CLOUD_SUN_BOLT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "cloud.sun.bolt.circle.fill" };
pub const CLOUD_SUN_BOLT_FILL: SFSymbol = SFSymbol { name: "cloud.sun.bolt.fill" };
pub const CLOUD_SUN_CIRCLE: SFSymbol = SFSymbol { name: "cloud.sun.circle" };
pub const CLOUD_SUN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "cloud.sun.circle.fill" };
pub const CLOUD_SUN_FILL: SFSymbol = SFSymbol { name: "cloud.sun.fill" };
pub const CLOUD_SUN_RAIN: SFSymbol = SFSymbol { name: "cloud.sun.rain" };
pub const CLOUD_SUN_RAIN_CIRCLE: SFSymbol = SFSymbol { name: "cloud.sun.rain.circle" };
pub const CLOUD_SUN_RAIN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "cloud.sun.rain.circle.fill" };
pub const CLOUD_SUN_RAIN_FILL: SFSymbol = SFSymbol { name: "cloud.sun.rain.fill" };
pub const COLONCURRENCYSIGN: SFSymbol = SFSymbol { name: "coloncurrencysign" };
pub const COLONCURRENCYSIGN_CIRCLE: SFSymbol = SFSymbol { name: "coloncurrencysign.circle" };
pub const COLONCURRENCYSIGN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "coloncurrencysign.circle.fill" };
pub const COLONCURRENCYSIGN_SQUARE: SFSymbol = SFSymbol { name: "coloncurrencysign.square" };
pub const COLONCURRENCYSIGN_SQUARE_FILL: SFSymbol = SFSymbol { name: "coloncurrencysign.square.fill" };
pub const COMB: SFSymbol = SFSymbol { name: "comb" };
pub const COMB_FILL: SFSymbol = SFSymbol { name: "comb.fill" };
pub const COMMAND: SFSymbol = SFSymbol { name: "command" };
pub const COMMAND_CIRCLE: SFSymbol = SFSymbol { name: "command.circle" };
pub const COMMAND_CIRCLE_FILL: SFSymbol = SFSymbol { name: "command.circle.fill" };
pub const COMMAND_SQUARE: SFSymbol = SFSymbol { name: "command.square" };
pub const COMMAND_SQUARE_FILL: SFSymbol = SFSymbol { name: "command.square.fill" };
pub const COMPASS_DRAWING: SFSymbol = SFSymbol { name: "compass.drawing" };
pub const COMPUTERMOUSE: SFSymbol = SFSymbol { name: "computermouse" };
pub const COMPUTERMOUSE_FILL: SFSymbol = SFSymbol { name: "computermouse.fill" };
pub const CONE: SFSymbol = SFSymbol { name: "cone" };
pub const CONE_FILL: SFSymbol = SFSymbol { name: "cone.fill" };
pub const CONTACT_SENSOR: SFSymbol = SFSymbol { name: "contact.sensor" };
pub const CONTACT_SENSOR_FILL: SFSymbol = SFSymbol { name: "contact.sensor.fill" };
pub const CONTEXTUALMENU_AND_CURSORARROW: SFSymbol = SFSymbol { name: "contextualmenu.and.cursorarrow" };
pub const CONTROL: SFSymbol = SFSymbol { name: "control" };
pub const COOKTOP: SFSymbol = SFSymbol { name: "cooktop" };
pub const COOKTOP_FILL: SFSymbol = SFSymbol { name: "cooktop.fill" };
pub const CPU: SFSymbol = SFSymbol { name: "cpu" };
pub const CPU_FILL: SFSymbol = SFSymbol { name: "cpu.fill" };
pub const CREDITCARD: SFSymbol = SFSymbol { name: "creditcard" };
pub const CREDITCARD_AND_123: SFSymbol = SFSymbol { name: "creditcard.and.123" };
pub const CREDITCARD_CIRCLE: SFSymbol = SFSymbol { name: "creditcard.circle" };
pub const CREDITCARD_CIRCLE_FILL: SFSymbol = SFSymbol { name: "creditcard.circle.fill" };
pub const CREDITCARD_FILL: SFSymbol = SFSymbol { name: "creditcard.fill" };
pub const CREDITCARD_TRIANGLEBADGE_EXCLAMATIONMARK: SFSymbol = SFSymbol { name: "creditcard.trianglebadge.exclamationmark" };
pub const CREDITCARD_VIEWFINDER: SFSymbol = SFSymbol { name: "creditcard.viewfinder" };
pub const CRICKET_BALL: SFSymbol = SFSymbol { name: "cricket.ball" };
pub const CRICKET_BALL_CIRCLE: SFSymbol = SFSymbol { name: "cricket.ball.circle" };
pub const CRICKET_BALL_CIRCLE_FILL: SFSymbol = SFSymbol { name: "cricket.ball.circle.fill" };
pub const CRICKET_BALL_FILL: SFSymbol = SFSymbol { name: "cricket.ball.fill" };
pub const CROP: SFSymbol = SFSymbol { name: "crop" };
pub const CROP_ROTATE: SFSymbol = SFSymbol { name: "crop.rotate" };
pub const CROSS: SFSymbol = SFSymbol { name: "cross" };
pub const CROSS_CASE: SFSymbol = SFSymbol { name: "cross.case" };
pub const CROSS_CASE_FILL: SFSymbol = SFSymbol { name: "cross.case.fill" };
pub const CROSS_CIRCLE: SFSymbol = SFSymbol { name: "cross.circle" };
pub const CROSS_CIRCLE_FILL: SFSymbol = SFSymbol { name: "cross.circle.fill" };
pub const CROSS_FILL: SFSymbol = SFSymbol { name: "cross.fill" };
pub const CROSS_VIAL: SFSymbol = SFSymbol { name: "cross.vial" };
pub const CROSS_VIAL_FILL: SFSymbol = SFSymbol { name: "cross.vial.fill" };
pub const CROWN: SFSymbol = SFSymbol { name: "crown" };
pub const CROWN_FILL: SFSymbol = SFSymbol { name: "crown.fill" };
pub const CRUZEIROSIGN: SFSymbol = SFSymbol { name: "cruzeirosign" };
pub const CRUZEIROSIGN_CIRCLE: SFSymbol = SFSymbol { name: "cruzeirosign.circle" };
pub const CRUZEIROSIGN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "cruzeirosign.circle.fill" };
pub const CRUZEIROSIGN_SQUARE: SFSymbol = SFSymbol { name: "cruzeirosign.square" };
pub const CRUZEIROSIGN_SQUARE_FILL: SFSymbol = SFSymbol { name: "cruzeirosign.square.fill" };
pub const CUBE: SFSymbol = SFSymbol { name: "cube" };
pub const CUBE_FILL: SFSymbol = SFSymbol { name: "cube.fill" };
pub const CUBE_TRANSPARENT: SFSymbol = SFSymbol { name: "cube.transparent" };
pub const CUBE_TRANSPARENT_FILL: SFSymbol = SFSymbol { name: "cube.transparent.fill" };
pub const CUP_AND_SAUCER: SFSymbol = SFSymbol { name: "cup.and.saucer" };
pub const CUP_AND_SAUCER_FILL: SFSymbol = SFSymbol { name: "cup.and.saucer.fill" };
pub const CURLYBRACES: SFSymbol = SFSymbol { name: "curlybraces" };
pub const CURLYBRACES_SQUARE: SFSymbol = SFSymbol { name: "curlybraces.square" };
pub const CURLYBRACES_SQUARE_FILL: SFSymbol = SFSymbol { name: "curlybraces.square.fill" };
pub const CURSORARROW: SFSymbol = SFSymbol { name: "cursorarrow" };
pub const CURSORARROW_AND_SQUARE_ON_SQUARE_DASHED: SFSymbol = SFSymbol { name: "cursorarrow.and.square.on.square.dashed" };
pub const CURSORARROW_CLICK: SFSymbol = SFSymbol { name: "cursorarrow.click" };
pub const CURSORARROW_CLICK_2: SFSymbol = SFSymbol { name: "cursorarrow.click.2" };
pub const CURSORARROW_CLICK_BADGE_CLOCK: SFSymbol = SFSymbol { name: "cursorarrow.click.badge.clock" };
pub const CURSORARROW_MOTIONLINES: SFSymbol = SFSymbol { name: "cursorarrow.motionlines" };
pub const CURSORARROW_MOTIONLINES_CLICK: SFSymbol = SFSymbol { name: "cursorarrow.motionlines.click" };
pub const CURSORARROW_RAYS: SFSymbol = SFSymbol { name: "cursorarrow.rays" };
pub const CURSORARROW_SQUARE: SFSymbol = SFSymbol { name: "cursorarrow.square" };
pub const CURSORARROW_SQUARE_FILL: SFSymbol = SFSymbol { name: "cursorarrow.square.fill" };
pub const CURTAINS_CLOSED: SFSymbol = SFSymbol { name: "curtains.closed" };
pub const CURTAINS_OPEN: SFSymbol = SFSymbol { name: "curtains.open" };
pub const CYLINDER: SFSymbol = SFSymbol { name: "cylinder" };
pub const CYLINDER_FILL: SFSymbol = SFSymbol { name: "cylinder.fill" };
pub const CYLINDER_SPLIT_1X2: SFSymbol = SFSymbol { name: "cylinder.split.1x2" };
pub const CYLINDER_SPLIT_1X2_FILL: SFSymbol = SFSymbol { name: "cylinder.split.1x2.fill" };
pub const D_CIRCLE: SFSymbol = SFSymbol { name: "d.circle" };
pub const D_CIRCLE_FILL: SFSymbol = SFSymbol { name: "d.circle.fill" };
pub const D_SQUARE: SFSymbol = SFSymbol { name: "d.square" };
pub const D_SQUARE_FILL: SFSymbol = SFSymbol { name: "d.square.fill" };
pub const DECREASE_INDENT: SFSymbol = SFSymbol { name: "decrease.indent" };
pub const DECREASE_QUOTELEVEL: SFSymbol = SFSymbol { name: "decrease.quotelevel" };
pub const DEHUMIDIFIER: SFSymbol = SFSymbol { name: "dehumidifier" };
pub const DEHUMIDIFIER_FILL: SFSymbol = SFSymbol { name: "dehumidifier.fill" };
pub const DELETE_BACKWARD: SFSymbol = SFSymbol { name: "delete.backward" };
pub const DELETE_BACKWARD_FILL: SFSymbol = SFSymbol { name: "delete.backward.fill" };
pub const DELETE_FORWARD: SFSymbol = SFSymbol { name: "delete.forward" };
pub const DELETE_FORWARD_FILL: SFSymbol = SFSymbol { name: "delete.forward.fill" };
pub const DELETE_LEFT: SFSymbol = SFSymbol { name: "delete.left" };
pub const DELETE_LEFT_FILL: SFSymbol = SFSymbol { name: "delete.left.fill" };
pub const DELETE_RIGHT: SFSymbol = SFSymbol { name: "delete.right" };
pub const DELETE_RIGHT_FILL: SFSymbol = SFSymbol { name: "delete.right.fill" };
pub const DESKCLOCK: SFSymbol = SFSymbol { name: "deskclock" };
pub const DESKCLOCK_FILL: SFSymbol = SFSymbol { name: "deskclock.fill" };
pub const DESKTOPCOMPUTER: SFSymbol = SFSymbol { name: "desktopcomputer" };
pub const DESKTOPCOMPUTER_AND_ARROW_DOWN: SFSymbol = SFSymbol { name: "desktopcomputer.and.arrow.down" };
pub const DESKTOPCOMPUTER_TRIANGLEBADGE_EXCLAMATIONMARK: SFSymbol = SFSymbol { name: "desktopcomputer.trianglebadge.exclamationmark" };
pub const DESKVIEW: SFSymbol = SFSymbol { name: "deskview" };
pub const DESKVIEW_FILL: SFSymbol = SFSymbol { name: "deskview.fill" };
pub const DIAL_HIGH: SFSymbol = SFSymbol { name: "dial.high" };
pub const DIAL_HIGH_FILL: SFSymbol = SFSymbol { name: "dial.high.fill" };
pub const DIAL_LOW: SFSymbol = SFSymbol { name: "dial.low" };
pub const DIAL_LOW_FILL: SFSymbol = SFSymbol { name: "dial.low.fill" };
pub const DIAL_MAX: SFSymbol = SFSymbol { name: "dial.max" };
pub const DIAL_MAX_FILL: SFSymbol = SFSymbol { name: "dial.max.fill" };
pub const DIAL_MEDIUM: SFSymbol = SFSymbol { name: "dial.medium" };
pub const DIAL_MEDIUM_FILL: SFSymbol = SFSymbol { name: "dial.medium.fill" };
pub const DIAL_MIN: SFSymbol = SFSymbol { name: "dial.min" };
pub const DIAL_MIN_FILL: SFSymbol = SFSymbol { name: "dial.min.fill" };
pub const DIAMOND: SFSymbol = SFSymbol { name: "diamond" };
pub const DIAMOND_BOTTOMHALF_FILLED: SFSymbol = SFSymbol { name: "diamond.bottomhalf.filled" };
pub const DIAMOND_CIRCLE: SFSymbol = SFSymbol { name: "diamond.circle" };
pub const DIAMOND_CIRCLE_FILL: SFSymbol = SFSymbol { name: "diamond.circle.fill" };
pub const DIAMOND_FILL: SFSymbol = SFSymbol { name: "diamond.fill" };
pub const DIAMOND_INSET_FILLED: SFSymbol = SFSymbol { name: "diamond.inset.filled" };
pub const DIAMOND_LEFTHALF_FILLED: SFSymbol = SFSymbol { name: "diamond.lefthalf.filled" };
pub const DIAMOND_RIGHTHALF_FILLED: SFSymbol = SFSymbol { name: "diamond.righthalf.filled" };
pub const DIAMOND_TOPHALF_FILLED: SFSymbol = SFSymbol { name: "diamond.tophalf.filled" };
pub const DICE: SFSymbol = SFSymbol { name: "dice" };
pub const DICE_FILL: SFSymbol = SFSymbol { name: "dice.fill" };
pub const DIE_FACE_1: SFSymbol = SFSymbol { name: "die.face.1" };
pub const DIE_FACE_1_FILL: SFSymbol = SFSymbol { name: "die.face.1.fill" };
pub const DIE_FACE_2: SFSymbol = SFSymbol { name: "die.face.2" };
pub const DIE_FACE_2_FILL: SFSymbol = SFSymbol { name: "die.face.2.fill" };
pub const DIE_FACE_3: SFSymbol = SFSymbol { name: "die.face.3" };
pub const DIE_FACE_3_FILL: SFSymbol = SFSymbol { name: "die.face.3.fill" };
pub const DIE_FACE_4: SFSymbol = SFSymbol { name: "die.face.4" };
pub const DIE_FACE_4_FILL: SFSymbol = SFSymbol { name: "die.face.4.fill" };
pub const DIE_FACE_5: SFSymbol = SFSymbol { name: "die.face.5" };
pub const DIE_FACE_5_FILL: SFSymbol = SFSymbol { name: "die.face.5.fill" };
pub const DIE_FACE_6: SFSymbol = SFSymbol { name: "die.face.6" };
pub const DIE_FACE_6_FILL: SFSymbol = SFSymbol { name: "die.face.6.fill" };
pub const DIGITALCROWN_ARROW_CLOCKWISE: SFSymbol = SFSymbol { name: "digitalcrown.arrow.clockwise" };
pub const DIGITALCROWN_ARROW_CLOCKWISE_FILL: SFSymbol = SFSymbol { name: "digitalcrown.arrow.clockwise.fill" };
pub const DIGITALCROWN_ARROW_COUNTERCLOCKWISE: SFSymbol = SFSymbol { name: "digitalcrown.arrow.counterclockwise" };
pub const DIGITALCROWN_ARROW_COUNTERCLOCKWISE_FILL: SFSymbol = SFSymbol { name: "digitalcrown.arrow.counterclockwise.fill" };
pub const DIGITALCROWN_HORIZONTAL_ARROW_CLOCKWISE: SFSymbol = SFSymbol { name: "digitalcrown.horizontal.arrow.clockwise" };
pub const DIGITALCROWN_HORIZONTAL_ARROW_CLOCKWISE_FILL: SFSymbol = SFSymbol { name: "digitalcrown.horizontal.arrow.clockwise.fill" };
pub const DIGITALCROWN_HORIZONTAL_ARROW_COUNTERCLOCKWISE: SFSymbol = SFSymbol { name: "digitalcrown.horizontal.arrow.counterclockwise" };
pub const DIGITALCROWN_HORIZONTAL_ARROW_COUNTERCLOCKWISE_FILL: SFSymbol = SFSymbol { name: "digitalcrown.horizontal.arrow.counterclockwise.fill" };
pub const DIGITALCROWN_HORIZONTAL_PRESS: SFSymbol = SFSymbol { name: "digitalcrown.horizontal.press" };
pub const DIGITALCROWN_HORIZONTAL_PRESS_FILL: SFSymbol = SFSymbol { name: "digitalcrown.horizontal.press.fill" };
pub const DIGITALCROWN_PRESS: SFSymbol = SFSymbol { name: "digitalcrown.press" };
pub const DIGITALCROWN_PRESS_FILL: SFSymbol = SFSymbol { name: "digitalcrown.press.fill" };
pub const DIRECTCURRENT: SFSymbol = SFSymbol { name: "directcurrent" };
pub const DISHWASHER: SFSymbol = SFSymbol { name: "dishwasher" };
pub const DISHWASHER_FILL: SFSymbol = SFSymbol { name: "dishwasher.fill" };
pub const DISPLAY: SFSymbol = SFSymbol { name: "display" };
pub const DISPLAY_2: SFSymbol = SFSymbol { name: "display.2" };
pub const DISPLAY_AND_ARROW_DOWN: SFSymbol = SFSymbol { name: "display.and.arrow.down" };
pub const DISPLAY_TRIANGLEBADGE_EXCLAMATIONMARK: SFSymbol = SFSymbol { name: "display.trianglebadge.exclamationmark" };
pub const DISTRIBUTE_HORIZONTAL_CENTER: SFSymbol = SFSymbol { name: "distribute.horizontal.center" };
pub const DISTRIBUTE_HORIZONTAL_CENTER_FILL: SFSymbol = SFSymbol { name: "distribute.horizontal.center.fill" };
pub const DISTRIBUTE_HORIZONTAL_LEFT: SFSymbol = SFSymbol { name: "distribute.horizontal.left" };
pub const DISTRIBUTE_HORIZONTAL_LEFT_FILL: SFSymbol = SFSymbol { name: "distribute.horizontal.left.fill" };
pub const DISTRIBUTE_HORIZONTAL_RIGHT: SFSymbol = SFSymbol { name: "distribute.horizontal.right" };
pub const DISTRIBUTE_HORIZONTAL_RIGHT_FILL: SFSymbol = SFSymbol { name: "distribute.horizontal.right.fill" };
pub const DISTRIBUTE_VERTICAL_BOTTOM: SFSymbol = SFSymbol { name: "distribute.vertical.bottom" };
pub const DISTRIBUTE_VERTICAL_BOTTOM_FILL: SFSymbol = SFSymbol { name: "distribute.vertical.bottom.fill" };
pub const DISTRIBUTE_VERTICAL_CENTER: SFSymbol = SFSymbol { name: "distribute.vertical.center" };
pub const DISTRIBUTE_VERTICAL_CENTER_FILL: SFSymbol = SFSymbol { name: "distribute.vertical.center.fill" };
pub const DISTRIBUTE_VERTICAL_TOP: SFSymbol = SFSymbol { name: "distribute.vertical.top" };
pub const DISTRIBUTE_VERTICAL_TOP_FILL: SFSymbol = SFSymbol { name: "distribute.vertical.top.fill" };
pub const DIVIDE: SFSymbol = SFSymbol { name: "divide" };
pub const DIVIDE_CIRCLE: SFSymbol = SFSymbol { name: "divide.circle" };
pub const DIVIDE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "divide.circle.fill" };
pub const DIVIDE_SQUARE: SFSymbol = SFSymbol { name: "divide.square" };
pub const DIVIDE_SQUARE_FILL: SFSymbol = SFSymbol { name: "divide.square.fill" };
pub const DOC: SFSymbol = SFSymbol { name: "doc" };
pub const DOC_APPEND: SFSymbol = SFSymbol { name: "doc.append" };
pub const DOC_APPEND_FILL: SFSymbol = SFSymbol { name: "doc.append.fill" };
pub const DOC_BADGE_ARROW_UP: SFSymbol = SFSymbol { name: "doc.badge.arrow.up" };
pub const DOC_BADGE_ARROW_UP_FILL: SFSymbol = SFSymbol { name: "doc.badge.arrow.up.fill" };
pub const DOC_BADGE_ELLIPSIS: SFSymbol = SFSymbol { name: "doc.badge.ellipsis" };
pub const DOC_BADGE_GEARSHAPE: SFSymbol = SFSymbol { name: "doc.badge.gearshape" };
pub const DOC_BADGE_GEARSHAPE_FILL: SFSymbol = SFSymbol { name: "doc.badge.gearshape.fill" };
pub const DOC_BADGE_PLUS: SFSymbol = SFSymbol { name: "doc.badge.plus" };
pub const DOC_CIRCLE: SFSymbol = SFSymbol { name: "doc.circle" };
pub const DOC_CIRCLE_FILL: SFSymbol = SFSymbol { name: "doc.circle.fill" };
pub const DOC_FILL: SFSymbol = SFSymbol { name: "doc.fill" };
pub const DOC_FILL_BADGE_ELLIPSIS: SFSymbol = SFSymbol { name: "doc.fill.badge.ellipsis" };
pub const DOC_FILL_BADGE_PLUS: SFSymbol = SFSymbol { name: "doc.fill.badge.plus" };
pub const DOC_ON_CLIPBOARD: SFSymbol = SFSymbol { name: "doc.on.clipboard" };
pub const DOC_ON_CLIPBOARD_FILL: SFSymbol = SFSymbol { name: "doc.on.clipboard.fill" };
pub const DOC_ON_DOC: SFSymbol = SFSymbol { name: "doc.on.doc" };
pub const DOC_ON_DOC_FILL: SFSymbol = SFSymbol { name: "doc.on.doc.fill" };
pub const DOC_PLAINTEXT: SFSymbol = SFSymbol { name: "doc.plaintext" };
pub const DOC_PLAINTEXT_FILL: SFSymbol = SFSymbol { name: "doc.plaintext.fill" };
pub const DOC_RICHTEXT: SFSymbol = SFSymbol { name: "doc.richtext" };
pub const DOC_RICHTEXT_FILL: SFSymbol = SFSymbol { name: "doc.richtext.fill" };
pub const DOC_TEXT: SFSymbol = SFSymbol { name: "doc.text" };
pub const DOC_TEXT_BELOW_ECG: SFSymbol = SFSymbol { name: "doc.text.below.ecg" };
pub const DOC_TEXT_BELOW_ECG_FILL: SFSymbol = SFSymbol { name: "doc.text.below.ecg.fill" };
pub const DOC_TEXT_FILL: SFSymbol = SFSymbol { name: "doc.text.fill" };
pub const DOC_TEXT_IMAGE: SFSymbol = SFSymbol { name: "doc.text.image" };
pub const DOC_TEXT_IMAGE_FILL: SFSymbol = SFSymbol { name: "doc.text.image.fill" };
pub const DOC_TEXT_MAGNIFYINGGLASS: SFSymbol = SFSymbol { name: "doc.text.magnifyingglass" };
pub const DOC_VIEWFINDER: SFSymbol = SFSymbol { name: "doc.viewfinder" };
pub const DOC_VIEWFINDER_FILL: SFSymbol = SFSymbol { name: "doc.viewfinder.fill" };
pub const DOC_ZIPPER: SFSymbol = SFSymbol { name: "doc.zipper" };
pub const DOCK_ARROW_DOWN_RECTANGLE: SFSymbol = SFSymbol { name: "dock.arrow.down.rectangle" };
pub const DOCK_ARROW_UP_RECTANGLE: SFSymbol = SFSymbol { name: "dock.arrow.up.rectangle" };
pub const DOCK_RECTANGLE: SFSymbol = SFSymbol { name: "dock.rectangle" };
pub const DOLLARSIGN: SFSymbol = SFSymbol { name: "dollarsign" };
pub const DOLLARSIGN_ARROW_CIRCLEPATH: SFSymbol = SFSymbol { name: "dollarsign.arrow.circlepath" };
pub const DOLLARSIGN_CIRCLE: SFSymbol = SFSymbol { name: "dollarsign.circle" };
pub const DOLLARSIGN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "dollarsign.circle.fill" };
pub const DOLLARSIGN_SQUARE: SFSymbol = SFSymbol { name: "dollarsign.square" };
pub const DOLLARSIGN_SQUARE_FILL: SFSymbol = SFSymbol { name: "dollarsign.square.fill" };
pub const DONGSIGN: SFSymbol = SFSymbol { name: "dongsign" };
pub const DONGSIGN_CIRCLE: SFSymbol = SFSymbol { name: "dongsign.circle" };
pub const DONGSIGN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "dongsign.circle.fill" };
pub const DONGSIGN_SQUARE: SFSymbol = SFSymbol { name: "dongsign.square" };
pub const DONGSIGN_SQUARE_FILL: SFSymbol = SFSymbol { name: "dongsign.square.fill" };
pub const DOOR_FRENCH_CLOSED: SFSymbol = SFSymbol { name: "door.french.closed" };
pub const DOOR_FRENCH_OPEN: SFSymbol = SFSymbol { name: "door.french.open" };
pub const DOOR_GARAGE_CLOSED: SFSymbol = SFSymbol { name: "door.garage.closed" };
pub const DOOR_GARAGE_CLOSED_TRIANGLEBADGE_EXCLAMATIONMARK: SFSymbol = SFSymbol { name: "door.garage.closed.trianglebadge.exclamationmark" };
pub const DOOR_GARAGE_DOUBLE_BAY_CLOSED: SFSymbol = SFSymbol { name: "door.garage.double.bay.closed" };
pub const DOOR_GARAGE_DOUBLE_BAY_CLOSED_TRIANGLEBADGE_EXCLAMATIONMARK: SFSymbol = SFSymbol { name: "door.garage.double.bay.closed.trianglebadge.exclamationmark" };
pub const DOOR_GARAGE_DOUBLE_BAY_OPEN: SFSymbol = SFSymbol { name: "door.garage.double.bay.open" };
pub const DOOR_GARAGE_DOUBLE_BAY_OPEN_TRIANGLEBADGE_EXCLAMATIONMARK: SFSymbol = SFSymbol { name: "door.garage.double.bay.open.trianglebadge.exclamationmark" };
pub const DOOR_GARAGE_OPEN: SFSymbol = SFSymbol { name: "door.garage.open" };
pub const DOOR_GARAGE_OPEN_TRIANGLEBADGE_EXCLAMATIONMARK: SFSymbol = SFSymbol { name: "door.garage.open.trianglebadge.exclamationmark" };
pub const DOOR_LEFT_HAND_CLOSED: SFSymbol = SFSymbol { name: "door.left.hand.closed" };
pub const DOOR_LEFT_HAND_OPEN: SFSymbol = SFSymbol { name: "door.left.hand.open" };
pub const DOOR_RIGHT_HAND_CLOSED: SFSymbol = SFSymbol { name: "door.right.hand.closed" };
pub const DOOR_RIGHT_HAND_OPEN: SFSymbol = SFSymbol { name: "door.right.hand.open" };
pub const DOOR_SLIDING_LEFT_HAND_CLOSED: SFSymbol = SFSymbol { name: "door.sliding.left.hand.closed" };
pub const DOOR_SLIDING_LEFT_HAND_OPEN: SFSymbol = SFSymbol { name: "door.sliding.left.hand.open" };
pub const DOOR_SLIDING_RIGHT_HAND_CLOSED: SFSymbol = SFSymbol { name: "door.sliding.right.hand.closed" };
pub const DOOR_SLIDING_RIGHT_HAND_OPEN: SFSymbol = SFSymbol { name: "door.sliding.right.hand.open" };
pub const DOT_ARROWTRIANGLES_UP_RIGHT_DOWN_LEFT_CIRCLE: SFSymbol = SFSymbol { name: "dot.arrowtriangles.up.right.down.left.circle" };
pub const DOT_CIRCLE_AND_CURSORARROW: SFSymbol = SFSymbol { name: "dot.circle.and.cursorarrow" };
pub const DOT_CIRCLE_AND_HAND_POINT_UP_LEFT_FILL: SFSymbol = SFSymbol { name: "dot.circle.and.hand.point.up.left.fill" };
pub const DOT_CIRCLE_VIEWFINDER: SFSymbol = SFSymbol { name: "dot.circle.viewfinder" };
pub const DOT_RADIOWAVES_FORWARD: SFSymbol = SFSymbol { name: "dot.radiowaves.forward" };
pub const DOT_RADIOWAVES_LEFT_AND_RIGHT: SFSymbol = SFSymbol { name: "dot.radiowaves.left.and.right" };
pub const DOT_RADIOWAVES_RIGHT: SFSymbol = SFSymbol { name: "dot.radiowaves.right" };
pub const DOT_RADIOWAVES_UP_FORWARD: SFSymbol = SFSymbol { name: "dot.radiowaves.up.forward" };
pub const DOT_SQUARE: SFSymbol = SFSymbol { name: "dot.square" };
pub const DOT_SQUARE_FILL: SFSymbol = SFSymbol { name: "dot.square.fill" };
pub const DOT_SQUARESHAPE: SFSymbol = SFSymbol { name: "dot.squareshape" };
pub const DOT_SQUARESHAPE_FILL: SFSymbol = SFSymbol { name: "dot.squareshape.fill" };
pub const DOT_SQUARESHAPE_SPLIT_2X2: SFSymbol = SFSymbol { name: "dot.squareshape.split.2x2" };
pub const DOT_VIEWFINDER: SFSymbol = SFSymbol { name: "dot.viewfinder" };
pub const DOTS_AND_LINE_VERTICAL_AND_CURSORARROW_RECTANGLE: SFSymbol = SFSymbol { name: "dots.and.line.vertical.and.cursorarrow.rectangle" };
pub const DPAD: SFSymbol = SFSymbol { name: "dpad" };
pub const DPAD_DOWN_FILLED: SFSymbol = SFSymbol { name: "dpad.down.filled" };
pub const DPAD_FILL: SFSymbol = SFSymbol { name: "dpad.fill" };
pub const DPAD_LEFT_FILLED: SFSymbol = SFSymbol { name: "dpad.left.filled" };
pub const DPAD_RIGHT_FILLED: SFSymbol = SFSymbol { name: "dpad.right.filled" };
pub const DPAD_UP_FILLED: SFSymbol = SFSymbol { name: "dpad.up.filled" };
pub const DROP: SFSymbol = SFSymbol { name: "drop" };
pub const DROP_CIRCLE: SFSymbol = SFSymbol { name: "drop.circle" };
pub const DROP_CIRCLE_FILL: SFSymbol = SFSymbol { name: "drop.circle.fill" };
pub const DROP_DEGREESIGN: SFSymbol = SFSymbol { name: "drop.degreesign" };
pub const DROP_DEGREESIGN_FILL: SFSymbol = SFSymbol { name: "drop.degreesign.fill" };
pub const DROP_DEGREESIGN_SLASH: SFSymbol = SFSymbol { name: "drop.degreesign.slash" };
pub const DROP_DEGREESIGN_SLASH_FILL: SFSymbol = SFSymbol { name: "drop.degreesign.slash.fill" };
pub const DROP_FILL: SFSymbol = SFSymbol { name: "drop.fill" };
pub const DROP_KEYPAD_RECTANGLE: SFSymbol = SFSymbol { name: "drop.keypad.rectangle" };
pub const DROP_KEYPAD_RECTANGLE_FILL: SFSymbol = SFSymbol { name: "drop.keypad.rectangle.fill" };
pub const DROP_TRIANGLE: SFSymbol = SFSymbol { name: "drop.triangle" };
pub const DROP_TRIANGLE_FILL: SFSymbol = SFSymbol { name: "drop.triangle.fill" };
pub const DRYER: SFSymbol = SFSymbol { name: "dryer" };
pub const DRYER_FILL: SFSymbol = SFSymbol { name: "dryer.fill" };
pub const DUMBBELL: SFSymbol = SFSymbol { name: "dumbbell" };
pub const DUMBBELL_FILL: SFSymbol = SFSymbol { name: "dumbbell.fill" };
pub const E_CIRCLE: SFSymbol = SFSymbol { name: "e.circle" };
pub const E_CIRCLE_FILL: SFSymbol = SFSymbol { name: "e.circle.fill" };
pub const E_SQUARE: SFSymbol = SFSymbol { name: "e.square" };
pub const E_SQUARE_FILL: SFSymbol = SFSymbol { name: "e.square.fill" };
pub const EAR: SFSymbol = SFSymbol { name: "ear" };
pub const EAR_AND_WAVEFORM: SFSymbol = SFSymbol { name: "ear.and.waveform" };
pub const EAR_BADGE_CHECKMARK: SFSymbol = SFSymbol { name: "ear.badge.checkmark" };
pub const EAR_FILL: SFSymbol = SFSymbol { name: "ear.fill" };
pub const EAR_TRIANGLEBADGE_EXCLAMATIONMARK: SFSymbol = SFSymbol { name: "ear.trianglebadge.exclamationmark" };
pub const EARBUDS: SFSymbol = SFSymbol { name: "earbuds" };
pub const EARBUDS_CASE: SFSymbol = SFSymbol { name: "earbuds.case" };
pub const EARBUDS_CASE_FILL: SFSymbol = SFSymbol { name: "earbuds.case.fill" };
pub const EARPODS: SFSymbol = SFSymbol { name: "earpods" };
pub const EJECT: SFSymbol = SFSymbol { name: "eject" };
pub const EJECT_CIRCLE: SFSymbol = SFSymbol { name: "eject.circle" };
pub const EJECT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "eject.circle.fill" };
pub const EJECT_FILL: SFSymbol = SFSymbol { name: "eject.fill" };
pub const ELLIPSIS: SFSymbol = SFSymbol { name: "ellipsis" };
pub const ELLIPSIS_BUBBLE: SFSymbol = SFSymbol { name: "ellipsis.bubble" };
pub const ELLIPSIS_BUBBLE_FILL: SFSymbol = SFSymbol { name: "ellipsis.bubble.fill" };
pub const ELLIPSIS_CIRCLE: SFSymbol = SFSymbol { name: "ellipsis.circle" };
pub const ELLIPSIS_CIRCLE_FILL: SFSymbol = SFSymbol { name: "ellipsis.circle.fill" };
pub const ELLIPSIS_CURLYBRACES: SFSymbol = SFSymbol { name: "ellipsis.curlybraces" };
pub const ELLIPSIS_MESSAGE: SFSymbol = SFSymbol { name: "ellipsis.message" };
pub const ELLIPSIS_MESSAGE_FILL: SFSymbol = SFSymbol { name: "ellipsis.message.fill" };
pub const ELLIPSIS_RECTANGLE: SFSymbol = SFSymbol { name: "ellipsis.rectangle" };
pub const ELLIPSIS_RECTANGLE_FILL: SFSymbol = SFSymbol { name: "ellipsis.rectangle.fill" };
pub const ELLIPSIS_VERTICAL_BUBBLE: SFSymbol = SFSymbol { name: "ellipsis.vertical.bubble" };
pub const ELLIPSIS_VERTICAL_BUBBLE_FILL: SFSymbol = SFSymbol { name: "ellipsis.vertical.bubble.fill" };
pub const ENTRY_LEVER_KEYPAD: SFSymbol = SFSymbol { name: "entry.lever.keypad" };
pub const ENTRY_LEVER_KEYPAD_FILL: SFSymbol = SFSymbol { name: "entry.lever.keypad.fill" };
pub const ENTRY_LEVER_KEYPAD_TRIANGLEBADGE_EXCLAMATIONMARK: SFSymbol = SFSymbol { name: "entry.lever.keypad.trianglebadge.exclamationmark" };
pub const ENTRY_LEVER_KEYPAD_TRIANGLEBADGE_EXCLAMATIONMARK_FILL: SFSymbol = SFSymbol { name: "entry.lever.keypad.trianglebadge.exclamationmark.fill" };
pub const ENVELOPE: SFSymbol = SFSymbol { name: "envelope" };
pub const ENVELOPE_ARROW_TRIANGLE_BRANCH: SFSymbol = SFSymbol { name: "envelope.arrow.triangle.branch" };
pub const ENVELOPE_ARROW_TRIANGLE_BRANCH_FILL: SFSymbol = SFSymbol { name: "envelope.arrow.triangle.branch.fill" };
pub const ENVELOPE_BADGE: SFSymbol = SFSymbol { name: "envelope.badge" };
pub const ENVELOPE_BADGE_FILL: SFSymbol = SFSymbol { name: "envelope.badge.fill" };
pub const ENVELOPE_BADGE_SHIELD_HALF_FILLED: SFSymbol = SFSymbol { name: "envelope.badge.shield.half.filled" };
pub const ENVELOPE_BADGE_SHIELD_HALF_FILLED_FILL: SFSymbol = SFSymbol { name: "envelope.badge.shield.half.filled.fill" };
pub const ENVELOPE_CIRCLE: SFSymbol = SFSymbol { name: "envelope.circle" };
pub const ENVELOPE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "envelope.circle.fill" };
pub const ENVELOPE_FILL: SFSymbol = SFSymbol { name: "envelope.fill" };
pub const ENVELOPE_OPEN: SFSymbol = SFSymbol { name: "envelope.open" };
pub const ENVELOPE_OPEN_BADGE_CLOCK: SFSymbol = SFSymbol { name: "envelope.open.badge.clock" };
pub const ENVELOPE_OPEN_FILL: SFSymbol = SFSymbol { name: "envelope.open.fill" };
pub const EQUAL: SFSymbol = SFSymbol { name: "equal" };
pub const EQUAL_CIRCLE: SFSymbol = SFSymbol { name: "equal.circle" };
pub const EQUAL_CIRCLE_FILL: SFSymbol = SFSymbol { name: "equal.circle.fill" };
pub const EQUAL_SQUARE: SFSymbol = SFSymbol { name: "equal.square" };
pub const EQUAL_SQUARE_FILL: SFSymbol = SFSymbol { name: "equal.square.fill" };
pub const ERASER: SFSymbol = SFSymbol { name: "eraser" };
pub const ERASER_FILL: SFSymbol = SFSymbol { name: "eraser.fill" };
pub const ERASER_LINE_DASHED: SFSymbol = SFSymbol { name: "eraser.line.dashed" };
pub const ERASER_LINE_DASHED_FILL: SFSymbol = SFSymbol { name: "eraser.line.dashed.fill" };
pub const ESCAPE: SFSymbol = SFSymbol { name: "escape" };
pub const ESIM: SFSymbol = SFSymbol { name: "esim" };
pub const ESIM_FILL: SFSymbol = SFSymbol { name: "esim.fill" };
pub const EUROSIGN: SFSymbol = SFSymbol { name: "eurosign" };
pub const EUROSIGN_CIRCLE: SFSymbol = SFSymbol { name: "eurosign.circle" };
pub const EUROSIGN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "eurosign.circle.fill" };
pub const EUROSIGN_SQUARE: SFSymbol = SFSymbol { name: "eurosign.square" };
pub const EUROSIGN_SQUARE_FILL: SFSymbol = SFSymbol { name: "eurosign.square.fill" };
pub const EXCLAMATIONMARK: SFSymbol = SFSymbol { name: "exclamationmark" };
pub const EXCLAMATIONMARK_2: SFSymbol = SFSymbol { name: "exclamationmark.2" };
pub const EXCLAMATIONMARK_3: SFSymbol = SFSymbol { name: "exclamationmark.3" };
pub const EXCLAMATIONMARK_APPLEWATCH: SFSymbol = SFSymbol { name: "exclamationmark.applewatch" };
pub const EXCLAMATIONMARK_ARROW_CIRCLEPATH: SFSymbol = SFSymbol { name: "exclamationmark.arrow.circlepath" };
pub const EXCLAMATIONMARK_ARROW_TRIANGLE_2_CIRCLEPATH: SFSymbol = SFSymbol { name: "exclamationmark.arrow.triangle.2.circlepath" };
pub const EXCLAMATIONMARK_BUBBLE: SFSymbol = SFSymbol { name: "exclamationmark.bubble" };
pub const EXCLAMATIONMARK_BUBBLE_CIRCLE: SFSymbol = SFSymbol { name: "exclamationmark.bubble.circle" };
pub const EXCLAMATIONMARK_BUBBLE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "exclamationmark.bubble.circle.fill" };
pub const EXCLAMATIONMARK_BUBBLE_FILL: SFSymbol = SFSymbol { name: "exclamationmark.bubble.fill" };
pub const EXCLAMATIONMARK_CIRCLE: SFSymbol = SFSymbol { name: "exclamationmark.circle" };
pub const EXCLAMATIONMARK_CIRCLE_FILL: SFSymbol = SFSymbol { name: "exclamationmark.circle.fill" };
pub const EXCLAMATIONMARK_ICLOUD: SFSymbol = SFSymbol { name: "exclamationmark.icloud" };
pub const EXCLAMATIONMARK_ICLOUD_FILL: SFSymbol = SFSymbol { name: "exclamationmark.icloud.fill" };
pub const EXCLAMATIONMARK_LOCK: SFSymbol = SFSymbol { name: "exclamationmark.lock" };
pub const EXCLAMATIONMARK_LOCK_FILL: SFSymbol = SFSymbol { name: "exclamationmark.lock.fill" };
pub const EXCLAMATIONMARK_OCTAGON: SFSymbol = SFSymbol { name: "exclamationmark.octagon" };
pub const EXCLAMATIONMARK_OCTAGON_FILL: SFSymbol = SFSymbol { name: "exclamationmark.octagon.fill" };
pub const EXCLAMATIONMARK_QUESTIONMARK: SFSymbol = SFSymbol { name: "exclamationmark.questionmark" };
pub const EXCLAMATIONMARK_SHIELD: SFSymbol = SFSymbol { name: "exclamationmark.shield" };
pub const EXCLAMATIONMARK_SHIELD_FILL: SFSymbol = SFSymbol { name: "exclamationmark.shield.fill" };
pub const EXCLAMATIONMARK_SQUARE: SFSymbol = SFSymbol { name: "exclamationmark.square" };
pub const EXCLAMATIONMARK_SQUARE_FILL: SFSymbol = SFSymbol { name: "exclamationmark.square.fill" };
pub const EXCLAMATIONMARK_TRIANGLE: SFSymbol = SFSymbol { name: "exclamationmark.triangle" };
pub const EXCLAMATIONMARK_TRIANGLE_FILL: SFSymbol = SFSymbol { name: "exclamationmark.triangle.fill" };
pub const EXTERNALDRIVE: SFSymbol = SFSymbol { name: "externaldrive" };
pub const EXTERNALDRIVE_BADGE_CHECKMARK: SFSymbol = SFSymbol { name: "externaldrive.badge.checkmark" };
pub const EXTERNALDRIVE_BADGE_EXCLAMATIONMARK: SFSymbol = SFSymbol { name: "externaldrive.badge.exclamationmark" };
pub const EXTERNALDRIVE_BADGE_ICLOUD: SFSymbol = SFSymbol { name: "externaldrive.badge.icloud" };
pub const EXTERNALDRIVE_BADGE_MINUS: SFSymbol = SFSymbol { name: "externaldrive.badge.minus" };
pub const EXTERNALDRIVE_BADGE_PERSON_CROP: SFSymbol = SFSymbol { name: "externaldrive.badge.person.crop" };
pub const EXTERNALDRIVE_BADGE_PLUS: SFSymbol = SFSymbol { name: "externaldrive.badge.plus" };
pub const EXTERNALDRIVE_BADGE_QUESTIONMARK: SFSymbol = SFSymbol { name: "externaldrive.badge.questionmark" };
pub const EXTERNALDRIVE_BADGE_TIMEMACHINE: SFSymbol = SFSymbol { name: "externaldrive.badge.timemachine" };
pub const EXTERNALDRIVE_BADGE_WIFI: SFSymbol = SFSymbol { name: "externaldrive.badge.wifi" };
pub const EXTERNALDRIVE_BADGE_XMARK: SFSymbol = SFSymbol { name: "externaldrive.badge.xmark" };
pub const EXTERNALDRIVE_CONNECTED_TO_LINE_BELOW: SFSymbol = SFSymbol { name: "externaldrive.connected.to.line.below" };
pub const EXTERNALDRIVE_CONNECTED_TO_LINE_BELOW_FILL: SFSymbol = SFSymbol { name: "externaldrive.connected.to.line.below.fill" };
pub const EXTERNALDRIVE_FILL: SFSymbol = SFSymbol { name: "externaldrive.fill" };
pub const EXTERNALDRIVE_FILL_BADGE_CHECKMARK: SFSymbol = SFSymbol { name: "externaldrive.fill.badge.checkmark" };
pub const EXTERNALDRIVE_FILL_BADGE_EXCLAMATIONMARK: SFSymbol = SFSymbol { name: "externaldrive.fill.badge.exclamationmark" };
pub const EXTERNALDRIVE_FILL_BADGE_ICLOUD: SFSymbol = SFSymbol { name: "externaldrive.fill.badge.icloud" };
pub const EXTERNALDRIVE_FILL_BADGE_MINUS: SFSymbol = SFSymbol { name: "externaldrive.fill.badge.minus" };
pub const EXTERNALDRIVE_FILL_BADGE_PERSON_CROP: SFSymbol = SFSymbol { name: "externaldrive.fill.badge.person.crop" };
pub const EXTERNALDRIVE_FILL_BADGE_PLUS: SFSymbol = SFSymbol { name: "externaldrive.fill.badge.plus" };
pub const EXTERNALDRIVE_FILL_BADGE_QUESTIONMARK: SFSymbol = SFSymbol { name: "externaldrive.fill.badge.questionmark" };
pub const EXTERNALDRIVE_FILL_BADGE_TIMEMACHINE: SFSymbol = SFSymbol { name: "externaldrive.fill.badge.timemachine" };
pub const EXTERNALDRIVE_FILL_BADGE_WIFI: SFSymbol = SFSymbol { name: "externaldrive.fill.badge.wifi" };
pub const EXTERNALDRIVE_FILL_BADGE_XMARK: SFSymbol = SFSymbol { name: "externaldrive.fill.badge.xmark" };
pub const EXTERNALDRIVE_FILL_TRIANGLEBADGE_EXCLAMATIONMARK: SFSymbol = SFSymbol { name: "externaldrive.fill.trianglebadge.exclamationmark" };
pub const EXTERNALDRIVE_TRIANGLEBADGE_EXCLAMATIONMARK: SFSymbol = SFSymbol { name: "externaldrive.trianglebadge.exclamationmark" };
pub const EYE: SFSymbol = SFSymbol { name: "eye" };
pub const EYE_CIRCLE: SFSymbol = SFSymbol { name: "eye.circle" };
pub const EYE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "eye.circle.fill" };
pub const EYE_FILL: SFSymbol = SFSymbol { name: "eye.fill" };
pub const EYE_SLASH: SFSymbol = SFSymbol { name: "eye.slash" };
pub const EYE_SLASH_CIRCLE: SFSymbol = SFSymbol { name: "eye.slash.circle" };
pub const EYE_SLASH_CIRCLE_FILL: SFSymbol = SFSymbol { name: "eye.slash.circle.fill" };
pub const EYE_SLASH_FILL: SFSymbol = SFSymbol { name: "eye.slash.fill" };
pub const EYE_SQUARE: SFSymbol = SFSymbol { name: "eye.square" };
pub const EYE_SQUARE_FILL: SFSymbol = SFSymbol { name: "eye.square.fill" };
pub const EYE_TRIANGLEBADGE_EXCLAMATIONMARK: SFSymbol = SFSymbol { name: "eye.trianglebadge.exclamationmark" };
pub const EYE_TRIANGLEBADGE_EXCLAMATIONMARK_FILL: SFSymbol = SFSymbol { name: "eye.trianglebadge.exclamationmark.fill" };
pub const EYEBROW: SFSymbol = SFSymbol { name: "eyebrow" };
pub const EYEDROPPER: SFSymbol = SFSymbol { name: "eyedropper" };
pub const EYEDROPPER_FULL: SFSymbol = SFSymbol { name: "eyedropper.full" };
pub const EYEDROPPER_HALFFULL: SFSymbol = SFSymbol { name: "eyedropper.halffull" };
pub const EYEGLASSES: SFSymbol = SFSymbol { name: "eyeglasses" };
pub const EYES: SFSymbol = SFSymbol { name: "eyes" };
pub const EYES_INVERSE: SFSymbol = SFSymbol { name: "eyes.inverse" };
pub const F_CIRCLE: SFSymbol = SFSymbol { name: "f.circle" };
pub const F_CIRCLE_FILL: SFSymbol = SFSymbol { name: "f.circle.fill" };
pub const F_CURSIVE: SFSymbol = SFSymbol { name: "f.cursive" };
pub const F_CURSIVE_CIRCLE: SFSymbol = SFSymbol { name: "f.cursive.circle" };
pub const F_CURSIVE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "f.cursive.circle.fill" };
pub const F_SQUARE: SFSymbol = SFSymbol { name: "f.square" };
pub const F_SQUARE_FILL: SFSymbol = SFSymbol { name: "f.square.fill" };
pub const FACE_DASHED: SFSymbol = SFSymbol { name: "face.dashed" };
pub const FACE_DASHED_FILL: SFSymbol = SFSymbol { name: "face.dashed.fill" };
pub const FACE_SMILING: SFSymbol = SFSymbol { name: "face.smiling" };
pub const FACE_SMILING_FILL: SFSymbol = SFSymbol { name: "face.smiling.fill" };
pub const FACEID: SFSymbol = SFSymbol { name: "faceid" };
pub const FACEMASK: SFSymbol = SFSymbol { name: "facemask" };
pub const FACEMASK_FILL: SFSymbol = SFSymbol { name: "facemask.fill" };
pub const FAN_AND_LIGHT_CEILING: SFSymbol = SFSymbol { name: "fan.and.light.ceiling" };
pub const FAN_AND_LIGHT_CEILING_FILL: SFSymbol = SFSymbol { name: "fan.and.light.ceiling.fill" };
pub const FAN_CEILING: SFSymbol = SFSymbol { name: "fan.ceiling" };
pub const FAN_CEILING_FILL: SFSymbol = SFSymbol { name: "fan.ceiling.fill" };
pub const FAN_DESK: SFSymbol = SFSymbol { name: "fan.desk" };
pub const FAN_DESK_FILL: SFSymbol = SFSymbol { name: "fan.desk.fill" };
pub const FAN_FLOOR: SFSymbol = SFSymbol { name: "fan.floor" };
pub const FAN_FLOOR_FILL: SFSymbol = SFSymbol { name: "fan.floor.fill" };
pub const FAN_OSCILLATION: SFSymbol = SFSymbol { name: "fan.oscillation" };
pub const FAN_OSCILLATION_FILL: SFSymbol = SFSymbol { name: "fan.oscillation.fill" };
pub const FANBLADES: SFSymbol = SFSymbol { name: "fanblades" };
pub const FANBLADES_FILL: SFSymbol = SFSymbol { name: "fanblades.fill" };
pub const FAXMACHINE: SFSymbol = SFSymbol { name: "faxmachine" };
pub const FAXMACHINE_FILL: SFSymbol = SFSymbol { name: "faxmachine.fill" };
pub const FERRY: SFSymbol = SFSymbol { name: "ferry" };
pub const FERRY_FILL: SFSymbol = SFSymbol { name: "ferry.fill" };
pub const FIBRECHANNEL: SFSymbol = SFSymbol { name: "fibrechannel" };
pub const FIGURE_2_AND_CHILD_HOLDINGHANDS: SFSymbol = SFSymbol { name: "figure.2.and.child.holdinghands" };
pub const FIGURE_2_ARMS_OPEN: SFSymbol = SFSymbol { name: "figure.2.arms.open" };
pub const FIGURE_AMERICAN_FOOTBALL: SFSymbol = SFSymbol { name: "figure.american.football" };
pub const FIGURE_AND_CHILD_HOLDINGHANDS: SFSymbol = SFSymbol { name: "figure.and.child.holdinghands" };
pub const FIGURE_ARCHERY: SFSymbol = SFSymbol { name: "figure.archery" };
pub const FIGURE_ARMS_OPEN: SFSymbol = SFSymbol { name: "figure.arms.open" };
pub const FIGURE_AUSTRALIAN_FOOTBALL: SFSymbol = SFSymbol { name: "figure.australian.football" };
pub const FIGURE_BADMINTON: SFSymbol = SFSymbol { name: "figure.badminton" };
pub const FIGURE_BARRE: SFSymbol = SFSymbol { name: "figure.barre" };
pub const FIGURE_BASEBALL: SFSymbol = SFSymbol { name: "figure.baseball" };
pub const FIGURE_BASKETBALL: SFSymbol = SFSymbol { name: "figure.basketball" };
pub const FIGURE_BOWLING: SFSymbol = SFSymbol { name: "figure.bowling" };
pub const FIGURE_BOXING: SFSymbol = SFSymbol { name: "figure.boxing" };
pub const FIGURE_CLIMBING: SFSymbol = SFSymbol { name: "figure.climbing" };
pub const FIGURE_COOLDOWN: SFSymbol = SFSymbol { name: "figure.cooldown" };
pub const FIGURE_CORE_TRAINING: SFSymbol = SFSymbol { name: "figure.core.training" };
pub const FIGURE_CRICKET: SFSymbol = SFSymbol { name: "figure.cricket" };
pub const FIGURE_CROSS_TRAINING: SFSymbol = SFSymbol { name: "figure.cross.training" };
pub const FIGURE_CURLING: SFSymbol = SFSymbol { name: "figure.curling" };
pub const FIGURE_DANCE: SFSymbol = SFSymbol { name: "figure.dance" };
pub const FIGURE_DISC_SPORTS: SFSymbol = SFSymbol { name: "figure.disc.sports" };
pub const FIGURE_DRESS_LINE_VERTICAL_FIGURE: SFSymbol = SFSymbol { name: "figure.dress.line.vertical.figure" };
pub const FIGURE_ELLIPTICAL: SFSymbol = SFSymbol { name: "figure.elliptical" };
pub const FIGURE_EQUESTRIAN_SPORTS: SFSymbol = SFSymbol { name: "figure.equestrian.sports" };
pub const FIGURE_FALL: SFSymbol = SFSymbol { name: "figure.fall" };
pub const FIGURE_FALL_CIRCLE: SFSymbol = SFSymbol { name: "figure.fall.circle" };
pub const FIGURE_FALL_CIRCLE_FILL: SFSymbol = SFSymbol { name: "figure.fall.circle.fill" };
pub const FIGURE_FENCING: SFSymbol = SFSymbol { name: "figure.fencing" };
pub const FIGURE_FISHING: SFSymbol = SFSymbol { name: "figure.fishing" };
pub const FIGURE_FLEXIBILITY: SFSymbol = SFSymbol { name: "figure.flexibility" };
pub const FIGURE_GOLF: SFSymbol = SFSymbol { name: "figure.golf" };
pub const FIGURE_GYMNASTICS: SFSymbol = SFSymbol { name: "figure.gymnastics" };
pub const FIGURE_HAND_CYCLING: SFSymbol = SFSymbol { name: "figure.hand.cycling" };
pub const FIGURE_HANDBALL: SFSymbol = SFSymbol { name: "figure.handball" };
pub const FIGURE_HIGHINTENSITY_INTERVALTRAINING: SFSymbol = SFSymbol { name: "figure.highintensity.intervaltraining" };
pub const FIGURE_HIKING: SFSymbol = SFSymbol { name: "figure.hiking" };
pub const FIGURE_HOCKEY: SFSymbol = SFSymbol { name: "figure.hockey" };
pub const FIGURE_HUNTING: SFSymbol = SFSymbol { name: "figure.hunting" };
pub const FIGURE_INDOOR_CYCLE: SFSymbol = SFSymbol { name: "figure.indoor.cycle" };
pub const FIGURE_JUMPROPE: SFSymbol = SFSymbol { name: "figure.jumprope" };
pub const FIGURE_KICKBOXING: SFSymbol = SFSymbol { name: "figure.kickboxing" };
pub const FIGURE_LACROSSE: SFSymbol = SFSymbol { name: "figure.lacrosse" };
pub const FIGURE_MARTIAL_ARTS: SFSymbol = SFSymbol { name: "figure.martial.arts" };
pub const FIGURE_MIND_AND_BODY: SFSymbol = SFSymbol { name: "figure.mind.and.body" };
pub const FIGURE_MIXED_CARDIO: SFSymbol = SFSymbol { name: "figure.mixed.cardio" };
pub const FIGURE_OPEN_WATER_SWIM: SFSymbol = SFSymbol { name: "figure.open.water.swim" };
pub const FIGURE_OUTDOOR_CYCLE: SFSymbol = SFSymbol { name: "figure.outdoor.cycle" };
pub const FIGURE_PICKLEBALL: SFSymbol = SFSymbol { name: "figure.pickleball" };
pub const FIGURE_PILATES: SFSymbol = SFSymbol { name: "figure.pilates" };
pub const FIGURE_PLAY: SFSymbol = SFSymbol { name: "figure.play" };
pub const FIGURE_POOL_SWIM: SFSymbol = SFSymbol { name: "figure.pool.swim" };
pub const FIGURE_RACQUETBALL: SFSymbol = SFSymbol { name: "figure.racquetball" };
pub const FIGURE_ROLL: SFSymbol = SFSymbol { name: "figure.roll" };
pub const FIGURE_ROLL_RUNNINGPACE: SFSymbol = SFSymbol { name: "figure.roll.runningpace" };
pub const FIGURE_ROLLING: SFSymbol = SFSymbol { name: "figure.rolling" };
pub const FIGURE_ROWER: SFSymbol = SFSymbol { name: "figure.rower" };
pub const FIGURE_RUGBY: SFSymbol = SFSymbol { name: "figure.rugby" };
pub const FIGURE_RUN: SFSymbol = SFSymbol { name: "figure.run" };
pub const FIGURE_RUN_CIRCLE: SFSymbol = SFSymbol { name: "figure.run.circle" };
pub const FIGURE_RUN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "figure.run.circle.fill" };
pub const FIGURE_SAILING: SFSymbol = SFSymbol { name: "figure.sailing" };
pub const FIGURE_SKATING: SFSymbol = SFSymbol { name: "figure.skating" };
pub const FIGURE_SKIING_CROSSCOUNTRY: SFSymbol = SFSymbol { name: "figure.skiing.crosscountry" };
pub const FIGURE_SKIING_DOWNHILL: SFSymbol = SFSymbol { name: "figure.skiing.downhill" };
pub const FIGURE_SNOWBOARDING: SFSymbol = SFSymbol { name: "figure.snowboarding" };
pub const FIGURE_SOCCER: SFSymbol = SFSymbol { name: "figure.soccer" };
pub const FIGURE_SOCIALDANCE: SFSymbol = SFSymbol { name: "figure.socialdance" };
pub const FIGURE_SOFTBALL: SFSymbol = SFSymbol { name: "figure.softball" };
pub const FIGURE_SQUASH: SFSymbol = SFSymbol { name: "figure.squash" };
pub const FIGURE_STAIR_STEPPER: SFSymbol = SFSymbol { name: "figure.stair.stepper" };
pub const FIGURE_STAIRS: SFSymbol = SFSymbol { name: "figure.stairs" };
pub const FIGURE_STAND: SFSymbol = SFSymbol { name: "figure.stand" };
pub const FIGURE_STAND_LINE_DOTTED_FIGURE_STAND: SFSymbol = SFSymbol { name: "figure.stand.line.dotted.figure.stand" };
pub const FIGURE_STEP_TRAINING: SFSymbol = SFSymbol { name: "figure.step.training" };
pub const FIGURE_STRENGTHTRAINING_FUNCTIONAL: SFSymbol = SFSymbol { name: "figure.strengthtraining.functional" };
pub const FIGURE_STRENGTHTRAINING_TRADITIONAL: SFSymbol = SFSymbol { name: "figure.strengthtraining.traditional" };
pub const FIGURE_SURFING: SFSymbol = SFSymbol { name: "figure.surfing" };
pub const FIGURE_TABLE_TENNIS: SFSymbol = SFSymbol { name: "figure.table.tennis" };
pub const FIGURE_TAICHI: SFSymbol = SFSymbol { name: "figure.taichi" };
pub const FIGURE_TENNIS: SFSymbol = SFSymbol { name: "figure.tennis" };
pub const FIGURE_TRACK_AND_FIELD: SFSymbol = SFSymbol { name: "figure.track.and.field" };
pub const FIGURE_VOLLEYBALL: SFSymbol = SFSymbol { name: "figure.volleyball" };
pub const FIGURE_WALK: SFSymbol = SFSymbol { name: "figure.walk" };
pub const FIGURE_WALK_ARRIVAL: SFSymbol = SFSymbol { name: "figure.walk.arrival" };
pub const FIGURE_WALK_CIRCLE: SFSymbol = SFSymbol { name: "figure.walk.circle" };
pub const FIGURE_WALK_CIRCLE_FILL: SFSymbol = SFSymbol { name: "figure.walk.circle.fill" };
pub const FIGURE_WALK_DEPARTURE: SFSymbol = SFSymbol { name: "figure.walk.departure" };
pub const FIGURE_WALK_DIAMOND: SFSymbol = SFSymbol { name: "figure.walk.diamond" };
pub const FIGURE_WALK_DIAMOND_FILL: SFSymbol = SFSymbol { name: "figure.walk.diamond.fill" };
pub const FIGURE_WALK_MOTION: SFSymbol = SFSymbol { name: "figure.walk.motion" };
pub const FIGURE_WATER_FITNESS: SFSymbol = SFSymbol { name: "figure.water.fitness" };
pub const FIGURE_WATERPOLO: SFSymbol = SFSymbol { name: "figure.waterpolo" };
pub const FIGURE_WAVE: SFSymbol = SFSymbol { name: "figure.wave" };
pub const FIGURE_WAVE_CIRCLE: SFSymbol = SFSymbol { name: "figure.wave.circle" };
pub const FIGURE_WAVE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "figure.wave.circle.fill" };
pub const FIGURE_WRESTLING: SFSymbol = SFSymbol { name: "figure.wrestling" };
pub const FIGURE_YOGA: SFSymbol = SFSymbol { name: "figure.yoga" };
pub const FILEMENU_AND_CURSORARROW: SFSymbol = SFSymbol { name: "filemenu.and.cursorarrow" };
pub const FILEMENU_AND_SELECTION: SFSymbol = SFSymbol { name: "filemenu.and.selection" };
pub const FILM: SFSymbol = SFSymbol { name: "film" };
pub const FILM_CIRCLE: SFSymbol = SFSymbol { name: "film.circle" };
pub const FILM_CIRCLE_FILL: SFSymbol = SFSymbol { name: "film.circle.fill" };
pub const FILM_FILL: SFSymbol = SFSymbol { name: "film.fill" };
pub const FILM_STACK: SFSymbol = SFSymbol { name: "film.stack" };
pub const FILM_STACK_FILL: SFSymbol = SFSymbol { name: "film.stack.fill" };
pub const FIREPLACE: SFSymbol = SFSymbol { name: "fireplace" };
pub const FIREPLACE_FILL: SFSymbol = SFSymbol { name: "fireplace.fill" };
pub const FIREWALL: SFSymbol = SFSymbol { name: "firewall" };
pub const FIREWALL_FILL: SFSymbol = SFSymbol { name: "firewall.fill" };
pub const FISH: SFSymbol = SFSymbol { name: "fish" };
pub const FISH_FILL: SFSymbol = SFSymbol { name: "fish.fill" };
pub const FLAG: SFSymbol = SFSymbol { name: "flag" };
pub const FLAG_2_CROSSED: SFSymbol = SFSymbol { name: "flag.2.crossed" };
pub const FLAG_2_CROSSED_CIRCLE: SFSymbol = SFSymbol { name: "flag.2.crossed.circle" };
pub const FLAG_2_CROSSED_CIRCLE_FILL: SFSymbol = SFSymbol { name: "flag.2.crossed.circle.fill" };
pub const FLAG_2_CROSSED_FILL: SFSymbol = SFSymbol { name: "flag.2.crossed.fill" };
pub const FLAG_AND_FLAG_FILLED_CROSSED: SFSymbol = SFSymbol { name: "flag.and.flag.filled.crossed" };
pub const FLAG_BADGE_ELLIPSIS: SFSymbol = SFSymbol { name: "flag.badge.ellipsis" };
pub const FLAG_BADGE_ELLIPSIS_FILL: SFSymbol = SFSymbol { name: "flag.badge.ellipsis.fill" };
pub const FLAG_CHECKERED: SFSymbol = SFSymbol { name: "flag.checkered" };
pub const FLAG_CHECKERED_2_CROSSED: SFSymbol = SFSymbol { name: "flag.checkered.2.crossed" };
pub const FLAG_CIRCLE: SFSymbol = SFSymbol { name: "flag.circle" };
pub const FLAG_CIRCLE_FILL: SFSymbol = SFSymbol { name: "flag.circle.fill" };
pub const FLAG_FILL: SFSymbol = SFSymbol { name: "flag.fill" };
pub const FLAG_FILLED_AND_FLAG_CROSSED: SFSymbol = SFSymbol { name: "flag.filled.and.flag.crossed" };
pub const FLAG_SLASH: SFSymbol = SFSymbol { name: "flag.slash" };
pub const FLAG_SLASH_CIRCLE: SFSymbol = SFSymbol { name: "flag.slash.circle" };
pub const FLAG_SLASH_CIRCLE_FILL: SFSymbol = SFSymbol { name: "flag.slash.circle.fill" };
pub const FLAG_SLASH_FILL: SFSymbol = SFSymbol { name: "flag.slash.fill" };
pub const FLAG_SQUARE: SFSymbol = SFSymbol { name: "flag.square" };
pub const FLAG_SQUARE_FILL: SFSymbol = SFSymbol { name: "flag.square.fill" };
pub const FLAME: SFSymbol = SFSymbol { name: "flame" };
pub const FLAME_CIRCLE: SFSymbol = SFSymbol { name: "flame.circle" };
pub const FLAME_CIRCLE_FILL: SFSymbol = SFSymbol { name: "flame.circle.fill" };
pub const FLAME_FILL: SFSymbol = SFSymbol { name: "flame.fill" };
pub const FLASHLIGHT_OFF_FILL: SFSymbol = SFSymbol { name: "flashlight.off.fill" };
pub const FLASHLIGHT_ON_FILL: SFSymbol = SFSymbol { name: "flashlight.on.fill" };
pub const FLEURON: SFSymbol = SFSymbol { name: "fleuron" };
pub const FLEURON_FILL: SFSymbol = SFSymbol { name: "fleuron.fill" };
pub const FLIPPHONE: SFSymbol = SFSymbol { name: "flipphone" };
pub const FLORINSIGN: SFSymbol = SFSymbol { name: "florinsign" };
pub const FLORINSIGN_CIRCLE: SFSymbol = SFSymbol { name: "florinsign.circle" };
pub const FLORINSIGN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "florinsign.circle.fill" };
pub const FLORINSIGN_SQUARE: SFSymbol = SFSymbol { name: "florinsign.square" };
pub const FLORINSIGN_SQUARE_FILL: SFSymbol = SFSymbol { name: "florinsign.square.fill" };
pub const FLOWCHART: SFSymbol = SFSymbol { name: "flowchart" };
pub const FLOWCHART_FILL: SFSymbol = SFSymbol { name: "flowchart.fill" };
pub const FN: SFSymbol = SFSymbol { name: "fn" };
pub const FOLDER: SFSymbol = SFSymbol { name: "folder" };
pub const FOLDER_BADGE_GEARSHAPE: SFSymbol = SFSymbol { name: "folder.badge.gearshape" };
pub const FOLDER_BADGE_MINUS: SFSymbol = SFSymbol { name: "folder.badge.minus" };
pub const FOLDER_BADGE_PERSON_CROP: SFSymbol = SFSymbol { name: "folder.badge.person.crop" };
pub const FOLDER_BADGE_PLUS: SFSymbol = SFSymbol { name: "folder.badge.plus" };
pub const FOLDER_BADGE_QUESTIONMARK: SFSymbol = SFSymbol { name: "folder.badge.questionmark" };
pub const FOLDER_CIRCLE: SFSymbol = SFSymbol { name: "folder.circle" };
pub const FOLDER_CIRCLE_FILL: SFSymbol = SFSymbol { name: "folder.circle.fill" };
pub const FOLDER_FILL: SFSymbol = SFSymbol { name: "folder.fill" };
pub const FOLDER_FILL_BADGE_GEARSHAPE: SFSymbol = SFSymbol { name: "folder.fill.badge.gearshape" };
pub const FOLDER_FILL_BADGE_MINUS: SFSymbol = SFSymbol { name: "folder.fill.badge.minus" };
pub const FOLDER_FILL_BADGE_PERSON_CROP: SFSymbol = SFSymbol { name: "folder.fill.badge.person.crop" };
pub const FOLDER_FILL_BADGE_PLUS: SFSymbol = SFSymbol { name: "folder.fill.badge.plus" };
pub const FOLDER_FILL_BADGE_QUESTIONMARK: SFSymbol = SFSymbol { name: "folder.fill.badge.questionmark" };
pub const FOOTBALL: SFSymbol = SFSymbol { name: "football" };
pub const FOOTBALL_CIRCLE: SFSymbol = SFSymbol { name: "football.circle" };
pub const FOOTBALL_CIRCLE_FILL: SFSymbol = SFSymbol { name: "football.circle.fill" };
pub const FOOTBALL_FILL: SFSymbol = SFSymbol { name: "football.fill" };
pub const FORK_KNIFE: SFSymbol = SFSymbol { name: "fork.knife" };
pub const FORK_KNIFE_CIRCLE: SFSymbol = SFSymbol { name: "fork.knife.circle" };
pub const FORK_KNIFE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "fork.knife.circle.fill" };
pub const FORWARD: SFSymbol = SFSymbol { name: "forward" };
pub const FORWARD_CIRCLE: SFSymbol = SFSymbol { name: "forward.circle" };
pub const FORWARD_CIRCLE_FILL: SFSymbol = SFSymbol { name: "forward.circle.fill" };
pub const FORWARD_END: SFSymbol = SFSymbol { name: "forward.end" };
pub const FORWARD_END_ALT: SFSymbol = SFSymbol { name: "forward.end.alt" };
pub const FORWARD_END_ALT_FILL: SFSymbol = SFSymbol { name: "forward.end.alt.fill" };
pub const FORWARD_END_CIRCLE: SFSymbol = SFSymbol { name: "forward.end.circle" };
pub const FORWARD_END_CIRCLE_FILL: SFSymbol = SFSymbol { name: "forward.end.circle.fill" };
pub const FORWARD_END_FILL: SFSymbol = SFSymbol { name: "forward.end.fill" };
pub const FORWARD_FILL: SFSymbol = SFSymbol { name: "forward.fill" };
pub const FORWARD_FRAME: SFSymbol = SFSymbol { name: "forward.frame" };
pub const FORWARD_FRAME_FILL: SFSymbol = SFSymbol { name: "forward.frame.fill" };
pub const FOSSIL_SHELL: SFSymbol = SFSymbol { name: "fossil.shell" };
pub const FOSSIL_SHELL_FILL: SFSymbol = SFSymbol { name: "fossil.shell.fill" };
pub const FRANCSIGN: SFSymbol = SFSymbol { name: "francsign" };
pub const FRANCSIGN_CIRCLE: SFSymbol = SFSymbol { name: "francsign.circle" };
pub const FRANCSIGN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "francsign.circle.fill" };
pub const FRANCSIGN_SQUARE: SFSymbol = SFSymbol { name: "francsign.square" };
pub const FRANCSIGN_SQUARE_FILL: SFSymbol = SFSymbol { name: "francsign.square.fill" };
pub const FRYING_PAN: SFSymbol = SFSymbol { name: "frying.pan" };
pub const FRYING_PAN_FILL: SFSymbol = SFSymbol { name: "frying.pan.fill" };
pub const FUELPUMP: SFSymbol = SFSymbol { name: "fuelpump" };
pub const FUELPUMP_CIRCLE: SFSymbol = SFSymbol { name: "fuelpump.circle" };
pub const FUELPUMP_CIRCLE_FILL: SFSymbol = SFSymbol { name: "fuelpump.circle.fill" };
pub const FUELPUMP_FILL: SFSymbol = SFSymbol { name: "fuelpump.fill" };
pub const FUNCTION: SFSymbol = SFSymbol { name: "function" };
pub const FX: SFSymbol = SFSymbol { name: "fx" };
pub const G_CIRCLE: SFSymbol = SFSymbol { name: "g.circle" };
pub const G_CIRCLE_FILL: SFSymbol = SFSymbol { name: "g.circle.fill" };
pub const G_SQUARE: SFSymbol = SFSymbol { name: "g.square" };
pub const G_SQUARE_FILL: SFSymbol = SFSymbol { name: "g.square.fill" };
pub const GAMECONTROLLER: SFSymbol = SFSymbol { name: "gamecontroller" };
pub const GAMECONTROLLER_FILL: SFSymbol = SFSymbol { name: "gamecontroller.fill" };
pub const GAUGE: SFSymbol = SFSymbol { name: "gauge" };
pub const GAUGE_BADGE_MINUS: SFSymbol = SFSymbol { name: "gauge.badge.minus" };
pub const GAUGE_BADGE_PLUS: SFSymbol = SFSymbol { name: "gauge.badge.plus" };
pub const GAUGE_HIGH: SFSymbol = SFSymbol { name: "gauge.high" };
pub const GAUGE_LOW: SFSymbol = SFSymbol { name: "gauge.low" };
pub const GAUGE_MEDIUM: SFSymbol = SFSymbol { name: "gauge.medium" };
pub const GAUGE_MEDIUM_BADGE_MINUS: SFSymbol = SFSymbol { name: "gauge.medium.badge.minus" };
pub const GAUGE_MEDIUM_BADGE_PLUS: SFSymbol = SFSymbol { name: "gauge.medium.badge.plus" };
pub const GEAR: SFSymbol = SFSymbol { name: "gear" };
pub const GEAR_BADGE: SFSymbol = SFSymbol { name: "gear.badge" };
pub const GEAR_BADGE_CHECKMARK: SFSymbol = SFSymbol { name: "gear.badge.checkmark" };
pub const GEAR_BADGE_QUESTIONMARK: SFSymbol = SFSymbol { name: "gear.badge.questionmark" };
pub const GEAR_BADGE_XMARK: SFSymbol = SFSymbol { name: "gear.badge.xmark" };
pub const GEAR_CIRCLE: SFSymbol = SFSymbol { name: "gear.circle" };
pub const GEAR_CIRCLE_FILL: SFSymbol = SFSymbol { name: "gear.circle.fill" };
pub const GEARSHAPE: SFSymbol = SFSymbol { name: "gearshape" };
pub const GEARSHAPE_2: SFSymbol = SFSymbol { name: "gearshape.2" };
pub const GEARSHAPE_2_FILL: SFSymbol = SFSymbol { name: "gearshape.2.fill" };
pub const GEARSHAPE_ARROW_TRIANGLE_2_CIRCLEPATH: SFSymbol = SFSymbol { name: "gearshape.arrow.triangle.2.circlepath" };
pub const GEARSHAPE_CIRCLE: SFSymbol = SFSymbol { name: "gearshape.circle" };
pub const GEARSHAPE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "gearshape.circle.fill" };
pub const GEARSHAPE_FILL: SFSymbol = SFSymbol { name: "gearshape.fill" };
pub const GIFT: SFSymbol = SFSymbol { name: "gift" };
pub const GIFT_CIRCLE: SFSymbol = SFSymbol { name: "gift.circle" };
pub const GIFT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "gift.circle.fill" };
pub const GIFT_FILL: SFSymbol = SFSymbol { name: "gift.fill" };
pub const GIFTCARD: SFSymbol = SFSymbol { name: "giftcard" };
pub const GIFTCARD_FILL: SFSymbol = SFSymbol { name: "giftcard.fill" };
pub const GLOBE: SFSymbol = SFSymbol { name: "globe" };
pub const GLOBE_AMERICAS: SFSymbol = SFSymbol { name: "globe.americas" };
pub const GLOBE_AMERICAS_FILL: SFSymbol = SFSymbol { name: "globe.americas.fill" };
pub const GLOBE_ASIA_AUSTRALIA: SFSymbol = SFSymbol { name: "globe.asia.australia" };
pub const GLOBE_ASIA_AUSTRALIA_FILL: SFSymbol = SFSymbol { name: "globe.asia.australia.fill" };
pub const GLOBE_BADGE_CHEVRON_BACKWARD: SFSymbol = SFSymbol { name: "globe.badge.chevron.backward" };
pub const GLOBE_CENTRAL_SOUTH_ASIA: SFSymbol = SFSymbol { name: "globe.central.south.asia" };
pub const GLOBE_CENTRAL_SOUTH_ASIA_FILL: SFSymbol = SFSymbol { name: "globe.central.south.asia.fill" };
pub const GLOBE_DESK: SFSymbol = SFSymbol { name: "globe.desk" };
pub const GLOBE_DESK_FILL: SFSymbol = SFSymbol { name: "globe.desk.fill" };
pub const GLOBE_EUROPE_AFRICA: SFSymbol = SFSymbol { name: "globe.europe.africa" };
pub const GLOBE_EUROPE_AFRICA_FILL: SFSymbol = SFSymbol { name: "globe.europe.africa.fill" };
pub const GOBACKWARD: SFSymbol = SFSymbol { name: "gobackward" };
pub const GOBACKWARD_10: SFSymbol = SFSymbol { name: "gobackward.10" };
pub const GOBACKWARD_15: SFSymbol = SFSymbol { name: "gobackward.15" };
pub const GOBACKWARD_30: SFSymbol = SFSymbol { name: "gobackward.30" };
pub const GOBACKWARD_45: SFSymbol = SFSymbol { name: "gobackward.45" };
pub const GOBACKWARD_5: SFSymbol = SFSymbol { name: "gobackward.5" };
pub const GOBACKWARD_60: SFSymbol = SFSymbol { name: "gobackward.60" };
pub const GOBACKWARD_75: SFSymbol = SFSymbol { name: "gobackward.75" };
pub const GOBACKWARD_90: SFSymbol = SFSymbol { name: "gobackward.90" };
pub const GOBACKWARD_MINUS: SFSymbol = SFSymbol { name: "gobackward.minus" };
pub const GOFORWARD: SFSymbol = SFSymbol { name: "goforward" };
pub const GOFORWARD_10: SFSymbol = SFSymbol { name: "goforward.10" };
pub const GOFORWARD_15: SFSymbol = SFSymbol { name: "goforward.15" };
pub const GOFORWARD_30: SFSymbol = SFSymbol { name: "goforward.30" };
pub const GOFORWARD_45: SFSymbol = SFSymbol { name: "goforward.45" };
pub const GOFORWARD_5: SFSymbol = SFSymbol { name: "goforward.5" };
pub const GOFORWARD_60: SFSymbol = SFSymbol { name: "goforward.60" };
pub const GOFORWARD_75: SFSymbol = SFSymbol { name: "goforward.75" };
pub const GOFORWARD_90: SFSymbol = SFSymbol { name: "goforward.90" };
pub const GOFORWARD_PLUS: SFSymbol = SFSymbol { name: "goforward.plus" };
pub const GRADUATIONCAP: SFSymbol = SFSymbol { name: "graduationcap" };
pub const GRADUATIONCAP_CIRCLE: SFSymbol = SFSymbol { name: "graduationcap.circle" };
pub const GRADUATIONCAP_CIRCLE_FILL: SFSymbol = SFSymbol { name: "graduationcap.circle.fill" };
pub const GRADUATIONCAP_FILL: SFSymbol = SFSymbol { name: "graduationcap.fill" };
pub const GREATERTHAN: SFSymbol = SFSymbol { name: "greaterthan" };
pub const GREATERTHAN_CIRCLE: SFSymbol = SFSymbol { name: "greaterthan.circle" };
pub const GREATERTHAN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "greaterthan.circle.fill" };
pub const GREATERTHAN_SQUARE: SFSymbol = SFSymbol { name: "greaterthan.square" };
pub const GREATERTHAN_SQUARE_FILL: SFSymbol = SFSymbol { name: "greaterthan.square.fill" };
pub const GREETINGCARD: SFSymbol = SFSymbol { name: "greetingcard" };
pub const GREETINGCARD_FILL: SFSymbol = SFSymbol { name: "greetingcard.fill" };
pub const GRID: SFSymbol = SFSymbol { name: "grid" };
pub const GRID_CIRCLE: SFSymbol = SFSymbol { name: "grid.circle" };
pub const GRID_CIRCLE_FILL: SFSymbol = SFSymbol { name: "grid.circle.fill" };
pub const GUARANISIGN: SFSymbol = SFSymbol { name: "guaranisign" };
pub const GUARANISIGN_CIRCLE: SFSymbol = SFSymbol { name: "guaranisign.circle" };
pub const GUARANISIGN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "guaranisign.circle.fill" };
pub const GUARANISIGN_SQUARE: SFSymbol = SFSymbol { name: "guaranisign.square" };
pub const GUARANISIGN_SQUARE_FILL: SFSymbol = SFSymbol { name: "guaranisign.square.fill" };
pub const GUITARS: SFSymbol = SFSymbol { name: "guitars" };
pub const GUITARS_FILL: SFSymbol = SFSymbol { name: "guitars.fill" };
pub const GYROSCOPE: SFSymbol = SFSymbol { name: "gyroscope" };
pub const H_CIRCLE: SFSymbol = SFSymbol { name: "h.circle" };
pub const H_CIRCLE_FILL: SFSymbol = SFSymbol { name: "h.circle.fill" };
pub const H_SQUARE: SFSymbol = SFSymbol { name: "h.square" };
pub const H_SQUARE_FILL: SFSymbol = SFSymbol { name: "h.square.fill" };
pub const H_SQUARE_ON_SQUARE: SFSymbol = SFSymbol { name: "h.square.on.square" };
pub const H_SQUARE_ON_SQUARE_FILL: SFSymbol = SFSymbol { name: "h.square.on.square.fill" };
pub const HAMMER: SFSymbol = SFSymbol { name: "hammer" };
pub const HAMMER_CIRCLE: SFSymbol = SFSymbol { name: "hammer.circle" };
pub const HAMMER_CIRCLE_FILL: SFSymbol = SFSymbol { name: "hammer.circle.fill" };
pub const HAMMER_FILL: SFSymbol = SFSymbol { name: "hammer.fill" };
pub const HAND_DRAW: SFSymbol = SFSymbol { name: "hand.draw" };
pub const HAND_DRAW_FILL: SFSymbol = SFSymbol { name: "hand.draw.fill" };
pub const HAND_POINT_DOWN: SFSymbol = SFSymbol { name: "hand.point.down" };
pub const HAND_POINT_DOWN_FILL: SFSymbol = SFSymbol { name: "hand.point.down.fill" };
pub const HAND_POINT_LEFT: SFSymbol = SFSymbol { name: "hand.point.left" };
pub const HAND_POINT_LEFT_FILL: SFSymbol = SFSymbol { name: "hand.point.left.fill" };
pub const HAND_POINT_RIGHT: SFSymbol = SFSymbol { name: "hand.point.right" };
pub const HAND_POINT_RIGHT_FILL: SFSymbol = SFSymbol { name: "hand.point.right.fill" };
pub const HAND_POINT_UP: SFSymbol = SFSymbol { name: "hand.point.up" };
pub const HAND_POINT_UP_BRAILLE: SFSymbol = SFSymbol { name: "hand.point.up.braille" };
pub const HAND_POINT_UP_BRAILLE_FILL: SFSymbol = SFSymbol { name: "hand.point.up.braille.fill" };
pub const HAND_POINT_UP_FILL: SFSymbol = SFSymbol { name: "hand.point.up.fill" };
pub const HAND_POINT_UP_LEFT: SFSymbol = SFSymbol { name: "hand.point.up.left" };
pub const HAND_POINT_UP_LEFT_FILL: SFSymbol = SFSymbol { name: "hand.point.up.left.fill" };
pub const HAND_RAISED: SFSymbol = SFSymbol { name: "hand.raised" };
pub const HAND_RAISED_CIRCLE: SFSymbol = SFSymbol { name: "hand.raised.circle" };
pub const HAND_RAISED_CIRCLE_FILL: SFSymbol = SFSymbol { name: "hand.raised.circle.fill" };
pub const HAND_RAISED_FILL: SFSymbol = SFSymbol { name: "hand.raised.fill" };
pub const HAND_RAISED_FINGERS_SPREAD: SFSymbol = SFSymbol { name: "hand.raised.fingers.spread" };
pub const HAND_RAISED_FINGERS_SPREAD_FILL: SFSymbol = SFSymbol { name: "hand.raised.fingers.spread.fill" };
pub const HAND_RAISED_SLASH: SFSymbol = SFSymbol { name: "hand.raised.slash" };
pub const HAND_RAISED_SLASH_FILL: SFSymbol = SFSymbol { name: "hand.raised.slash.fill" };
pub const HAND_RAISED_SQUARE: SFSymbol = SFSymbol { name: "hand.raised.square" };
pub const HAND_RAISED_SQUARE_FILL: SFSymbol = SFSymbol { name: "hand.raised.square.fill" };
pub const HAND_RAISED_SQUARE_ON_SQUARE: SFSymbol = SFSymbol { name: "hand.raised.square.on.square" };
pub const HAND_RAISED_SQUARE_ON_SQUARE_FILL: SFSymbol = SFSymbol { name: "hand.raised.square.on.square.fill" };
pub const HAND_TAP: SFSymbol = SFSymbol { name: "hand.tap" };
pub const HAND_TAP_FILL: SFSymbol = SFSymbol { name: "hand.tap.fill" };
pub const HAND_THUMBSDOWN: SFSymbol = SFSymbol { name: "hand.thumbsdown" };
pub const HAND_THUMBSDOWN_CIRCLE: SFSymbol = SFSymbol { name: "hand.thumbsdown.circle" };
pub const HAND_THUMBSDOWN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "hand.thumbsdown.circle.fill" };
pub const HAND_THUMBSDOWN_FILL: SFSymbol = SFSymbol { name: "hand.thumbsdown.fill" };
pub const HAND_THUMBSUP: SFSymbol = SFSymbol { name: "hand.thumbsup" };
pub const HAND_THUMBSUP_CIRCLE: SFSymbol = SFSymbol { name: "hand.thumbsup.circle" };
pub const HAND_THUMBSUP_CIRCLE_FILL: SFSymbol = SFSymbol { name: "hand.thumbsup.circle.fill" };
pub const HAND_THUMBSUP_FILL: SFSymbol = SFSymbol { name: "hand.thumbsup.fill" };
pub const HAND_WAVE: SFSymbol = SFSymbol { name: "hand.wave" };
pub const HAND_WAVE_FILL: SFSymbol = SFSymbol { name: "hand.wave.fill" };
pub const HANDS_CLAP: SFSymbol = SFSymbol { name: "hands.clap" };
pub const HANDS_CLAP_FILL: SFSymbol = SFSymbol { name: "hands.clap.fill" };
pub const HANDS_SPARKLES: SFSymbol = SFSymbol { name: "hands.sparkles" };
pub const HANDS_SPARKLES_FILL: SFSymbol = SFSymbol { name: "hands.sparkles.fill" };
pub const HARE: SFSymbol = SFSymbol { name: "hare" };
pub const HARE_FILL: SFSymbol = SFSymbol { name: "hare.fill" };
pub const HEADPHONES: SFSymbol = SFSymbol { name: "headphones" };
pub const HEADPHONES_CIRCLE: SFSymbol = SFSymbol { name: "headphones.circle" };
pub const HEADPHONES_CIRCLE_FILL: SFSymbol = SFSymbol { name: "headphones.circle.fill" };
pub const HEARINGDEVICE_AND_SIGNAL_METER: SFSymbol = SFSymbol { name: "hearingdevice.and.signal.meter" };
pub const HEARINGDEVICE_AND_SIGNAL_METER_FILL: SFSymbol = SFSymbol { name: "hearingdevice.and.signal.meter.fill" };
pub const HEARINGDEVICE_EAR: SFSymbol = SFSymbol { name: "hearingdevice.ear" };
pub const HEARINGDEVICE_EAR_FILL: SFSymbol = SFSymbol { name: "hearingdevice.ear.fill" };
pub const HEART: SFSymbol = SFSymbol { name: "heart" };
pub const HEART_CIRCLE: SFSymbol = SFSymbol { name: "heart.circle" };
pub const HEART_CIRCLE_FILL: SFSymbol = SFSymbol { name: "heart.circle.fill" };
pub const HEART_FILL: SFSymbol = SFSymbol { name: "heart.fill" };
pub const HEART_RECTANGLE: SFSymbol = SFSymbol { name: "heart.rectangle" };
pub const HEART_RECTANGLE_FILL: SFSymbol = SFSymbol { name: "heart.rectangle.fill" };
pub const HEART_SLASH: SFSymbol = SFSymbol { name: "heart.slash" };
pub const HEART_SLASH_CIRCLE: SFSymbol = SFSymbol { name: "heart.slash.circle" };
pub const HEART_SLASH_CIRCLE_FILL: SFSymbol = SFSymbol { name: "heart.slash.circle.fill" };
pub const HEART_SLASH_FILL: SFSymbol = SFSymbol { name: "heart.slash.fill" };
pub const HEART_SQUARE: SFSymbol = SFSymbol { name: "heart.square" };
pub const HEART_SQUARE_FILL: SFSymbol = SFSymbol { name: "heart.square.fill" };
pub const HEART_TEXT_SQUARE: SFSymbol = SFSymbol { name: "heart.text.square" };
pub const HEART_TEXT_SQUARE_FILL: SFSymbol = SFSymbol { name: "heart.text.square.fill" };
pub const HEATER_VERTICAL: SFSymbol = SFSymbol { name: "heater.vertical" };
pub const HEATER_VERTICAL_FILL: SFSymbol = SFSymbol { name: "heater.vertical.fill" };
pub const HELM: SFSymbol = SFSymbol { name: "helm" };
pub const HEXAGON: SFSymbol = SFSymbol { name: "hexagon" };
pub const HEXAGON_BOTTOMHALF_FILLED: SFSymbol = SFSymbol { name: "hexagon.bottomhalf.filled" };
pub const HEXAGON_FILL: SFSymbol = SFSymbol { name: "hexagon.fill" };
pub const HEXAGON_LEFTHALF_FILLED: SFSymbol = SFSymbol { name: "hexagon.lefthalf.filled" };
pub const HEXAGON_RIGHTHALF_FILLED: SFSymbol = SFSymbol { name: "hexagon.righthalf.filled" };
pub const HEXAGON_TOPHALF_FILLED: SFSymbol = SFSymbol { name: "hexagon.tophalf.filled" };
pub const HIFIRECEIVER: SFSymbol = SFSymbol { name: "hifireceiver" };
pub const HIFIRECEIVER_FILL: SFSymbol = SFSymbol { name: "hifireceiver.fill" };
pub const HIFISPEAKER: SFSymbol = SFSymbol { name: "hifispeaker" };
pub const HIFISPEAKER_2: SFSymbol = SFSymbol { name: "hifispeaker.2" };
pub const HIFISPEAKER_2_FILL: SFSymbol = SFSymbol { name: "hifispeaker.2.fill" };
pub const HIFISPEAKER_AND_APPLETV: SFSymbol = SFSymbol { name: "hifispeaker.and.appletv" };
pub const HIFISPEAKER_AND_APPLETV_FILL: SFSymbol = SFSymbol { name: "hifispeaker.and.appletv.fill" };
pub const HIFISPEAKER_AND_HOMEPOD: SFSymbol = SFSymbol { name: "hifispeaker.and.homepod" };
pub const HIFISPEAKER_AND_HOMEPOD_FILL: SFSymbol = SFSymbol { name: "hifispeaker.and.homepod.fill" };
pub const HIFISPEAKER_AND_HOMEPODMINI: SFSymbol = SFSymbol { name: "hifispeaker.and.homepodmini" };
pub const HIFISPEAKER_AND_HOMEPODMINI_FILL: SFSymbol = SFSymbol { name: "hifispeaker.and.homepodmini.fill" };
pub const HIFISPEAKER_FILL: SFSymbol = SFSymbol { name: "hifispeaker.fill" };
pub const HIGHLIGHTER: SFSymbol = SFSymbol { name: "highlighter" };
pub const HOCKEY_PUCK: SFSymbol = SFSymbol { name: "hockey.puck" };
pub const HOCKEY_PUCK_CIRCLE: SFSymbol = SFSymbol { name: "hockey.puck.circle" };
pub const HOCKEY_PUCK_CIRCLE_FILL: SFSymbol = SFSymbol { name: "hockey.puck.circle.fill" };
pub const HOCKEY_PUCK_FILL: SFSymbol = SFSymbol { name: "hockey.puck.fill" };
pub const HOMEKIT: SFSymbol = SFSymbol { name: "homekit" };
pub const HOMEPOD: SFSymbol = SFSymbol { name: "homepod" };
pub const HOMEPOD_2: SFSymbol = SFSymbol { name: "homepod.2" };
pub const HOMEPOD_2_FILL: SFSymbol = SFSymbol { name: "homepod.2.fill" };
pub const HOMEPOD_AND_APPLETV: SFSymbol = SFSymbol { name: "homepod.and.appletv" };
pub const HOMEPOD_AND_APPLETV_FILL: SFSymbol = SFSymbol { name: "homepod.and.appletv.fill" };
pub const HOMEPOD_AND_HOMEPODMINI: SFSymbol = SFSymbol { name: "homepod.and.homepodmini" };
pub const HOMEPOD_AND_HOMEPODMINI_FILL: SFSymbol = SFSymbol { name: "homepod.and.homepodmini.fill" };
pub const HOMEPOD_FILL: SFSymbol = SFSymbol { name: "homepod.fill" };
pub const HOMEPODMINI: SFSymbol = SFSymbol { name: "homepodmini" };
pub const HOMEPODMINI_2: SFSymbol = SFSymbol { name: "homepodmini.2" };
pub const HOMEPODMINI_2_FILL: SFSymbol = SFSymbol { name: "homepodmini.2.fill" };
pub const HOMEPODMINI_AND_APPLETV: SFSymbol = SFSymbol { name: "homepodmini.and.appletv" };
pub const HOMEPODMINI_AND_APPLETV_FILL: SFSymbol = SFSymbol { name: "homepodmini.and.appletv.fill" };
pub const HOMEPODMINI_FILL: SFSymbol = SFSymbol { name: "homepodmini.fill" };
pub const HOURGLASS: SFSymbol = SFSymbol { name: "hourglass" };
pub const HOURGLASS_BADGE_PLUS: SFSymbol = SFSymbol { name: "hourglass.badge.plus" };
pub const HOURGLASS_BOTTOMHALF_FILLED: SFSymbol = SFSymbol { name: "hourglass.bottomhalf.filled" };
pub const HOURGLASS_CIRCLE: SFSymbol = SFSymbol { name: "hourglass.circle" };
pub const HOURGLASS_CIRCLE_FILL: SFSymbol = SFSymbol { name: "hourglass.circle.fill" };
pub const HOURGLASS_TOPHALF_FILLED: SFSymbol = SFSymbol { name: "hourglass.tophalf.filled" };
pub const HOUSE: SFSymbol = SFSymbol { name: "house" };
pub const HOUSE_CIRCLE: SFSymbol = SFSymbol { name: "house.circle" };
pub const HOUSE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "house.circle.fill" };
pub const HOUSE_FILL: SFSymbol = SFSymbol { name: "house.fill" };
pub const HRYVNIASIGN: SFSymbol = SFSymbol { name: "hryvniasign" };
pub const HRYVNIASIGN_CIRCLE: SFSymbol = SFSymbol { name: "hryvniasign.circle" };
pub const HRYVNIASIGN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "hryvniasign.circle.fill" };
pub const HRYVNIASIGN_SQUARE: SFSymbol = SFSymbol { name: "hryvniasign.square" };
pub const HRYVNIASIGN_SQUARE_FILL: SFSymbol = SFSymbol { name: "hryvniasign.square.fill" };
pub const HUMIDIFIER: SFSymbol = SFSymbol { name: "humidifier" };
pub const HUMIDIFIER_AND_DROPLETS: SFSymbol = SFSymbol { name: "humidifier.and.droplets" };
pub const HUMIDIFIER_AND_DROPLETS_FILL: SFSymbol = SFSymbol { name: "humidifier.and.droplets.fill" };
pub const HUMIDIFIER_FILL: SFSymbol = SFSymbol { name: "humidifier.fill" };
pub const HUMIDITY: SFSymbol = SFSymbol { name: "humidity" };
pub const HUMIDITY_FILL: SFSymbol = SFSymbol { name: "humidity.fill" };
pub const HURRICANE: SFSymbol = SFSymbol { name: "hurricane" };
pub const HURRICANE_CIRCLE: SFSymbol = SFSymbol { name: "hurricane.circle" };
pub const HURRICANE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "hurricane.circle.fill" };
pub const I_CIRCLE: SFSymbol = SFSymbol { name: "i.circle" };
pub const I_CIRCLE_FILL: SFSymbol = SFSymbol { name: "i.circle.fill" };
pub const I_SQUARE: SFSymbol = SFSymbol { name: "i.square" };
pub const I_SQUARE_FILL: SFSymbol = SFSymbol { name: "i.square.fill" };
pub const ICLOUD: SFSymbol = SFSymbol { name: "icloud" };
pub const ICLOUD_AND_ARROW_DOWN: SFSymbol = SFSymbol { name: "icloud.and.arrow.down" };
pub const ICLOUD_AND_ARROW_DOWN_FILL: SFSymbol = SFSymbol { name: "icloud.and.arrow.down.fill" };
pub const ICLOUD_AND_ARROW_UP: SFSymbol = SFSymbol { name: "icloud.and.arrow.up" };
pub const ICLOUD_AND_ARROW_UP_FILL: SFSymbol = SFSymbol { name: "icloud.and.arrow.up.fill" };
pub const ICLOUD_CIRCLE: SFSymbol = SFSymbol { name: "icloud.circle" };
pub const ICLOUD_CIRCLE_FILL: SFSymbol = SFSymbol { name: "icloud.circle.fill" };
pub const ICLOUD_FILL: SFSymbol = SFSymbol { name: "icloud.fill" };
pub const ICLOUD_SLASH: SFSymbol = SFSymbol { name: "icloud.slash" };
pub const ICLOUD_SLASH_FILL: SFSymbol = SFSymbol { name: "icloud.slash.fill" };
pub const ICLOUD_SQUARE: SFSymbol = SFSymbol { name: "icloud.square" };
pub const ICLOUD_SQUARE_FILL: SFSymbol = SFSymbol { name: "icloud.square.fill" };
pub const INCREASE_INDENT: SFSymbol = SFSymbol { name: "increase.indent" };
pub const INCREASE_QUOTELEVEL: SFSymbol = SFSymbol { name: "increase.quotelevel" };
pub const INDIANRUPEESIGN: SFSymbol = SFSymbol { name: "indianrupeesign" };
pub const INDIANRUPEESIGN_CIRCLE: SFSymbol = SFSymbol { name: "indianrupeesign.circle" };
pub const INDIANRUPEESIGN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "indianrupeesign.circle.fill" };
pub const INDIANRUPEESIGN_SQUARE: SFSymbol = SFSymbol { name: "indianrupeesign.square" };
pub const INDIANRUPEESIGN_SQUARE_FILL: SFSymbol = SFSymbol { name: "indianrupeesign.square.fill" };
pub const INFINITY: SFSymbol = SFSymbol { name: "infinity" };
pub const INFINITY_CIRCLE: SFSymbol = SFSymbol { name: "infinity.circle" };
pub const INFINITY_CIRCLE_FILL: SFSymbol = SFSymbol { name: "infinity.circle.fill" };
pub const INFO: SFSymbol = SFSymbol { name: "info" };
pub const INFO_BUBBLE: SFSymbol = SFSymbol { name: "info.bubble" };
pub const INFO_BUBBLE_FILL: SFSymbol = SFSymbol { name: "info.bubble.fill" };
pub const INFO_CIRCLE: SFSymbol = SFSymbol { name: "info.circle" };
pub const INFO_CIRCLE_FILL: SFSymbol = SFSymbol { name: "info.circle.fill" };
pub const INFO_SQUARE: SFSymbol = SFSymbol { name: "info.square" };
pub const INFO_SQUARE_FILL: SFSymbol = SFSymbol { name: "info.square.fill" };
pub const INTERNALDRIVE: SFSymbol = SFSymbol { name: "internaldrive" };
pub const INTERNALDRIVE_FILL: SFSymbol = SFSymbol { name: "internaldrive.fill" };
pub const IPAD: SFSymbol = SFSymbol { name: "ipad" };
pub const IPAD_AND_ARROW_FORWARD: SFSymbol = SFSymbol { name: "ipad.and.arrow.forward" };
pub const IPAD_AND_IPHONE: SFSymbol = SFSymbol { name: "ipad.and.iphone" };
pub const IPAD_BADGE_PLAY: SFSymbol = SFSymbol { name: "ipad.badge.play" };
pub const IPAD_HOMEBUTTON: SFSymbol = SFSymbol { name: "ipad.homebutton" };
pub const IPAD_HOMEBUTTON_BADGE_PLAY: SFSymbol = SFSymbol { name: "ipad.homebutton.badge.play" };
pub const IPAD_HOMEBUTTON_LANDSCAPE: SFSymbol = SFSymbol { name: "ipad.homebutton.landscape" };
pub const IPAD_HOMEBUTTON_LANDSCAPE_BADGE_PLAY: SFSymbol = SFSymbol { name: "ipad.homebutton.landscape.badge.play" };
pub const IPAD_LANDSCAPE: SFSymbol = SFSymbol { name: "ipad.landscape" };
pub const IPAD_LANDSCAPE_BADGE_PLAY: SFSymbol = SFSymbol { name: "ipad.landscape.badge.play" };
pub const IPAD_REAR_CAMERA: SFSymbol = SFSymbol { name: "ipad.rear.camera" };
pub const IPHONE: SFSymbol = SFSymbol { name: "iphone" };
pub const IPHONE_AND_ARROW_FORWARD: SFSymbol = SFSymbol { name: "iphone.and.arrow.forward" };
pub const IPHONE_BADGE_PLAY: SFSymbol = SFSymbol { name: "iphone.badge.play" };
pub const IPHONE_CIRCLE: SFSymbol = SFSymbol { name: "iphone.circle" };
pub const IPHONE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "iphone.circle.fill" };
pub const IPHONE_HOMEBUTTON: SFSymbol = SFSymbol { name: "iphone.homebutton" };
pub const IPHONE_HOMEBUTTON_BADGE_PLAY: SFSymbol = SFSymbol { name: "iphone.homebutton.badge.play" };
pub const IPHONE_HOMEBUTTON_CIRCLE: SFSymbol = SFSymbol { name: "iphone.homebutton.circle" };
pub const IPHONE_HOMEBUTTON_CIRCLE_FILL: SFSymbol = SFSymbol { name: "iphone.homebutton.circle.fill" };
pub const IPHONE_HOMEBUTTON_LANDSCAPE: SFSymbol = SFSymbol { name: "iphone.homebutton.landscape" };
pub const IPHONE_HOMEBUTTON_RADIOWAVES_LEFT_AND_RIGHT: SFSymbol = SFSymbol { name: "iphone.homebutton.radiowaves.left.and.right" };
pub const IPHONE_HOMEBUTTON_RADIOWAVES_LEFT_AND_RIGHT_CIRCLE: SFSymbol = SFSymbol { name: "iphone.homebutton.radiowaves.left.and.right.circle" };
pub const IPHONE_HOMEBUTTON_RADIOWAVES_LEFT_AND_RIGHT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "iphone.homebutton.radiowaves.left.and.right.circle.fill" };
pub const IPHONE_HOMEBUTTON_SLASH: SFSymbol = SFSymbol { name: "iphone.homebutton.slash" };
pub const IPHONE_HOMEBUTTON_SLASH_CIRCLE: SFSymbol = SFSymbol { name: "iphone.homebutton.slash.circle" };
pub const IPHONE_HOMEBUTTON_SLASH_CIRCLE_FILL: SFSymbol = SFSymbol { name: "iphone.homebutton.slash.circle.fill" };
pub const IPHONE_LANDSCAPE: SFSymbol = SFSymbol { name: "iphone.landscape" };
pub const IPHONE_RADIOWAVES_LEFT_AND_RIGHT: SFSymbol = SFSymbol { name: "iphone.radiowaves.left.and.right" };
pub const IPHONE_RADIOWAVES_LEFT_AND_RIGHT_CIRCLE: SFSymbol = SFSymbol { name: "iphone.radiowaves.left.and.right.circle" };
pub const IPHONE_RADIOWAVES_LEFT_AND_RIGHT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "iphone.radiowaves.left.and.right.circle.fill" };
pub const IPHONE_REAR_CAMERA: SFSymbol = SFSymbol { name: "iphone.rear.camera" };
pub const IPHONE_SLASH: SFSymbol = SFSymbol { name: "iphone.slash" };
pub const IPHONE_SLASH_CIRCLE: SFSymbol = SFSymbol { name: "iphone.slash.circle" };
pub const IPHONE_SLASH_CIRCLE_FILL: SFSymbol = SFSymbol { name: "iphone.slash.circle.fill" };
pub const IPHONE_SMARTBATTERYCASE_GEN1: SFSymbol = SFSymbol { name: "iphone.smartbatterycase.gen1" };
pub const IPHONE_SMARTBATTERYCASE_GEN2: SFSymbol = SFSymbol { name: "iphone.smartbatterycase.gen2" };
pub const IPOD: SFSymbol = SFSymbol { name: "ipod" };
pub const IPODSHUFFLE_GEN1: SFSymbol = SFSymbol { name: "ipodshuffle.gen1" };
pub const IPODSHUFFLE_GEN2: SFSymbol = SFSymbol { name: "ipodshuffle.gen2" };
pub const IPODSHUFFLE_GEN3: SFSymbol = SFSymbol { name: "ipodshuffle.gen3" };
pub const IPODSHUFFLE_GEN4: SFSymbol = SFSymbol { name: "ipodshuffle.gen4" };
pub const IPODTOUCH: SFSymbol = SFSymbol { name: "ipodtouch" };
pub const IPODTOUCH_LANDSCAPE: SFSymbol = SFSymbol { name: "ipodtouch.landscape" };
pub const IPODTOUCH_SLASH: SFSymbol = SFSymbol { name: "ipodtouch.slash" };
pub const ITALIC: SFSymbol = SFSymbol { name: "italic" };
pub const IVFLUID_BAG: SFSymbol = SFSymbol { name: "ivfluid.bag" };
pub const IVFLUID_BAG_FILL: SFSymbol = SFSymbol { name: "ivfluid.bag.fill" };
pub const J_CIRCLE: SFSymbol = SFSymbol { name: "j.circle" };
pub const J_CIRCLE_FILL: SFSymbol = SFSymbol { name: "j.circle.fill" };
pub const J_SQUARE: SFSymbol = SFSymbol { name: "j.square" };
pub const J_SQUARE_FILL: SFSymbol = SFSymbol { name: "j.square.fill" };
pub const J_SQUARE_ON_SQUARE: SFSymbol = SFSymbol { name: "j.square.on.square" };
pub const J_SQUARE_ON_SQUARE_FILL: SFSymbol = SFSymbol { name: "j.square.on.square.fill" };
pub const K: SFSymbol = SFSymbol { name: "k" };
pub const K_CIRCLE: SFSymbol = SFSymbol { name: "k.circle" };
pub const K_CIRCLE_FILL: SFSymbol = SFSymbol { name: "k.circle.fill" };
pub const K_SQUARE: SFSymbol = SFSymbol { name: "k.square" };
pub const K_SQUARE_FILL: SFSymbol = SFSymbol { name: "k.square.fill" };
pub const KEY: SFSymbol = SFSymbol { name: "key" };
pub const KEY_FILL: SFSymbol = SFSymbol { name: "key.fill" };
pub const KEY_ICLOUD: SFSymbol = SFSymbol { name: "key.icloud" };
pub const KEY_ICLOUD_FILL: SFSymbol = SFSymbol { name: "key.icloud.fill" };
pub const KEY_VIEWFINDER: SFSymbol = SFSymbol { name: "key.viewfinder" };
pub const KEYBOARD: SFSymbol = SFSymbol { name: "keyboard" };
pub const KEYBOARD_BADGE_ELLIPSIS: SFSymbol = SFSymbol { name: "keyboard.badge.ellipsis" };
pub const KEYBOARD_BADGE_ELLIPSIS_FILL: SFSymbol = SFSymbol { name: "keyboard.badge.ellipsis.fill" };
pub const KEYBOARD_BADGE_EYE: SFSymbol = SFSymbol { name: "keyboard.badge.eye" };
pub const KEYBOARD_BADGE_EYE_FILL: SFSymbol = SFSymbol { name: "keyboard.badge.eye.fill" };
pub const KEYBOARD_CHEVRON_COMPACT_DOWN: SFSymbol = SFSymbol { name: "keyboard.chevron.compact.down" };
pub const KEYBOARD_CHEVRON_COMPACT_DOWN_FILL: SFSymbol = SFSymbol { name: "keyboard.chevron.compact.down.fill" };
pub const KEYBOARD_CHEVRON_COMPACT_LEFT: SFSymbol = SFSymbol { name: "keyboard.chevron.compact.left" };
pub const KEYBOARD_CHEVRON_COMPACT_LEFT_FILL: SFSymbol = SFSymbol { name: "keyboard.chevron.compact.left.fill" };
pub const KEYBOARD_FILL: SFSymbol = SFSymbol { name: "keyboard.fill" };
pub const KEYBOARD_MACWINDOW: SFSymbol = SFSymbol { name: "keyboard.macwindow" };
pub const KEYBOARD_ONEHANDED_LEFT: SFSymbol = SFSymbol { name: "keyboard.onehanded.left" };
pub const KEYBOARD_ONEHANDED_LEFT_FILL: SFSymbol = SFSymbol { name: "keyboard.onehanded.left.fill" };
pub const KEYBOARD_ONEHANDED_RIGHT: SFSymbol = SFSymbol { name: "keyboard.onehanded.right" };
pub const KEYBOARD_ONEHANDED_RIGHT_FILL: SFSymbol = SFSymbol { name: "keyboard.onehanded.right.fill" };
pub const KIPSIGN: SFSymbol = SFSymbol { name: "kipsign" };
pub const KIPSIGN_CIRCLE: SFSymbol = SFSymbol { name: "kipsign.circle" };
pub const KIPSIGN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "kipsign.circle.fill" };
pub const KIPSIGN_SQUARE: SFSymbol = SFSymbol { name: "kipsign.square" };
pub const KIPSIGN_SQUARE_FILL: SFSymbol = SFSymbol { name: "kipsign.square.fill" };
pub const L_CIRCLE: SFSymbol = SFSymbol { name: "l.circle" };
pub const L_CIRCLE_FILL: SFSymbol = SFSymbol { name: "l.circle.fill" };
pub const L_JOYSTICK: SFSymbol = SFSymbol { name: "l.joystick" };
pub const L_JOYSTICK_FILL: SFSymbol = SFSymbol { name: "l.joystick.fill" };
pub const L_JOYSTICK_PRESS_DOWN: SFSymbol = SFSymbol { name: "l.joystick.press.down" };
pub const L_JOYSTICK_PRESS_DOWN_FILL: SFSymbol = SFSymbol { name: "l.joystick.press.down.fill" };
pub const L_JOYSTICK_TILT_DOWN: SFSymbol = SFSymbol { name: "l.joystick.tilt.down" };
pub const L_JOYSTICK_TILT_DOWN_FILL: SFSymbol = SFSymbol { name: "l.joystick.tilt.down.fill" };
pub const L_JOYSTICK_TILT_LEFT: SFSymbol = SFSymbol { name: "l.joystick.tilt.left" };
pub const L_JOYSTICK_TILT_LEFT_FILL: SFSymbol = SFSymbol { name: "l.joystick.tilt.left.fill" };
pub const L_JOYSTICK_TILT_RIGHT: SFSymbol = SFSymbol { name: "l.joystick.tilt.right" };
pub const L_JOYSTICK_TILT_RIGHT_FILL: SFSymbol = SFSymbol { name: "l.joystick.tilt.right.fill" };
pub const L_JOYSTICK_TILT_UP: SFSymbol = SFSymbol { name: "l.joystick.tilt.up" };
pub const L_JOYSTICK_TILT_UP_FILL: SFSymbol = SFSymbol { name: "l.joystick.tilt.up.fill" };
pub const L_RECTANGLE_ROUNDEDBOTTOM: SFSymbol = SFSymbol { name: "l.rectangle.roundedbottom" };
pub const L_RECTANGLE_ROUNDEDBOTTOM_FILL: SFSymbol = SFSymbol { name: "l.rectangle.roundedbottom.fill" };
pub const L_SQUARE: SFSymbol = SFSymbol { name: "l.square" };
pub const L_SQUARE_FILL: SFSymbol = SFSymbol { name: "l.square.fill" };
pub const L1_RECTANGLE_ROUNDEDBOTTOM: SFSymbol = SFSymbol { name: "l1.rectangle.roundedbottom" };
pub const L1_RECTANGLE_ROUNDEDBOTTOM_FILL: SFSymbol = SFSymbol { name: "l1.rectangle.roundedbottom.fill" };
pub const L2_RECTANGLE_ROUNDEDTOP: SFSymbol = SFSymbol { name: "l2.rectangle.roundedtop" };
pub const L2_RECTANGLE_ROUNDEDTOP_FILL: SFSymbol = SFSymbol { name: "l2.rectangle.roundedtop.fill" };
pub const LADYBUG: SFSymbol = SFSymbol { name: "ladybug" };
pub const LADYBUG_FILL: SFSymbol = SFSymbol { name: "ladybug.fill" };
pub const LAMP_CEILING: SFSymbol = SFSymbol { name: "lamp.ceiling" };
pub const LAMP_CEILING_FILL: SFSymbol = SFSymbol { name: "lamp.ceiling.fill" };
pub const LAMP_CEILING_INVERSE: SFSymbol = SFSymbol { name: "lamp.ceiling.inverse" };
pub const LAMP_DESK: SFSymbol = SFSymbol { name: "lamp.desk" };
pub const LAMP_DESK_FILL: SFSymbol = SFSymbol { name: "lamp.desk.fill" };
pub const LAMP_FLOOR: SFSymbol = SFSymbol { name: "lamp.floor" };
pub const LAMP_FLOOR_FILL: SFSymbol = SFSymbol { name: "lamp.floor.fill" };
pub const LAMP_TABLE: SFSymbol = SFSymbol { name: "lamp.table" };
pub const LAMP_TABLE_FILL: SFSymbol = SFSymbol { name: "lamp.table.fill" };
pub const LANYARDCARD: SFSymbol = SFSymbol { name: "lanyardcard" };
pub const LANYARDCARD_FILL: SFSymbol = SFSymbol { name: "lanyardcard.fill" };
pub const LAPTOPCOMPUTER: SFSymbol = SFSymbol { name: "laptopcomputer" };
pub const LAPTOPCOMPUTER_AND_ARROW_DOWN: SFSymbol = SFSymbol { name: "laptopcomputer.and.arrow.down" };
pub const LAPTOPCOMPUTER_AND_IPAD: SFSymbol = SFSymbol { name: "laptopcomputer.and.ipad" };
pub const LAPTOPCOMPUTER_AND_IPHONE: SFSymbol = SFSymbol { name: "laptopcomputer.and.iphone" };
pub const LAPTOPCOMPUTER_TRIANGLEBADGE_EXCLAMATIONMARK: SFSymbol = SFSymbol { name: "laptopcomputer.trianglebadge.exclamationmark" };
pub const LARISIGN: SFSymbol = SFSymbol { name: "larisign" };
pub const LARISIGN_CIRCLE: SFSymbol = SFSymbol { name: "larisign.circle" };
pub const LARISIGN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "larisign.circle.fill" };
pub const LARISIGN_SQUARE: SFSymbol = SFSymbol { name: "larisign.square" };
pub const LARISIGN_SQUARE_FILL: SFSymbol = SFSymbol { name: "larisign.square.fill" };
pub const LASSO: SFSymbol = SFSymbol { name: "lasso" };
pub const LASSO_AND_SPARKLES: SFSymbol = SFSymbol { name: "lasso.and.sparkles" };
pub const LATCH_2_CASE: SFSymbol = SFSymbol { name: "latch.2.case" };
pub const LATCH_2_CASE_FILL: SFSymbol = SFSymbol { name: "latch.2.case.fill" };
pub const LAUREL_LEADING: SFSymbol = SFSymbol { name: "laurel.leading" };
pub const LAUREL_TRAILING: SFSymbol = SFSymbol { name: "laurel.trailing" };
pub const LB_RECTANGLE_ROUNDEDBOTTOM: SFSymbol = SFSymbol { name: "lb.rectangle.roundedbottom" };
pub const LB_RECTANGLE_ROUNDEDBOTTOM_FILL: SFSymbol = SFSymbol { name: "lb.rectangle.roundedbottom.fill" };
pub const LEAF: SFSymbol = SFSymbol { name: "leaf" };
pub const LEAF_ARROW_TRIANGLE_CIRCLEPATH: SFSymbol = SFSymbol { name: "leaf.arrow.triangle.circlepath" };
pub const LEAF_CIRCLE: SFSymbol = SFSymbol { name: "leaf.circle" };
pub const LEAF_CIRCLE_FILL: SFSymbol = SFSymbol { name: "leaf.circle.fill" };
pub const LEAF_FILL: SFSymbol = SFSymbol { name: "leaf.fill" };
pub const LESSTHAN: SFSymbol = SFSymbol { name: "lessthan" };
pub const LESSTHAN_CIRCLE: SFSymbol = SFSymbol { name: "lessthan.circle" };
pub const LESSTHAN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "lessthan.circle.fill" };
pub const LESSTHAN_SQUARE: SFSymbol = SFSymbol { name: "lessthan.square" };
pub const LESSTHAN_SQUARE_FILL: SFSymbol = SFSymbol { name: "lessthan.square.fill" };
pub const LEVEL: SFSymbol = SFSymbol { name: "level" };
pub const LEVEL_FILL: SFSymbol = SFSymbol { name: "level.fill" };
pub const LIFEPRESERVER: SFSymbol = SFSymbol { name: "lifepreserver" };
pub const LIFEPRESERVER_FILL: SFSymbol = SFSymbol { name: "lifepreserver.fill" };
pub const LIGHT_BEACON_MAX: SFSymbol = SFSymbol { name: "light.beacon.max" };
pub const LIGHT_BEACON_MAX_FILL: SFSymbol = SFSymbol { name: "light.beacon.max.fill" };
pub const LIGHT_BEACON_MIN: SFSymbol = SFSymbol { name: "light.beacon.min" };
pub const LIGHT_BEACON_MIN_FILL: SFSymbol = SFSymbol { name: "light.beacon.min.fill" };
pub const LIGHT_CYLINDRICAL_CEILING: SFSymbol = SFSymbol { name: "light.cylindrical.ceiling" };
pub const LIGHT_CYLINDRICAL_CEILING_FILL: SFSymbol = SFSymbol { name: "light.cylindrical.ceiling.fill" };
pub const LIGHT_CYLINDRICAL_CEILING_INVERSE: SFSymbol = SFSymbol { name: "light.cylindrical.ceiling.inverse" };
pub const LIGHT_MAX: SFSymbol = SFSymbol { name: "light.max" };
pub const LIGHT_MIN: SFSymbol = SFSymbol { name: "light.min" };
pub const LIGHT_PANEL: SFSymbol = SFSymbol { name: "light.panel" };
pub const LIGHT_PANEL_FILL: SFSymbol = SFSymbol { name: "light.panel.fill" };
pub const LIGHT_RECESSED: SFSymbol = SFSymbol { name: "light.recessed" };
pub const LIGHT_RECESSED_3: SFSymbol = SFSymbol { name: "light.recessed.3" };
pub const LIGHT_RECESSED_3_FILL: SFSymbol = SFSymbol { name: "light.recessed.3.fill" };
pub const LIGHT_RECESSED_3_INVERSE: SFSymbol = SFSymbol { name: "light.recessed.3.inverse" };
pub const LIGHT_RECESSED_FILL: SFSymbol = SFSymbol { name: "light.recessed.fill" };
pub const LIGHT_RECESSED_INVERSE: SFSymbol = SFSymbol { name: "light.recessed.inverse" };
pub const LIGHT_RIBBON: SFSymbol = SFSymbol { name: "light.ribbon" };
pub const LIGHT_RIBBON_FILL: SFSymbol = SFSymbol { name: "light.ribbon.fill" };
pub const LIGHT_STRIP_2: SFSymbol = SFSymbol { name: "light.strip.2" };
pub const LIGHT_STRIP_2_FILL: SFSymbol = SFSymbol { name: "light.strip.2.fill" };
pub const LIGHTBULB: SFSymbol = SFSymbol { name: "lightbulb" };
pub const LIGHTBULB_2: SFSymbol = SFSymbol { name: "lightbulb.2" };
pub const LIGHTBULB_2_FILL: SFSymbol = SFSymbol { name: "lightbulb.2.fill" };
pub const LIGHTBULB_CIRCLE: SFSymbol = SFSymbol { name: "lightbulb.circle" };
pub const LIGHTBULB_CIRCLE_FILL: SFSymbol = SFSymbol { name: "lightbulb.circle.fill" };
pub const LIGHTBULB_FILL: SFSymbol = SFSymbol { name: "lightbulb.fill" };
pub const LIGHTBULB_LED: SFSymbol = SFSymbol { name: "lightbulb.led" };
pub const LIGHTBULB_LED_FILL: SFSymbol = SFSymbol { name: "lightbulb.led.fill" };
pub const LIGHTBULB_LED_WIDE: SFSymbol = SFSymbol { name: "lightbulb.led.wide" };
pub const LIGHTBULB_LED_WIDE_FILL: SFSymbol = SFSymbol { name: "lightbulb.led.wide.fill" };
pub const LIGHTBULB_SLASH: SFSymbol = SFSymbol { name: "lightbulb.slash" };
pub const LIGHTBULB_SLASH_FILL: SFSymbol = SFSymbol { name: "lightbulb.slash.fill" };
pub const LIGHTSWITCH_OFF: SFSymbol = SFSymbol { name: "lightswitch.off" };
pub const LIGHTSWITCH_OFF_FILL: SFSymbol = SFSymbol { name: "lightswitch.off.fill" };
pub const LIGHTSWITCH_OFF_SQUARE: SFSymbol = SFSymbol { name: "lightswitch.off.square" };
pub const LIGHTSWITCH_OFF_SQUARE_FILL: SFSymbol = SFSymbol { name: "lightswitch.off.square.fill" };
pub const LIGHTSWITCH_ON: SFSymbol = SFSymbol { name: "lightswitch.on" };
pub const LIGHTSWITCH_ON_FILL: SFSymbol = SFSymbol { name: "lightswitch.on.fill" };
pub const LIGHTSWITCH_ON_SQUARE: SFSymbol = SFSymbol { name: "lightswitch.on.square" };
pub const LIGHTSWITCH_ON_SQUARE_FILL: SFSymbol = SFSymbol { name: "lightswitch.on.square.fill" };
pub const LINE_2_HORIZONTAL_DECREASE_CIRCLE: SFSymbol = SFSymbol { name: "line.2.horizontal.decrease.circle" };
pub const LINE_2_HORIZONTAL_DECREASE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "line.2.horizontal.decrease.circle.fill" };
pub const LINE_3_CROSSED_SWIRL_CIRCLE: SFSymbol = SFSymbol { name: "line.3.crossed.swirl.circle" };
pub const LINE_3_CROSSED_SWIRL_CIRCLE_FILL: SFSymbol = SFSymbol { name: "line.3.crossed.swirl.circle.fill" };
pub const LINE_3_HORIZONTAL: SFSymbol = SFSymbol { name: "line.3.horizontal" };
pub const LINE_3_HORIZONTAL_CIRCLE: SFSymbol = SFSymbol { name: "line.3.horizontal.circle" };
pub const LINE_3_HORIZONTAL_CIRCLE_FILL: SFSymbol = SFSymbol { name: "line.3.horizontal.circle.fill" };
pub const LINE_3_HORIZONTAL_DECREASE: SFSymbol = SFSymbol { name: "line.3.horizontal.decrease" };
pub const LINE_3_HORIZONTAL_DECREASE_CIRCLE: SFSymbol = SFSymbol { name: "line.3.horizontal.decrease.circle" };
pub const LINE_3_HORIZONTAL_DECREASE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "line.3.horizontal.decrease.circle.fill" };
pub const LINE_DIAGONAL: SFSymbol = SFSymbol { name: "line.diagonal" };
pub const LINE_DIAGONAL_ARROW: SFSymbol = SFSymbol { name: "line.diagonal.arrow" };
pub const LINE_HORIZONTAL_STAR_FILL_LINE_HORIZONTAL: SFSymbol = SFSymbol { name: "line.horizontal.star.fill.line.horizontal" };
pub const LINES_MEASUREMENT_HORIZONTAL: SFSymbol = SFSymbol { name: "lines.measurement.horizontal" };
pub const LINEWEIGHT: SFSymbol = SFSymbol { name: "lineweight" };
pub const LINK: SFSymbol = SFSymbol { name: "link" };
pub const LINK_BADGE_PLUS: SFSymbol = SFSymbol { name: "link.badge.plus" };
pub const LINK_CIRCLE: SFSymbol = SFSymbol { name: "link.circle" };
pub const LINK_CIRCLE_FILL: SFSymbol = SFSymbol { name: "link.circle.fill" };
pub const LINK_ICLOUD: SFSymbol = SFSymbol { name: "link.icloud" };
pub const LINK_ICLOUD_FILL: SFSymbol = SFSymbol { name: "link.icloud.fill" };
pub const LIRASIGN: SFSymbol = SFSymbol { name: "lirasign" };
pub const LIRASIGN_CIRCLE: SFSymbol = SFSymbol { name: "lirasign.circle" };
pub const LIRASIGN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "lirasign.circle.fill" };
pub const LIRASIGN_SQUARE: SFSymbol = SFSymbol { name: "lirasign.square" };
pub const LIRASIGN_SQUARE_FILL: SFSymbol = SFSymbol { name: "lirasign.square.fill" };
pub const LIST_AND_FILM: SFSymbol = SFSymbol { name: "list.and.film" };
pub const LIST_BULLET: SFSymbol = SFSymbol { name: "list.bullet" };
pub const LIST_BULLET_BELOW_RECTANGLE: SFSymbol = SFSymbol { name: "list.bullet.below.rectangle" };
pub const LIST_BULLET_CIRCLE: SFSymbol = SFSymbol { name: "list.bullet.circle" };
pub const LIST_BULLET_CIRCLE_FILL: SFSymbol = SFSymbol { name: "list.bullet.circle.fill" };
pub const LIST_BULLET_CLIPBOARD: SFSymbol = SFSymbol { name: "list.bullet.clipboard" };
pub const LIST_BULLET_CLIPBOARD_FILL: SFSymbol = SFSymbol { name: "list.bullet.clipboard.fill" };
pub const LIST_BULLET_INDENT: SFSymbol = SFSymbol { name: "list.bullet.indent" };
pub const LIST_BULLET_RECTANGLE: SFSymbol = SFSymbol { name: "list.bullet.rectangle" };
pub const LIST_BULLET_RECTANGLE_FILL: SFSymbol = SFSymbol { name: "list.bullet.rectangle.fill" };
pub const LIST_BULLET_RECTANGLE_PORTRAIT: SFSymbol = SFSymbol { name: "list.bullet.rectangle.portrait" };
pub const LIST_BULLET_RECTANGLE_PORTRAIT_FILL: SFSymbol = SFSymbol { name: "list.bullet.rectangle.portrait.fill" };
pub const LIST_CLIPBOARD: SFSymbol = SFSymbol { name: "list.clipboard" };
pub const LIST_CLIPBOARD_FILL: SFSymbol = SFSymbol { name: "list.clipboard.fill" };
pub const LIST_DASH: SFSymbol = SFSymbol { name: "list.dash" };
pub const LIST_DASH_HEADER_RECTANGLE: SFSymbol = SFSymbol { name: "list.dash.header.rectangle" };
pub const LIST_NUMBER: SFSymbol = SFSymbol { name: "list.number" };
pub const LIST_STAR: SFSymbol = SFSymbol { name: "list.star" };
pub const LIST_TRIANGLE: SFSymbol = SFSymbol { name: "list.triangle" };
pub const LIVEPHOTO: SFSymbol = SFSymbol { name: "livephoto" };
pub const LIVEPHOTO_BADGE_A: SFSymbol = SFSymbol { name: "livephoto.badge.a" };
pub const LIVEPHOTO_PLAY: SFSymbol = SFSymbol { name: "livephoto.play" };
pub const LIVEPHOTO_SLASH: SFSymbol = SFSymbol { name: "livephoto.slash" };
pub const LIZARD: SFSymbol = SFSymbol { name: "lizard" };
pub const LIZARD_FILL: SFSymbol = SFSymbol { name: "lizard.fill" };
pub const LOCATION: SFSymbol = SFSymbol { name: "location" };
pub const LOCATION_CIRCLE: SFSymbol = SFSymbol { name: "location.circle" };
pub const LOCATION_CIRCLE_FILL: SFSymbol = SFSymbol { name: "location.circle.fill" };
pub const LOCATION_FILL: SFSymbol = SFSymbol { name: "location.fill" };
pub const LOCATION_FILL_VIEWFINDER: SFSymbol = SFSymbol { name: "location.fill.viewfinder" };
pub const LOCATION_MAGNIFYINGGLASS: SFSymbol = SFSymbol { name: "location.magnifyingglass" };
pub const LOCATION_NORTH: SFSymbol = SFSymbol { name: "location.north" };
pub const LOCATION_NORTH_CIRCLE: SFSymbol = SFSymbol { name: "location.north.circle" };
pub const LOCATION_NORTH_CIRCLE_FILL: SFSymbol = SFSymbol { name: "location.north.circle.fill" };
pub const LOCATION_NORTH_FILL: SFSymbol = SFSymbol { name: "location.north.fill" };
pub const LOCATION_NORTH_LINE: SFSymbol = SFSymbol { name: "location.north.line" };
pub const LOCATION_NORTH_LINE_FILL: SFSymbol = SFSymbol { name: "location.north.line.fill" };
pub const LOCATION_SLASH: SFSymbol = SFSymbol { name: "location.slash" };
pub const LOCATION_SLASH_CIRCLE: SFSymbol = SFSymbol { name: "location.slash.circle" };
pub const LOCATION_SLASH_CIRCLE_FILL: SFSymbol = SFSymbol { name: "location.slash.circle.fill" };
pub const LOCATION_SLASH_FILL: SFSymbol = SFSymbol { name: "location.slash.fill" };
pub const LOCATION_SQUARE: SFSymbol = SFSymbol { name: "location.square" };
pub const LOCATION_SQUARE_FILL: SFSymbol = SFSymbol { name: "location.square.fill" };
pub const LOCATION_VIEWFINDER: SFSymbol = SFSymbol { name: "location.viewfinder" };
pub const LOCK: SFSymbol = SFSymbol { name: "lock" };
pub const LOCK_APPLEWATCH: SFSymbol = SFSymbol { name: "lock.applewatch" };
pub const LOCK_CIRCLE: SFSymbol = SFSymbol { name: "lock.circle" };
pub const LOCK_CIRCLE_FILL: SFSymbol = SFSymbol { name: "lock.circle.fill" };
pub const LOCK_DESKTOPCOMPUTER: SFSymbol = SFSymbol { name: "lock.desktopcomputer" };
pub const LOCK_DISPLAY: SFSymbol = SFSymbol { name: "lock.display" };
pub const LOCK_DOC: SFSymbol = SFSymbol { name: "lock.doc" };
pub const LOCK_DOC_FILL: SFSymbol = SFSymbol { name: "lock.doc.fill" };
pub const LOCK_FILL: SFSymbol = SFSymbol { name: "lock.fill" };
pub const LOCK_ICLOUD: SFSymbol = SFSymbol { name: "lock.icloud" };
pub const LOCK_ICLOUD_FILL: SFSymbol = SFSymbol { name: "lock.icloud.fill" };
pub const LOCK_IPAD: SFSymbol = SFSymbol { name: "lock.ipad" };
pub const LOCK_IPHONE: SFSymbol = SFSymbol { name: "lock.iphone" };
pub const LOCK_LAPTOPCOMPUTER: SFSymbol = SFSymbol { name: "lock.laptopcomputer" };
pub const LOCK_OPEN: SFSymbol = SFSymbol { name: "lock.open" };
pub const LOCK_OPEN_APPLEWATCH: SFSymbol = SFSymbol { name: "lock.open.applewatch" };
pub const LOCK_OPEN_DESKTOPCOMPUTER: SFSymbol = SFSymbol { name: "lock.open.desktopcomputer" };
pub const LOCK_OPEN_DISPLAY: SFSymbol = SFSymbol { name: "lock.open.display" };
pub const LOCK_OPEN_FILL: SFSymbol = SFSymbol { name: "lock.open.fill" };
pub const LOCK_OPEN_IPAD: SFSymbol = SFSymbol { name: "lock.open.ipad" };
pub const LOCK_OPEN_IPHONE: SFSymbol = SFSymbol { name: "lock.open.iphone" };
pub const LOCK_OPEN_LAPTOPCOMPUTER: SFSymbol = SFSymbol { name: "lock.open.laptopcomputer" };
pub const LOCK_OPEN_TRIANGLEBADGE_EXCLAMATIONMARK: SFSymbol = SFSymbol { name: "lock.open.trianglebadge.exclamationmark" };
pub const LOCK_OPEN_TRIANGLEBADGE_EXCLAMATIONMARK_FILL: SFSymbol = SFSymbol { name: "lock.open.trianglebadge.exclamationmark.fill" };
pub const LOCK_RECTANGLE: SFSymbol = SFSymbol { name: "lock.rectangle" };
pub const LOCK_RECTANGLE_FILL: SFSymbol = SFSymbol { name: "lock.rectangle.fill" };
pub const LOCK_RECTANGLE_ON_RECTANGLE: SFSymbol = SFSymbol { name: "lock.rectangle.on.rectangle" };
pub const LOCK_RECTANGLE_ON_RECTANGLE_FILL: SFSymbol = SFSymbol { name: "lock.rectangle.on.rectangle.fill" };
pub const LOCK_RECTANGLE_STACK: SFSymbol = SFSymbol { name: "lock.rectangle.stack" };
pub const LOCK_RECTANGLE_STACK_FILL: SFSymbol = SFSymbol { name: "lock.rectangle.stack.fill" };
pub const LOCK_ROTATION: SFSymbol = SFSymbol { name: "lock.rotation" };
pub const LOCK_ROTATION_OPEN: SFSymbol = SFSymbol { name: "lock.rotation.open" };
pub const LOCK_SHIELD: SFSymbol = SFSymbol { name: "lock.shield" };
pub const LOCK_SHIELD_FILL: SFSymbol = SFSymbol { name: "lock.shield.fill" };
pub const LOCK_SLASH: SFSymbol = SFSymbol { name: "lock.slash" };
pub const LOCK_SLASH_FILL: SFSymbol = SFSymbol { name: "lock.slash.fill" };
pub const LOCK_SQUARE: SFSymbol = SFSymbol { name: "lock.square" };
pub const LOCK_SQUARE_FILL: SFSymbol = SFSymbol { name: "lock.square.fill" };
pub const LOCK_SQUARE_STACK: SFSymbol = SFSymbol { name: "lock.square.stack" };
pub const LOCK_SQUARE_STACK_FILL: SFSymbol = SFSymbol { name: "lock.square.stack.fill" };
pub const LOCK_TRIANGLEBADGE_EXCLAMATIONMARK: SFSymbol = SFSymbol { name: "lock.trianglebadge.exclamationmark" };
pub const LOCK_TRIANGLEBADGE_EXCLAMATIONMARK_FILL: SFSymbol = SFSymbol { name: "lock.trianglebadge.exclamationmark.fill" };
pub const LOGO_PLAYSTATION: SFSymbol = SFSymbol { name: "logo.playstation" };
pub const LOGO_XBOX: SFSymbol = SFSymbol { name: "logo.xbox" };
pub const LOUPE: SFSymbol = SFSymbol { name: "loupe" };
pub const LT_RECTANGLE_ROUNDEDTOP: SFSymbol = SFSymbol { name: "lt.rectangle.roundedtop" };
pub const LT_RECTANGLE_ROUNDEDTOP_FILL: SFSymbol = SFSymbol { name: "lt.rectangle.roundedtop.fill" };
pub const LUNGS: SFSymbol = SFSymbol { name: "lungs" };
pub const LUNGS_FILL: SFSymbol = SFSymbol { name: "lungs.fill" };
pub const M_CIRCLE: SFSymbol = SFSymbol { name: "m.circle" };
pub const M_CIRCLE_FILL: SFSymbol = SFSymbol { name: "m.circle.fill" };
pub const M_SQUARE: SFSymbol = SFSymbol { name: "m.square" };
pub const M_SQUARE_FILL: SFSymbol = SFSymbol { name: "m.square.fill" };
pub const MACMINI: SFSymbol = SFSymbol { name: "macmini" };
pub const MACMINI_FILL: SFSymbol = SFSymbol { name: "macmini.fill" };
pub const MACPRO_GEN1: SFSymbol = SFSymbol { name: "macpro.gen1" };
pub const MACPRO_GEN1_FILL: SFSymbol = SFSymbol { name: "macpro.gen1.fill" };
pub const MACPRO_GEN2: SFSymbol = SFSymbol { name: "macpro.gen2" };
pub const MACPRO_GEN2_FILL: SFSymbol = SFSymbol { name: "macpro.gen2.fill" };
pub const MACPRO_GEN3: SFSymbol = SFSymbol { name: "macpro.gen3" };
pub const MACPRO_GEN3_FILL: SFSymbol = SFSymbol { name: "macpro.gen3.fill" };
pub const MACPRO_GEN3_SERVER: SFSymbol = SFSymbol { name: "macpro.gen3.server" };
pub const MACSTUDIO: SFSymbol = SFSymbol { name: "macstudio" };
pub const MACSTUDIO_FILL: SFSymbol = SFSymbol { name: "macstudio.fill" };
pub const MACWINDOW: SFSymbol = SFSymbol { name: "macwindow" };
pub const MACWINDOW_BADGE_PLUS: SFSymbol = SFSymbol { name: "macwindow.badge.plus" };
pub const MACWINDOW_ON_RECTANGLE: SFSymbol = SFSymbol { name: "macwindow.on.rectangle" };
pub const MAGAZINE: SFSymbol = SFSymbol { name: "magazine" };
pub const MAGAZINE_FILL: SFSymbol = SFSymbol { name: "magazine.fill" };
pub const MAGICMOUSE: SFSymbol = SFSymbol { name: "magicmouse" };
pub const MAGICMOUSE_FILL: SFSymbol = SFSymbol { name: "magicmouse.fill" };
pub const MAGNIFYINGGLASS: SFSymbol = SFSymbol { name: "magnifyingglass" };
pub const MAGNIFYINGGLASS_CIRCLE: SFSymbol = SFSymbol { name: "magnifyingglass.circle" };
pub const MAGNIFYINGGLASS_CIRCLE_FILL: SFSymbol = SFSymbol { name: "magnifyingglass.circle.fill" };
pub const MAGSAFE_BATTERYPACK: SFSymbol = SFSymbol { name: "magsafe.batterypack" };
pub const MAGSAFE_BATTERYPACK_FILL: SFSymbol = SFSymbol { name: "magsafe.batterypack.fill" };
pub const MAIL: SFSymbol = SFSymbol { name: "mail" };
pub const MAIL_AND_TEXT_MAGNIFYINGGLASS: SFSymbol = SFSymbol { name: "mail.and.text.magnifyingglass" };
pub const MAIL_FILL: SFSymbol = SFSymbol { name: "mail.fill" };
pub const MAIL_STACK: SFSymbol = SFSymbol { name: "mail.stack" };
pub const MAIL_STACK_FILL: SFSymbol = SFSymbol { name: "mail.stack.fill" };
pub const MANATSIGN: SFSymbol = SFSymbol { name: "manatsign" };
pub const MANATSIGN_CIRCLE: SFSymbol = SFSymbol { name: "manatsign.circle" };
pub const MANATSIGN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "manatsign.circle.fill" };
pub const MANATSIGN_SQUARE: SFSymbol = SFSymbol { name: "manatsign.square" };
pub const MANATSIGN_SQUARE_FILL: SFSymbol = SFSymbol { name: "manatsign.square.fill" };
pub const MAP: SFSymbol = SFSymbol { name: "map" };
pub const MAP_CIRCLE: SFSymbol = SFSymbol { name: "map.circle" };
pub const MAP_CIRCLE_FILL: SFSymbol = SFSymbol { name: "map.circle.fill" };
pub const MAP_FILL: SFSymbol = SFSymbol { name: "map.fill" };
pub const MAPPIN: SFSymbol = SFSymbol { name: "mappin" };
pub const MAPPIN_AND_ELLIPSE: SFSymbol = SFSymbol { name: "mappin.and.ellipse" };
pub const MAPPIN_CIRCLE: SFSymbol = SFSymbol { name: "mappin.circle" };
pub const MAPPIN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "mappin.circle.fill" };
pub const MAPPIN_SLASH: SFSymbol = SFSymbol { name: "mappin.slash" };
pub const MAPPIN_SLASH_CIRCLE: SFSymbol = SFSymbol { name: "mappin.slash.circle" };
pub const MAPPIN_SLASH_CIRCLE_FILL: SFSymbol = SFSymbol { name: "mappin.slash.circle.fill" };
pub const MAPPIN_SQUARE: SFSymbol = SFSymbol { name: "mappin.square" };
pub const MAPPIN_SQUARE_FILL: SFSymbol = SFSymbol { name: "mappin.square.fill" };
pub const MEDAL: SFSymbol = SFSymbol { name: "medal" };
pub const MEDAL_FILL: SFSymbol = SFSymbol { name: "medal.fill" };
pub const MEDIASTICK: SFSymbol = SFSymbol { name: "mediastick" };
pub const MEDICAL_THERMOMETER: SFSymbol = SFSymbol { name: "medical.thermometer" };
pub const MEDICAL_THERMOMETER_FILL: SFSymbol = SFSymbol { name: "medical.thermometer.fill" };
pub const MEGAPHONE: SFSymbol = SFSymbol { name: "megaphone" };
pub const MEGAPHONE_FILL: SFSymbol = SFSymbol { name: "megaphone.fill" };
pub const MEMORIES: SFSymbol = SFSymbol { name: "memories" };
pub const MEMORIES_BADGE_MINUS: SFSymbol = SFSymbol { name: "memories.badge.minus" };
pub const MEMORIES_BADGE_PLUS: SFSymbol = SFSymbol { name: "memories.badge.plus" };
pub const MEMORYCHIP: SFSymbol = SFSymbol { name: "memorychip" };
pub const MEMORYCHIP_FILL: SFSymbol = SFSymbol { name: "memorychip.fill" };
pub const MENUBAR_ARROW_DOWN_RECTANGLE: SFSymbol = SFSymbol { name: "menubar.arrow.down.rectangle" };
pub const MENUBAR_ARROW_UP_RECTANGLE: SFSymbol = SFSymbol { name: "menubar.arrow.up.rectangle" };
pub const MENUBAR_DOCK_RECTANGLE: SFSymbol = SFSymbol { name: "menubar.dock.rectangle" };
pub const MENUBAR_DOCK_RECTANGLE_BADGE_RECORD: SFSymbol = SFSymbol { name: "menubar.dock.rectangle.badge.record" };
pub const MENUBAR_RECTANGLE: SFSymbol = SFSymbol { name: "menubar.rectangle" };
pub const MENUCARD: SFSymbol = SFSymbol { name: "menucard" };
pub const MENUCARD_FILL: SFSymbol = SFSymbol { name: "menucard.fill" };
pub const MESSAGE: SFSymbol = SFSymbol { name: "message" };
pub const MESSAGE_AND_WAVEFORM: SFSymbol = SFSymbol { name: "message.and.waveform" };
pub const MESSAGE_AND_WAVEFORM_FILL: SFSymbol = SFSymbol { name: "message.and.waveform.fill" };
pub const MESSAGE_BADGE: SFSymbol = SFSymbol { name: "message.badge" };
pub const MESSAGE_BADGE_CIRCLE: SFSymbol = SFSymbol { name: "message.badge.circle" };
pub const MESSAGE_BADGE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "message.badge.circle.fill" };
pub const MESSAGE_BADGE_FILL: SFSymbol = SFSymbol { name: "message.badge.fill" };
pub const MESSAGE_BADGE_FILLED_FILL: SFSymbol = SFSymbol { name: "message.badge.filled.fill" };
pub const MESSAGE_CIRCLE: SFSymbol = SFSymbol { name: "message.circle" };
pub const MESSAGE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "message.circle.fill" };
pub const MESSAGE_FILL: SFSymbol = SFSymbol { name: "message.fill" };
pub const METRONOME: SFSymbol = SFSymbol { name: "metronome" };
pub const METRONOME_FILL: SFSymbol = SFSymbol { name: "metronome.fill" };
pub const MIC: SFSymbol = SFSymbol { name: "mic" };
pub const MIC_AND_SIGNAL_METER: SFSymbol = SFSymbol { name: "mic.and.signal.meter" };
pub const MIC_AND_SIGNAL_METER_FILL: SFSymbol = SFSymbol { name: "mic.and.signal.meter.fill" };
pub const MIC_BADGE_PLUS: SFSymbol = SFSymbol { name: "mic.badge.plus" };
pub const MIC_BADGE_XMARK: SFSymbol = SFSymbol { name: "mic.badge.xmark" };
pub const MIC_CIRCLE: SFSymbol = SFSymbol { name: "mic.circle" };
pub const MIC_CIRCLE_FILL: SFSymbol = SFSymbol { name: "mic.circle.fill" };
pub const MIC_FILL: SFSymbol = SFSymbol { name: "mic.fill" };
pub const MIC_FILL_BADGE_PLUS: SFSymbol = SFSymbol { name: "mic.fill.badge.plus" };
pub const MIC_FILL_BADGE_XMARK: SFSymbol = SFSymbol { name: "mic.fill.badge.xmark" };
pub const MIC_SLASH: SFSymbol = SFSymbol { name: "mic.slash" };
pub const MIC_SLASH_CIRCLE: SFSymbol = SFSymbol { name: "mic.slash.circle" };
pub const MIC_SLASH_CIRCLE_FILL: SFSymbol = SFSymbol { name: "mic.slash.circle.fill" };
pub const MIC_SLASH_FILL: SFSymbol = SFSymbol { name: "mic.slash.fill" };
pub const MIC_SQUARE: SFSymbol = SFSymbol { name: "mic.square" };
pub const MIC_SQUARE_FILL: SFSymbol = SFSymbol { name: "mic.square.fill" };
pub const MICROBE: SFSymbol = SFSymbol { name: "microbe" };
pub const MICROBE_CIRCLE: SFSymbol = SFSymbol { name: "microbe.circle" };
pub const MICROBE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "microbe.circle.fill" };
pub const MICROBE_FILL: SFSymbol = SFSymbol { name: "microbe.fill" };
pub const MICROWAVE: SFSymbol = SFSymbol { name: "microwave" };
pub const MICROWAVE_FILL: SFSymbol = SFSymbol { name: "microwave.fill" };
pub const MILLSIGN: SFSymbol = SFSymbol { name: "millsign" };
pub const MILLSIGN_CIRCLE: SFSymbol = SFSymbol { name: "millsign.circle" };
pub const MILLSIGN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "millsign.circle.fill" };
pub const MILLSIGN_SQUARE: SFSymbol = SFSymbol { name: "millsign.square" };
pub const MILLSIGN_SQUARE_FILL: SFSymbol = SFSymbol { name: "millsign.square.fill" };
pub const MINUS: SFSymbol = SFSymbol { name: "minus" };
pub const MINUS_CIRCLE: SFSymbol = SFSymbol { name: "minus.circle" };
pub const MINUS_CIRCLE_FILL: SFSymbol = SFSymbol { name: "minus.circle.fill" };
pub const MINUS_DIAMOND: SFSymbol = SFSymbol { name: "minus.diamond" };
pub const MINUS_DIAMOND_FILL: SFSymbol = SFSymbol { name: "minus.diamond.fill" };
pub const MINUS_FORWARDSLASH_PLUS: SFSymbol = SFSymbol { name: "minus.forwardslash.plus" };
pub const MINUS_MAGNIFYINGGLASS: SFSymbol = SFSymbol { name: "minus.magnifyingglass" };
pub const MINUS_PLUS_BATTERYBLOCK: SFSymbol = SFSymbol { name: "minus.plus.batteryblock" };
pub const MINUS_PLUS_BATTERYBLOCK_FILL: SFSymbol = SFSymbol { name: "minus.plus.batteryblock.fill" };
pub const MINUS_RECTANGLE: SFSymbol = SFSymbol { name: "minus.rectangle" };
pub const MINUS_RECTANGLE_FILL: SFSymbol = SFSymbol { name: "minus.rectangle.fill" };
pub const MINUS_RECTANGLE_PORTRAIT: SFSymbol = SFSymbol { name: "minus.rectangle.portrait" };
pub const MINUS_RECTANGLE_PORTRAIT_FILL: SFSymbol = SFSymbol { name: "minus.rectangle.portrait.fill" };
pub const MINUS_SQUARE: SFSymbol = SFSymbol { name: "minus.square" };
pub const MINUS_SQUARE_FILL: SFSymbol = SFSymbol { name: "minus.square.fill" };
pub const MOON: SFSymbol = SFSymbol { name: "moon" };
pub const MOON_CIRCLE: SFSymbol = SFSymbol { name: "moon.circle" };
pub const MOON_CIRCLE_FILL: SFSymbol = SFSymbol { name: "moon.circle.fill" };
pub const MOON_FILL: SFSymbol = SFSymbol { name: "moon.fill" };
pub const MOON_HAZE: SFSymbol = SFSymbol { name: "moon.haze" };
pub const MOON_HAZE_CIRCLE: SFSymbol = SFSymbol { name: "moon.haze.circle" };
pub const MOON_HAZE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "moon.haze.circle.fill" };
pub const MOON_HAZE_FILL: SFSymbol = SFSymbol { name: "moon.haze.fill" };
pub const MOON_STARS: SFSymbol = SFSymbol { name: "moon.stars" };
pub const MOON_STARS_CIRCLE: SFSymbol = SFSymbol { name: "moon.stars.circle" };
pub const MOON_STARS_CIRCLE_FILL: SFSymbol = SFSymbol { name: "moon.stars.circle.fill" };
pub const MOON_STARS_FILL: SFSymbol = SFSymbol { name: "moon.stars.fill" };
pub const MOON_ZZZ: SFSymbol = SFSymbol { name: "moon.zzz" };
pub const MOON_ZZZ_FILL: SFSymbol = SFSymbol { name: "moon.zzz.fill" };
pub const MOONPHASE_FIRST_QUARTER: SFSymbol = SFSymbol { name: "moonphase.first.quarter" };
pub const MOONPHASE_FIRST_QUARTER_INVERSE: SFSymbol = SFSymbol { name: "moonphase.first.quarter.inverse" };
pub const MOONPHASE_FULL_MOON: SFSymbol = SFSymbol { name: "moonphase.full.moon" };
pub const MOONPHASE_FULL_MOON_INVERSE: SFSymbol = SFSymbol { name: "moonphase.full.moon.inverse" };
pub const MOONPHASE_LAST_QUARTER: SFSymbol = SFSymbol { name: "moonphase.last.quarter" };
pub const MOONPHASE_LAST_QUARTER_INVERSE: SFSymbol = SFSymbol { name: "moonphase.last.quarter.inverse" };
pub const MOONPHASE_NEW_MOON: SFSymbol = SFSymbol { name: "moonphase.new.moon" };
pub const MOONPHASE_NEW_MOON_INVERSE: SFSymbol = SFSymbol { name: "moonphase.new.moon.inverse" };
pub const MOONPHASE_WANING_CRESCENT: SFSymbol = SFSymbol { name: "moonphase.waning.crescent" };
pub const MOONPHASE_WANING_CRESCENT_INVERSE: SFSymbol = SFSymbol { name: "moonphase.waning.crescent.inverse" };
pub const MOONPHASE_WANING_GIBBOUS: SFSymbol = SFSymbol { name: "moonphase.waning.gibbous" };
pub const MOONPHASE_WANING_GIBBOUS_INVERSE: SFSymbol = SFSymbol { name: "moonphase.waning.gibbous.inverse" };
pub const MOONPHASE_WAXING_CRESCENT: SFSymbol = SFSymbol { name: "moonphase.waxing.crescent" };
pub const MOONPHASE_WAXING_CRESCENT_INVERSE: SFSymbol = SFSymbol { name: "moonphase.waxing.crescent.inverse" };
pub const MOONPHASE_WAXING_GIBBOUS: SFSymbol = SFSymbol { name: "moonphase.waxing.gibbous" };
pub const MOONPHASE_WAXING_GIBBOUS_INVERSE: SFSymbol = SFSymbol { name: "moonphase.waxing.gibbous.inverse" };
pub const MOSAIC: SFSymbol = SFSymbol { name: "mosaic" };
pub const MOSAIC_FILL: SFSymbol = SFSymbol { name: "mosaic.fill" };
pub const MOUNT: SFSymbol = SFSymbol { name: "mount" };
pub const MOUNT_FILL: SFSymbol = SFSymbol { name: "mount.fill" };
pub const MOUTH: SFSymbol = SFSymbol { name: "mouth" };
pub const MOUTH_FILL: SFSymbol = SFSymbol { name: "mouth.fill" };
pub const MOVE_3D: SFSymbol = SFSymbol { name: "move.3d" };
pub const MULTIPLY: SFSymbol = SFSymbol { name: "multiply" };
pub const MULTIPLY_CIRCLE: SFSymbol = SFSymbol { name: "multiply.circle" };
pub const MULTIPLY_CIRCLE_FILL: SFSymbol = SFSymbol { name: "multiply.circle.fill" };
pub const MULTIPLY_SQUARE: SFSymbol = SFSymbol { name: "multiply.square" };
pub const MULTIPLY_SQUARE_FILL: SFSymbol = SFSymbol { name: "multiply.square.fill" };
pub const MUSIC_MIC: SFSymbol = SFSymbol { name: "music.mic" };
pub const MUSIC_MIC_CIRCLE: SFSymbol = SFSymbol { name: "music.mic.circle" };
pub const MUSIC_MIC_CIRCLE_FILL: SFSymbol = SFSymbol { name: "music.mic.circle.fill" };
pub const MUSIC_NOTE: SFSymbol = SFSymbol { name: "music.note" };
pub const MUSIC_NOTE_HOUSE: SFSymbol = SFSymbol { name: "music.note.house" };
pub const MUSIC_NOTE_HOUSE_FILL: SFSymbol = SFSymbol { name: "music.note.house.fill" };
pub const MUSIC_NOTE_LIST: SFSymbol = SFSymbol { name: "music.note.list" };
pub const MUSIC_NOTE_TV: SFSymbol = SFSymbol { name: "music.note.tv" };
pub const MUSIC_NOTE_TV_FILL: SFSymbol = SFSymbol { name: "music.note.tv.fill" };
pub const MUSIC_QUARTERNOTE_3: SFSymbol = SFSymbol { name: "music.quarternote.3" };
pub const MUSTACHE: SFSymbol = SFSymbol { name: "mustache" };
pub const MUSTACHE_FILL: SFSymbol = SFSymbol { name: "mustache.fill" };
pub const N_CIRCLE: SFSymbol = SFSymbol { name: "n.circle" };
pub const N_CIRCLE_FILL: SFSymbol = SFSymbol { name: "n.circle.fill" };
pub const N_SQUARE: SFSymbol = SFSymbol { name: "n.square" };
pub const N_SQUARE_FILL: SFSymbol = SFSymbol { name: "n.square.fill" };
pub const NAIRASIGN: SFSymbol = SFSymbol { name: "nairasign" };
pub const NAIRASIGN_CIRCLE: SFSymbol = SFSymbol { name: "nairasign.circle" };
pub const NAIRASIGN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "nairasign.circle.fill" };
pub const NAIRASIGN_SQUARE: SFSymbol = SFSymbol { name: "nairasign.square" };
pub const NAIRASIGN_SQUARE_FILL: SFSymbol = SFSymbol { name: "nairasign.square.fill" };
pub const NETWORK: SFSymbol = SFSymbol { name: "network" };
pub const NETWORK_BADGE_SHIELD_HALF_FILLED: SFSymbol = SFSymbol { name: "network.badge.shield.half.filled" };
pub const NEWSPAPER: SFSymbol = SFSymbol { name: "newspaper" };
pub const NEWSPAPER_CIRCLE: SFSymbol = SFSymbol { name: "newspaper.circle" };
pub const NEWSPAPER_CIRCLE_FILL: SFSymbol = SFSymbol { name: "newspaper.circle.fill" };
pub const NEWSPAPER_FILL: SFSymbol = SFSymbol { name: "newspaper.fill" };
pub const NOSE: SFSymbol = SFSymbol { name: "nose" };
pub const NOSE_FILL: SFSymbol = SFSymbol { name: "nose.fill" };
pub const NOSIGN: SFSymbol = SFSymbol { name: "nosign" };
pub const NOTE: SFSymbol = SFSymbol { name: "note" };
pub const NOTE_TEXT: SFSymbol = SFSymbol { name: "note.text" };
pub const NOTE_TEXT_BADGE_PLUS: SFSymbol = SFSymbol { name: "note.text.badge.plus" };
pub const NUMBER: SFSymbol = SFSymbol { name: "number" };
pub const NUMBER_CIRCLE: SFSymbol = SFSymbol { name: "number.circle" };
pub const NUMBER_CIRCLE_FILL: SFSymbol = SFSymbol { name: "number.circle.fill" };
pub const NUMBER_SQUARE: SFSymbol = SFSymbol { name: "number.square" };
pub const NUMBER_SQUARE_FILL: SFSymbol = SFSymbol { name: "number.square.fill" };
pub const NUMBERSIGN: SFSymbol = SFSymbol { name: "numbersign" };
pub const O_CIRCLE: SFSymbol = SFSymbol { name: "o.circle" };
pub const O_CIRCLE_FILL: SFSymbol = SFSymbol { name: "o.circle.fill" };
pub const O_SQUARE: SFSymbol = SFSymbol { name: "o.square" };
pub const O_SQUARE_FILL: SFSymbol = SFSymbol { name: "o.square.fill" };
pub const OAR_2_CROSSED: SFSymbol = SFSymbol { name: "oar.2.crossed" };
pub const OCTAGON: SFSymbol = SFSymbol { name: "octagon" };
pub const OCTAGON_BOTTOMHALF_FILLED: SFSymbol = SFSymbol { name: "octagon.bottomhalf.filled" };
pub const OCTAGON_FILL: SFSymbol = SFSymbol { name: "octagon.fill" };
pub const OCTAGON_LEFTHALF_FILLED: SFSymbol = SFSymbol { name: "octagon.lefthalf.filled" };
pub const OCTAGON_RIGHTHALF_FILLED: SFSymbol = SFSymbol { name: "octagon.righthalf.filled" };
pub const OCTAGON_TOPHALF_FILLED: SFSymbol = SFSymbol { name: "octagon.tophalf.filled" };
pub const OPTICALDISC: SFSymbol = SFSymbol { name: "opticaldisc" };
pub const OPTICALDISC_FILL: SFSymbol = SFSymbol { name: "opticaldisc.fill" };
pub const OPTICALDISCDRIVE: SFSymbol = SFSymbol { name: "opticaldiscdrive" };
pub const OPTICALDISCDRIVE_FILL: SFSymbol = SFSymbol { name: "opticaldiscdrive.fill" };
pub const OPTION: SFSymbol = SFSymbol { name: "option" };
pub const OVAL: SFSymbol = SFSymbol { name: "oval" };
pub const OVAL_BOTTOMHALF_FILLED: SFSymbol = SFSymbol { name: "oval.bottomhalf.filled" };
pub const OVAL_FILL: SFSymbol = SFSymbol { name: "oval.fill" };
pub const OVAL_INSET_FILLED: SFSymbol = SFSymbol { name: "oval.inset.filled" };
pub const OVAL_LEFTHALF_FILLED: SFSymbol = SFSymbol { name: "oval.lefthalf.filled" };
pub const OVAL_PORTRAIT: SFSymbol = SFSymbol { name: "oval.portrait" };
pub const OVAL_PORTRAIT_BOTTOMHALF_FILLED: SFSymbol = SFSymbol { name: "oval.portrait.bottomhalf.filled" };
pub const OVAL_PORTRAIT_FILL: SFSymbol = SFSymbol { name: "oval.portrait.fill" };
pub const OVAL_PORTRAIT_INSET_FILLED: SFSymbol = SFSymbol { name: "oval.portrait.inset.filled" };
pub const OVAL_PORTRAIT_LEFTHALF_FILLED: SFSymbol = SFSymbol { name: "oval.portrait.lefthalf.filled" };
pub const OVAL_PORTRAIT_RIGHTHALF_FILLED: SFSymbol = SFSymbol { name: "oval.portrait.righthalf.filled" };
pub const OVAL_PORTRAIT_TOPHALF_FILLED: SFSymbol = SFSymbol { name: "oval.portrait.tophalf.filled" };
pub const OVAL_RIGHTHALF_FILLED: SFSymbol = SFSymbol { name: "oval.righthalf.filled" };
pub const OVAL_TOPHALF_FILLED: SFSymbol = SFSymbol { name: "oval.tophalf.filled" };
pub const OVEN: SFSymbol = SFSymbol { name: "oven" };
pub const OVEN_FILL: SFSymbol = SFSymbol { name: "oven.fill" };
pub const P_CIRCLE: SFSymbol = SFSymbol { name: "p.circle" };
pub const P_CIRCLE_FILL: SFSymbol = SFSymbol { name: "p.circle.fill" };
pub const P_SQUARE: SFSymbol = SFSymbol { name: "p.square" };
pub const P_SQUARE_FILL: SFSymbol = SFSymbol { name: "p.square.fill" };
pub const PAINTBRUSH: SFSymbol = SFSymbol { name: "paintbrush" };
pub const PAINTBRUSH_FILL: SFSymbol = SFSymbol { name: "paintbrush.fill" };
pub const PAINTBRUSH_POINTED: SFSymbol = SFSymbol { name: "paintbrush.pointed" };
pub const PAINTBRUSH_POINTED_FILL: SFSymbol = SFSymbol { name: "paintbrush.pointed.fill" };
pub const PAINTPALETTE: SFSymbol = SFSymbol { name: "paintpalette" };
pub const PAINTPALETTE_FILL: SFSymbol = SFSymbol { name: "paintpalette.fill" };
pub const PANO: SFSymbol = SFSymbol { name: "pano" };
pub const PANO_FILL: SFSymbol = SFSymbol { name: "pano.fill" };
pub const PAPERCLIP: SFSymbol = SFSymbol { name: "paperclip" };
pub const PAPERCLIP_BADGE_ELLIPSIS: SFSymbol = SFSymbol { name: "paperclip.badge.ellipsis" };
pub const PAPERCLIP_CIRCLE: SFSymbol = SFSymbol { name: "paperclip.circle" };
pub const PAPERCLIP_CIRCLE_FILL: SFSymbol = SFSymbol { name: "paperclip.circle.fill" };
pub const PAPERPLANE: SFSymbol = SFSymbol { name: "paperplane" };
pub const PAPERPLANE_CIRCLE: SFSymbol = SFSymbol { name: "paperplane.circle" };
pub const PAPERPLANE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "paperplane.circle.fill" };
pub const PAPERPLANE_FILL: SFSymbol = SFSymbol { name: "paperplane.fill" };
pub const PARAGRAPHSIGN: SFSymbol = SFSymbol { name: "paragraphsign" };
pub const PARENTHESES: SFSymbol = SFSymbol { name: "parentheses" };
pub const PARKINGSIGN: SFSymbol = SFSymbol { name: "parkingsign" };
pub const PARKINGSIGN_CIRCLE: SFSymbol = SFSymbol { name: "parkingsign.circle" };
pub const PARKINGSIGN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "parkingsign.circle.fill" };
pub const PARTY_POPPER: SFSymbol = SFSymbol { name: "party.popper" };
pub const PARTY_POPPER_FILL: SFSymbol = SFSymbol { name: "party.popper.fill" };
pub const PAUSE: SFSymbol = SFSymbol { name: "pause" };
pub const PAUSE_CIRCLE: SFSymbol = SFSymbol { name: "pause.circle" };
pub const PAUSE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "pause.circle.fill" };
pub const PAUSE_FILL: SFSymbol = SFSymbol { name: "pause.fill" };
pub const PAUSE_RECTANGLE: SFSymbol = SFSymbol { name: "pause.rectangle" };
pub const PAUSE_RECTANGLE_FILL: SFSymbol = SFSymbol { name: "pause.rectangle.fill" };
pub const PAWPRINT: SFSymbol = SFSymbol { name: "pawprint" };
pub const PAWPRINT_CIRCLE: SFSymbol = SFSymbol { name: "pawprint.circle" };
pub const PAWPRINT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "pawprint.circle.fill" };
pub const PAWPRINT_FILL: SFSymbol = SFSymbol { name: "pawprint.fill" };
pub const PC: SFSymbol = SFSymbol { name: "pc" };
pub const PEACESIGN: SFSymbol = SFSymbol { name: "peacesign" };
pub const PEDESTRIAN_GATE_CLOSED: SFSymbol = SFSymbol { name: "pedestrian.gate.closed" };
pub const PEDESTRIAN_GATE_OPEN: SFSymbol = SFSymbol { name: "pedestrian.gate.open" };
pub const PENCIL: SFSymbol = SFSymbol { name: "pencil" };
pub const PENCIL_AND_OUTLINE: SFSymbol = SFSymbol { name: "pencil.and.outline" };
pub const PENCIL_AND_RULER: SFSymbol = SFSymbol { name: "pencil.and.ruler" };
pub const PENCIL_AND_RULER_FILL: SFSymbol = SFSymbol { name: "pencil.and.ruler.fill" };
pub const PENCIL_CIRCLE: SFSymbol = SFSymbol { name: "pencil.circle" };
pub const PENCIL_CIRCLE_FILL: SFSymbol = SFSymbol { name: "pencil.circle.fill" };
pub const PENCIL_LINE: SFSymbol = SFSymbol { name: "pencil.line" };
pub const PENCIL_SLASH: SFSymbol = SFSymbol { name: "pencil.slash" };
pub const PENCIL_TIP: SFSymbol = SFSymbol { name: "pencil.tip" };
pub const PENCIL_TIP_CROP_CIRCLE: SFSymbol = SFSymbol { name: "pencil.tip.crop.circle" };
pub const PENCIL_TIP_CROP_CIRCLE_BADGE_ARROW_FORWARD: SFSymbol = SFSymbol { name: "pencil.tip.crop.circle.badge.arrow.forward" };
pub const PENCIL_TIP_CROP_CIRCLE_BADGE_MINUS: SFSymbol = SFSymbol { name: "pencil.tip.crop.circle.badge.minus" };
pub const PENCIL_TIP_CROP_CIRCLE_BADGE_PLUS: SFSymbol = SFSymbol { name: "pencil.tip.crop.circle.badge.plus" };
pub const PENTAGON: SFSymbol = SFSymbol { name: "pentagon" };
pub const PENTAGON_BOTTOMHALF_FILLED: SFSymbol = SFSymbol { name: "pentagon.bottomhalf.filled" };
pub const PENTAGON_FILL: SFSymbol = SFSymbol { name: "pentagon.fill" };
pub const PENTAGON_LEFTHALF_FILLED: SFSymbol = SFSymbol { name: "pentagon.lefthalf.filled" };
pub const PENTAGON_RIGHTHALF_FILLED: SFSymbol = SFSymbol { name: "pentagon.righthalf.filled" };
pub const PENTAGON_TOPHALF_FILLED: SFSymbol = SFSymbol { name: "pentagon.tophalf.filled" };
pub const PERCENT: SFSymbol = SFSymbol { name: "percent" };
pub const PERSON: SFSymbol = SFSymbol { name: "person" };
pub const PERSON_2: SFSymbol = SFSymbol { name: "person.2" };
pub const PERSON_2_BADGE_GEARSHAPE: SFSymbol = SFSymbol { name: "person.2.badge.gearshape" };
pub const PERSON_2_BADGE_GEARSHAPE_FILL: SFSymbol = SFSymbol { name: "person.2.badge.gearshape.fill" };
pub const PERSON_2_CIRCLE: SFSymbol = SFSymbol { name: "person.2.circle" };
pub const PERSON_2_CIRCLE_FILL: SFSymbol = SFSymbol { name: "person.2.circle.fill" };
pub const PERSON_2_CROP_SQUARE_STACK: SFSymbol = SFSymbol { name: "person.2.crop.square.stack" };
pub const PERSON_2_CROP_SQUARE_STACK_FILL: SFSymbol = SFSymbol { name: "person.2.crop.square.stack.fill" };
pub const PERSON_2_FILL: SFSymbol = SFSymbol { name: "person.2.fill" };
pub const PERSON_2_GOBACKWARD: SFSymbol = SFSymbol { name: "person.2.gobackward" };
pub const PERSON_2_WAVE_2: SFSymbol = SFSymbol { name: "person.2.wave.2" };
pub const PERSON_2_WAVE_2_FILL: SFSymbol = SFSymbol { name: "person.2.wave.2.fill" };
pub const PERSON_3: SFSymbol = SFSymbol { name: "person.3" };
pub const PERSON_3_FILL: SFSymbol = SFSymbol { name: "person.3.fill" };
pub const PERSON_3_SEQUENCE: SFSymbol = SFSymbol { name: "person.3.sequence" };
pub const PERSON_3_SEQUENCE_FILL: SFSymbol = SFSymbol { name: "person.3.sequence.fill" };
pub const PERSON_AND_ARROW_LEFT_AND_ARROW_RIGHT: SFSymbol = SFSymbol { name: "person.and.arrow.left.and.arrow.right" };
pub const PERSON_BADGE_CLOCK: SFSymbol = SFSymbol { name: "person.badge.clock" };
pub const PERSON_BADGE_CLOCK_FILL: SFSymbol = SFSymbol { name: "person.badge.clock.fill" };
pub const PERSON_BADGE_KEY: SFSymbol = SFSymbol { name: "person.badge.key" };
pub const PERSON_BADGE_KEY_FILL: SFSymbol = SFSymbol { name: "person.badge.key.fill" };
pub const PERSON_BADGE_MINUS: SFSymbol = SFSymbol { name: "person.badge.minus" };
pub const PERSON_BADGE_PLUS: SFSymbol = SFSymbol { name: "person.badge.plus" };
pub const PERSON_BADGE_SHIELD_CHECKMARK: SFSymbol = SFSymbol { name: "person.badge.shield.checkmark" };
pub const PERSON_BADGE_SHIELD_CHECKMARK_FILL: SFSymbol = SFSymbol { name: "person.badge.shield.checkmark.fill" };
pub const PERSON_BUST: SFSymbol = SFSymbol { name: "person.bust" };
pub const PERSON_BUST_FILL: SFSymbol = SFSymbol { name: "person.bust.fill" };
pub const PERSON_CIRCLE: SFSymbol = SFSymbol { name: "person.circle" };
pub const PERSON_CIRCLE_FILL: SFSymbol = SFSymbol { name: "person.circle.fill" };
pub const PERSON_CROP_ARTFRAME: SFSymbol = SFSymbol { name: "person.crop.artframe" };
pub const PERSON_CROP_BACKGROUND_DOTTED: SFSymbol = SFSymbol { name: "person.crop.background.dotted" };
pub const PERSON_CROP_CIRCLE: SFSymbol = SFSymbol { name: "person.crop.circle" };
pub const PERSON_CROP_CIRCLE_BADGE: SFSymbol = SFSymbol { name: "person.crop.circle.badge" };
pub const PERSON_CROP_CIRCLE_BADGE_CHECKMARK: SFSymbol = SFSymbol { name: "person.crop.circle.badge.checkmark" };
pub const PERSON_CROP_CIRCLE_BADGE_CLOCK: SFSymbol = SFSymbol { name: "person.crop.circle.badge.clock" };
pub const PERSON_CROP_CIRCLE_BADGE_CLOCK_FILL: SFSymbol = SFSymbol { name: "person.crop.circle.badge.clock.fill" };
pub const PERSON_CROP_CIRCLE_BADGE_EXCLAMATIONMARK: SFSymbol = SFSymbol { name: "person.crop.circle.badge.exclamationmark" };
pub const PERSON_CROP_CIRCLE_BADGE_EXCLAMATIONMARK_FILL: SFSymbol = SFSymbol { name: "person.crop.circle.badge.exclamationmark.fill" };
pub const PERSON_CROP_CIRCLE_BADGE_FILL: SFSymbol = SFSymbol { name: "person.crop.circle.badge.fill" };
pub const PERSON_CROP_CIRCLE_BADGE_MINUS: SFSymbol = SFSymbol { name: "person.crop.circle.badge.minus" };
pub const PERSON_CROP_CIRCLE_BADGE_MOON: SFSymbol = SFSymbol { name: "person.crop.circle.badge.moon" };
pub const PERSON_CROP_CIRCLE_BADGE_MOON_FILL: SFSymbol = SFSymbol { name: "person.crop.circle.badge.moon.fill" };
pub const PERSON_CROP_CIRCLE_BADGE_PLUS: SFSymbol = SFSymbol { name: "person.crop.circle.badge.plus" };
pub const PERSON_CROP_CIRCLE_BADGE_QUESTIONMARK: SFSymbol = SFSymbol { name: "person.crop.circle.badge.questionmark" };
pub const PERSON_CROP_CIRCLE_BADGE_QUESTIONMARK_FILL: SFSymbol = SFSymbol { name: "person.crop.circle.badge.questionmark.fill" };
pub const PERSON_CROP_CIRCLE_BADGE_XMARK: SFSymbol = SFSymbol { name: "person.crop.circle.badge.xmark" };
pub const PERSON_CROP_CIRCLE_FILL: SFSymbol = SFSymbol { name: "person.crop.circle.fill" };
pub const PERSON_CROP_CIRCLE_FILL_BADGE_CHECKMARK: SFSymbol = SFSymbol { name: "person.crop.circle.fill.badge.checkmark" };
pub const PERSON_CROP_CIRCLE_FILL_BADGE_MINUS: SFSymbol = SFSymbol { name: "person.crop.circle.fill.badge.minus" };
pub const PERSON_CROP_CIRCLE_FILL_BADGE_PLUS: SFSymbol = SFSymbol { name: "person.crop.circle.fill.badge.plus" };
pub const PERSON_CROP_CIRCLE_FILL_BADGE_XMARK: SFSymbol = SFSymbol { name: "person.crop.circle.fill.badge.xmark" };
pub const PERSON_CROP_RECTANGLE: SFSymbol = SFSymbol { name: "person.crop.rectangle" };
pub const PERSON_CROP_RECTANGLE_BADGE_PLUS: SFSymbol = SFSymbol { name: "person.crop.rectangle.badge.plus" };
pub const PERSON_CROP_RECTANGLE_BADGE_PLUS_FILL: SFSymbol = SFSymbol { name: "person.crop.rectangle.badge.plus.fill" };
pub const PERSON_CROP_RECTANGLE_FILL: SFSymbol = SFSymbol { name: "person.crop.rectangle.fill" };
pub const PERSON_CROP_RECTANGLE_STACK: SFSymbol = SFSymbol { name: "person.crop.rectangle.stack" };
pub const PERSON_CROP_RECTANGLE_STACK_FILL: SFSymbol = SFSymbol { name: "person.crop.rectangle.stack.fill" };
pub const PERSON_CROP_SQUARE: SFSymbol = SFSymbol { name: "person.crop.square" };
pub const PERSON_CROP_SQUARE_FILL: SFSymbol = SFSymbol { name: "person.crop.square.fill" };
pub const PERSON_CROP_SQUARE_FILLED_AND_AT_RECTANGLE: SFSymbol = SFSymbol { name: "person.crop.square.filled.and.at.rectangle" };
pub const PERSON_CROP_SQUARE_FILLED_AND_AT_RECTANGLE_FILL: SFSymbol = SFSymbol { name: "person.crop.square.filled.and.at.rectangle.fill" };
pub const PERSON_FILL: SFSymbol = SFSymbol { name: "person.fill" };
pub const PERSON_FILL_AND_ARROW_LEFT_AND_ARROW_RIGHT: SFSymbol = SFSymbol { name: "person.fill.and.arrow.left.and.arrow.right" };
pub const PERSON_FILL_BADGE_MINUS: SFSymbol = SFSymbol { name: "person.fill.badge.minus" };
pub const PERSON_FILL_BADGE_PLUS: SFSymbol = SFSymbol { name: "person.fill.badge.plus" };
pub const PERSON_FILL_CHECKMARK: SFSymbol = SFSymbol { name: "person.fill.checkmark" };
pub const PERSON_FILL_QUESTIONMARK: SFSymbol = SFSymbol { name: "person.fill.questionmark" };
pub const PERSON_FILL_TURN_DOWN: SFSymbol = SFSymbol { name: "person.fill.turn.down" };
pub const PERSON_FILL_TURN_LEFT: SFSymbol = SFSymbol { name: "person.fill.turn.left" };
pub const PERSON_FILL_TURN_RIGHT: SFSymbol = SFSymbol { name: "person.fill.turn.right" };
pub const PERSON_FILL_VIEWFINDER: SFSymbol = SFSymbol { name: "person.fill.viewfinder" };
pub const PERSON_FILL_XMARK: SFSymbol = SFSymbol { name: "person.fill.xmark" };
pub const PERSON_ICLOUD: SFSymbol = SFSymbol { name: "person.icloud" };
pub const PERSON_ICLOUD_FILL: SFSymbol = SFSymbol { name: "person.icloud.fill" };
pub const PERSON_LINE_DOTTED_PERSON: SFSymbol = SFSymbol { name: "person.line.dotted.person" };
pub const PERSON_LINE_DOTTED_PERSON_FILL: SFSymbol = SFSymbol { name: "person.line.dotted.person.fill" };
pub const PERSON_TEXT_RECTANGLE: SFSymbol = SFSymbol { name: "person.text.rectangle" };
pub const PERSON_TEXT_RECTANGLE_FILL: SFSymbol = SFSymbol { name: "person.text.rectangle.fill" };
pub const PERSON_WAVE_2: SFSymbol = SFSymbol { name: "person.wave.2" };
pub const PERSON_WAVE_2_FILL: SFSymbol = SFSymbol { name: "person.wave.2.fill" };
pub const PERSONALHOTSPOT: SFSymbol = SFSymbol { name: "personalhotspot" };
pub const PERSONALHOTSPOT_CIRCLE: SFSymbol = SFSymbol { name: "personalhotspot.circle" };
pub const PERSONALHOTSPOT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "personalhotspot.circle.fill" };
pub const PERSPECTIVE: SFSymbol = SFSymbol { name: "perspective" };
pub const PESETASIGN: SFSymbol = SFSymbol { name: "pesetasign" };
pub const PESETASIGN_CIRCLE: SFSymbol = SFSymbol { name: "pesetasign.circle" };
pub const PESETASIGN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "pesetasign.circle.fill" };
pub const PESETASIGN_SQUARE: SFSymbol = SFSymbol { name: "pesetasign.square" };
pub const PESETASIGN_SQUARE_FILL: SFSymbol = SFSymbol { name: "pesetasign.square.fill" };
pub const PESOSIGN: SFSymbol = SFSymbol { name: "pesosign" };
pub const PESOSIGN_CIRCLE: SFSymbol = SFSymbol { name: "pesosign.circle" };
pub const PESOSIGN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "pesosign.circle.fill" };
pub const PESOSIGN_SQUARE: SFSymbol = SFSymbol { name: "pesosign.square" };
pub const PESOSIGN_SQUARE_FILL: SFSymbol = SFSymbol { name: "pesosign.square.fill" };
pub const PHONE: SFSymbol = SFSymbol { name: "phone" };
pub const PHONE_AND_WAVEFORM: SFSymbol = SFSymbol { name: "phone.and.waveform" };
pub const PHONE_AND_WAVEFORM_FILL: SFSymbol = SFSymbol { name: "phone.and.waveform.fill" };
pub const PHONE_ARROW_DOWN_LEFT: SFSymbol = SFSymbol { name: "phone.arrow.down.left" };
pub const PHONE_ARROW_DOWN_LEFT_FILL: SFSymbol = SFSymbol { name: "phone.arrow.down.left.fill" };
pub const PHONE_ARROW_RIGHT: SFSymbol = SFSymbol { name: "phone.arrow.right" };
pub const PHONE_ARROW_RIGHT_FILL: SFSymbol = SFSymbol { name: "phone.arrow.right.fill" };
pub const PHONE_ARROW_UP_RIGHT: SFSymbol = SFSymbol { name: "phone.arrow.up.right" };
pub const PHONE_ARROW_UP_RIGHT_CIRCLE: SFSymbol = SFSymbol { name: "phone.arrow.up.right.circle" };
pub const PHONE_ARROW_UP_RIGHT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "phone.arrow.up.right.circle.fill" };
pub const PHONE_ARROW_UP_RIGHT_FILL: SFSymbol = SFSymbol { name: "phone.arrow.up.right.fill" };
pub const PHONE_BADGE_CHECKMARK: SFSymbol = SFSymbol { name: "phone.badge.checkmark" };
pub const PHONE_BADGE_PLUS: SFSymbol = SFSymbol { name: "phone.badge.plus" };
pub const PHONE_BUBBLE_LEFT: SFSymbol = SFSymbol { name: "phone.bubble.left" };
pub const PHONE_BUBBLE_LEFT_FILL: SFSymbol = SFSymbol { name: "phone.bubble.left.fill" };
pub const PHONE_CIRCLE: SFSymbol = SFSymbol { name: "phone.circle" };
pub const PHONE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "phone.circle.fill" };
pub const PHONE_CONNECTION: SFSymbol = SFSymbol { name: "phone.connection" };
pub const PHONE_CONNECTION_FILL: SFSymbol = SFSymbol { name: "phone.connection.fill" };
pub const PHONE_DOWN: SFSymbol = SFSymbol { name: "phone.down" };
pub const PHONE_DOWN_CIRCLE: SFSymbol = SFSymbol { name: "phone.down.circle" };
pub const PHONE_DOWN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "phone.down.circle.fill" };
pub const PHONE_DOWN_FILL: SFSymbol = SFSymbol { name: "phone.down.fill" };
pub const PHONE_DOWN_WAVES_LEFT_AND_RIGHT: SFSymbol = SFSymbol { name: "phone.down.waves.left.and.right" };
pub const PHONE_FILL: SFSymbol = SFSymbol { name: "phone.fill" };
pub const PHONE_FILL_ARROW_DOWN_LEFT: SFSymbol = SFSymbol { name: "phone.fill.arrow.down.left" };
pub const PHONE_FILL_ARROW_RIGHT: SFSymbol = SFSymbol { name: "phone.fill.arrow.right" };
pub const PHONE_FILL_ARROW_UP_RIGHT: SFSymbol = SFSymbol { name: "phone.fill.arrow.up.right" };
pub const PHONE_FILL_BADGE_CHECKMARK: SFSymbol = SFSymbol { name: "phone.fill.badge.checkmark" };
pub const PHONE_FILL_BADGE_PLUS: SFSymbol = SFSymbol { name: "phone.fill.badge.plus" };
pub const PHONE_FILL_CONNECTION: SFSymbol = SFSymbol { name: "phone.fill.connection" };
pub const PHOTO: SFSymbol = SFSymbol { name: "photo" };
pub const PHOTO_ARTFRAME: SFSymbol = SFSymbol { name: "photo.artframe" };
pub const PHOTO_CIRCLE: SFSymbol = SFSymbol { name: "photo.circle" };
pub const PHOTO_CIRCLE_FILL: SFSymbol = SFSymbol { name: "photo.circle.fill" };
pub const PHOTO_FILL: SFSymbol = SFSymbol { name: "photo.fill" };
pub const PHOTO_FILL_ON_RECTANGLE_FILL: SFSymbol = SFSymbol { name: "photo.fill.on.rectangle.fill" };
pub const PHOTO_ON_RECTANGLE: SFSymbol = SFSymbol { name: "photo.on.rectangle" };
pub const PHOTO_ON_RECTANGLE_ANGLED: SFSymbol = SFSymbol { name: "photo.on.rectangle.angled" };
pub const PHOTO_STACK: SFSymbol = SFSymbol { name: "photo.stack" };
pub const PHOTO_STACK_FILL: SFSymbol = SFSymbol { name: "photo.stack.fill" };
pub const PHOTO_TV: SFSymbol = SFSymbol { name: "photo.tv" };
pub const PIANOKEYS: SFSymbol = SFSymbol { name: "pianokeys" };
pub const PIANOKEYS_INVERSE: SFSymbol = SFSymbol { name: "pianokeys.inverse" };
pub const PILL: SFSymbol = SFSymbol { name: "pill" };
pub const PILL_CIRCLE: SFSymbol = SFSymbol { name: "pill.circle" };
pub const PILL_CIRCLE_FILL: SFSymbol = SFSymbol { name: "pill.circle.fill" };
pub const PILL_FILL: SFSymbol = SFSymbol { name: "pill.fill" };
pub const PILLS: SFSymbol = SFSymbol { name: "pills" };
pub const PILLS_CIRCLE: SFSymbol = SFSymbol { name: "pills.circle" };
pub const PILLS_CIRCLE_FILL: SFSymbol = SFSymbol { name: "pills.circle.fill" };
pub const PILLS_FILL: SFSymbol = SFSymbol { name: "pills.fill" };
pub const PIN: SFSymbol = SFSymbol { name: "pin" };
pub const PIN_CIRCLE: SFSymbol = SFSymbol { name: "pin.circle" };
pub const PIN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "pin.circle.fill" };
pub const PIN_FILL: SFSymbol = SFSymbol { name: "pin.fill" };
pub const PIN_SLASH: SFSymbol = SFSymbol { name: "pin.slash" };
pub const PIN_SLASH_FILL: SFSymbol = SFSymbol { name: "pin.slash.fill" };
pub const PIN_SQUARE: SFSymbol = SFSymbol { name: "pin.square" };
pub const PIN_SQUARE_FILL: SFSymbol = SFSymbol { name: "pin.square.fill" };
pub const PIP: SFSymbol = SFSymbol { name: "pip" };
pub const PIP_ENTER: SFSymbol = SFSymbol { name: "pip.enter" };
pub const PIP_EXIT: SFSymbol = SFSymbol { name: "pip.exit" };
pub const PIP_FILL: SFSymbol = SFSymbol { name: "pip.fill" };
pub const PIP_REMOVE: SFSymbol = SFSymbol { name: "pip.remove" };
pub const PIP_SWAP: SFSymbol = SFSymbol { name: "pip.swap" };
pub const PIPE_AND_DROP: SFSymbol = SFSymbol { name: "pipe.and.drop" };
pub const PIPE_AND_DROP_FILL: SFSymbol = SFSymbol { name: "pipe.and.drop.fill" };
pub const PLACEHOLDERTEXT_FILL: SFSymbol = SFSymbol { name: "placeholdertext.fill" };
pub const PLATTER_2_FILLED_IPAD: SFSymbol = SFSymbol { name: "platter.2.filled.ipad" };
pub const PLATTER_2_FILLED_IPAD_LANDSCAPE: SFSymbol = SFSymbol { name: "platter.2.filled.ipad.landscape" };
pub const PLATTER_2_FILLED_IPHONE: SFSymbol = SFSymbol { name: "platter.2.filled.iphone" };
pub const PLATTER_2_FILLED_IPHONE_LANDSCAPE: SFSymbol = SFSymbol { name: "platter.2.filled.iphone.landscape" };
pub const PLATTER_BOTTOM_APPLEWATCH_CASE: SFSymbol = SFSymbol { name: "platter.bottom.applewatch.case" };
pub const PLATTER_FILLED_BOTTOM_AND_ARROW_DOWN_IPHONE: SFSymbol = SFSymbol { name: "platter.filled.bottom.and.arrow.down.iphone" };
pub const PLATTER_FILLED_BOTTOM_APPLEWATCH_CASE: SFSymbol = SFSymbol { name: "platter.filled.bottom.applewatch.case" };
pub const PLATTER_FILLED_BOTTOM_IPHONE: SFSymbol = SFSymbol { name: "platter.filled.bottom.iphone" };
pub const PLATTER_FILLED_TOP_AND_ARROW_UP_IPHONE: SFSymbol = SFSymbol { name: "platter.filled.top.and.arrow.up.iphone" };
pub const PLATTER_FILLED_TOP_APPLEWATCH_CASE: SFSymbol = SFSymbol { name: "platter.filled.top.applewatch.case" };
pub const PLATTER_FILLED_TOP_IPHONE: SFSymbol = SFSymbol { name: "platter.filled.top.iphone" };
pub const PLATTER_TOP_APPLEWATCH_CASE: SFSymbol = SFSymbol { name: "platter.top.applewatch.case" };
pub const PLAY: SFSymbol = SFSymbol { name: "play" };
pub const PLAY_CIRCLE: SFSymbol = SFSymbol { name: "play.circle" };
pub const PLAY_CIRCLE_FILL: SFSymbol = SFSymbol { name: "play.circle.fill" };
pub const PLAY_DESKTOPCOMPUTER: SFSymbol = SFSymbol { name: "play.desktopcomputer" };
pub const PLAY_DISPLAY: SFSymbol = SFSymbol { name: "play.display" };
pub const PLAY_FILL: SFSymbol = SFSymbol { name: "play.fill" };
pub const PLAY_LAPTOPCOMPUTER: SFSymbol = SFSymbol { name: "play.laptopcomputer" };
pub const PLAY_RECTANGLE: SFSymbol = SFSymbol { name: "play.rectangle" };
pub const PLAY_RECTANGLE_FILL: SFSymbol = SFSymbol { name: "play.rectangle.fill" };
pub const PLAY_RECTANGLE_ON_RECTANGLE: SFSymbol = SFSymbol { name: "play.rectangle.on.rectangle" };
pub const PLAY_RECTANGLE_ON_RECTANGLE_CIRCLE: SFSymbol = SFSymbol { name: "play.rectangle.on.rectangle.circle" };
pub const PLAY_RECTANGLE_ON_RECTANGLE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "play.rectangle.on.rectangle.circle.fill" };
pub const PLAY_RECTANGLE_ON_RECTANGLE_FILL: SFSymbol = SFSymbol { name: "play.rectangle.on.rectangle.fill" };
pub const PLAY_SLASH: SFSymbol = SFSymbol { name: "play.slash" };
pub const PLAY_SLASH_FILL: SFSymbol = SFSymbol { name: "play.slash.fill" };
pub const PLAY_SQUARE: SFSymbol = SFSymbol { name: "play.square" };
pub const PLAY_SQUARE_FILL: SFSymbol = SFSymbol { name: "play.square.fill" };
pub const PLAY_TV: SFSymbol = SFSymbol { name: "play.tv" };
pub const PLAY_TV_FILL: SFSymbol = SFSymbol { name: "play.tv.fill" };
pub const PLAYPAUSE: SFSymbol = SFSymbol { name: "playpause" };
pub const PLAYPAUSE_CIRCLE: SFSymbol = SFSymbol { name: "playpause.circle" };
pub const PLAYPAUSE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "playpause.circle.fill" };
pub const PLAYPAUSE_FILL: SFSymbol = SFSymbol { name: "playpause.fill" };
pub const PLAYSTATION_LOGO: SFSymbol = SFSymbol { name: "playstation.logo" };
pub const PLUS: SFSymbol = SFSymbol { name: "plus" };
pub const PLUS_APP: SFSymbol = SFSymbol { name: "plus.app" };
pub const PLUS_APP_FILL: SFSymbol = SFSymbol { name: "plus.app.fill" };
pub const PLUS_BUBBLE: SFSymbol = SFSymbol { name: "plus.bubble" };
pub const PLUS_BUBBLE_FILL: SFSymbol = SFSymbol { name: "plus.bubble.fill" };
pub const PLUS_CIRCLE: SFSymbol = SFSymbol { name: "plus.circle" };
pub const PLUS_CIRCLE_FILL: SFSymbol = SFSymbol { name: "plus.circle.fill" };
pub const PLUS_DIAMOND: SFSymbol = SFSymbol { name: "plus.diamond" };
pub const PLUS_DIAMOND_FILL: SFSymbol = SFSymbol { name: "plus.diamond.fill" };
pub const PLUS_FORWARDSLASH_MINUS: SFSymbol = SFSymbol { name: "plus.forwardslash.minus" };
pub const PLUS_MAGNIFYINGGLASS: SFSymbol = SFSymbol { name: "plus.magnifyingglass" };
pub const PLUS_MESSAGE: SFSymbol = SFSymbol { name: "plus.message" };
pub const PLUS_MESSAGE_FILL: SFSymbol = SFSymbol { name: "plus.message.fill" };
pub const PLUS_RECTANGLE: SFSymbol = SFSymbol { name: "plus.rectangle" };
pub const PLUS_RECTANGLE_FILL: SFSymbol = SFSymbol { name: "plus.rectangle.fill" };
pub const PLUS_RECTANGLE_FILL_ON_RECTANGLE_FILL: SFSymbol = SFSymbol { name: "plus.rectangle.fill.on.rectangle.fill" };
pub const PLUS_RECTANGLE_ON_FOLDER: SFSymbol = SFSymbol { name: "plus.rectangle.on.folder" };
pub const PLUS_RECTANGLE_ON_FOLDER_FILL: SFSymbol = SFSymbol { name: "plus.rectangle.on.folder.fill" };
pub const PLUS_RECTANGLE_ON_RECTANGLE: SFSymbol = SFSymbol { name: "plus.rectangle.on.rectangle" };
pub const PLUS_RECTANGLE_PORTRAIT: SFSymbol = SFSymbol { name: "plus.rectangle.portrait" };
pub const PLUS_RECTANGLE_PORTRAIT_FILL: SFSymbol = SFSymbol { name: "plus.rectangle.portrait.fill" };
pub const PLUS_SQUARE: SFSymbol = SFSymbol { name: "plus.square" };
pub const PLUS_SQUARE_DASHED: SFSymbol = SFSymbol { name: "plus.square.dashed" };
pub const PLUS_SQUARE_FILL: SFSymbol = SFSymbol { name: "plus.square.fill" };
pub const PLUS_SQUARE_FILL_ON_SQUARE_FILL: SFSymbol = SFSymbol { name: "plus.square.fill.on.square.fill" };
pub const PLUS_SQUARE_ON_SQUARE: SFSymbol = SFSymbol { name: "plus.square.on.square" };
pub const PLUS_VIEWFINDER: SFSymbol = SFSymbol { name: "plus.viewfinder" };
pub const PLUSMINUS: SFSymbol = SFSymbol { name: "plusminus" };
pub const PLUSMINUS_CIRCLE: SFSymbol = SFSymbol { name: "plusminus.circle" };
pub const PLUSMINUS_CIRCLE_FILL: SFSymbol = SFSymbol { name: "plusminus.circle.fill" };
pub const POINT_3_CONNECTED_TRIANGLEPATH_DOTTED: SFSymbol = SFSymbol { name: "point.3.connected.trianglepath.dotted" };
pub const POINT_3_FILLED_CONNECTED_TRIANGLEPATH_DOTTED: SFSymbol = SFSymbol { name: "point.3.filled.connected.trianglepath.dotted" };
pub const POINT_FILLED_TOPLEFT_DOWN_CURVEDTO_POINT_BOTTOMRIGHT_UP: SFSymbol = SFSymbol { name: "point.filled.topleft.down.curvedto.point.bottomright.up" };
pub const POINT_TOPLEFT_DOWN_CURVEDTO_POINT_BOTTOMRIGHT_UP: SFSymbol = SFSymbol { name: "point.topleft.down.curvedto.point.bottomright.up" };
pub const POINT_TOPLEFT_DOWN_CURVEDTO_POINT_BOTTOMRIGHT_UP_FILL: SFSymbol = SFSymbol { name: "point.topleft.down.curvedto.point.bottomright.up.fill" };
pub const POINT_TOPLEFT_DOWN_CURVEDTO_POINT_FILLED_BOTTOMRIGHT_UP: SFSymbol = SFSymbol { name: "point.topleft.down.curvedto.point.filled.bottomright.up" };
pub const POPCORN: SFSymbol = SFSymbol { name: "popcorn" };
pub const POPCORN_CIRCLE: SFSymbol = SFSymbol { name: "popcorn.circle" };
pub const POPCORN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "popcorn.circle.fill" };
pub const POPCORN_FILL: SFSymbol = SFSymbol { name: "popcorn.fill" };
pub const POWER: SFSymbol = SFSymbol { name: "power" };
pub const POWER_CIRCLE: SFSymbol = SFSymbol { name: "power.circle" };
pub const POWER_CIRCLE_FILL: SFSymbol = SFSymbol { name: "power.circle.fill" };
pub const POWER_DOTTED: SFSymbol = SFSymbol { name: "power.dotted" };
pub const POWEROFF: SFSymbol = SFSymbol { name: "poweroff" };
pub const POWERON: SFSymbol = SFSymbol { name: "poweron" };
pub const POWEROUTLET_STRIP: SFSymbol = SFSymbol { name: "poweroutlet.strip" };
pub const POWEROUTLET_STRIP_FILL: SFSymbol = SFSymbol { name: "poweroutlet.strip.fill" };
pub const POWEROUTLET_TYPE_A: SFSymbol = SFSymbol { name: "poweroutlet.type.a" };
pub const POWEROUTLET_TYPE_A_FILL: SFSymbol = SFSymbol { name: "poweroutlet.type.a.fill" };
pub const POWEROUTLET_TYPE_A_SQUARE: SFSymbol = SFSymbol { name: "poweroutlet.type.a.square" };
pub const POWEROUTLET_TYPE_A_SQUARE_FILL: SFSymbol = SFSymbol { name: "poweroutlet.type.a.square.fill" };
pub const POWEROUTLET_TYPE_B: SFSymbol = SFSymbol { name: "poweroutlet.type.b" };
pub const POWEROUTLET_TYPE_B_FILL: SFSymbol = SFSymbol { name: "poweroutlet.type.b.fill" };
pub const POWEROUTLET_TYPE_B_SQUARE: SFSymbol = SFSymbol { name: "poweroutlet.type.b.square" };
pub const POWEROUTLET_TYPE_B_SQUARE_FILL: SFSymbol = SFSymbol { name: "poweroutlet.type.b.square.fill" };
pub const POWEROUTLET_TYPE_C: SFSymbol = SFSymbol { name: "poweroutlet.type.c" };
pub const POWEROUTLET_TYPE_C_FILL: SFSymbol = SFSymbol { name: "poweroutlet.type.c.fill" };
pub const POWEROUTLET_TYPE_C_SQUARE: SFSymbol = SFSymbol { name: "poweroutlet.type.c.square" };
pub const POWEROUTLET_TYPE_C_SQUARE_FILL: SFSymbol = SFSymbol { name: "poweroutlet.type.c.square.fill" };
pub const POWEROUTLET_TYPE_D: SFSymbol = SFSymbol { name: "poweroutlet.type.d" };
pub const POWEROUTLET_TYPE_D_FILL: SFSymbol = SFSymbol { name: "poweroutlet.type.d.fill" };
pub const POWEROUTLET_TYPE_D_SQUARE: SFSymbol = SFSymbol { name: "poweroutlet.type.d.square" };
pub const POWEROUTLET_TYPE_D_SQUARE_FILL: SFSymbol = SFSymbol { name: "poweroutlet.type.d.square.fill" };
pub const POWEROUTLET_TYPE_E: SFSymbol = SFSymbol { name: "poweroutlet.type.e" };
pub const POWEROUTLET_TYPE_E_FILL: SFSymbol = SFSymbol { name: "poweroutlet.type.e.fill" };
pub const POWEROUTLET_TYPE_E_SQUARE: SFSymbol = SFSymbol { name: "poweroutlet.type.e.square" };
pub const POWEROUTLET_TYPE_E_SQUARE_FILL: SFSymbol = SFSymbol { name: "poweroutlet.type.e.square.fill" };
pub const POWEROUTLET_TYPE_F: SFSymbol = SFSymbol { name: "poweroutlet.type.f" };
pub const POWEROUTLET_TYPE_F_FILL: SFSymbol = SFSymbol { name: "poweroutlet.type.f.fill" };
pub const POWEROUTLET_TYPE_F_SQUARE: SFSymbol = SFSymbol { name: "poweroutlet.type.f.square" };
pub const POWEROUTLET_TYPE_F_SQUARE_FILL: SFSymbol = SFSymbol { name: "poweroutlet.type.f.square.fill" };
pub const POWEROUTLET_TYPE_G: SFSymbol = SFSymbol { name: "poweroutlet.type.g" };
pub const POWEROUTLET_TYPE_G_FILL: SFSymbol = SFSymbol { name: "poweroutlet.type.g.fill" };
pub const POWEROUTLET_TYPE_G_SQUARE: SFSymbol = SFSymbol { name: "poweroutlet.type.g.square" };
pub const POWEROUTLET_TYPE_G_SQUARE_FILL: SFSymbol = SFSymbol { name: "poweroutlet.type.g.square.fill" };
pub const POWEROUTLET_TYPE_H: SFSymbol = SFSymbol { name: "poweroutlet.type.h" };
pub const POWEROUTLET_TYPE_H_FILL: SFSymbol = SFSymbol { name: "poweroutlet.type.h.fill" };
pub const POWEROUTLET_TYPE_H_SQUARE: SFSymbol = SFSymbol { name: "poweroutlet.type.h.square" };
pub const POWEROUTLET_TYPE_H_SQUARE_FILL: SFSymbol = SFSymbol { name: "poweroutlet.type.h.square.fill" };
pub const POWEROUTLET_TYPE_I: SFSymbol = SFSymbol { name: "poweroutlet.type.i" };
pub const POWEROUTLET_TYPE_I_FILL: SFSymbol = SFSymbol { name: "poweroutlet.type.i.fill" };
pub const POWEROUTLET_TYPE_I_SQUARE: SFSymbol = SFSymbol { name: "poweroutlet.type.i.square" };
pub const POWEROUTLET_TYPE_I_SQUARE_FILL: SFSymbol = SFSymbol { name: "poweroutlet.type.i.square.fill" };
pub const POWEROUTLET_TYPE_J: SFSymbol = SFSymbol { name: "poweroutlet.type.j" };
pub const POWEROUTLET_TYPE_J_FILL: SFSymbol = SFSymbol { name: "poweroutlet.type.j.fill" };
pub const POWEROUTLET_TYPE_J_SQUARE: SFSymbol = SFSymbol { name: "poweroutlet.type.j.square" };
pub const POWEROUTLET_TYPE_J_SQUARE_FILL: SFSymbol = SFSymbol { name: "poweroutlet.type.j.square.fill" };
pub const POWEROUTLET_TYPE_K: SFSymbol = SFSymbol { name: "poweroutlet.type.k" };
pub const POWEROUTLET_TYPE_K_FILL: SFSymbol = SFSymbol { name: "poweroutlet.type.k.fill" };
pub const POWEROUTLET_TYPE_K_SQUARE: SFSymbol = SFSymbol { name: "poweroutlet.type.k.square" };
pub const POWEROUTLET_TYPE_K_SQUARE_FILL: SFSymbol = SFSymbol { name: "poweroutlet.type.k.square.fill" };
pub const POWEROUTLET_TYPE_L: SFSymbol = SFSymbol { name: "poweroutlet.type.l" };
pub const POWEROUTLET_TYPE_L_FILL: SFSymbol = SFSymbol { name: "poweroutlet.type.l.fill" };
pub const POWEROUTLET_TYPE_L_SQUARE: SFSymbol = SFSymbol { name: "poweroutlet.type.l.square" };
pub const POWEROUTLET_TYPE_L_SQUARE_FILL: SFSymbol = SFSymbol { name: "poweroutlet.type.l.square.fill" };
pub const POWEROUTLET_TYPE_M: SFSymbol = SFSymbol { name: "poweroutlet.type.m" };
pub const POWEROUTLET_TYPE_M_FILL: SFSymbol = SFSymbol { name: "poweroutlet.type.m.fill" };
pub const POWEROUTLET_TYPE_M_SQUARE: SFSymbol = SFSymbol { name: "poweroutlet.type.m.square" };
pub const POWEROUTLET_TYPE_M_SQUARE_FILL: SFSymbol = SFSymbol { name: "poweroutlet.type.m.square.fill" };
pub const POWEROUTLET_TYPE_N: SFSymbol = SFSymbol { name: "poweroutlet.type.n" };
pub const POWEROUTLET_TYPE_N_FILL: SFSymbol = SFSymbol { name: "poweroutlet.type.n.fill" };
pub const POWEROUTLET_TYPE_N_SQUARE: SFSymbol = SFSymbol { name: "poweroutlet.type.n.square" };
pub const POWEROUTLET_TYPE_N_SQUARE_FILL: SFSymbol = SFSymbol { name: "poweroutlet.type.n.square.fill" };
pub const POWEROUTLET_TYPE_O: SFSymbol = SFSymbol { name: "poweroutlet.type.o" };
pub const POWEROUTLET_TYPE_O_FILL: SFSymbol = SFSymbol { name: "poweroutlet.type.o.fill" };
pub const POWEROUTLET_TYPE_O_SQUARE: SFSymbol = SFSymbol { name: "poweroutlet.type.o.square" };
pub const POWEROUTLET_TYPE_O_SQUARE_FILL: SFSymbol = SFSymbol { name: "poweroutlet.type.o.square.fill" };
pub const POWERPLUG: SFSymbol = SFSymbol { name: "powerplug" };
pub const POWERPLUG_FILL: SFSymbol = SFSymbol { name: "powerplug.fill" };
pub const POWERSLEEP: SFSymbol = SFSymbol { name: "powersleep" };
pub const PRINTER: SFSymbol = SFSymbol { name: "printer" };
pub const PRINTER_DOTMATRIX: SFSymbol = SFSymbol { name: "printer.dotmatrix" };
pub const PRINTER_DOTMATRIX_FILL: SFSymbol = SFSymbol { name: "printer.dotmatrix.fill" };
pub const PRINTER_DOTMATRIX_FILLED_AND_PAPER: SFSymbol = SFSymbol { name: "printer.dotmatrix.filled.and.paper" };
pub const PRINTER_FILL: SFSymbol = SFSymbol { name: "printer.fill" };
pub const PRINTER_FILLED_AND_PAPER: SFSymbol = SFSymbol { name: "printer.filled.and.paper" };
pub const PROJECTIVE: SFSymbol = SFSymbol { name: "projective" };
pub const PURCHASED: SFSymbol = SFSymbol { name: "purchased" };
pub const PURCHASED_CIRCLE: SFSymbol = SFSymbol { name: "purchased.circle" };
pub const PURCHASED_CIRCLE_FILL: SFSymbol = SFSymbol { name: "purchased.circle.fill" };
pub const PUZZLEPIECE: SFSymbol = SFSymbol { name: "puzzlepiece" };
pub const PUZZLEPIECE_EXTENSION: SFSymbol = SFSymbol { name: "puzzlepiece.extension" };
pub const PUZZLEPIECE_EXTENSION_FILL: SFSymbol = SFSymbol { name: "puzzlepiece.extension.fill" };
pub const PUZZLEPIECE_FILL: SFSymbol = SFSymbol { name: "puzzlepiece.fill" };
pub const PYRAMID: SFSymbol = SFSymbol { name: "pyramid" };
pub const PYRAMID_FILL: SFSymbol = SFSymbol { name: "pyramid.fill" };
pub const Q_CIRCLE: SFSymbol = SFSymbol { name: "q.circle" };
pub const Q_CIRCLE_FILL: SFSymbol = SFSymbol { name: "q.circle.fill" };
pub const Q_SQUARE: SFSymbol = SFSymbol { name: "q.square" };
pub const Q_SQUARE_FILL: SFSymbol = SFSymbol { name: "q.square.fill" };
pub const QRCODE: SFSymbol = SFSymbol { name: "qrcode" };
pub const QRCODE_VIEWFINDER: SFSymbol = SFSymbol { name: "qrcode.viewfinder" };
pub const QUESTIONMARK: SFSymbol = SFSymbol { name: "questionmark" };
pub const QUESTIONMARK_APP: SFSymbol = SFSymbol { name: "questionmark.app" };
pub const QUESTIONMARK_APP_DASHED: SFSymbol = SFSymbol { name: "questionmark.app.dashed" };
pub const QUESTIONMARK_APP_FILL: SFSymbol = SFSymbol { name: "questionmark.app.fill" };
pub const QUESTIONMARK_BUBBLE: SFSymbol = SFSymbol { name: "questionmark.bubble" };
pub const QUESTIONMARK_BUBBLE_FILL: SFSymbol = SFSymbol { name: "questionmark.bubble.fill" };
pub const QUESTIONMARK_CIRCLE: SFSymbol = SFSymbol { name: "questionmark.circle" };
pub const QUESTIONMARK_CIRCLE_FILL: SFSymbol = SFSymbol { name: "questionmark.circle.fill" };
pub const QUESTIONMARK_DIAMOND: SFSymbol = SFSymbol { name: "questionmark.diamond" };
pub const QUESTIONMARK_DIAMOND_FILL: SFSymbol = SFSymbol { name: "questionmark.diamond.fill" };
pub const QUESTIONMARK_FOLDER: SFSymbol = SFSymbol { name: "questionmark.folder" };
pub const QUESTIONMARK_FOLDER_FILL: SFSymbol = SFSymbol { name: "questionmark.folder.fill" };
pub const QUESTIONMARK_SQUARE: SFSymbol = SFSymbol { name: "questionmark.square" };
pub const QUESTIONMARK_SQUARE_DASHED: SFSymbol = SFSymbol { name: "questionmark.square.dashed" };
pub const QUESTIONMARK_SQUARE_FILL: SFSymbol = SFSymbol { name: "questionmark.square.fill" };
pub const QUESTIONMARK_VIDEO: SFSymbol = SFSymbol { name: "questionmark.video" };
pub const QUESTIONMARK_VIDEO_FILL: SFSymbol = SFSymbol { name: "questionmark.video.fill" };
pub const QUOTE_BUBBLE: SFSymbol = SFSymbol { name: "quote.bubble" };
pub const QUOTE_BUBBLE_FILL: SFSymbol = SFSymbol { name: "quote.bubble.fill" };
pub const QUOTE_CLOSING: SFSymbol = SFSymbol { name: "quote.closing" };
pub const QUOTE_OPENING: SFSymbol = SFSymbol { name: "quote.opening" };
pub const QUOTELEVEL: SFSymbol = SFSymbol { name: "quotelevel" };
pub const R_CIRCLE: SFSymbol = SFSymbol { name: "r.circle" };
pub const R_CIRCLE_FILL: SFSymbol = SFSymbol { name: "r.circle.fill" };
pub const R_JOYSTICK: SFSymbol = SFSymbol { name: "r.joystick" };
pub const R_JOYSTICK_FILL: SFSymbol = SFSymbol { name: "r.joystick.fill" };
pub const R_JOYSTICK_PRESS_DOWN: SFSymbol = SFSymbol { name: "r.joystick.press.down" };
pub const R_JOYSTICK_PRESS_DOWN_FILL: SFSymbol = SFSymbol { name: "r.joystick.press.down.fill" };
pub const R_JOYSTICK_TILT_DOWN: SFSymbol = SFSymbol { name: "r.joystick.tilt.down" };
pub const R_JOYSTICK_TILT_DOWN_FILL: SFSymbol = SFSymbol { name: "r.joystick.tilt.down.fill" };
pub const R_JOYSTICK_TILT_LEFT: SFSymbol = SFSymbol { name: "r.joystick.tilt.left" };
pub const R_JOYSTICK_TILT_LEFT_FILL: SFSymbol = SFSymbol { name: "r.joystick.tilt.left.fill" };
pub const R_JOYSTICK_TILT_RIGHT: SFSymbol = SFSymbol { name: "r.joystick.tilt.right" };
pub const R_JOYSTICK_TILT_RIGHT_FILL: SFSymbol = SFSymbol { name: "r.joystick.tilt.right.fill" };
pub const R_JOYSTICK_TILT_UP: SFSymbol = SFSymbol { name: "r.joystick.tilt.up" };
pub const R_JOYSTICK_TILT_UP_FILL: SFSymbol = SFSymbol { name: "r.joystick.tilt.up.fill" };
pub const R_RECTANGLE_ROUNDEDBOTTOM: SFSymbol = SFSymbol { name: "r.rectangle.roundedbottom" };
pub const R_RECTANGLE_ROUNDEDBOTTOM_FILL: SFSymbol = SFSymbol { name: "r.rectangle.roundedbottom.fill" };
pub const R_SQUARE: SFSymbol = SFSymbol { name: "r.square" };
pub const R_SQUARE_FILL: SFSymbol = SFSymbol { name: "r.square.fill" };
pub const R_SQUARE_ON_SQUARE: SFSymbol = SFSymbol { name: "r.square.on.square" };
pub const R_SQUARE_ON_SQUARE_FILL: SFSymbol = SFSymbol { name: "r.square.on.square.fill" };
pub const R1_RECTANGLE_ROUNDEDBOTTOM: SFSymbol = SFSymbol { name: "r1.rectangle.roundedbottom" };
pub const R1_RECTANGLE_ROUNDEDBOTTOM_FILL: SFSymbol = SFSymbol { name: "r1.rectangle.roundedbottom.fill" };
pub const R2_RECTANGLE_ROUNDEDTOP: SFSymbol = SFSymbol { name: "r2.rectangle.roundedtop" };
pub const R2_RECTANGLE_ROUNDEDTOP_FILL: SFSymbol = SFSymbol { name: "r2.rectangle.roundedtop.fill" };
pub const RADIO: SFSymbol = SFSymbol { name: "radio" };
pub const RADIO_FILL: SFSymbol = SFSymbol { name: "radio.fill" };
pub const RAYS: SFSymbol = SFSymbol { name: "rays" };
pub const RB_RECTANGLE_ROUNDEDBOTTOM: SFSymbol = SFSymbol { name: "rb.rectangle.roundedbottom" };
pub const RB_RECTANGLE_ROUNDEDBOTTOM_FILL: SFSymbol = SFSymbol { name: "rb.rectangle.roundedbottom.fill" };
pub const RECORD_CIRCLE: SFSymbol = SFSymbol { name: "record.circle" };
pub const RECORD_CIRCLE_FILL: SFSymbol = SFSymbol { name: "record.circle.fill" };
pub const RECORDINGTAPE: SFSymbol = SFSymbol { name: "recordingtape" };
pub const RECORDINGTAPE_CIRCLE: SFSymbol = SFSymbol { name: "recordingtape.circle" };
pub const RECORDINGTAPE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "recordingtape.circle.fill" };
pub const RECTANGLE: SFSymbol = SFSymbol { name: "rectangle" };
pub const RECTANGLE_2_SWAP: SFSymbol = SFSymbol { name: "rectangle.2.swap" };
pub const RECTANGLE_3_GROUP: SFSymbol = SFSymbol { name: "rectangle.3.group" };
pub const RECTANGLE_3_GROUP_BUBBLE_LEFT: SFSymbol = SFSymbol { name: "rectangle.3.group.bubble.left" };
pub const RECTANGLE_3_GROUP_BUBBLE_LEFT_FILL: SFSymbol = SFSymbol { name: "rectangle.3.group.bubble.left.fill" };
pub const RECTANGLE_3_GROUP_FILL: SFSymbol = SFSymbol { name: "rectangle.3.group.fill" };
pub const RECTANGLE_AND_ARROW_UP_RIGHT_AND_ARROW_DOWN_LEFT: SFSymbol = SFSymbol { name: "rectangle.and.arrow.up.right.and.arrow.down.left" };
pub const RECTANGLE_AND_ARROW_UP_RIGHT_AND_ARROW_DOWN_LEFT_SLASH: SFSymbol = SFSymbol { name: "rectangle.and.arrow.up.right.and.arrow.down.left.slash" };
pub const RECTANGLE_AND_HAND_POINT_UP_LEFT: SFSymbol = SFSymbol { name: "rectangle.and.hand.point.up.left" };
pub const RECTANGLE_AND_HAND_POINT_UP_LEFT_FILL: SFSymbol = SFSymbol { name: "rectangle.and.hand.point.up.left.fill" };
pub const RECTANGLE_AND_HAND_POINT_UP_LEFT_FILLED: SFSymbol = SFSymbol { name: "rectangle.and.hand.point.up.left.filled" };
pub const RECTANGLE_AND_PAPERCLIP: SFSymbol = SFSymbol { name: "rectangle.and.paperclip" };
pub const RECTANGLE_AND_PENCIL_AND_ELLIPSIS: SFSymbol = SFSymbol { name: "rectangle.and.pencil.and.ellipsis" };
pub const RECTANGLE_AND_TEXT_MAGNIFYINGGLASS: SFSymbol = SFSymbol { name: "rectangle.and.text.magnifyingglass" };
pub const RECTANGLE_ARROWTRIANGLE_2_INWARD: SFSymbol = SFSymbol { name: "rectangle.arrowtriangle.2.inward" };
pub const RECTANGLE_ARROWTRIANGLE_2_OUTWARD: SFSymbol = SFSymbol { name: "rectangle.arrowtriangle.2.outward" };
pub const RECTANGLE_BADGE_CHECKMARK: SFSymbol = SFSymbol { name: "rectangle.badge.checkmark" };
pub const RECTANGLE_BADGE_MINUS: SFSymbol = SFSymbol { name: "rectangle.badge.minus" };
pub const RECTANGLE_BADGE_PERSON_CROP: SFSymbol = SFSymbol { name: "rectangle.badge.person.crop" };
pub const RECTANGLE_BADGE_PLUS: SFSymbol = SFSymbol { name: "rectangle.badge.plus" };
pub const RECTANGLE_BADGE_XMARK: SFSymbol = SFSymbol { name: "rectangle.badge.xmark" };
pub const RECTANGLE_BOTTOMHALF_FILLED: SFSymbol = SFSymbol { name: "rectangle.bottomhalf.filled" };
pub const RECTANGLE_BOTTOMHALF_INSET_FILLED: SFSymbol = SFSymbol { name: "rectangle.bottomhalf.inset.filled" };
pub const RECTANGLE_BOTTOMTHIRD_INSET_FILLED: SFSymbol = SFSymbol { name: "rectangle.bottomthird.inset.filled" };
pub const RECTANGLE_CENTER_INSET_FILLED: SFSymbol = SFSymbol { name: "rectangle.center.inset.filled" };
pub const RECTANGLE_CENTER_INSET_FILLED_BADGE_PLUS: SFSymbol = SFSymbol { name: "rectangle.center.inset.filled.badge.plus" };
pub const RECTANGLE_COMPRESS_VERTICAL: SFSymbol = SFSymbol { name: "rectangle.compress.vertical" };
pub const RECTANGLE_CONNECTED_TO_LINE_BELOW: SFSymbol = SFSymbol { name: "rectangle.connected.to.line.below" };
pub const RECTANGLE_DASHED: SFSymbol = SFSymbol { name: "rectangle.dashed" };
pub const RECTANGLE_DASHED_AND_PAPERCLIP: SFSymbol = SFSymbol { name: "rectangle.dashed.and.paperclip" };
pub const RECTANGLE_DASHED_BADGE_RECORD: SFSymbol = SFSymbol { name: "rectangle.dashed.badge.record" };
pub const RECTANGLE_EXPAND_VERTICAL: SFSymbol = SFSymbol { name: "rectangle.expand.vertical" };
pub const RECTANGLE_FILL: SFSymbol = SFSymbol { name: "rectangle.fill" };
pub const RECTANGLE_FILL_BADGE_CHECKMARK: SFSymbol = SFSymbol { name: "rectangle.fill.badge.checkmark" };
pub const RECTANGLE_FILL_BADGE_MINUS: SFSymbol = SFSymbol { name: "rectangle.fill.badge.minus" };
pub const RECTANGLE_FILL_BADGE_PERSON_CROP: SFSymbol = SFSymbol { name: "rectangle.fill.badge.person.crop" };
pub const RECTANGLE_FILL_BADGE_PLUS: SFSymbol = SFSymbol { name: "rectangle.fill.badge.plus" };
pub const RECTANGLE_FILL_BADGE_XMARK: SFSymbol = SFSymbol { name: "rectangle.fill.badge.xmark" };
pub const RECTANGLE_FILL_ON_RECTANGLE_ANGLED_FILL: SFSymbol = SFSymbol { name: "rectangle.fill.on.rectangle.angled.fill" };
pub const RECTANGLE_FILL_ON_RECTANGLE_FILL: SFSymbol = SFSymbol { name: "rectangle.fill.on.rectangle.fill" };
pub const RECTANGLE_FILLED_AND_HAND_POINT_UP_LEFT: SFSymbol = SFSymbol { name: "rectangle.filled.and.hand.point.up.left" };
pub const RECTANGLE_GRID_1X2: SFSymbol = SFSymbol { name: "rectangle.grid.1x2" };
pub const RECTANGLE_GRID_1X2_FILL: SFSymbol = SFSymbol { name: "rectangle.grid.1x2.fill" };
pub const RECTANGLE_GRID_2X2: SFSymbol = SFSymbol { name: "rectangle.grid.2x2" };
pub const RECTANGLE_GRID_2X2_FILL: SFSymbol = SFSymbol { name: "rectangle.grid.2x2.fill" };
pub const RECTANGLE_GRID_3X2: SFSymbol = SFSymbol { name: "rectangle.grid.3x2" };
pub const RECTANGLE_GRID_3X2_FILL: SFSymbol = SFSymbol { name: "rectangle.grid.3x2.fill" };
pub const RECTANGLE_INSET_BOTTOMLEADING_FILLED: SFSymbol = SFSymbol { name: "rectangle.inset.bottomleading.filled" };
pub const RECTANGLE_INSET_BOTTOMLEFT_FILLED: SFSymbol = SFSymbol { name: "rectangle.inset.bottomleft.filled" };
pub const RECTANGLE_INSET_BOTTOMRIGHT_FILLED: SFSymbol = SFSymbol { name: "rectangle.inset.bottomright.filled" };
pub const RECTANGLE_INSET_BOTTOMTRAILING_FILLED: SFSymbol = SFSymbol { name: "rectangle.inset.bottomtrailing.filled" };
pub const RECTANGLE_INSET_FILLED: SFSymbol = SFSymbol { name: "rectangle.inset.filled" };
pub const RECTANGLE_INSET_FILLED_AND_PERSON_FILLED: SFSymbol = SFSymbol { name: "rectangle.inset.filled.and.person.filled" };
pub const RECTANGLE_INSET_FILLED_ON_RECTANGLE: SFSymbol = SFSymbol { name: "rectangle.inset.filled.on.rectangle" };
pub const RECTANGLE_INSET_TOPLEADING_FILLED: SFSymbol = SFSymbol { name: "rectangle.inset.topleading.filled" };
pub const RECTANGLE_INSET_TOPLEFT_FILLED: SFSymbol = SFSymbol { name: "rectangle.inset.topleft.filled" };
pub const RECTANGLE_INSET_TOPRIGHT_FILLED: SFSymbol = SFSymbol { name: "rectangle.inset.topright.filled" };
pub const RECTANGLE_INSET_TOPTRAILING_FILLED: SFSymbol = SFSymbol { name: "rectangle.inset.toptrailing.filled" };
pub const RECTANGLE_LEADINGHALF_FILLED: SFSymbol = SFSymbol { name: "rectangle.leadinghalf.filled" };
pub const RECTANGLE_LEADINGHALF_INSET_FILLED: SFSymbol = SFSymbol { name: "rectangle.leadinghalf.inset.filled" };
pub const RECTANGLE_LEADINGHALF_INSET_FILLED_ARROW_LEADING: SFSymbol = SFSymbol { name: "rectangle.leadinghalf.inset.filled.arrow.leading" };
pub const RECTANGLE_LEADINGTHIRD_INSET_FILLED: SFSymbol = SFSymbol { name: "rectangle.leadingthird.inset.filled" };
pub const RECTANGLE_LEFTHALF_FILLED: SFSymbol = SFSymbol { name: "rectangle.lefthalf.filled" };
pub const RECTANGLE_LEFTHALF_INSET_FILLED: SFSymbol = SFSymbol { name: "rectangle.lefthalf.inset.filled" };
pub const RECTANGLE_LEFTHALF_INSET_FILLED_ARROW_LEFT: SFSymbol = SFSymbol { name: "rectangle.lefthalf.inset.filled.arrow.left" };
pub const RECTANGLE_LEFTTHIRD_INSET_FILLED: SFSymbol = SFSymbol { name: "rectangle.leftthird.inset.filled" };
pub const RECTANGLE_ON_RECTANGLE: SFSymbol = SFSymbol { name: "rectangle.on.rectangle" };
pub const RECTANGLE_ON_RECTANGLE_ANGLED: SFSymbol = SFSymbol { name: "rectangle.on.rectangle.angled" };
pub const RECTANGLE_ON_RECTANGLE_CIRCLE: SFSymbol = SFSymbol { name: "rectangle.on.rectangle.circle" };
pub const RECTANGLE_ON_RECTANGLE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "rectangle.on.rectangle.circle.fill" };
pub const RECTANGLE_ON_RECTANGLE_SLASH: SFSymbol = SFSymbol { name: "rectangle.on.rectangle.slash" };
pub const RECTANGLE_ON_RECTANGLE_SLASH_CIRCLE: SFSymbol = SFSymbol { name: "rectangle.on.rectangle.slash.circle" };
pub const RECTANGLE_ON_RECTANGLE_SLASH_CIRCLE_FILL: SFSymbol = SFSymbol { name: "rectangle.on.rectangle.slash.circle.fill" };
pub const RECTANGLE_ON_RECTANGLE_SLASH_FILL: SFSymbol = SFSymbol { name: "rectangle.on.rectangle.slash.fill" };
pub const RECTANGLE_ON_RECTANGLE_SQUARE: SFSymbol = SFSymbol { name: "rectangle.on.rectangle.square" };
pub const RECTANGLE_ON_RECTANGLE_SQUARE_FILL: SFSymbol = SFSymbol { name: "rectangle.on.rectangle.square.fill" };
pub const RECTANGLE_PORTRAIT: SFSymbol = SFSymbol { name: "rectangle.portrait" };
pub const RECTANGLE_PORTRAIT_AND_ARROW_FORWARD: SFSymbol = SFSymbol { name: "rectangle.portrait.and.arrow.forward" };
pub const RECTANGLE_PORTRAIT_AND_ARROW_FORWARD_FILL: SFSymbol = SFSymbol { name: "rectangle.portrait.and.arrow.forward.fill" };
pub const RECTANGLE_PORTRAIT_AND_ARROW_RIGHT: SFSymbol = SFSymbol { name: "rectangle.portrait.and.arrow.right" };
pub const RECTANGLE_PORTRAIT_AND_ARROW_RIGHT_FILL: SFSymbol = SFSymbol { name: "rectangle.portrait.and.arrow.right.fill" };
pub const RECTANGLE_PORTRAIT_ARROWTRIANGLE_2_INWARD: SFSymbol = SFSymbol { name: "rectangle.portrait.arrowtriangle.2.inward" };
pub const RECTANGLE_PORTRAIT_ARROWTRIANGLE_2_OUTWARD: SFSymbol = SFSymbol { name: "rectangle.portrait.arrowtriangle.2.outward" };
pub const RECTANGLE_PORTRAIT_BOTTOMHALF_FILLED: SFSymbol = SFSymbol { name: "rectangle.portrait.bottomhalf.filled" };
pub const RECTANGLE_PORTRAIT_BOTTOMHALF_INSET_FILLED: SFSymbol = SFSymbol { name: "rectangle.portrait.bottomhalf.inset.filled" };
pub const RECTANGLE_PORTRAIT_BOTTOMLEADING_INSET_FILLED: SFSymbol = SFSymbol { name: "rectangle.portrait.bottomleading.inset.filled" };
pub const RECTANGLE_PORTRAIT_BOTTOMLEFT_INSET_FILLED: SFSymbol = SFSymbol { name: "rectangle.portrait.bottomleft.inset.filled" };
pub const RECTANGLE_PORTRAIT_BOTTOMRIGHT_INSET_FILLED: SFSymbol = SFSymbol { name: "rectangle.portrait.bottomright.inset.filled" };
pub const RECTANGLE_PORTRAIT_BOTTOMTHIRD_INSET_FILLED: SFSymbol = SFSymbol { name: "rectangle.portrait.bottomthird.inset.filled" };
pub const RECTANGLE_PORTRAIT_BOTTOMTRAILING_INSET_FILLED: SFSymbol = SFSymbol { name: "rectangle.portrait.bottomtrailing.inset.filled" };
pub const RECTANGLE_PORTRAIT_CENTER_INSET_FILLED: SFSymbol = SFSymbol { name: "rectangle.portrait.center.inset.filled" };
pub const RECTANGLE_PORTRAIT_FILL: SFSymbol = SFSymbol { name: "rectangle.portrait.fill" };
pub const RECTANGLE_PORTRAIT_INSET_FILLED: SFSymbol = SFSymbol { name: "rectangle.portrait.inset.filled" };
pub const RECTANGLE_PORTRAIT_LEADINGHALF_INSET_FILLED: SFSymbol = SFSymbol { name: "rectangle.portrait.leadinghalf.inset.filled" };
pub const RECTANGLE_PORTRAIT_LEADINGTHIRD_INSET_FILLED: SFSymbol = SFSymbol { name: "rectangle.portrait.leadingthird.inset.filled" };
pub const RECTANGLE_PORTRAIT_LEFTHALF_FILLED: SFSymbol = SFSymbol { name: "rectangle.portrait.lefthalf.filled" };
pub const RECTANGLE_PORTRAIT_LEFTHALF_INSET_FILLED: SFSymbol = SFSymbol { name: "rectangle.portrait.lefthalf.inset.filled" };
pub const RECTANGLE_PORTRAIT_LEFTTHIRD_INSET_FILLED: SFSymbol = SFSymbol { name: "rectangle.portrait.leftthird.inset.filled" };
pub const RECTANGLE_PORTRAIT_ON_RECTANGLE_PORTRAIT: SFSymbol = SFSymbol { name: "rectangle.portrait.on.rectangle.portrait" };
pub const RECTANGLE_PORTRAIT_ON_RECTANGLE_PORTRAIT_ANGLED: SFSymbol = SFSymbol { name: "rectangle.portrait.on.rectangle.portrait.angled" };
pub const RECTANGLE_PORTRAIT_ON_RECTANGLE_PORTRAIT_ANGLED_FILL: SFSymbol = SFSymbol { name: "rectangle.portrait.on.rectangle.portrait.angled.fill" };
pub const RECTANGLE_PORTRAIT_ON_RECTANGLE_PORTRAIT_FILL: SFSymbol = SFSymbol { name: "rectangle.portrait.on.rectangle.portrait.fill" };
pub const RECTANGLE_PORTRAIT_ON_RECTANGLE_PORTRAIT_SLASH: SFSymbol = SFSymbol { name: "rectangle.portrait.on.rectangle.portrait.slash" };
pub const RECTANGLE_PORTRAIT_ON_RECTANGLE_PORTRAIT_SLASH_FILL: SFSymbol = SFSymbol { name: "rectangle.portrait.on.rectangle.portrait.slash.fill" };
pub const RECTANGLE_PORTRAIT_RIGHTHALF_FILLED: SFSymbol = SFSymbol { name: "rectangle.portrait.righthalf.filled" };
pub const RECTANGLE_PORTRAIT_RIGHTHALF_INSET_FILLED: SFSymbol = SFSymbol { name: "rectangle.portrait.righthalf.inset.filled" };
pub const RECTANGLE_PORTRAIT_RIGHTTHIRD_INSET_FILLED: SFSymbol = SFSymbol { name: "rectangle.portrait.rightthird.inset.filled" };
pub const RECTANGLE_PORTRAIT_SLASH: SFSymbol = SFSymbol { name: "rectangle.portrait.slash" };
pub const RECTANGLE_PORTRAIT_SLASH_FILL: SFSymbol = SFSymbol { name: "rectangle.portrait.slash.fill" };
pub const RECTANGLE_PORTRAIT_SPLIT_2X1: SFSymbol = SFSymbol { name: "rectangle.portrait.split.2x1" };
pub const RECTANGLE_PORTRAIT_SPLIT_2X1_FILL: SFSymbol = SFSymbol { name: "rectangle.portrait.split.2x1.fill" };
pub const RECTANGLE_PORTRAIT_SPLIT_2X1_SLASH: SFSymbol = SFSymbol { name: "rectangle.portrait.split.2x1.slash" };
pub const RECTANGLE_PORTRAIT_SPLIT_2X1_SLASH_FILL: SFSymbol = SFSymbol { name: "rectangle.portrait.split.2x1.slash.fill" };
pub const RECTANGLE_PORTRAIT_TOPHALF_FILLED: SFSymbol = SFSymbol { name: "rectangle.portrait.tophalf.filled" };
pub const RECTANGLE_PORTRAIT_TOPHALF_INSET_FILLED: SFSymbol = SFSymbol { name: "rectangle.portrait.tophalf.inset.filled" };
pub const RECTANGLE_PORTRAIT_TOPLEADING_INSET_FILLED: SFSymbol = SFSymbol { name: "rectangle.portrait.topleading.inset.filled" };
pub const RECTANGLE_PORTRAIT_TOPLEFT_INSET_FILLED: SFSymbol = SFSymbol { name: "rectangle.portrait.topleft.inset.filled" };
pub const RECTANGLE_PORTRAIT_TOPRIGHT_INSET_FILLED: SFSymbol = SFSymbol { name: "rectangle.portrait.topright.inset.filled" };
pub const RECTANGLE_PORTRAIT_TOPTHIRD_INSET_FILLED: SFSymbol = SFSymbol { name: "rectangle.portrait.topthird.inset.filled" };
pub const RECTANGLE_PORTRAIT_TOPTRAILING_INSET_FILLED: SFSymbol = SFSymbol { name: "rectangle.portrait.toptrailing.inset.filled" };
pub const RECTANGLE_PORTRAIT_TRAILINGHALF_INSET_FILLED: SFSymbol = SFSymbol { name: "rectangle.portrait.trailinghalf.inset.filled" };
pub const RECTANGLE_PORTRAIT_TRAILINGTHIRD_INSET_FILLED: SFSymbol = SFSymbol { name: "rectangle.portrait.trailingthird.inset.filled" };
pub const RECTANGLE_RIGHTHALF_FILLED: SFSymbol = SFSymbol { name: "rectangle.righthalf.filled" };
pub const RECTANGLE_RIGHTHALF_INSET_FILLED: SFSymbol = SFSymbol { name: "rectangle.righthalf.inset.filled" };
pub const RECTANGLE_RIGHTHALF_INSET_FILLED_ARROW_RIGHT: SFSymbol = SFSymbol { name: "rectangle.righthalf.inset.filled.arrow.right" };
pub const RECTANGLE_RIGHTTHIRD_INSET_FILLED: SFSymbol = SFSymbol { name: "rectangle.rightthird.inset.filled" };
pub const RECTANGLE_ROUNDEDBOTTOM: SFSymbol = SFSymbol { name: "rectangle.roundedbottom" };
pub const RECTANGLE_ROUNDEDBOTTOM_FILL: SFSymbol = SFSymbol { name: "rectangle.roundedbottom.fill" };
pub const RECTANGLE_ROUNDEDTOP: SFSymbol = SFSymbol { name: "rectangle.roundedtop" };
pub const RECTANGLE_ROUNDEDTOP_FILL: SFSymbol = SFSymbol { name: "rectangle.roundedtop.fill" };
pub const RECTANGLE_SLASH: SFSymbol = SFSymbol { name: "rectangle.slash" };
pub const RECTANGLE_SLASH_FILL: SFSymbol = SFSymbol { name: "rectangle.slash.fill" };
pub const RECTANGLE_SPLIT_1X2: SFSymbol = SFSymbol { name: "rectangle.split.1x2" };
pub const RECTANGLE_SPLIT_1X2_FILL: SFSymbol = SFSymbol { name: "rectangle.split.1x2.fill" };
pub const RECTANGLE_SPLIT_2X1: SFSymbol = SFSymbol { name: "rectangle.split.2x1" };
pub const RECTANGLE_SPLIT_2X1_FILL: SFSymbol = SFSymbol { name: "rectangle.split.2x1.fill" };
pub const RECTANGLE_SPLIT_2X1_SLASH: SFSymbol = SFSymbol { name: "rectangle.split.2x1.slash" };
pub const RECTANGLE_SPLIT_2X1_SLASH_FILL: SFSymbol = SFSymbol { name: "rectangle.split.2x1.slash.fill" };
pub const RECTANGLE_SPLIT_2X2: SFSymbol = SFSymbol { name: "rectangle.split.2x2" };
pub const RECTANGLE_SPLIT_2X2_FILL: SFSymbol = SFSymbol { name: "rectangle.split.2x2.fill" };
pub const RECTANGLE_SPLIT_3X1: SFSymbol = SFSymbol { name: "rectangle.split.3x1" };
pub const RECTANGLE_SPLIT_3X1_FILL: SFSymbol = SFSymbol { name: "rectangle.split.3x1.fill" };
pub const RECTANGLE_SPLIT_3X3: SFSymbol = SFSymbol { name: "rectangle.split.3x3" };
pub const RECTANGLE_SPLIT_3X3_FILL: SFSymbol = SFSymbol { name: "rectangle.split.3x3.fill" };
pub const RECTANGLE_STACK: SFSymbol = SFSymbol { name: "rectangle.stack" };
pub const RECTANGLE_STACK_BADGE_MINUS: SFSymbol = SFSymbol { name: "rectangle.stack.badge.minus" };
pub const RECTANGLE_STACK_BADGE_PERSON_CROP: SFSymbol = SFSymbol { name: "rectangle.stack.badge.person.crop" };
pub const RECTANGLE_STACK_BADGE_PERSON_CROP_FILL: SFSymbol = SFSymbol { name: "rectangle.stack.badge.person.crop.fill" };
pub const RECTANGLE_STACK_BADGE_PLAY: SFSymbol = SFSymbol { name: "rectangle.stack.badge.play" };
pub const RECTANGLE_STACK_BADGE_PLAY_FILL: SFSymbol = SFSymbol { name: "rectangle.stack.badge.play.fill" };
pub const RECTANGLE_STACK_BADGE_PLUS: SFSymbol = SFSymbol { name: "rectangle.stack.badge.plus" };
pub const RECTANGLE_STACK_FILL: SFSymbol = SFSymbol { name: "rectangle.stack.fill" };
pub const RECTANGLE_STACK_FILL_BADGE_MINUS: SFSymbol = SFSymbol { name: "rectangle.stack.fill.badge.minus" };
pub const RECTANGLE_STACK_FILL_BADGE_PLUS: SFSymbol = SFSymbol { name: "rectangle.stack.fill.badge.plus" };
pub const RECTANGLE_TOPHALF_FILLED: SFSymbol = SFSymbol { name: "rectangle.tophalf.filled" };
pub const RECTANGLE_TOPHALF_INSET_FILLED: SFSymbol = SFSymbol { name: "rectangle.tophalf.inset.filled" };
pub const RECTANGLE_TOPTHIRD_INSET_FILLED: SFSymbol = SFSymbol { name: "rectangle.topthird.inset.filled" };
pub const RECTANGLE_TRAILINGHALF_FILLED: SFSymbol = SFSymbol { name: "rectangle.trailinghalf.filled" };
pub const RECTANGLE_TRAILINGHALF_INSET_FILLED: SFSymbol = SFSymbol { name: "rectangle.trailinghalf.inset.filled" };
pub const RECTANGLE_TRAILINGHALF_INSET_FILLED_ARROW_TRAILING: SFSymbol = SFSymbol { name: "rectangle.trailinghalf.inset.filled.arrow.trailing" };
pub const RECTANGLE_TRAILINGTHIRD_INSET_FILLED: SFSymbol = SFSymbol { name: "rectangle.trailingthird.inset.filled" };
pub const REFRIGERATOR: SFSymbol = SFSymbol { name: "refrigerator" };
pub const REFRIGERATOR_FILL: SFSymbol = SFSymbol { name: "refrigerator.fill" };
pub const REPEAT: SFSymbol = SFSymbol { name: "repeat" };
pub const REPEAT_1: SFSymbol = SFSymbol { name: "repeat.1" };
pub const REPEAT_1_CIRCLE: SFSymbol = SFSymbol { name: "repeat.1.circle" };
pub const REPEAT_1_CIRCLE_FILL: SFSymbol = SFSymbol { name: "repeat.1.circle.fill" };
pub const REPEAT_CIRCLE: SFSymbol = SFSymbol { name: "repeat.circle" };
pub const REPEAT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "repeat.circle.fill" };
pub const RESTART: SFSymbol = SFSymbol { name: "restart" };
pub const RESTART_CIRCLE: SFSymbol = SFSymbol { name: "restart.circle" };
pub const RESTART_CIRCLE_FILL: SFSymbol = SFSymbol { name: "restart.circle.fill" };
pub const RETURN: SFSymbol = SFSymbol { name: "return" };
pub const RETURN_LEFT: SFSymbol = SFSymbol { name: "return.left" };
pub const RETURN_RIGHT: SFSymbol = SFSymbol { name: "return.right" };
pub const RHOMBUS: SFSymbol = SFSymbol { name: "rhombus" };
pub const RHOMBUS_FILL: SFSymbol = SFSymbol { name: "rhombus.fill" };
pub const ROLLER_SHADE_CLOSED: SFSymbol = SFSymbol { name: "roller.shade.closed" };
pub const ROLLER_SHADE_OPEN: SFSymbol = SFSymbol { name: "roller.shade.open" };
pub const ROMAN_SHADE_CLOSED: SFSymbol = SFSymbol { name: "roman.shade.closed" };
pub const ROMAN_SHADE_OPEN: SFSymbol = SFSymbol { name: "roman.shade.open" };
pub const ROSETTE: SFSymbol = SFSymbol { name: "rosette" };
pub const ROTATE_3D: SFSymbol = SFSymbol { name: "rotate.3d" };
pub const ROTATE_LEFT: SFSymbol = SFSymbol { name: "rotate.left" };
pub const ROTATE_LEFT_FILL: SFSymbol = SFSymbol { name: "rotate.left.fill" };
pub const ROTATE_RIGHT: SFSymbol = SFSymbol { name: "rotate.right" };
pub const ROTATE_RIGHT_FILL: SFSymbol = SFSymbol { name: "rotate.right.fill" };
pub const RT_RECTANGLE_ROUNDEDTOP: SFSymbol = SFSymbol { name: "rt.rectangle.roundedtop" };
pub const RT_RECTANGLE_ROUNDEDTOP_FILL: SFSymbol = SFSymbol { name: "rt.rectangle.roundedtop.fill" };
pub const RUBLESIGN: SFSymbol = SFSymbol { name: "rublesign" };
pub const RUBLESIGN_CIRCLE: SFSymbol = SFSymbol { name: "rublesign.circle" };
pub const RUBLESIGN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "rublesign.circle.fill" };
pub const RUBLESIGN_SQUARE: SFSymbol = SFSymbol { name: "rublesign.square" };
pub const RUBLESIGN_SQUARE_FILL: SFSymbol = SFSymbol { name: "rublesign.square.fill" };
pub const RULER: SFSymbol = SFSymbol { name: "ruler" };
pub const RULER_FILL: SFSymbol = SFSymbol { name: "ruler.fill" };
pub const RUPEESIGN: SFSymbol = SFSymbol { name: "rupeesign" };
pub const RUPEESIGN_CIRCLE: SFSymbol = SFSymbol { name: "rupeesign.circle" };
pub const RUPEESIGN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "rupeesign.circle.fill" };
pub const RUPEESIGN_SQUARE: SFSymbol = SFSymbol { name: "rupeesign.square" };
pub const RUPEESIGN_SQUARE_FILL: SFSymbol = SFSymbol { name: "rupeesign.square.fill" };
pub const S_CIRCLE: SFSymbol = SFSymbol { name: "s.circle" };
pub const S_CIRCLE_FILL: SFSymbol = SFSymbol { name: "s.circle.fill" };
pub const S_SQUARE: SFSymbol = SFSymbol { name: "s.square" };
pub const S_SQUARE_FILL: SFSymbol = SFSymbol { name: "s.square.fill" };
pub const SAFARI: SFSymbol = SFSymbol { name: "safari" };
pub const SAFARI_FILL: SFSymbol = SFSymbol { name: "safari.fill" };
pub const SAILBOAT: SFSymbol = SFSymbol { name: "sailboat" };
pub const SAILBOAT_FILL: SFSymbol = SFSymbol { name: "sailboat.fill" };
pub const SCALE_3D: SFSymbol = SFSymbol { name: "scale.3d" };
pub const SCALEMASS: SFSymbol = SFSymbol { name: "scalemass" };
pub const SCALEMASS_FILL: SFSymbol = SFSymbol { name: "scalemass.fill" };
pub const SCANNER: SFSymbol = SFSymbol { name: "scanner" };
pub const SCANNER_FILL: SFSymbol = SFSymbol { name: "scanner.fill" };
pub const SCISSORS: SFSymbol = SFSymbol { name: "scissors" };
pub const SCISSORS_BADGE_ELLIPSIS: SFSymbol = SFSymbol { name: "scissors.badge.ellipsis" };
pub const SCISSORS_CIRCLE: SFSymbol = SFSymbol { name: "scissors.circle" };
pub const SCISSORS_CIRCLE_FILL: SFSymbol = SFSymbol { name: "scissors.circle.fill" };
pub const SCOOTER: SFSymbol = SFSymbol { name: "scooter" };
pub const SCOPE: SFSymbol = SFSymbol { name: "scope" };
pub const SCREWDRIVER: SFSymbol = SFSymbol { name: "screwdriver" };
pub const SCREWDRIVER_FILL: SFSymbol = SFSymbol { name: "screwdriver.fill" };
pub const SCRIBBLE: SFSymbol = SFSymbol { name: "scribble" };
pub const SCRIBBLE_VARIABLE: SFSymbol = SFSymbol { name: "scribble.variable" };
pub const SCROLL: SFSymbol = SFSymbol { name: "scroll" };
pub const SCROLL_FILL: SFSymbol = SFSymbol { name: "scroll.fill" };
pub const SDCARD: SFSymbol = SFSymbol { name: "sdcard" };
pub const SDCARD_FILL: SFSymbol = SFSymbol { name: "sdcard.fill" };
pub const SEAL: SFSymbol = SFSymbol { name: "seal" };
pub const SEAL_FILL: SFSymbol = SFSymbol { name: "seal.fill" };
pub const SELECTION_PIN_IN_OUT: SFSymbol = SFSymbol { name: "selection.pin.in.out" };
pub const SENSOR: SFSymbol = SFSymbol { name: "sensor" };
pub const SENSOR_FILL: SFSymbol = SFSymbol { name: "sensor.fill" };
pub const SENSOR_TAG_RADIOWAVES_FORWARD: SFSymbol = SFSymbol { name: "sensor.tag.radiowaves.forward" };
pub const SENSOR_TAG_RADIOWAVES_FORWARD_FILL: SFSymbol = SFSymbol { name: "sensor.tag.radiowaves.forward.fill" };
pub const SERVER_RACK: SFSymbol = SFSymbol { name: "server.rack" };
pub const SHADOW: SFSymbol = SFSymbol { name: "shadow" };
pub const SHARED_WITH_YOU: SFSymbol = SFSymbol { name: "shared.with.you" };
pub const SHARED_WITH_YOU_SLASH: SFSymbol = SFSymbol { name: "shared.with.you.slash" };
pub const SHAREPLAY: SFSymbol = SFSymbol { name: "shareplay" };
pub const SHAREPLAY_SLASH: SFSymbol = SFSymbol { name: "shareplay.slash" };
pub const SHAZAM_LOGO: SFSymbol = SFSymbol { name: "shazam.logo" };
pub const SHAZAM_LOGO_FILL: SFSymbol = SFSymbol { name: "shazam.logo.fill" };
pub const SHEKELSIGN: SFSymbol = SFSymbol { name: "shekelsign" };
pub const SHEKELSIGN_CIRCLE: SFSymbol = SFSymbol { name: "shekelsign.circle" };
pub const SHEKELSIGN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "shekelsign.circle.fill" };
pub const SHEKELSIGN_SQUARE: SFSymbol = SFSymbol { name: "shekelsign.square" };
pub const SHEKELSIGN_SQUARE_FILL: SFSymbol = SFSymbol { name: "shekelsign.square.fill" };
pub const SHIELD: SFSymbol = SFSymbol { name: "shield" };
pub const SHIELD_FILL: SFSymbol = SFSymbol { name: "shield.fill" };
pub const SHIELD_LEFTHALF_FILLED: SFSymbol = SFSymbol { name: "shield.lefthalf.filled" };
pub const SHIELD_LEFTHALF_FILLED_SLASH: SFSymbol = SFSymbol { name: "shield.lefthalf.filled.slash" };
pub const SHIELD_RIGHTHALF_FILLED: SFSymbol = SFSymbol { name: "shield.righthalf.filled" };
pub const SHIELD_SLASH: SFSymbol = SFSymbol { name: "shield.slash" };
pub const SHIELD_SLASH_FILL: SFSymbol = SFSymbol { name: "shield.slash.fill" };
pub const SHIFT: SFSymbol = SFSymbol { name: "shift" };
pub const SHIFT_FILL: SFSymbol = SFSymbol { name: "shift.fill" };
pub const SHIPPINGBOX: SFSymbol = SFSymbol { name: "shippingbox" };
pub const SHIPPINGBOX_AND_ARROW_BACKWARD: SFSymbol = SFSymbol { name: "shippingbox.and.arrow.backward" };
pub const SHIPPINGBOX_AND_ARROW_BACKWARD_FILL: SFSymbol = SFSymbol { name: "shippingbox.and.arrow.backward.fill" };
pub const SHIPPINGBOX_CIRCLE: SFSymbol = SFSymbol { name: "shippingbox.circle" };
pub const SHIPPINGBOX_CIRCLE_FILL: SFSymbol = SFSymbol { name: "shippingbox.circle.fill" };
pub const SHIPPINGBOX_FILL: SFSymbol = SFSymbol { name: "shippingbox.fill" };
pub const SHOEPRINTS_FILL: SFSymbol = SFSymbol { name: "shoeprints.fill" };
pub const SHOWER: SFSymbol = SFSymbol { name: "shower" };
pub const SHOWER_FILL: SFSymbol = SFSymbol { name: "shower.fill" };
pub const SHOWER_HANDHELD: SFSymbol = SFSymbol { name: "shower.handheld" };
pub const SHOWER_HANDHELD_FILL: SFSymbol = SFSymbol { name: "shower.handheld.fill" };
pub const SHOWER_SIDEJET: SFSymbol = SFSymbol { name: "shower.sidejet" };
pub const SHOWER_SIDEJET_FILL: SFSymbol = SFSymbol { name: "shower.sidejet.fill" };
pub const SHUFFLE: SFSymbol = SFSymbol { name: "shuffle" };
pub const SHUFFLE_CIRCLE: SFSymbol = SFSymbol { name: "shuffle.circle" };
pub const SHUFFLE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "shuffle.circle.fill" };
pub const SIDEBAR_LEADING: SFSymbol = SFSymbol { name: "sidebar.leading" };
pub const SIDEBAR_LEFT: SFSymbol = SFSymbol { name: "sidebar.left" };
pub const SIDEBAR_RIGHT: SFSymbol = SFSymbol { name: "sidebar.right" };
pub const SIDEBAR_SQUARES_LEADING: SFSymbol = SFSymbol { name: "sidebar.squares.leading" };
pub const SIDEBAR_SQUARES_LEFT: SFSymbol = SFSymbol { name: "sidebar.squares.left" };
pub const SIDEBAR_SQUARES_RIGHT: SFSymbol = SFSymbol { name: "sidebar.squares.right" };
pub const SIDEBAR_SQUARES_TRAILING: SFSymbol = SFSymbol { name: "sidebar.squares.trailing" };
pub const SIDEBAR_TRAILING: SFSymbol = SFSymbol { name: "sidebar.trailing" };
pub const SIGNATURE: SFSymbol = SFSymbol { name: "signature" };
pub const SIGNPOST_LEFT: SFSymbol = SFSymbol { name: "signpost.left" };
pub const SIGNPOST_LEFT_FILL: SFSymbol = SFSymbol { name: "signpost.left.fill" };
pub const SIGNPOST_RIGHT: SFSymbol = SFSymbol { name: "signpost.right" };
pub const SIGNPOST_RIGHT_FILL: SFSymbol = SFSymbol { name: "signpost.right.fill" };
pub const SIMCARD: SFSymbol = SFSymbol { name: "simcard" };
pub const SIMCARD_2: SFSymbol = SFSymbol { name: "simcard.2" };
pub const SIMCARD_2_FILL: SFSymbol = SFSymbol { name: "simcard.2.fill" };
pub const SIMCARD_FILL: SFSymbol = SFSymbol { name: "simcard.fill" };
pub const SINK: SFSymbol = SFSymbol { name: "sink" };
pub const SINK_FILL: SFSymbol = SFSymbol { name: "sink.fill" };
pub const SKEW: SFSymbol = SFSymbol { name: "skew" };
pub const SLASH_CIRCLE: SFSymbol = SFSymbol { name: "slash.circle" };
pub const SLASH_CIRCLE_FILL: SFSymbol = SFSymbol { name: "slash.circle.fill" };
pub const SLEEP: SFSymbol = SFSymbol { name: "sleep" };
pub const SLEEP_CIRCLE: SFSymbol = SFSymbol { name: "sleep.circle" };
pub const SLEEP_CIRCLE_FILL: SFSymbol = SFSymbol { name: "sleep.circle.fill" };
pub const SLIDER_HORIZONTAL_2_GOBACKWARD: SFSymbol = SFSymbol { name: "slider.horizontal.2.gobackward" };
pub const SLIDER_HORIZONTAL_2_RECTANGLE_AND_ARROW_TRIANGLE_2_CIRCLEPATH: SFSymbol = SFSymbol { name: "slider.horizontal.2.rectangle.and.arrow.triangle.2.circlepath" };
pub const SLIDER_HORIZONTAL_2_SQUARE_BADGE_ARROW_DOWN: SFSymbol = SFSymbol { name: "slider.horizontal.2.square.badge.arrow.down" };
pub const SLIDER_HORIZONTAL_2_SQUARE_ON_SQUARE: SFSymbol = SFSymbol { name: "slider.horizontal.2.square.on.square" };
pub const SLIDER_HORIZONTAL_3: SFSymbol = SFSymbol { name: "slider.horizontal.3" };
pub const SLIDER_HORIZONTAL_BELOW_RECTANGLE: SFSymbol = SFSymbol { name: "slider.horizontal.below.rectangle" };
pub const SLIDER_HORIZONTAL_BELOW_SQUARE_AND_SQUARE_FILLED: SFSymbol = SFSymbol { name: "slider.horizontal.below.square.and.square.filled" };
pub const SLIDER_HORIZONTAL_BELOW_SQUARE_FILLED_AND_SQUARE: SFSymbol = SFSymbol { name: "slider.horizontal.below.square.filled.and.square" };
pub const SLIDER_VERTICAL_3: SFSymbol = SFSymbol { name: "slider.vertical.3" };
pub const SLOWMO: SFSymbol = SFSymbol { name: "slowmo" };
pub const SMALLCIRCLE_CIRCLE: SFSymbol = SFSymbol { name: "smallcircle.circle" };
pub const SMALLCIRCLE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "smallcircle.circle.fill" };
pub const SMALLCIRCLE_FILLED_CIRCLE: SFSymbol = SFSymbol { name: "smallcircle.filled.circle" };
pub const SMALLCIRCLE_FILLED_CIRCLE_FILL: SFSymbol = SFSymbol { name: "smallcircle.filled.circle.fill" };
pub const SMOKE: SFSymbol = SFSymbol { name: "smoke" };
pub const SMOKE_CIRCLE: SFSymbol = SFSymbol { name: "smoke.circle" };
pub const SMOKE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "smoke.circle.fill" };
pub const SMOKE_FILL: SFSymbol = SFSymbol { name: "smoke.fill" };
pub const SNOWFLAKE: SFSymbol = SFSymbol { name: "snowflake" };
pub const SNOWFLAKE_CIRCLE: SFSymbol = SFSymbol { name: "snowflake.circle" };
pub const SNOWFLAKE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "snowflake.circle.fill" };
pub const SOCCERBALL: SFSymbol = SFSymbol { name: "soccerball" };
pub const SOCCERBALL_CIRCLE: SFSymbol = SFSymbol { name: "soccerball.circle" };
pub const SOCCERBALL_CIRCLE_FILL: SFSymbol = SFSymbol { name: "soccerball.circle.fill" };
pub const SOCCERBALL_CIRCLE_FILL_INVERSE: SFSymbol = SFSymbol { name: "soccerball.circle.fill.inverse" };
pub const SOCCERBALL_CIRCLE_INVERSE: SFSymbol = SFSymbol { name: "soccerball.circle.inverse" };
pub const SOCCERBALL_INVERSE: SFSymbol = SFSymbol { name: "soccerball.inverse" };
pub const SOFA: SFSymbol = SFSymbol { name: "sofa" };
pub const SOFA_FILL: SFSymbol = SFSymbol { name: "sofa.fill" };
pub const SPACE: SFSymbol = SFSymbol { name: "space" };
pub const SPARKLE: SFSymbol = SFSymbol { name: "sparkle" };
pub const SPARKLE_MAGNIFYINGGLASS: SFSymbol = SFSymbol { name: "sparkle.magnifyingglass" };
pub const SPARKLES: SFSymbol = SFSymbol { name: "sparkles" };
pub const SPARKLES_RECTANGLE_STACK: SFSymbol = SFSymbol { name: "sparkles.rectangle.stack" };
pub const SPARKLES_RECTANGLE_STACK_FILL: SFSymbol = SFSymbol { name: "sparkles.rectangle.stack.fill" };
pub const SPARKLES_SQUARE_FILLED_ON_SQUARE: SFSymbol = SFSymbol { name: "sparkles.square.filled.on.square" };
pub const SPARKLES_TV: SFSymbol = SFSymbol { name: "sparkles.tv" };
pub const SPARKLES_TV_FILL: SFSymbol = SFSymbol { name: "sparkles.tv.fill" };
pub const SPEAKER: SFSymbol = SFSymbol { name: "speaker" };
pub const SPEAKER_BADGE_EXCLAMATIONMARK: SFSymbol = SFSymbol { name: "speaker.badge.exclamationmark" };
pub const SPEAKER_BADGE_EXCLAMATIONMARK_FILL: SFSymbol = SFSymbol { name: "speaker.badge.exclamationmark.fill" };
pub const SPEAKER_CIRCLE: SFSymbol = SFSymbol { name: "speaker.circle" };
pub const SPEAKER_CIRCLE_FILL: SFSymbol = SFSymbol { name: "speaker.circle.fill" };
pub const SPEAKER_FILL: SFSymbol = SFSymbol { name: "speaker.fill" };
pub const SPEAKER_MINUS: SFSymbol = SFSymbol { name: "speaker.minus" };
pub const SPEAKER_MINUS_FILL: SFSymbol = SFSymbol { name: "speaker.minus.fill" };
pub const SPEAKER_PLUS: SFSymbol = SFSymbol { name: "speaker.plus" };
pub const SPEAKER_PLUS_FILL: SFSymbol = SFSymbol { name: "speaker.plus.fill" };
pub const SPEAKER_SLASH: SFSymbol = SFSymbol { name: "speaker.slash" };
pub const SPEAKER_SLASH_CIRCLE: SFSymbol = SFSymbol { name: "speaker.slash.circle" };
pub const SPEAKER_SLASH_CIRCLE_FILL: SFSymbol = SFSymbol { name: "speaker.slash.circle.fill" };
pub const SPEAKER_SLASH_FILL: SFSymbol = SFSymbol { name: "speaker.slash.fill" };
pub const SPEAKER_SQUARE: SFSymbol = SFSymbol { name: "speaker.square" };
pub const SPEAKER_SQUARE_FILL: SFSymbol = SFSymbol { name: "speaker.square.fill" };
pub const SPEAKER_WAVE_1: SFSymbol = SFSymbol { name: "speaker.wave.1" };
pub const SPEAKER_WAVE_1_FILL: SFSymbol = SFSymbol { name: "speaker.wave.1.fill" };
pub const SPEAKER_WAVE_2: SFSymbol = SFSymbol { name: "speaker.wave.2" };
pub const SPEAKER_WAVE_2_BUBBLE_LEFT: SFSymbol = SFSymbol { name: "speaker.wave.2.bubble.left" };
pub const SPEAKER_WAVE_2_BUBBLE_LEFT_FILL: SFSymbol = SFSymbol { name: "speaker.wave.2.bubble.left.fill" };
pub const SPEAKER_WAVE_2_CIRCLE: SFSymbol = SFSymbol { name: "speaker.wave.2.circle" };
pub const SPEAKER_WAVE_2_CIRCLE_FILL: SFSymbol = SFSymbol { name: "speaker.wave.2.circle.fill" };
pub const SPEAKER_WAVE_2_FILL: SFSymbol = SFSymbol { name: "speaker.wave.2.fill" };
pub const SPEAKER_WAVE_3: SFSymbol = SFSymbol { name: "speaker.wave.3" };
pub const SPEAKER_WAVE_3_FILL: SFSymbol = SFSymbol { name: "speaker.wave.3.fill" };
pub const SPEAKER_ZZZ: SFSymbol = SFSymbol { name: "speaker.zzz" };
pub const SPEAKER_ZZZ_FILL: SFSymbol = SFSymbol { name: "speaker.zzz.fill" };
pub const SPEEDOMETER: SFSymbol = SFSymbol { name: "speedometer" };
pub const SPIGOT: SFSymbol = SFSymbol { name: "spigot" };
pub const SPIGOT_FILL: SFSymbol = SFSymbol { name: "spigot.fill" };
pub const SPORTSCOURT: SFSymbol = SFSymbol { name: "sportscourt" };
pub const SPORTSCOURT_CIRCLE: SFSymbol = SFSymbol { name: "sportscourt.circle" };
pub const SPORTSCOURT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "sportscourt.circle.fill" };
pub const SPORTSCOURT_FILL: SFSymbol = SFSymbol { name: "sportscourt.fill" };
pub const SPRINKLER: SFSymbol = SFSymbol { name: "sprinkler" };
pub const SPRINKLER_AND_DROPLETS: SFSymbol = SFSymbol { name: "sprinkler.and.droplets" };
pub const SPRINKLER_AND_DROPLETS_FILL: SFSymbol = SFSymbol { name: "sprinkler.and.droplets.fill" };
pub const SPRINKLER_FILL: SFSymbol = SFSymbol { name: "sprinkler.fill" };
pub const SQUARE: SFSymbol = SFSymbol { name: "square" };
pub const SQUARE_2_LAYERS_3D: SFSymbol = SFSymbol { name: "square.2.layers.3d" };
pub const SQUARE_2_LAYERS_3D_BOTTOM_FILLED: SFSymbol = SFSymbol { name: "square.2.layers.3d.bottom.filled" };
pub const SQUARE_2_LAYERS_3D_TOP_FILLED: SFSymbol = SFSymbol { name: "square.2.layers.3d.top.filled" };
pub const SQUARE_2_STACK_3D: SFSymbol = SFSymbol { name: "square.2.stack.3d" };
pub const SQUARE_2_STACK_3D_BOTTOM_FILLED: SFSymbol = SFSymbol { name: "square.2.stack.3d.bottom.filled" };
pub const SQUARE_2_STACK_3D_TOP_FILLED: SFSymbol = SFSymbol { name: "square.2.stack.3d.top.filled" };
pub const SQUARE_3_LAYERS_3D: SFSymbol = SFSymbol { name: "square.3.layers.3d" };
pub const SQUARE_3_LAYERS_3D_BOTTOM_FILLED: SFSymbol = SFSymbol { name: "square.3.layers.3d.bottom.filled" };
pub const SQUARE_3_LAYERS_3D_DOWN_BACKWARD: SFSymbol = SFSymbol { name: "square.3.layers.3d.down.backward" };
pub const SQUARE_3_LAYERS_3D_DOWN_FORWARD: SFSymbol = SFSymbol { name: "square.3.layers.3d.down.forward" };
pub const SQUARE_3_LAYERS_3D_DOWN_LEFT: SFSymbol = SFSymbol { name: "square.3.layers.3d.down.left" };
pub const SQUARE_3_LAYERS_3D_DOWN_LEFT_SLASH: SFSymbol = SFSymbol { name: "square.3.layers.3d.down.left.slash" };
pub const SQUARE_3_LAYERS_3D_DOWN_RIGHT: SFSymbol = SFSymbol { name: "square.3.layers.3d.down.right" };
pub const SQUARE_3_LAYERS_3D_DOWN_RIGHT_SLASH: SFSymbol = SFSymbol { name: "square.3.layers.3d.down.right.slash" };
pub const SQUARE_3_LAYERS_3D_MIDDLE_FILLED: SFSymbol = SFSymbol { name: "square.3.layers.3d.middle.filled" };
pub const SQUARE_3_LAYERS_3D_SLASH: SFSymbol = SFSymbol { name: "square.3.layers.3d.slash" };
pub const SQUARE_3_LAYERS_3D_TOP_FILLED: SFSymbol = SFSymbol { name: "square.3.layers.3d.top.filled" };
pub const SQUARE_3_STACK_3D: SFSymbol = SFSymbol { name: "square.3.stack.3d" };
pub const SQUARE_3_STACK_3D_BOTTOM_FILLED: SFSymbol = SFSymbol { name: "square.3.stack.3d.bottom.filled" };
pub const SQUARE_3_STACK_3D_MIDDLE_FILLED: SFSymbol = SFSymbol { name: "square.3.stack.3d.middle.filled" };
pub const SQUARE_3_STACK_3D_SLASH: SFSymbol = SFSymbol { name: "square.3.stack.3d.slash" };
pub const SQUARE_3_STACK_3D_TOP_FILLED: SFSymbol = SFSymbol { name: "square.3.stack.3d.top.filled" };
pub const SQUARE_AND_ARROW_DOWN: SFSymbol = SFSymbol { name: "square.and.arrow.down" };
pub const SQUARE_AND_ARROW_DOWN_FILL: SFSymbol = SFSymbol { name: "square.and.arrow.down.fill" };
pub const SQUARE_AND_ARROW_DOWN_ON_SQUARE: SFSymbol = SFSymbol { name: "square.and.arrow.down.on.square" };
pub const SQUARE_AND_ARROW_DOWN_ON_SQUARE_FILL: SFSymbol = SFSymbol { name: "square.and.arrow.down.on.square.fill" };
pub const SQUARE_AND_ARROW_UP: SFSymbol = SFSymbol { name: "square.and.arrow.up" };
pub const SQUARE_AND_ARROW_UP_CIRCLE: SFSymbol = SFSymbol { name: "square.and.arrow.up.circle" };
pub const SQUARE_AND_ARROW_UP_CIRCLE_FILL: SFSymbol = SFSymbol { name: "square.and.arrow.up.circle.fill" };
pub const SQUARE_AND_ARROW_UP_FILL: SFSymbol = SFSymbol { name: "square.and.arrow.up.fill" };
pub const SQUARE_AND_ARROW_UP_ON_SQUARE: SFSymbol = SFSymbol { name: "square.and.arrow.up.on.square" };
pub const SQUARE_AND_ARROW_UP_ON_SQUARE_FILL: SFSymbol = SFSymbol { name: "square.and.arrow.up.on.square.fill" };
pub const SQUARE_AND_ARROW_UP_TRIANGLEBADGE_EXCLAMATIONMARK: SFSymbol = SFSymbol { name: "square.and.arrow.up.trianglebadge.exclamationmark" };
pub const SQUARE_AND_AT_RECTANGLE: SFSymbol = SFSymbol { name: "square.and.at.rectangle" };
pub const SQUARE_AND_AT_RECTANGLE_FILL: SFSymbol = SFSymbol { name: "square.and.at.rectangle.fill" };
pub const SQUARE_AND_LINE_VERTICAL_AND_SQUARE: SFSymbol = SFSymbol { name: "square.and.line.vertical.and.square" };
pub const SQUARE_AND_LINE_VERTICAL_AND_SQUARE_FILLED: SFSymbol = SFSymbol { name: "square.and.line.vertical.and.square.filled" };
pub const SQUARE_AND_PENCIL: SFSymbol = SFSymbol { name: "square.and.pencil" };
pub const SQUARE_AND_PENCIL_CIRCLE: SFSymbol = SFSymbol { name: "square.and.pencil.circle" };
pub const SQUARE_AND_PENCIL_CIRCLE_FILL: SFSymbol = SFSymbol { name: "square.and.pencil.circle.fill" };
pub const SQUARE_BOTTOMHALF_FILLED: SFSymbol = SFSymbol { name: "square.bottomhalf.filled" };
pub const SQUARE_BOTTOMTHIRD_INSET_FILLED: SFSymbol = SFSymbol { name: "square.bottomthird.inset.filled" };
pub const SQUARE_CIRCLE: SFSymbol = SFSymbol { name: "square.circle" };
pub const SQUARE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "square.circle.fill" };
pub const SQUARE_DASHED: SFSymbol = SFSymbol { name: "square.dashed" };
pub const SQUARE_DASHED_INSET_FILLED: SFSymbol = SFSymbol { name: "square.dashed.inset.filled" };
pub const SQUARE_DOTTED: SFSymbol = SFSymbol { name: "square.dotted" };
pub const SQUARE_FILL: SFSymbol = SFSymbol { name: "square.fill" };
pub const SQUARE_FILL_AND_LINE_VERTICAL_AND_SQUARE_FILL: SFSymbol = SFSymbol { name: "square.fill.and.line.vertical.and.square.fill" };
pub const SQUARE_FILL_ON_CIRCLE_FILL: SFSymbol = SFSymbol { name: "square.fill.on.circle.fill" };
pub const SQUARE_FILL_ON_SQUARE_FILL: SFSymbol = SFSymbol { name: "square.fill.on.square.fill" };
pub const SQUARE_FILL_TEXT_GRID_1X2: SFSymbol = SFSymbol { name: "square.fill.text.grid.1x2" };
pub const SQUARE_FILLED_AND_LINE_VERTICAL_AND_SQUARE: SFSymbol = SFSymbol { name: "square.filled.and.line.vertical.and.square" };
pub const SQUARE_FILLED_ON_SQUARE: SFSymbol = SFSymbol { name: "square.filled.on.square" };
pub const SQUARE_GRID_2X2: SFSymbol = SFSymbol { name: "square.grid.2x2" };
pub const SQUARE_GRID_2X2_FILL: SFSymbol = SFSymbol { name: "square.grid.2x2.fill" };
pub const SQUARE_GRID_3X1_BELOW_LINE_GRID_1X2: SFSymbol = SFSymbol { name: "square.grid.3x1.below.line.grid.1x2" };
pub const SQUARE_GRID_3X1_BELOW_LINE_GRID_1X2_FILL: SFSymbol = SFSymbol { name: "square.grid.3x1.below.line.grid.1x2.fill" };
pub const SQUARE_GRID_3X1_FOLDER_BADGE_PLUS: SFSymbol = SFSymbol { name: "square.grid.3x1.folder.badge.plus" };
pub const SQUARE_GRID_3X1_FOLDER_FILL_BADGE_PLUS: SFSymbol = SFSymbol { name: "square.grid.3x1.folder.fill.badge.plus" };
pub const SQUARE_GRID_3X2: SFSymbol = SFSymbol { name: "square.grid.3x2" };
pub const SQUARE_GRID_3X2_FILL: SFSymbol = SFSymbol { name: "square.grid.3x2.fill" };
pub const SQUARE_GRID_3X3: SFSymbol = SFSymbol { name: "square.grid.3x3" };
pub const SQUARE_GRID_3X3_BOTTOMLEFT_FILLED: SFSymbol = SFSymbol { name: "square.grid.3x3.bottomleft.filled" };
pub const SQUARE_GRID_3X3_BOTTOMMIDDLE_FILLED: SFSymbol = SFSymbol { name: "square.grid.3x3.bottommiddle.filled" };
pub const SQUARE_GRID_3X3_BOTTOMRIGHT_FILLED: SFSymbol = SFSymbol { name: "square.grid.3x3.bottomright.filled" };
pub const SQUARE_GRID_3X3_FILL: SFSymbol = SFSymbol { name: "square.grid.3x3.fill" };
pub const SQUARE_GRID_3X3_MIDDLE_FILLED: SFSymbol = SFSymbol { name: "square.grid.3x3.middle.filled" };
pub const SQUARE_GRID_3X3_MIDDLELEFT_FILLED: SFSymbol = SFSymbol { name: "square.grid.3x3.middleleft.filled" };
pub const SQUARE_GRID_3X3_MIDDLERIGHT_FILLED: SFSymbol = SFSymbol { name: "square.grid.3x3.middleright.filled" };
pub const SQUARE_GRID_3X3_SQUARE: SFSymbol = SFSymbol { name: "square.grid.3x3.square" };
pub const SQUARE_GRID_3X3_TOPLEFT_FILLED: SFSymbol = SFSymbol { name: "square.grid.3x3.topleft.filled" };
pub const SQUARE_GRID_3X3_TOPMIDDLE_FILLED: SFSymbol = SFSymbol { name: "square.grid.3x3.topmiddle.filled" };
pub const SQUARE_GRID_3X3_TOPRIGHT_FILLED: SFSymbol = SFSymbol { name: "square.grid.3x3.topright.filled" };
pub const SQUARE_GRID_4X3_FILL: SFSymbol = SFSymbol { name: "square.grid.4x3.fill" };
pub const SQUARE_INSET_FILLED: SFSymbol = SFSymbol { name: "square.inset.filled" };
pub const SQUARE_LEADINGTHIRD_INSET_FILLED: SFSymbol = SFSymbol { name: "square.leadingthird.inset.filled" };
pub const SQUARE_LEFTHALF_FILLED: SFSymbol = SFSymbol { name: "square.lefthalf.filled" };
pub const SQUARE_LEFTTHIRD_INSET_FILLED: SFSymbol = SFSymbol { name: "square.leftthird.inset.filled" };
pub const SQUARE_ON_CIRCLE: SFSymbol = SFSymbol { name: "square.on.circle" };
pub const SQUARE_ON_SQUARE: SFSymbol = SFSymbol { name: "square.on.square" };
pub const SQUARE_ON_SQUARE_BADGE_PERSON_CROP: SFSymbol = SFSymbol { name: "square.on.square.badge.person.crop" };
pub const SQUARE_ON_SQUARE_BADGE_PERSON_CROP_FILL: SFSymbol = SFSymbol { name: "square.on.square.badge.person.crop.fill" };
pub const SQUARE_ON_SQUARE_DASHED: SFSymbol = SFSymbol { name: "square.on.square.dashed" };
pub const SQUARE_ON_SQUARE_INTERSECTION_DASHED: SFSymbol = SFSymbol { name: "square.on.square.intersection.dashed" };
pub const SQUARE_ON_SQUARE_SQUARESHAPE_CONTROLHANDLES: SFSymbol = SFSymbol { name: "square.on.square.squareshape.controlhandles" };
pub const SQUARE_RIGHTHALF_FILLED: SFSymbol = SFSymbol { name: "square.righthalf.filled" };
pub const SQUARE_RIGHTTHIRD_INSET_FILLED: SFSymbol = SFSymbol { name: "square.rightthird.inset.filled" };
pub const SQUARE_SLASH: SFSymbol = SFSymbol { name: "square.slash" };
pub const SQUARE_SLASH_FILL: SFSymbol = SFSymbol { name: "square.slash.fill" };
pub const SQUARE_SPLIT_1X2: SFSymbol = SFSymbol { name: "square.split.1x2" };
pub const SQUARE_SPLIT_1X2_FILL: SFSymbol = SFSymbol { name: "square.split.1x2.fill" };
pub const SQUARE_SPLIT_2X1: SFSymbol = SFSymbol { name: "square.split.2x1" };
pub const SQUARE_SPLIT_2X1_FILL: SFSymbol = SFSymbol { name: "square.split.2x1.fill" };
pub const SQUARE_SPLIT_2X2: SFSymbol = SFSymbol { name: "square.split.2x2" };
pub const SQUARE_SPLIT_2X2_FILL: SFSymbol = SFSymbol { name: "square.split.2x2.fill" };
pub const SQUARE_SPLIT_BOTTOMRIGHTQUARTER: SFSymbol = SFSymbol { name: "square.split.bottomrightquarter" };
pub const SQUARE_SPLIT_BOTTOMRIGHTQUARTER_FILL: SFSymbol = SFSymbol { name: "square.split.bottomrightquarter.fill" };
pub const SQUARE_SPLIT_DIAGONAL: SFSymbol = SFSymbol { name: "square.split.diagonal" };
pub const SQUARE_SPLIT_DIAGONAL_2X2: SFSymbol = SFSymbol { name: "square.split.diagonal.2x2" };
pub const SQUARE_SPLIT_DIAGONAL_2X2_FILL: SFSymbol = SFSymbol { name: "square.split.diagonal.2x2.fill" };
pub const SQUARE_SPLIT_DIAGONAL_FILL: SFSymbol = SFSymbol { name: "square.split.diagonal.fill" };
pub const SQUARE_STACK: SFSymbol = SFSymbol { name: "square.stack" };
pub const SQUARE_STACK_3D_DOWN_FORWARD: SFSymbol = SFSymbol { name: "square.stack.3d.down.forward" };
pub const SQUARE_STACK_3D_DOWN_FORWARD_FILL: SFSymbol = SFSymbol { name: "square.stack.3d.down.forward.fill" };
pub const SQUARE_STACK_3D_DOWN_RIGHT: SFSymbol = SFSymbol { name: "square.stack.3d.down.right" };
pub const SQUARE_STACK_3D_DOWN_RIGHT_FILL: SFSymbol = SFSymbol { name: "square.stack.3d.down.right.fill" };
pub const SQUARE_STACK_3D_FORWARD_DOTTEDLINE: SFSymbol = SFSymbol { name: "square.stack.3d.forward.dottedline" };
pub const SQUARE_STACK_3D_FORWARD_DOTTEDLINE_FILL: SFSymbol = SFSymbol { name: "square.stack.3d.forward.dottedline.fill" };
pub const SQUARE_STACK_3D_UP: SFSymbol = SFSymbol { name: "square.stack.3d.up" };
pub const SQUARE_STACK_3D_UP_BADGE_A: SFSymbol = SFSymbol { name: "square.stack.3d.up.badge.a" };
pub const SQUARE_STACK_3D_UP_BADGE_A_FILL: SFSymbol = SFSymbol { name: "square.stack.3d.up.badge.a.fill" };
pub const SQUARE_STACK_3D_UP_FILL: SFSymbol = SFSymbol { name: "square.stack.3d.up.fill" };
pub const SQUARE_STACK_3D_UP_SLASH: SFSymbol = SFSymbol { name: "square.stack.3d.up.slash" };
pub const SQUARE_STACK_3D_UP_SLASH_FILL: SFSymbol = SFSymbol { name: "square.stack.3d.up.slash.fill" };
pub const SQUARE_STACK_FILL: SFSymbol = SFSymbol { name: "square.stack.fill" };
pub const SQUARE_TEXT_SQUARE: SFSymbol = SFSymbol { name: "square.text.square" };
pub const SQUARE_TEXT_SQUARE_FILL: SFSymbol = SFSymbol { name: "square.text.square.fill" };
pub const SQUARE_TOPHALF_FILLED: SFSymbol = SFSymbol { name: "square.tophalf.filled" };
pub const SQUARE_TOPTHIRD_INSET_FILLED: SFSymbol = SFSymbol { name: "square.topthird.inset.filled" };
pub const SQUARE_TRAILINGTHIRD_INSET_FILLED: SFSymbol = SFSymbol { name: "square.trailingthird.inset.filled" };
pub const SQUARES_BELOW_RECTANGLE: SFSymbol = SFSymbol { name: "squares.below.rectangle" };
pub const SQUARES_LEADING_RECTANGLE: SFSymbol = SFSymbol { name: "squares.leading.rectangle" };
pub const SQUARESHAPE: SFSymbol = SFSymbol { name: "squareshape" };
pub const SQUARESHAPE_CONTROLHANDLES_ON_SQUARESHAPE_CONTROLHANDLES: SFSymbol = SFSymbol { name: "squareshape.controlhandles.on.squareshape.controlhandles" };
pub const SQUARESHAPE_DASHED_SQUARESHAPE: SFSymbol = SFSymbol { name: "squareshape.dashed.squareshape" };
pub const SQUARESHAPE_DOTTED_SPLIT_2X2: SFSymbol = SFSymbol { name: "squareshape.dotted.split.2x2" };
pub const SQUARESHAPE_FILL: SFSymbol = SFSymbol { name: "squareshape.fill" };
pub const SQUARESHAPE_SPLIT_2X2: SFSymbol = SFSymbol { name: "squareshape.split.2x2" };
pub const SQUARESHAPE_SPLIT_2X2_DOTTED: SFSymbol = SFSymbol { name: "squareshape.split.2x2.dotted" };
pub const SQUARESHAPE_SPLIT_3X3: SFSymbol = SFSymbol { name: "squareshape.split.3x3" };
pub const SQUARESHAPE_SQUARESHAPE_DASHED: SFSymbol = SFSymbol { name: "squareshape.squareshape.dashed" };
pub const STAIRS: SFSymbol = SFSymbol { name: "stairs" };
pub const STAR: SFSymbol = SFSymbol { name: "star" };
pub const STAR_BUBBLE: SFSymbol = SFSymbol { name: "star.bubble" };
pub const STAR_BUBBLE_FILL: SFSymbol = SFSymbol { name: "star.bubble.fill" };
pub const STAR_CIRCLE: SFSymbol = SFSymbol { name: "star.circle" };
pub const STAR_CIRCLE_FILL: SFSymbol = SFSymbol { name: "star.circle.fill" };
pub const STAR_FILL: SFSymbol = SFSymbol { name: "star.fill" };
pub const STAR_LEADINGHALF_FILLED: SFSymbol = SFSymbol { name: "star.leadinghalf.filled" };
pub const STAR_SLASH: SFSymbol = SFSymbol { name: "star.slash" };
pub const STAR_SLASH_FILL: SFSymbol = SFSymbol { name: "star.slash.fill" };
pub const STAR_SQUARE: SFSymbol = SFSymbol { name: "star.square" };
pub const STAR_SQUARE_FILL: SFSymbol = SFSymbol { name: "star.square.fill" };
pub const STAR_SQUARE_ON_SQUARE: SFSymbol = SFSymbol { name: "star.square.on.square" };
pub const STAR_SQUARE_ON_SQUARE_FILL: SFSymbol = SFSymbol { name: "star.square.on.square.fill" };
pub const STAROFLIFE: SFSymbol = SFSymbol { name: "staroflife" };
pub const STAROFLIFE_CIRCLE: SFSymbol = SFSymbol { name: "staroflife.circle" };
pub const STAROFLIFE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "staroflife.circle.fill" };
pub const STAROFLIFE_FILL: SFSymbol = SFSymbol { name: "staroflife.fill" };
pub const STERLINGSIGN: SFSymbol = SFSymbol { name: "sterlingsign" };
pub const STERLINGSIGN_CIRCLE: SFSymbol = SFSymbol { name: "sterlingsign.circle" };
pub const STERLINGSIGN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "sterlingsign.circle.fill" };
pub const STERLINGSIGN_SQUARE: SFSymbol = SFSymbol { name: "sterlingsign.square" };
pub const STERLINGSIGN_SQUARE_FILL: SFSymbol = SFSymbol { name: "sterlingsign.square.fill" };
pub const STETHOSCOPE: SFSymbol = SFSymbol { name: "stethoscope" };
pub const STETHOSCOPE_CIRCLE: SFSymbol = SFSymbol { name: "stethoscope.circle" };
pub const STETHOSCOPE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "stethoscope.circle.fill" };
pub const STOP: SFSymbol = SFSymbol { name: "stop" };
pub const STOP_CIRCLE: SFSymbol = SFSymbol { name: "stop.circle" };
pub const STOP_CIRCLE_FILL: SFSymbol = SFSymbol { name: "stop.circle.fill" };
pub const STOP_FILL: SFSymbol = SFSymbol { name: "stop.fill" };
pub const STOPWATCH: SFSymbol = SFSymbol { name: "stopwatch" };
pub const STOPWATCH_FILL: SFSymbol = SFSymbol { name: "stopwatch.fill" };
pub const STOVE: SFSymbol = SFSymbol { name: "stove" };
pub const STOVE_FILL: SFSymbol = SFSymbol { name: "stove.fill" };
pub const STRIKETHROUGH: SFSymbol = SFSymbol { name: "strikethrough" };
pub const STUDENTDESK: SFSymbol = SFSymbol { name: "studentdesk" };
pub const SUIT_CLUB: SFSymbol = SFSymbol { name: "suit.club" };
pub const SUIT_CLUB_FILL: SFSymbol = SFSymbol { name: "suit.club.fill" };
pub const SUIT_DIAMOND: SFSymbol = SFSymbol { name: "suit.diamond" };
pub const SUIT_DIAMOND_FILL: SFSymbol = SFSymbol { name: "suit.diamond.fill" };
pub const SUIT_HEART: SFSymbol = SFSymbol { name: "suit.heart" };
pub const SUIT_HEART_FILL: SFSymbol = SFSymbol { name: "suit.heart.fill" };
pub const SUIT_SPADE: SFSymbol = SFSymbol { name: "suit.spade" };
pub const SUIT_SPADE_FILL: SFSymbol = SFSymbol { name: "suit.spade.fill" };
pub const SUITCASE: SFSymbol = SFSymbol { name: "suitcase" };
pub const SUITCASE_CART: SFSymbol = SFSymbol { name: "suitcase.cart" };
pub const SUITCASE_CART_FILL: SFSymbol = SFSymbol { name: "suitcase.cart.fill" };
pub const SUITCASE_FILL: SFSymbol = SFSymbol { name: "suitcase.fill" };
pub const SUM: SFSymbol = SFSymbol { name: "sum" };
pub const SUN_AND_HORIZON: SFSymbol = SFSymbol { name: "sun.and.horizon" };
pub const SUN_AND_HORIZON_CIRCLE: SFSymbol = SFSymbol { name: "sun.and.horizon.circle" };
pub const SUN_AND_HORIZON_CIRCLE_FILL: SFSymbol = SFSymbol { name: "sun.and.horizon.circle.fill" };
pub const SUN_AND_HORIZON_FILL: SFSymbol = SFSymbol { name: "sun.and.horizon.fill" };
pub const SUN_DUST: SFSymbol = SFSymbol { name: "sun.dust" };
pub const SUN_DUST_CIRCLE: SFSymbol = SFSymbol { name: "sun.dust.circle" };
pub const SUN_DUST_CIRCLE_FILL: SFSymbol = SFSymbol { name: "sun.dust.circle.fill" };
pub const SUN_DUST_FILL: SFSymbol = SFSymbol { name: "sun.dust.fill" };
pub const SUN_HAZE: SFSymbol = SFSymbol { name: "sun.haze" };
pub const SUN_HAZE_CIRCLE: SFSymbol = SFSymbol { name: "sun.haze.circle" };
pub const SUN_HAZE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "sun.haze.circle.fill" };
pub const SUN_HAZE_FILL: SFSymbol = SFSymbol { name: "sun.haze.fill" };
pub const SUN_MAX: SFSymbol = SFSymbol { name: "sun.max" };
pub const SUN_MAX_CIRCLE: SFSymbol = SFSymbol { name: "sun.max.circle" };
pub const SUN_MAX_CIRCLE_FILL: SFSymbol = SFSymbol { name: "sun.max.circle.fill" };
pub const SUN_MAX_FILL: SFSymbol = SFSymbol { name: "sun.max.fill" };
pub const SUN_MAX_TRIANGLEBADGE_EXCLAMATIONMARK: SFSymbol = SFSymbol { name: "sun.max.trianglebadge.exclamationmark" };
pub const SUN_MAX_TRIANGLEBADGE_EXCLAMATIONMARK_FILL: SFSymbol = SFSymbol { name: "sun.max.trianglebadge.exclamationmark.fill" };
pub const SUN_MIN: SFSymbol = SFSymbol { name: "sun.min" };
pub const SUN_MIN_FILL: SFSymbol = SFSymbol { name: "sun.min.fill" };
pub const SUNRISE: SFSymbol = SFSymbol { name: "sunrise" };
pub const SUNRISE_CIRCLE: SFSymbol = SFSymbol { name: "sunrise.circle" };
pub const SUNRISE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "sunrise.circle.fill" };
pub const SUNRISE_FILL: SFSymbol = SFSymbol { name: "sunrise.fill" };
pub const SUNSET: SFSymbol = SFSymbol { name: "sunset" };
pub const SUNSET_CIRCLE: SFSymbol = SFSymbol { name: "sunset.circle" };
pub const SUNSET_CIRCLE_FILL: SFSymbol = SFSymbol { name: "sunset.circle.fill" };
pub const SUNSET_FILL: SFSymbol = SFSymbol { name: "sunset.fill" };
pub const SWATCHPALETTE: SFSymbol = SFSymbol { name: "swatchpalette" };
pub const SWATCHPALETTE_FILL: SFSymbol = SFSymbol { name: "swatchpalette.fill" };
pub const SWIFT: SFSymbol = SFSymbol { name: "swift" };
pub const SWITCH_2: SFSymbol = SFSymbol { name: "switch.2" };
pub const SWITCH_PROGRAMMABLE: SFSymbol = SFSymbol { name: "switch.programmable" };
pub const SWITCH_PROGRAMMABLE_FILL: SFSymbol = SFSymbol { name: "switch.programmable.fill" };
pub const SWITCH_PROGRAMMABLE_SQUARE: SFSymbol = SFSymbol { name: "switch.programmable.square" };
pub const SWITCH_PROGRAMMABLE_SQUARE_FILL: SFSymbol = SFSymbol { name: "switch.programmable.square.fill" };
pub const SYRINGE: SFSymbol = SFSymbol { name: "syringe" };
pub const SYRINGE_FILL: SFSymbol = SFSymbol { name: "syringe.fill" };
pub const T_CIRCLE: SFSymbol = SFSymbol { name: "t.circle" };
pub const T_CIRCLE_FILL: SFSymbol = SFSymbol { name: "t.circle.fill" };
pub const T_SQUARE: SFSymbol = SFSymbol { name: "t.square" };
pub const T_SQUARE_FILL: SFSymbol = SFSymbol { name: "t.square.fill" };
pub const TABLE_FURNITURE: SFSymbol = SFSymbol { name: "table.furniture" };
pub const TABLE_FURNITURE_FILL: SFSymbol = SFSymbol { name: "table.furniture.fill" };
pub const TABLECELLS: SFSymbol = SFSymbol { name: "tablecells" };
pub const TABLECELLS_BADGE_ELLIPSIS: SFSymbol = SFSymbol { name: "tablecells.badge.ellipsis" };
pub const TABLECELLS_FILL: SFSymbol = SFSymbol { name: "tablecells.fill" };
pub const TABLECELLS_FILL_BADGE_ELLIPSIS: SFSymbol = SFSymbol { name: "tablecells.fill.badge.ellipsis" };
pub const TAG: SFSymbol = SFSymbol { name: "tag" };
pub const TAG_CIRCLE: SFSymbol = SFSymbol { name: "tag.circle" };
pub const TAG_CIRCLE_FILL: SFSymbol = SFSymbol { name: "tag.circle.fill" };
pub const TAG_FILL: SFSymbol = SFSymbol { name: "tag.fill" };
pub const TAG_SLASH: SFSymbol = SFSymbol { name: "tag.slash" };
pub const TAG_SLASH_FILL: SFSymbol = SFSymbol { name: "tag.slash.fill" };
pub const TAG_SQUARE: SFSymbol = SFSymbol { name: "tag.square" };
pub const TAG_SQUARE_FILL: SFSymbol = SFSymbol { name: "tag.square.fill" };
pub const TAKEOUTBAG_AND_CUP_AND_STRAW: SFSymbol = SFSymbol { name: "takeoutbag.and.cup.and.straw" };
pub const TAKEOUTBAG_AND_CUP_AND_STRAW_FILL: SFSymbol = SFSymbol { name: "takeoutbag.and.cup.and.straw.fill" };
pub const TARGET: SFSymbol = SFSymbol { name: "target" };
pub const TEDDYBEAR: SFSymbol = SFSymbol { name: "teddybear" };
pub const TEDDYBEAR_FILL: SFSymbol = SFSymbol { name: "teddybear.fill" };
pub const TELETYPE: SFSymbol = SFSymbol { name: "teletype" };
pub const TELETYPE_ANSWER: SFSymbol = SFSymbol { name: "teletype.answer" };
pub const TELETYPE_ANSWER_CIRCLE: SFSymbol = SFSymbol { name: "teletype.answer.circle" };
pub const TELETYPE_ANSWER_CIRCLE_FILL: SFSymbol = SFSymbol { name: "teletype.answer.circle.fill" };
pub const TELETYPE_CIRCLE: SFSymbol = SFSymbol { name: "teletype.circle" };
pub const TELETYPE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "teletype.circle.fill" };
pub const TENGESIGN: SFSymbol = SFSymbol { name: "tengesign" };
pub const TENGESIGN_CIRCLE: SFSymbol = SFSymbol { name: "tengesign.circle" };
pub const TENGESIGN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "tengesign.circle.fill" };
pub const TENGESIGN_SQUARE: SFSymbol = SFSymbol { name: "tengesign.square" };
pub const TENGESIGN_SQUARE_FILL: SFSymbol = SFSymbol { name: "tengesign.square.fill" };
pub const TENNIS_RACKET: SFSymbol = SFSymbol { name: "tennis.racket" };
pub const TENNIS_RACKET_CIRCLE: SFSymbol = SFSymbol { name: "tennis.racket.circle" };
pub const TENNIS_RACKET_CIRCLE_FILL: SFSymbol = SFSymbol { name: "tennis.racket.circle.fill" };
pub const TENNISBALL: SFSymbol = SFSymbol { name: "tennisball" };
pub const TENNISBALL_CIRCLE: SFSymbol = SFSymbol { name: "tennisball.circle" };
pub const TENNISBALL_CIRCLE_FILL: SFSymbol = SFSymbol { name: "tennisball.circle.fill" };
pub const TENNISBALL_FILL: SFSymbol = SFSymbol { name: "tennisball.fill" };
pub const TENT: SFSymbol = SFSymbol { name: "tent" };
pub const TENT_FILL: SFSymbol = SFSymbol { name: "tent.fill" };
pub const TERMINAL: SFSymbol = SFSymbol { name: "terminal" };
pub const TERMINAL_FILL: SFSymbol = SFSymbol { name: "terminal.fill" };
pub const TESTTUBE_2: SFSymbol = SFSymbol { name: "testtube.2" };
pub const TEXT_ALIGNCENTER: SFSymbol = SFSymbol { name: "text.aligncenter" };
pub const TEXT_ALIGNLEFT: SFSymbol = SFSymbol { name: "text.alignleft" };
pub const TEXT_ALIGNRIGHT: SFSymbol = SFSymbol { name: "text.alignright" };
pub const TEXT_AND_COMMAND_MACWINDOW: SFSymbol = SFSymbol { name: "text.and.command.macwindow" };
pub const TEXT_APPEND: SFSymbol = SFSymbol { name: "text.append" };
pub const TEXT_BADGE_CHECKMARK: SFSymbol = SFSymbol { name: "text.badge.checkmark" };
pub const TEXT_BADGE_MINUS: SFSymbol = SFSymbol { name: "text.badge.minus" };
pub const TEXT_BADGE_PLUS: SFSymbol = SFSymbol { name: "text.badge.plus" };
pub const TEXT_BADGE_STAR: SFSymbol = SFSymbol { name: "text.badge.star" };
pub const TEXT_BADGE_XMARK: SFSymbol = SFSymbol { name: "text.badge.xmark" };
pub const TEXT_BELOW_PHOTO: SFSymbol = SFSymbol { name: "text.below.photo" };
pub const TEXT_BELOW_PHOTO_FILL: SFSymbol = SFSymbol { name: "text.below.photo.fill" };
pub const TEXT_BOOK_CLOSED: SFSymbol = SFSymbol { name: "text.book.closed" };
pub const TEXT_BOOK_CLOSED_FILL: SFSymbol = SFSymbol { name: "text.book.closed.fill" };
pub const TEXT_BUBBLE: SFSymbol = SFSymbol { name: "text.bubble" };
pub const TEXT_BUBBLE_FILL: SFSymbol = SFSymbol { name: "text.bubble.fill" };
pub const TEXT_INSERT: SFSymbol = SFSymbol { name: "text.insert" };
pub const TEXT_JUSTIFY: SFSymbol = SFSymbol { name: "text.justify" };
pub const TEXT_JUSTIFY_LEADING: SFSymbol = SFSymbol { name: "text.justify.leading" };
pub const TEXT_JUSTIFY_LEFT: SFSymbol = SFSymbol { name: "text.justify.left" };
pub const TEXT_JUSTIFY_RIGHT: SFSymbol = SFSymbol { name: "text.justify.right" };
pub const TEXT_JUSTIFY_TRAILING: SFSymbol = SFSymbol { name: "text.justify.trailing" };
pub const TEXT_LINE_FIRST_AND_ARROWTRIANGLE_FORWARD: SFSymbol = SFSymbol { name: "text.line.first.and.arrowtriangle.forward" };
pub const TEXT_LINE_LAST_AND_ARROWTRIANGLE_FORWARD: SFSymbol = SFSymbol { name: "text.line.last.and.arrowtriangle.forward" };
pub const TEXT_MAGNIFYINGGLASS: SFSymbol = SFSymbol { name: "text.magnifyingglass" };
pub const TEXT_QUOTE: SFSymbol = SFSymbol { name: "text.quote" };
pub const TEXT_REDACTION: SFSymbol = SFSymbol { name: "text.redaction" };
pub const TEXT_VIEWFINDER: SFSymbol = SFSymbol { name: "text.viewfinder" };
pub const TEXT_WORD_SPACING: SFSymbol = SFSymbol { name: "text.word.spacing" };
pub const TEXTFORMAT: SFSymbol = SFSymbol { name: "textformat" };
pub const TEXTFORMAT_12: SFSymbol = SFSymbol { name: "textformat.12" };
pub const TEXTFORMAT_123: SFSymbol = SFSymbol { name: "textformat.123" };
pub const TEXTFORMAT_ABC: SFSymbol = SFSymbol { name: "textformat.abc" };
pub const TEXTFORMAT_ABC_DOTTEDUNDERLINE: SFSymbol = SFSymbol { name: "textformat.abc.dottedunderline" };
pub const TEXTFORMAT_ALT: SFSymbol = SFSymbol { name: "textformat.alt" };
pub const TEXTFORMAT_SIZE: SFSymbol = SFSymbol { name: "textformat.size" };
pub const TEXTFORMAT_SIZE_LARGER: SFSymbol = SFSymbol { name: "textformat.size.larger" };
pub const TEXTFORMAT_SIZE_SMALLER: SFSymbol = SFSymbol { name: "textformat.size.smaller" };
pub const TEXTFORMAT_SUBSCRIPT: SFSymbol = SFSymbol { name: "textformat.subscript" };
pub const TEXTFORMAT_SUPERSCRIPT: SFSymbol = SFSymbol { name: "textformat.superscript" };
pub const THEATERMASK_AND_PAINTBRUSH: SFSymbol = SFSymbol { name: "theatermask.and.paintbrush" };
pub const THEATERMASK_AND_PAINTBRUSH_FILL: SFSymbol = SFSymbol { name: "theatermask.and.paintbrush.fill" };
pub const THEATERMASKS: SFSymbol = SFSymbol { name: "theatermasks" };
pub const THEATERMASKS_CIRCLE: SFSymbol = SFSymbol { name: "theatermasks.circle" };
pub const THEATERMASKS_CIRCLE_FILL: SFSymbol = SFSymbol { name: "theatermasks.circle.fill" };
pub const THEATERMASKS_FILL: SFSymbol = SFSymbol { name: "theatermasks.fill" };
pub const THERMOMETER: SFSymbol = SFSymbol { name: "thermometer" };
pub const THERMOMETER_HIGH: SFSymbol = SFSymbol { name: "thermometer.high" };
pub const THERMOMETER_LOW: SFSymbol = SFSymbol { name: "thermometer.low" };
pub const THERMOMETER_MEDIUM: SFSymbol = SFSymbol { name: "thermometer.medium" };
pub const THERMOMETER_MEDIUM_SLASH: SFSymbol = SFSymbol { name: "thermometer.medium.slash" };
pub const THERMOMETER_SNOWFLAKE: SFSymbol = SFSymbol { name: "thermometer.snowflake" };
pub const THERMOMETER_SNOWFLAKE_CIRCLE: SFSymbol = SFSymbol { name: "thermometer.snowflake.circle" };
pub const THERMOMETER_SNOWFLAKE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "thermometer.snowflake.circle.fill" };
pub const THERMOMETER_SUN: SFSymbol = SFSymbol { name: "thermometer.sun" };
pub const THERMOMETER_SUN_CIRCLE: SFSymbol = SFSymbol { name: "thermometer.sun.circle" };
pub const THERMOMETER_SUN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "thermometer.sun.circle.fill" };
pub const THERMOMETER_SUN_FILL: SFSymbol = SFSymbol { name: "thermometer.sun.fill" };
pub const TICKET: SFSymbol = SFSymbol { name: "ticket" };
pub const TICKET_FILL: SFSymbol = SFSymbol { name: "ticket.fill" };
pub const TIMELAPSE: SFSymbol = SFSymbol { name: "timelapse" };
pub const TIMELINE_SELECTION: SFSymbol = SFSymbol { name: "timeline.selection" };
pub const TIMER: SFSymbol = SFSymbol { name: "timer" };
pub const TIMER_CIRCLE: SFSymbol = SFSymbol { name: "timer.circle" };
pub const TIMER_CIRCLE_FILL: SFSymbol = SFSymbol { name: "timer.circle.fill" };
pub const TIMER_SQUARE: SFSymbol = SFSymbol { name: "timer.square" };
pub const TOGGLEPOWER: SFSymbol = SFSymbol { name: "togglepower" };
pub const TOILET: SFSymbol = SFSymbol { name: "toilet" };
pub const TOILET_FILL: SFSymbol = SFSymbol { name: "toilet.fill" };
pub const TORNADO: SFSymbol = SFSymbol { name: "tornado" };
pub const TORNADO_CIRCLE: SFSymbol = SFSymbol { name: "tornado.circle" };
pub const TORNADO_CIRCLE_FILL: SFSymbol = SFSymbol { name: "tornado.circle.fill" };
pub const TORTOISE: SFSymbol = SFSymbol { name: "tortoise" };
pub const TORTOISE_FILL: SFSymbol = SFSymbol { name: "tortoise.fill" };
pub const TORUS: SFSymbol = SFSymbol { name: "torus" };
pub const TOUCHID: SFSymbol = SFSymbol { name: "touchid" };
pub const TRAIN_SIDE_FRONT_CAR: SFSymbol = SFSymbol { name: "train.side.front.car" };
pub const TRAIN_SIDE_MIDDLE_CAR: SFSymbol = SFSymbol { name: "train.side.middle.car" };
pub const TRAIN_SIDE_REAR_CAR: SFSymbol = SFSymbol { name: "train.side.rear.car" };
pub const TRAM: SFSymbol = SFSymbol { name: "tram" };
pub const TRAM_CIRCLE: SFSymbol = SFSymbol { name: "tram.circle" };
pub const TRAM_CIRCLE_FILL: SFSymbol = SFSymbol { name: "tram.circle.fill" };
pub const TRAM_FILL: SFSymbol = SFSymbol { name: "tram.fill" };
pub const TRAM_FILL_TUNNEL: SFSymbol = SFSymbol { name: "tram.fill.tunnel" };
pub const TRAPEZOID_AND_LINE_HORIZONTAL: SFSymbol = SFSymbol { name: "trapezoid.and.line.horizontal" };
pub const TRAPEZOID_AND_LINE_HORIZONTAL_FILL: SFSymbol = SFSymbol { name: "trapezoid.and.line.horizontal.fill" };
pub const TRAPEZOID_AND_LINE_VERTICAL: SFSymbol = SFSymbol { name: "trapezoid.and.line.vertical" };
pub const TRAPEZOID_AND_LINE_VERTICAL_FILL: SFSymbol = SFSymbol { name: "trapezoid.and.line.vertical.fill" };
pub const TRASH: SFSymbol = SFSymbol { name: "trash" };
pub const TRASH_CIRCLE: SFSymbol = SFSymbol { name: "trash.circle" };
pub const TRASH_CIRCLE_FILL: SFSymbol = SFSymbol { name: "trash.circle.fill" };
pub const TRASH_FILL: SFSymbol = SFSymbol { name: "trash.fill" };
pub const TRASH_SLASH: SFSymbol = SFSymbol { name: "trash.slash" };
pub const TRASH_SLASH_CIRCLE: SFSymbol = SFSymbol { name: "trash.slash.circle" };
pub const TRASH_SLASH_CIRCLE_FILL: SFSymbol = SFSymbol { name: "trash.slash.circle.fill" };
pub const TRASH_SLASH_FILL: SFSymbol = SFSymbol { name: "trash.slash.fill" };
pub const TRASH_SLASH_SQUARE: SFSymbol = SFSymbol { name: "trash.slash.square" };
pub const TRASH_SLASH_SQUARE_FILL: SFSymbol = SFSymbol { name: "trash.slash.square.fill" };
pub const TRASH_SQUARE: SFSymbol = SFSymbol { name: "trash.square" };
pub const TRASH_SQUARE_FILL: SFSymbol = SFSymbol { name: "trash.square.fill" };
pub const TRAY: SFSymbol = SFSymbol { name: "tray" };
pub const TRAY_2: SFSymbol = SFSymbol { name: "tray.2" };
pub const TRAY_2_FILL: SFSymbol = SFSymbol { name: "tray.2.fill" };
pub const TRAY_AND_ARROW_DOWN: SFSymbol = SFSymbol { name: "tray.and.arrow.down" };
pub const TRAY_AND_ARROW_DOWN_FILL: SFSymbol = SFSymbol { name: "tray.and.arrow.down.fill" };
pub const TRAY_AND_ARROW_UP: SFSymbol = SFSymbol { name: "tray.and.arrow.up" };
pub const TRAY_AND_ARROW_UP_FILL: SFSymbol = SFSymbol { name: "tray.and.arrow.up.fill" };
pub const TRAY_CIRCLE: SFSymbol = SFSymbol { name: "tray.circle" };
pub const TRAY_CIRCLE_FILL: SFSymbol = SFSymbol { name: "tray.circle.fill" };
pub const TRAY_FILL: SFSymbol = SFSymbol { name: "tray.fill" };
pub const TRAY_FULL: SFSymbol = SFSymbol { name: "tray.full" };
pub const TRAY_FULL_FILL: SFSymbol = SFSymbol { name: "tray.full.fill" };
pub const TRIANGLE: SFSymbol = SFSymbol { name: "triangle" };
pub const TRIANGLE_BOTTOMHALF_FILLED: SFSymbol = SFSymbol { name: "triangle.bottomhalf.filled" };
pub const TRIANGLE_CIRCLE: SFSymbol = SFSymbol { name: "triangle.circle" };
pub const TRIANGLE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "triangle.circle.fill" };
pub const TRIANGLE_FILL: SFSymbol = SFSymbol { name: "triangle.fill" };
pub const TRIANGLE_INSET_FILLED: SFSymbol = SFSymbol { name: "triangle.inset.filled" };
pub const TRIANGLE_LEFTHALF_FILLED: SFSymbol = SFSymbol { name: "triangle.lefthalf.filled" };
pub const TRIANGLE_RIGHTHALF_FILLED: SFSymbol = SFSymbol { name: "triangle.righthalf.filled" };
pub const TRIANGLE_TOPHALF_FILLED: SFSymbol = SFSymbol { name: "triangle.tophalf.filled" };
pub const TROPHY: SFSymbol = SFSymbol { name: "trophy" };
pub const TROPHY_CIRCLE: SFSymbol = SFSymbol { name: "trophy.circle" };
pub const TROPHY_CIRCLE_FILL: SFSymbol = SFSymbol { name: "trophy.circle.fill" };
pub const TROPHY_FILL: SFSymbol = SFSymbol { name: "trophy.fill" };
pub const TROPICALSTORM: SFSymbol = SFSymbol { name: "tropicalstorm" };
pub const TROPICALSTORM_CIRCLE: SFSymbol = SFSymbol { name: "tropicalstorm.circle" };
pub const TROPICALSTORM_CIRCLE_FILL: SFSymbol = SFSymbol { name: "tropicalstorm.circle.fill" };
pub const TSHIRT: SFSymbol = SFSymbol { name: "tshirt" };
pub const TSHIRT_FILL: SFSymbol = SFSymbol { name: "tshirt.fill" };
pub const TUGRIKSIGN: SFSymbol = SFSymbol { name: "tugriksign" };
pub const TUGRIKSIGN_CIRCLE: SFSymbol = SFSymbol { name: "tugriksign.circle" };
pub const TUGRIKSIGN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "tugriksign.circle.fill" };
pub const TUGRIKSIGN_SQUARE: SFSymbol = SFSymbol { name: "tugriksign.square" };
pub const TUGRIKSIGN_SQUARE_FILL: SFSymbol = SFSymbol { name: "tugriksign.square.fill" };
pub const TUNINGFORK: SFSymbol = SFSymbol { name: "tuningfork" };
pub const TURKISHLIRASIGN: SFSymbol = SFSymbol { name: "turkishlirasign" };
pub const TURKISHLIRASIGN_CIRCLE: SFSymbol = SFSymbol { name: "turkishlirasign.circle" };
pub const TURKISHLIRASIGN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "turkishlirasign.circle.fill" };
pub const TURKISHLIRASIGN_SQUARE: SFSymbol = SFSymbol { name: "turkishlirasign.square" };
pub const TURKISHLIRASIGN_SQUARE_FILL: SFSymbol = SFSymbol { name: "turkishlirasign.square.fill" };
pub const TV: SFSymbol = SFSymbol { name: "tv" };
pub const TV_AND_HIFISPEAKER_FILL: SFSymbol = SFSymbol { name: "tv.and.hifispeaker.fill" };
pub const TV_AND_MEDIABOX: SFSymbol = SFSymbol { name: "tv.and.mediabox" };
pub const TV_CIRCLE: SFSymbol = SFSymbol { name: "tv.circle" };
pub const TV_CIRCLE_FILL: SFSymbol = SFSymbol { name: "tv.circle.fill" };
pub const TV_FILL: SFSymbol = SFSymbol { name: "tv.fill" };
pub const TV_INSET_FILLED: SFSymbol = SFSymbol { name: "tv.inset.filled" };
pub const U_CIRCLE: SFSymbol = SFSymbol { name: "u.circle" };
pub const U_CIRCLE_FILL: SFSymbol = SFSymbol { name: "u.circle.fill" };
pub const U_SQUARE: SFSymbol = SFSymbol { name: "u.square" };
pub const U_SQUARE_FILL: SFSymbol = SFSymbol { name: "u.square.fill" };
pub const UIWINDOW_SPLIT_2X1: SFSymbol = SFSymbol { name: "uiwindow.split.2x1" };
pub const UMBRELLA: SFSymbol = SFSymbol { name: "umbrella" };
pub const UMBRELLA_FILL: SFSymbol = SFSymbol { name: "umbrella.fill" };
pub const UMBRELLA_PERCENT: SFSymbol = SFSymbol { name: "umbrella.percent" };
pub const UMBRELLA_PERCENT_FILL: SFSymbol = SFSymbol { name: "umbrella.percent.fill" };
pub const UNDERLINE: SFSymbol = SFSymbol { name: "underline" };
pub const V_CIRCLE: SFSymbol = SFSymbol { name: "v.circle" };
pub const V_CIRCLE_FILL: SFSymbol = SFSymbol { name: "v.circle.fill" };
pub const V_SQUARE: SFSymbol = SFSymbol { name: "v.square" };
pub const V_SQUARE_FILL: SFSymbol = SFSymbol { name: "v.square.fill" };
pub const VIAL_VIEWFINDER: SFSymbol = SFSymbol { name: "vial.viewfinder" };
pub const VIDEO: SFSymbol = SFSymbol { name: "video" };
pub const VIDEO_AND_WAVEFORM: SFSymbol = SFSymbol { name: "video.and.waveform" };
pub const VIDEO_AND_WAVEFORM_FILL: SFSymbol = SFSymbol { name: "video.and.waveform.fill" };
pub const VIDEO_BADGE_CHECKMARK: SFSymbol = SFSymbol { name: "video.badge.checkmark" };
pub const VIDEO_BADGE_ELLIPSIS: SFSymbol = SFSymbol { name: "video.badge.ellipsis" };
pub const VIDEO_BADGE_PLUS: SFSymbol = SFSymbol { name: "video.badge.plus" };
pub const VIDEO_BUBBLE_LEFT: SFSymbol = SFSymbol { name: "video.bubble.left" };
pub const VIDEO_BUBBLE_LEFT_FILL: SFSymbol = SFSymbol { name: "video.bubble.left.fill" };
pub const VIDEO_CIRCLE: SFSymbol = SFSymbol { name: "video.circle" };
pub const VIDEO_CIRCLE_FILL: SFSymbol = SFSymbol { name: "video.circle.fill" };
pub const VIDEO_DOORBELL: SFSymbol = SFSymbol { name: "video.doorbell" };
pub const VIDEO_DOORBELL_FILL: SFSymbol = SFSymbol { name: "video.doorbell.fill" };
pub const VIDEO_FILL: SFSymbol = SFSymbol { name: "video.fill" };
pub const VIDEO_FILL_BADGE_CHECKMARK: SFSymbol = SFSymbol { name: "video.fill.badge.checkmark" };
pub const VIDEO_FILL_BADGE_ELLIPSIS: SFSymbol = SFSymbol { name: "video.fill.badge.ellipsis" };
pub const VIDEO_FILL_BADGE_PLUS: SFSymbol = SFSymbol { name: "video.fill.badge.plus" };
pub const VIDEO_SLASH: SFSymbol = SFSymbol { name: "video.slash" };
pub const VIDEO_SLASH_FILL: SFSymbol = SFSymbol { name: "video.slash.fill" };
pub const VIDEO_SQUARE: SFSymbol = SFSymbol { name: "video.square" };
pub const VIDEO_SQUARE_FILL: SFSymbol = SFSymbol { name: "video.square.fill" };
pub const VIDEOPROJECTOR: SFSymbol = SFSymbol { name: "videoprojector" };
pub const VIDEOPROJECTOR_FILL: SFSymbol = SFSymbol { name: "videoprojector.fill" };
pub const VIEW_2D: SFSymbol = SFSymbol { name: "view.2d" };
pub const VIEW_3D: SFSymbol = SFSymbol { name: "view.3d" };
pub const VIEWFINDER: SFSymbol = SFSymbol { name: "viewfinder" };
pub const VIEWFINDER_CIRCLE: SFSymbol = SFSymbol { name: "viewfinder.circle" };
pub const VIEWFINDER_CIRCLE_FILL: SFSymbol = SFSymbol { name: "viewfinder.circle.fill" };
pub const VOLLEYBALL: SFSymbol = SFSymbol { name: "volleyball" };
pub const VOLLEYBALL_CIRCLE: SFSymbol = SFSymbol { name: "volleyball.circle" };
pub const VOLLEYBALL_CIRCLE_FILL: SFSymbol = SFSymbol { name: "volleyball.circle.fill" };
pub const VOLLEYBALL_FILL: SFSymbol = SFSymbol { name: "volleyball.fill" };
pub const W_CIRCLE: SFSymbol = SFSymbol { name: "w.circle" };
pub const W_CIRCLE_FILL: SFSymbol = SFSymbol { name: "w.circle.fill" };
pub const W_SQUARE: SFSymbol = SFSymbol { name: "w.square" };
pub const W_SQUARE_FILL: SFSymbol = SFSymbol { name: "w.square.fill" };
pub const WAKE: SFSymbol = SFSymbol { name: "wake" };
pub const WAKE_CIRCLE: SFSymbol = SFSymbol { name: "wake.circle" };
pub const WAKE_CIRCLE_FILL: SFSymbol = SFSymbol { name: "wake.circle.fill" };
pub const WALLET_PASS: SFSymbol = SFSymbol { name: "wallet.pass" };
pub const WALLET_PASS_FILL: SFSymbol = SFSymbol { name: "wallet.pass.fill" };
pub const WAND_AND_RAYS: SFSymbol = SFSymbol { name: "wand.and.rays" };
pub const WAND_AND_RAYS_INVERSE: SFSymbol = SFSymbol { name: "wand.and.rays.inverse" };
pub const WAND_AND_STARS: SFSymbol = SFSymbol { name: "wand.and.stars" };
pub const WAND_AND_STARS_INVERSE: SFSymbol = SFSymbol { name: "wand.and.stars.inverse" };
pub const WASHER: SFSymbol = SFSymbol { name: "washer" };
pub const WASHER_FILL: SFSymbol = SFSymbol { name: "washer.fill" };
pub const WATCHFACE_APPLEWATCH_CASE: SFSymbol = SFSymbol { name: "watchface.applewatch.case" };
pub const WATER_WAVES: SFSymbol = SFSymbol { name: "water.waves" };
pub const WATER_WAVES_AND_ARROW_DOWN: SFSymbol = SFSymbol { name: "water.waves.and.arrow.down" };
pub const WATER_WAVES_AND_ARROW_DOWN_TRIANGLEBADGE_EXCLAMATIONMARK: SFSymbol = SFSymbol { name: "water.waves.and.arrow.down.trianglebadge.exclamationmark" };
pub const WATER_WAVES_AND_ARROW_UP: SFSymbol = SFSymbol { name: "water.waves.and.arrow.up" };
pub const WATER_WAVES_SLASH: SFSymbol = SFSymbol { name: "water.waves.slash" };
pub const WAVE_3_BACKWARD: SFSymbol = SFSymbol { name: "wave.3.backward" };
pub const WAVE_3_BACKWARD_CIRCLE: SFSymbol = SFSymbol { name: "wave.3.backward.circle" };
pub const WAVE_3_BACKWARD_CIRCLE_FILL: SFSymbol = SFSymbol { name: "wave.3.backward.circle.fill" };
pub const WAVE_3_FORWARD: SFSymbol = SFSymbol { name: "wave.3.forward" };
pub const WAVE_3_FORWARD_CIRCLE: SFSymbol = SFSymbol { name: "wave.3.forward.circle" };
pub const WAVE_3_FORWARD_CIRCLE_FILL: SFSymbol = SFSymbol { name: "wave.3.forward.circle.fill" };
pub const WAVE_3_LEFT: SFSymbol = SFSymbol { name: "wave.3.left" };
pub const WAVE_3_LEFT_CIRCLE: SFSymbol = SFSymbol { name: "wave.3.left.circle" };
pub const WAVE_3_LEFT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "wave.3.left.circle.fill" };
pub const WAVE_3_RIGHT: SFSymbol = SFSymbol { name: "wave.3.right" };
pub const WAVE_3_RIGHT_CIRCLE: SFSymbol = SFSymbol { name: "wave.3.right.circle" };
pub const WAVE_3_RIGHT_CIRCLE_FILL: SFSymbol = SFSymbol { name: "wave.3.right.circle.fill" };
pub const WAVEFORM: SFSymbol = SFSymbol { name: "waveform" };
pub const WAVEFORM_AND_MAGNIFYINGGLASS: SFSymbol = SFSymbol { name: "waveform.and.magnifyingglass" };
pub const WAVEFORM_AND_MIC: SFSymbol = SFSymbol { name: "waveform.and.mic" };
pub const WAVEFORM_BADGE_EXCLAMATIONMARK: SFSymbol = SFSymbol { name: "waveform.badge.exclamationmark" };
pub const WAVEFORM_BADGE_MINUS: SFSymbol = SFSymbol { name: "waveform.badge.minus" };
pub const WAVEFORM_BADGE_PLUS: SFSymbol = SFSymbol { name: "waveform.badge.plus" };
pub const WAVEFORM_CIRCLE: SFSymbol = SFSymbol { name: "waveform.circle" };
pub const WAVEFORM_CIRCLE_FILL: SFSymbol = SFSymbol { name: "waveform.circle.fill" };
pub const WAVEFORM_PATH: SFSymbol = SFSymbol { name: "waveform.path" };
pub const WAVEFORM_PATH_BADGE_MINUS: SFSymbol = SFSymbol { name: "waveform.path.badge.minus" };
pub const WAVEFORM_PATH_BADGE_PLUS: SFSymbol = SFSymbol { name: "waveform.path.badge.plus" };
pub const WAVEFORM_PATH_ECG: SFSymbol = SFSymbol { name: "waveform.path.ecg" };
pub const WAVEFORM_PATH_ECG_RECTANGLE: SFSymbol = SFSymbol { name: "waveform.path.ecg.rectangle" };
pub const WAVEFORM_PATH_ECG_RECTANGLE_FILL: SFSymbol = SFSymbol { name: "waveform.path.ecg.rectangle.fill" };
pub const WAVEFORM_SLASH: SFSymbol = SFSymbol { name: "waveform.slash" };
pub const WEB_CAMERA: SFSymbol = SFSymbol { name: "web.camera" };
pub const WEB_CAMERA_FILL: SFSymbol = SFSymbol { name: "web.camera.fill" };
pub const WIFI: SFSymbol = SFSymbol { name: "wifi" };
pub const WIFI_CIRCLE: SFSymbol = SFSymbol { name: "wifi.circle" };
pub const WIFI_CIRCLE_FILL: SFSymbol = SFSymbol { name: "wifi.circle.fill" };
pub const WIFI_EXCLAMATIONMARK: SFSymbol = SFSymbol { name: "wifi.exclamationmark" };
pub const WIFI_ROUTER: SFSymbol = SFSymbol { name: "wifi.router" };
pub const WIFI_ROUTER_FILL: SFSymbol = SFSymbol { name: "wifi.router.fill" };
pub const WIFI_SLASH: SFSymbol = SFSymbol { name: "wifi.slash" };
pub const WIFI_SQUARE: SFSymbol = SFSymbol { name: "wifi.square" };
pub const WIFI_SQUARE_FILL: SFSymbol = SFSymbol { name: "wifi.square.fill" };
pub const WIND: SFSymbol = SFSymbol { name: "wind" };
pub const WIND_CIRCLE: SFSymbol = SFSymbol { name: "wind.circle" };
pub const WIND_CIRCLE_FILL: SFSymbol = SFSymbol { name: "wind.circle.fill" };
pub const WIND_SNOW: SFSymbol = SFSymbol { name: "wind.snow" };
pub const WIND_SNOW_CIRCLE: SFSymbol = SFSymbol { name: "wind.snow.circle" };
pub const WIND_SNOW_CIRCLE_FILL: SFSymbol = SFSymbol { name: "wind.snow.circle.fill" };
pub const WINDOW_AWNING: SFSymbol = SFSymbol { name: "window.awning" };
pub const WINDOW_AWNING_CLOSED: SFSymbol = SFSymbol { name: "window.awning.closed" };
pub const WINDOW_CASEMENT: SFSymbol = SFSymbol { name: "window.casement" };
pub const WINDOW_CASEMENT_CLOSED: SFSymbol = SFSymbol { name: "window.casement.closed" };
pub const WINDOW_CEILING: SFSymbol = SFSymbol { name: "window.ceiling" };
pub const WINDOW_CEILING_CLOSED: SFSymbol = SFSymbol { name: "window.ceiling.closed" };
pub const WINDOW_HORIZONTAL: SFSymbol = SFSymbol { name: "window.horizontal" };
pub const WINDOW_HORIZONTAL_CLOSED: SFSymbol = SFSymbol { name: "window.horizontal.closed" };
pub const WINDOW_SHADE_CLOSED: SFSymbol = SFSymbol { name: "window.shade.closed" };
pub const WINDOW_SHADE_OPEN: SFSymbol = SFSymbol { name: "window.shade.open" };
pub const WINDOW_VERTICAL_CLOSED: SFSymbol = SFSymbol { name: "window.vertical.closed" };
pub const WINDOW_VERTICAL_OPEN: SFSymbol = SFSymbol { name: "window.vertical.open" };
pub const WINEGLASS: SFSymbol = SFSymbol { name: "wineglass" };
pub const WINEGLASS_FILL: SFSymbol = SFSymbol { name: "wineglass.fill" };
pub const WONSIGN: SFSymbol = SFSymbol { name: "wonsign" };
pub const WONSIGN_CIRCLE: SFSymbol = SFSymbol { name: "wonsign.circle" };
pub const WONSIGN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "wonsign.circle.fill" };
pub const WONSIGN_SQUARE: SFSymbol = SFSymbol { name: "wonsign.square" };
pub const WONSIGN_SQUARE_FILL: SFSymbol = SFSymbol { name: "wonsign.square.fill" };
pub const WRENCH: SFSymbol = SFSymbol { name: "wrench" };
pub const WRENCH_ADJUSTABLE: SFSymbol = SFSymbol { name: "wrench.adjustable" };
pub const WRENCH_ADJUSTABLE_FILL: SFSymbol = SFSymbol { name: "wrench.adjustable.fill" };
pub const WRENCH_AND_SCREWDRIVER: SFSymbol = SFSymbol { name: "wrench.and.screwdriver" };
pub const WRENCH_AND_SCREWDRIVER_FILL: SFSymbol = SFSymbol { name: "wrench.and.screwdriver.fill" };
pub const WRENCH_FILL: SFSymbol = SFSymbol { name: "wrench.fill" };
pub const X_CIRCLE: SFSymbol = SFSymbol { name: "x.circle" };
pub const X_CIRCLE_FILL: SFSymbol = SFSymbol { name: "x.circle.fill" };
pub const X_SQUARE: SFSymbol = SFSymbol { name: "x.square" };
pub const X_SQUARE_FILL: SFSymbol = SFSymbol { name: "x.square.fill" };
pub const X_SQUAREROOT: SFSymbol = SFSymbol { name: "x.squareroot" };
pub const XBOX_LOGO: SFSymbol = SFSymbol { name: "xbox.logo" };
pub const XMARK: SFSymbol = SFSymbol { name: "xmark" };
pub const XMARK_APP: SFSymbol = SFSymbol { name: "xmark.app" };
pub const XMARK_APP_FILL: SFSymbol = SFSymbol { name: "xmark.app.fill" };
pub const XMARK_BIN: SFSymbol = SFSymbol { name: "xmark.bin" };
pub const XMARK_BIN_CIRCLE: SFSymbol = SFSymbol { name: "xmark.bin.circle" };
pub const XMARK_BIN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "xmark.bin.circle.fill" };
pub const XMARK_BIN_FILL: SFSymbol = SFSymbol { name: "xmark.bin.fill" };
pub const XMARK_CIRCLE: SFSymbol = SFSymbol { name: "xmark.circle" };
pub const XMARK_CIRCLE_FILL: SFSymbol = SFSymbol { name: "xmark.circle.fill" };
pub const XMARK_DIAMOND: SFSymbol = SFSymbol { name: "xmark.diamond" };
pub const XMARK_DIAMOND_FILL: SFSymbol = SFSymbol { name: "xmark.diamond.fill" };
pub const XMARK_ICLOUD: SFSymbol = SFSymbol { name: "xmark.icloud" };
pub const XMARK_ICLOUD_FILL: SFSymbol = SFSymbol { name: "xmark.icloud.fill" };
pub const XMARK_OCTAGON: SFSymbol = SFSymbol { name: "xmark.octagon" };
pub const XMARK_OCTAGON_FILL: SFSymbol = SFSymbol { name: "xmark.octagon.fill" };
pub const XMARK_RECTANGLE: SFSymbol = SFSymbol { name: "xmark.rectangle" };
pub const XMARK_RECTANGLE_FILL: SFSymbol = SFSymbol { name: "xmark.rectangle.fill" };
pub const XMARK_RECTANGLE_PORTRAIT: SFSymbol = SFSymbol { name: "xmark.rectangle.portrait" };
pub const XMARK_RECTANGLE_PORTRAIT_FILL: SFSymbol = SFSymbol { name: "xmark.rectangle.portrait.fill" };
pub const XMARK_SEAL: SFSymbol = SFSymbol { name: "xmark.seal" };
pub const XMARK_SEAL_FILL: SFSymbol = SFSymbol { name: "xmark.seal.fill" };
pub const XMARK_SHIELD: SFSymbol = SFSymbol { name: "xmark.shield" };
pub const XMARK_SHIELD_FILL: SFSymbol = SFSymbol { name: "xmark.shield.fill" };
pub const XMARK_SQUARE: SFSymbol = SFSymbol { name: "xmark.square" };
pub const XMARK_SQUARE_FILL: SFSymbol = SFSymbol { name: "xmark.square.fill" };
pub const XSERVE: SFSymbol = SFSymbol { name: "xserve" };
pub const Y_CIRCLE: SFSymbol = SFSymbol { name: "y.circle" };
pub const Y_CIRCLE_FILL: SFSymbol = SFSymbol { name: "y.circle.fill" };
pub const Y_SQUARE: SFSymbol = SFSymbol { name: "y.square" };
pub const Y_SQUARE_FILL: SFSymbol = SFSymbol { name: "y.square.fill" };
pub const YENSIGN: SFSymbol = SFSymbol { name: "yensign" };
pub const YENSIGN_CIRCLE: SFSymbol = SFSymbol { name: "yensign.circle" };
pub const YENSIGN_CIRCLE_FILL: SFSymbol = SFSymbol { name: "yensign.circle.fill" };
pub const YENSIGN_SQUARE: SFSymbol = SFSymbol { name: "yensign.square" };
pub const YENSIGN_SQUARE_FILL: SFSymbol = SFSymbol { name: "yensign.square.fill" };
pub const Z_CIRCLE: SFSymbol = SFSymbol { name: "z.circle" };
pub const Z_CIRCLE_FILL: SFSymbol = SFSymbol { name: "z.circle.fill" };
pub const Z_SQUARE: SFSymbol = SFSymbol { name: "z.square" };
pub const Z_SQUARE_FILL: SFSymbol = SFSymbol { name: "z.square.fill" };
pub const ZL_RECTANGLE_ROUNDEDTOP: SFSymbol = SFSymbol { name: "zl.rectangle.roundedtop" };
pub const ZL_RECTANGLE_ROUNDEDTOP_FILL: SFSymbol = SFSymbol { name: "zl.rectangle.roundedtop.fill" };
pub const ZR_RECTANGLE_ROUNDEDTOP: SFSymbol = SFSymbol { name: "zr.rectangle.roundedtop" };
pub const ZR_RECTANGLE_ROUNDEDTOP_FILL: SFSymbol = SFSymbol { name: "zr.rectangle.roundedtop.fill" };
pub const ZZZ: SFSymbol = SFSymbol { name: "zzz" };

pub const ALL: &[SFSymbol] = &[
    _0_CIRCLE,
    _0_CIRCLE_FILL,
    _0_SQUARE,
    _0_SQUARE_FILL,
    _00_CIRCLE,
    _00_CIRCLE_FILL,
    _00_SQUARE,
    _00_SQUARE_FILL,
    _01_CIRCLE,
    _01_CIRCLE_FILL,
    _01_SQUARE,
    _01_SQUARE_FILL,
    _02_CIRCLE,
    _02_CIRCLE_FILL,
    _02_SQUARE,
    _02_SQUARE_FILL,
    _03_CIRCLE,
    _03_CIRCLE_FILL,
    _03_SQUARE,
    _03_SQUARE_FILL,
    _04_CIRCLE,
    _04_CIRCLE_FILL,
    _04_SQUARE,
    _04_SQUARE_FILL,
    _05_CIRCLE,
    _05_CIRCLE_FILL,
    _05_SQUARE,
    _05_SQUARE_FILL,
    _06_CIRCLE,
    _06_CIRCLE_FILL,
    _06_SQUARE,
    _06_SQUARE_FILL,
    _07_CIRCLE,
    _07_CIRCLE_FILL,
    _07_SQUARE,
    _07_SQUARE_FILL,
    _08_CIRCLE,
    _08_CIRCLE_FILL,
    _08_SQUARE,
    _08_SQUARE_FILL,
    _09_CIRCLE,
    _09_CIRCLE_FILL,
    _09_SQUARE,
    _09_SQUARE_FILL,
    _1_CIRCLE,
    _1_CIRCLE_FILL,
    _1_MAGNIFYINGGLASS,
    _1_SQUARE,
    _1_SQUARE_FILL,
    _10_CIRCLE,
    _10_CIRCLE_FILL,
    _10_SQUARE,
    _10_SQUARE_FILL,
    _11_CIRCLE,
    _11_CIRCLE_FILL,
    _11_SQUARE,
    _11_SQUARE_FILL,
    _12_CIRCLE,
    _12_CIRCLE_FILL,
    _12_SQUARE,
    _12_SQUARE_FILL,
    _123_RECTANGLE,
    _123_RECTANGLE_FILL,
    _13_CIRCLE,
    _13_CIRCLE_FILL,
    _13_SQUARE,
    _13_SQUARE_FILL,
    _14_CIRCLE,
    _14_CIRCLE_FILL,
    _14_SQUARE,
    _14_SQUARE_FILL,
    _15_CIRCLE,
    _15_CIRCLE_FILL,
    _15_SQUARE,
    _15_SQUARE_FILL,
    _16_CIRCLE,
    _16_CIRCLE_FILL,
    _16_SQUARE,
    _16_SQUARE_FILL,
    _17_CIRCLE,
    _17_CIRCLE_FILL,
    _17_SQUARE,
    _17_SQUARE_FILL,
    _18_CIRCLE,
    _18_CIRCLE_FILL,
    _18_SQUARE,
    _18_SQUARE_FILL,
    _19_CIRCLE,
    _19_CIRCLE_FILL,
    _19_SQUARE,
    _19_SQUARE_FILL,
    _2_CIRCLE,
    _2_CIRCLE_FILL,
    _2_SQUARE,
    _2_SQUARE_FILL,
    _20_CIRCLE,
    _20_CIRCLE_FILL,
    _20_SQUARE,
    _20_SQUARE_FILL,
    _21_CIRCLE,
    _21_CIRCLE_FILL,
    _21_SQUARE,
    _21_SQUARE_FILL,
    _22_CIRCLE,
    _22_CIRCLE_FILL,
    _22_SQUARE,
    _22_SQUARE_FILL,
    _23_CIRCLE,
    _23_CIRCLE_FILL,
    _23_SQUARE,
    _23_SQUARE_FILL,
    _24_CIRCLE,
    _24_CIRCLE_FILL,
    _24_SQUARE,
    _24_SQUARE_FILL,
    _25_CIRCLE,
    _25_CIRCLE_FILL,
    _25_SQUARE,
    _25_SQUARE_FILL,
    _26_CIRCLE,
    _26_CIRCLE_FILL,
    _26_SQUARE,
    _26_SQUARE_FILL,
    _27_CIRCLE,
    _27_CIRCLE_FILL,
    _27_SQUARE,
    _27_SQUARE_FILL,
    _28_CIRCLE,
    _28_CIRCLE_FILL,
    _28_SQUARE,
    _28_SQUARE_FILL,
    _29_CIRCLE,
    _29_CIRCLE_FILL,
    _29_SQUARE,
    _29_SQUARE_FILL,
    _3_CIRCLE,
    _3_CIRCLE_FILL,
    _3_SQUARE,
    _3_SQUARE_FILL,
    _30_CIRCLE,
    _30_CIRCLE_FILL,
    _30_SQUARE,
    _30_SQUARE_FILL,
    _31_CIRCLE,
    _31_CIRCLE_FILL,
    _31_SQUARE,
    _31_SQUARE_FILL,
    _32_CIRCLE,
    _32_CIRCLE_FILL,
    _32_SQUARE,
    _32_SQUARE_FILL,
    _33_CIRCLE,
    _33_CIRCLE_FILL,
    _33_SQUARE,
    _33_SQUARE_FILL,
    _34_CIRCLE,
    _34_CIRCLE_FILL,
    _34_SQUARE,
    _34_SQUARE_FILL,
    _35_CIRCLE,
    _35_CIRCLE_FILL,
    _35_SQUARE,
    _35_SQUARE_FILL,
    _36_CIRCLE,
    _36_CIRCLE_FILL,
    _36_SQUARE,
    _36_SQUARE_FILL,
    _37_CIRCLE,
    _37_CIRCLE_FILL,
    _37_SQUARE,
    _37_SQUARE_FILL,
    _38_CIRCLE,
    _38_CIRCLE_FILL,
    _38_SQUARE,
    _38_SQUARE_FILL,
    _39_CIRCLE,
    _39_CIRCLE_FILL,
    _39_SQUARE,
    _39_SQUARE_FILL,
    _4_ALT_CIRCLE,
    _4_ALT_CIRCLE_FILL,
    _4_ALT_SQUARE,
    _4_ALT_SQUARE_FILL,
    _4_CIRCLE,
    _4_CIRCLE_FILL,
    _4_SQUARE,
    _4_SQUARE_FILL,
    _40_CIRCLE,
    _40_CIRCLE_FILL,
    _40_SQUARE,
    _40_SQUARE_FILL,
    _41_CIRCLE,
    _41_CIRCLE_FILL,
    _41_SQUARE,
    _41_SQUARE_FILL,
    _42_CIRCLE,
    _42_CIRCLE_FILL,
    _42_SQUARE,
    _42_SQUARE_FILL,
    _43_CIRCLE,
    _43_CIRCLE_FILL,
    _43_SQUARE,
    _43_SQUARE_FILL,
    _44_CIRCLE,
    _44_CIRCLE_FILL,
    _44_SQUARE,
    _44_SQUARE_FILL,
    _45_CIRCLE,
    _45_CIRCLE_FILL,
    _45_SQUARE,
    _45_SQUARE_FILL,
    _46_CIRCLE,
    _46_CIRCLE_FILL,
    _46_SQUARE,
    _46_SQUARE_FILL,
    _47_CIRCLE,
    _47_CIRCLE_FILL,
    _47_SQUARE,
    _47_SQUARE_FILL,
    _48_CIRCLE,
    _48_CIRCLE_FILL,
    _48_SQUARE,
    _48_SQUARE_FILL,
    _49_CIRCLE,
    _49_CIRCLE_FILL,
    _49_SQUARE,
    _49_SQUARE_FILL,
    _4K_TV,
    _4K_TV_FILL,
    _5_CIRCLE,
    _5_CIRCLE_FILL,
    _5_SQUARE,
    _5_SQUARE_FILL,
    _50_CIRCLE,
    _50_CIRCLE_FILL,
    _50_SQUARE,
    _50_SQUARE_FILL,
    _6_ALT_CIRCLE,
    _6_ALT_CIRCLE_FILL,
    _6_ALT_SQUARE,
    _6_ALT_SQUARE_FILL,
    _6_CIRCLE,
    _6_CIRCLE_FILL,
    _6_SQUARE,
    _6_SQUARE_FILL,
    _7_CIRCLE,
    _7_CIRCLE_FILL,
    _7_SQUARE,
    _7_SQUARE_FILL,
    _8_CIRCLE,
    _8_CIRCLE_FILL,
    _8_SQUARE,
    _8_SQUARE_FILL,
    _9_ALT_CIRCLE,
    _9_ALT_CIRCLE_FILL,
    _9_ALT_SQUARE,
    _9_ALT_SQUARE_FILL,
    _9_CIRCLE,
    _9_CIRCLE_FILL,
    _9_SQUARE,
    _9_SQUARE_FILL,
    A_CIRCLE,
    A_CIRCLE_FILL,
    A_MAGNIFY,
    A_SQUARE,
    A_SQUARE_FILL,
    ABC,
    AIR_CONDITIONER_HORIZONTAL,
    AIR_CONDITIONER_HORIZONTAL_FILL,
    AIR_CONDITIONER_VERTICAL,
    AIR_CONDITIONER_VERTICAL_FILL,
    AIR_PURIFIER,
    AIR_PURIFIER_FILL,
    AIRPLANE,
    AIRPLANE_ARRIVAL,
    AIRPLANE_CIRCLE,
    AIRPLANE_CIRCLE_FILL,
    AIRPLANE_DEPARTURE,
    AIRPLAYAUDIO,
    AIRPLAYAUDIO_BADGE_EXCLAMATIONMARK,
    AIRPLAYAUDIO_CIRCLE,
    AIRPLAYAUDIO_CIRCLE_FILL,
    AIRPLAYVIDEO,
    AIRPLAYVIDEO_BADGE_EXCLAMATIONMARK,
    AIRPLAYVIDEO_CIRCLE,
    AIRPLAYVIDEO_CIRCLE_FILL,
    AIRPOD_GEN3_LEFT,
    AIRPOD_GEN3_RIGHT,
    AIRPOD_LEFT,
    AIRPOD_RIGHT,
    AIRPODPRO_LEFT,
    AIRPODPRO_RIGHT,
    AIRPODS,
    AIRPODS_CHARGINGCASE,
    AIRPODS_CHARGINGCASE_FILL,
    AIRPODS_CHARGINGCASE_WIRELESS,
    AIRPODS_CHARGINGCASE_WIRELESS_FILL,
    AIRPODS_GEN3,
    AIRPODS_GEN3_CHARGINGCASE_WIRELESS,
    AIRPODS_GEN3_CHARGINGCASE_WIRELESS_FILL,
    AIRPODSMAX,
    AIRPODSPRO,
    AIRPODSPRO_CHARGINGCASE_WIRELESS,
    AIRPODSPRO_CHARGINGCASE_WIRELESS_FILL,
    AIRPORT_EXPRESS,
    AIRPORT_EXTREME,
    AIRPORT_EXTREME_TOWER,
    AIRTAG,
    AIRTAG_FILL,
    AIRTAG_RADIOWAVES_FORWARD,
    AIRTAG_RADIOWAVES_FORWARD_FILL,
    ALARM,
    ALARM_FILL,
    ALARM_WAVES_LEFT_AND_RIGHT,
    ALARM_WAVES_LEFT_AND_RIGHT_FILL,
    ALIGN_HORIZONTAL_CENTER,
    ALIGN_HORIZONTAL_CENTER_FILL,
    ALIGN_HORIZONTAL_LEFT,
    ALIGN_HORIZONTAL_LEFT_FILL,
    ALIGN_HORIZONTAL_RIGHT,
    ALIGN_HORIZONTAL_RIGHT_FILL,
    ALIGN_VERTICAL_BOTTOM,
    ALIGN_VERTICAL_BOTTOM_FILL,
    ALIGN_VERTICAL_CENTER,
    ALIGN_VERTICAL_CENTER_FILL,
    ALIGN_VERTICAL_TOP,
    ALIGN_VERTICAL_TOP_FILL,
    ALLERGENS,
    ALLERGENS_FILL,
    ALT,
    ALTERNATINGCURRENT,
    AMPLIFIER,
    ANGLE,
    ANT,
    ANT_CIRCLE,
    ANT_CIRCLE_FILL,
    ANT_FILL,
    ANTENNA_RADIOWAVES_LEFT_AND_RIGHT,
    ANTENNA_RADIOWAVES_LEFT_AND_RIGHT_CIRCLE,
    ANTENNA_RADIOWAVES_LEFT_AND_RIGHT_CIRCLE_FILL,
    ANTENNA_RADIOWAVES_LEFT_AND_RIGHT_SLASH,
    APP,
    APP_BADGE,
    APP_BADGE_CHECKMARK,
    APP_BADGE_CHECKMARK_FILL,
    APP_BADGE_FILL,
    APP_CONNECTED_TO_APP_BELOW_FILL,
    APP_DASHED,
    APP_FILL,
    APP_GIFT,
    APP_GIFT_FILL,
    APPCLIP,
    APPLE_LOGO,
    APPLELOGO,
    APPLEPENCIL,
    APPLESCRIPT,
    APPLESCRIPT_FILL,
    APPLETV,
    APPLETV_FILL,
    APPLETVREMOTE_GEN1,
    APPLETVREMOTE_GEN1_FILL,
    APPLETVREMOTE_GEN2,
    APPLETVREMOTE_GEN2_FILL,
    APPLETVREMOTE_GEN3,
    APPLETVREMOTE_GEN3_FILL,
    APPLETVREMOTE_GEN4,
    APPLETVREMOTE_GEN4_FILL,
    APPLEWATCH,
    APPLEWATCH_CASE_INSET_FILLED,
    APPLEWATCH_RADIOWAVES_LEFT_AND_RIGHT,
    APPLEWATCH_SIDE_RIGHT,
    APPLEWATCH_SLASH,
    APPLEWATCH_WATCHFACE,
    APPS_IPAD,
    APPS_IPAD_LANDSCAPE,
    APPS_IPHONE,
    APPS_IPHONE_BADGE_PLUS,
    APPS_IPHONE_LANDSCAPE,
    AQI_HIGH,
    AQI_LOW,
    AQI_MEDIUM,
    ARCHIVEBOX,
    ARCHIVEBOX_CIRCLE,
    ARCHIVEBOX_CIRCLE_FILL,
    ARCHIVEBOX_FILL,
    ARKIT,
    ARKIT_BADGE_XMARK,
    ARROW_2_SQUAREPATH,
    ARROW_3_TRIANGLEPATH,
    ARROW_BACKWARD,
    ARROW_BACKWARD_CIRCLE,
    ARROW_BACKWARD_CIRCLE_FILL,
    ARROW_BACKWARD_SQUARE,
    ARROW_BACKWARD_SQUARE_FILL,
    ARROW_BACKWARD_TO_LINE,
    ARROW_BACKWARD_TO_LINE_CIRCLE,
    ARROW_BACKWARD_TO_LINE_CIRCLE_FILL,
    ARROW_CLOCKWISE,
    ARROW_CLOCKWISE_CIRCLE,
    ARROW_CLOCKWISE_CIRCLE_FILL,
    ARROW_CLOCKWISE_HEART,
    ARROW_CLOCKWISE_HEART_FILL,
    ARROW_CLOCKWISE_ICLOUD,
    ARROW_CLOCKWISE_ICLOUD_FILL,
    ARROW_COUNTERCLOCKWISE,
    ARROW_COUNTERCLOCKWISE_CIRCLE,
    ARROW_COUNTERCLOCKWISE_CIRCLE_FILL,
    ARROW_COUNTERCLOCKWISE_ICLOUD,
    ARROW_COUNTERCLOCKWISE_ICLOUD_FILL,
    ARROW_DOWN,
    ARROW_DOWN_AND_LINE_HORIZONTAL_AND_ARROW_UP,
    ARROW_DOWN_APP,
    ARROW_DOWN_APP_FILL,
    ARROW_DOWN_BACKWARD,
    ARROW_DOWN_BACKWARD_CIRCLE,
    ARROW_DOWN_BACKWARD_CIRCLE_FILL,
    ARROW_DOWN_BACKWARD_SQUARE,
    ARROW_DOWN_BACKWARD_SQUARE_FILL,
    ARROW_DOWN_CIRCLE,
    ARROW_DOWN_CIRCLE_FILL,
    ARROW_DOWN_DOC,
    ARROW_DOWN_DOC_FILL,
    ARROW_DOWN_FORWARD,
    ARROW_DOWN_FORWARD_AND_ARROW_UP_BACKWARD,
    ARROW_DOWN_FORWARD_AND_ARROW_UP_BACKWARD_CIRCLE,
    ARROW_DOWN_FORWARD_AND_ARROW_UP_BACKWARD_CIRCLE_FILL,
    ARROW_DOWN_FORWARD_CIRCLE,
    ARROW_DOWN_FORWARD_CIRCLE_FILL,
    ARROW_DOWN_FORWARD_SQUARE,
    ARROW_DOWN_FORWARD_SQUARE_FILL,
    ARROW_DOWN_HEART,
    ARROW_DOWN_HEART_FILL,
    ARROW_DOWN_LEFT,
    ARROW_DOWN_LEFT_CIRCLE,
    ARROW_DOWN_LEFT_CIRCLE_FILL,
    ARROW_DOWN_LEFT_SQUARE,
    ARROW_DOWN_LEFT_SQUARE_FILL,
    ARROW_DOWN_LEFT_VIDEO,
    ARROW_DOWN_LEFT_VIDEO_FILL,
    ARROW_DOWN_MESSAGE,
    ARROW_DOWN_MESSAGE_FILL,
    ARROW_DOWN_RIGHT,
    ARROW_DOWN_RIGHT_AND_ARROW_UP_LEFT,
    ARROW_DOWN_RIGHT_AND_ARROW_UP_LEFT_CIRCLE,
    ARROW_DOWN_RIGHT_AND_ARROW_UP_LEFT_CIRCLE_FILL,
    ARROW_DOWN_RIGHT_CIRCLE,
    ARROW_DOWN_RIGHT_CIRCLE_FILL,
    ARROW_DOWN_RIGHT_SQUARE,
    ARROW_DOWN_RIGHT_SQUARE_FILL,
    ARROW_DOWN_SQUARE,
    ARROW_DOWN_SQUARE_FILL,
    ARROW_DOWN_TO_LINE,
    ARROW_DOWN_TO_LINE_CIRCLE,
    ARROW_DOWN_TO_LINE_CIRCLE_FILL,
    ARROW_DOWN_TO_LINE_COMPACT,
    ARROW_FORWARD,
    ARROW_FORWARD_CIRCLE,
    ARROW_FORWARD_CIRCLE_FILL,
    ARROW_FORWARD_SQUARE,
    ARROW_FORWARD_SQUARE_FILL,
    ARROW_FORWARD_TO_LINE,
    ARROW_FORWARD_TO_LINE_CIRCLE,
    ARROW_FORWARD_TO_LINE_CIRCLE_FILL,
    ARROW_LEFT,
    ARROW_LEFT_AND_LINE_VERTICAL_AND_ARROW_RIGHT,
    ARROW_LEFT_AND_RIGHT,
    ARROW_LEFT_AND_RIGHT_CIRCLE,
    ARROW_LEFT_AND_RIGHT_CIRCLE_FILL,
    ARROW_LEFT_AND_RIGHT_RIGHTTRIANGLE_LEFT_RIGHTTRIANGLE_RIGHT,
    ARROW_LEFT_AND_RIGHT_RIGHTTRIANGLE_LEFT_RIGHTTRIANGLE_RIGHT_FILL,
    ARROW_LEFT_AND_RIGHT_SQUARE,
    ARROW_LEFT_AND_RIGHT_SQUARE_FILL,
    ARROW_LEFT_AND_RIGHT_TEXT_VERTICAL,
    ARROW_LEFT_ARROW_RIGHT,
    ARROW_LEFT_ARROW_RIGHT_CIRCLE,
    ARROW_LEFT_ARROW_RIGHT_CIRCLE_FILL,
    ARROW_LEFT_ARROW_RIGHT_SQUARE,
    ARROW_LEFT_ARROW_RIGHT_SQUARE_FILL,
    ARROW_LEFT_CIRCLE,
    ARROW_LEFT_CIRCLE_FILL,
    ARROW_LEFT_SQUARE,
    ARROW_LEFT_SQUARE_FILL,
    ARROW_LEFT_TO_LINE,
    ARROW_LEFT_TO_LINE_CIRCLE,
    ARROW_LEFT_TO_LINE_CIRCLE_FILL,
    ARROW_LEFT_TO_LINE_COMPACT,
    ARROW_RECTANGLEPATH,
    ARROW_RIGHT,
    ARROW_RIGHT_AND_LINE_VERTICAL_AND_ARROW_LEFT,
    ARROW_RIGHT_CIRCLE,
    ARROW_RIGHT_CIRCLE_FILL,
    ARROW_RIGHT_DOC_ON_CLIPBOARD,
    ARROW_RIGHT_SQUARE,
    ARROW_RIGHT_SQUARE_FILL,
    ARROW_RIGHT_TO_LINE,
    ARROW_RIGHT_TO_LINE_CIRCLE,
    ARROW_RIGHT_TO_LINE_CIRCLE_FILL,
    ARROW_RIGHT_TO_LINE_COMPACT,
    ARROW_TRIANGLE_2_CIRCLEPATH,
    ARROW_TRIANGLE_2_CIRCLEPATH_CAMERA,
    ARROW_TRIANGLE_2_CIRCLEPATH_CAMERA_FILL,
    ARROW_TRIANGLE_2_CIRCLEPATH_CIRCLE,
    ARROW_TRIANGLE_2_CIRCLEPATH_CIRCLE_FILL,
    ARROW_TRIANGLE_2_CIRCLEPATH_DOC_ON_CLIPBOARD,
    ARROW_TRIANGLE_BRANCH,
    ARROW_TRIANGLE_CAPSULEPATH,
    ARROW_TRIANGLE_MERGE,
    ARROW_TRIANGLE_PULL,
    ARROW_TRIANGLE_SWAP,
    ARROW_TRIANGLE_TURN_UP_RIGHT_CIRCLE,
    ARROW_TRIANGLE_TURN_UP_RIGHT_CIRCLE_FILL,
    ARROW_TRIANGLE_TURN_UP_RIGHT_DIAMOND,
    ARROW_TRIANGLE_TURN_UP_RIGHT_DIAMOND_FILL,
    ARROW_TURN_DOWN_LEFT,
    ARROW_TURN_DOWN_RIGHT,
    ARROW_TURN_LEFT_DOWN,
    ARROW_TURN_LEFT_UP,
    ARROW_TURN_RIGHT_DOWN,
    ARROW_TURN_RIGHT_UP,
    ARROW_TURN_UP_FORWARD_IPHONE,
    ARROW_TURN_UP_FORWARD_IPHONE_FILL,
    ARROW_TURN_UP_LEFT,
    ARROW_TURN_UP_RIGHT,
    ARROW_UP,
    ARROW_UP_AND_DOWN,
    ARROW_UP_AND_DOWN_AND_ARROW_LEFT_AND_RIGHT,
    ARROW_UP_AND_DOWN_AND_SPARKLES,
    ARROW_UP_AND_DOWN_CIRCLE,
    ARROW_UP_AND_DOWN_CIRCLE_FILL,
    ARROW_UP_AND_DOWN_RIGHTTRIANGLE_UP_RIGHTTRIANGLE_DOWN,
    ARROW_UP_AND_DOWN_RIGHTTRIANGLE_UP_RIGHTTRIANGLE_DOWN_FILL,
    ARROW_UP_AND_DOWN_SQUARE,
    ARROW_UP_AND_DOWN_SQUARE_FILL,
    ARROW_UP_AND_DOWN_TEXT_HORIZONTAL,
    ARROW_UP_AND_LINE_HORIZONTAL_AND_ARROW_DOWN,
    ARROW_UP_AND_PERSON_RECTANGLE_PORTRAIT,
    ARROW_UP_AND_PERSON_RECTANGLE_TURN_LEFT,
    ARROW_UP_AND_PERSON_RECTANGLE_TURN_RIGHT,
    ARROW_UP_ARROW_DOWN,
    ARROW_UP_ARROW_DOWN_CIRCLE,
    ARROW_UP_ARROW_DOWN_CIRCLE_FILL,
    ARROW_UP_ARROW_DOWN_SQUARE,
    ARROW_UP_ARROW_DOWN_SQUARE_FILL,
    ARROW_UP_BACKWARD,
    ARROW_UP_BACKWARD_AND_ARROW_DOWN_FORWARD,
    ARROW_UP_BACKWARD_AND_ARROW_DOWN_FORWARD_CIRCLE,
    ARROW_UP_BACKWARD_AND_ARROW_DOWN_FORWARD_CIRCLE_FILL,
    ARROW_UP_BACKWARD_CIRCLE,
    ARROW_UP_BACKWARD_CIRCLE_FILL,
    ARROW_UP_BACKWARD_SQUARE,
    ARROW_UP_BACKWARD_SQUARE_FILL,
    ARROW_UP_BIN,
    ARROW_UP_BIN_FILL,
    ARROW_UP_CIRCLE,
    ARROW_UP_CIRCLE_BADGE_CLOCK,
    ARROW_UP_CIRCLE_FILL,
    ARROW_UP_DOC,
    ARROW_UP_DOC_FILL,
    ARROW_UP_DOC_ON_CLIPBOARD,
    ARROW_UP_FORWARD,
    ARROW_UP_FORWARD_APP,
    ARROW_UP_FORWARD_APP_FILL,
    ARROW_UP_FORWARD_CIRCLE,
    ARROW_UP_FORWARD_CIRCLE_FILL,
    ARROW_UP_FORWARD_SQUARE,
    ARROW_UP_FORWARD_SQUARE_FILL,
    ARROW_UP_HEART,
    ARROW_UP_HEART_FILL,
    ARROW_UP_LEFT,
    ARROW_UP_LEFT_AND_ARROW_DOWN_RIGHT,
    ARROW_UP_LEFT_AND_ARROW_DOWN_RIGHT_CIRCLE,
    ARROW_UP_LEFT_AND_ARROW_DOWN_RIGHT_CIRCLE_FILL,
    ARROW_UP_LEFT_AND_DOWN_RIGHT_AND_ARROW_UP_RIGHT_AND_DOWN_LEFT,
    ARROW_UP_LEFT_AND_DOWN_RIGHT_MAGNIFYINGGLASS,
    ARROW_UP_LEFT_CIRCLE,
    ARROW_UP_LEFT_CIRCLE_FILL,
    ARROW_UP_LEFT_SQUARE,
    ARROW_UP_LEFT_SQUARE_FILL,
    ARROW_UP_MESSAGE,
    ARROW_UP_MESSAGE_FILL,
    ARROW_UP_RIGHT,
    ARROW_UP_RIGHT_AND_ARROW_DOWN_LEFT_RECTANGLE,
    ARROW_UP_RIGHT_AND_ARROW_DOWN_LEFT_RECTANGLE_FILL,
    ARROW_UP_RIGHT_CIRCLE,
    ARROW_UP_RIGHT_CIRCLE_FILL,
    ARROW_UP_RIGHT_SQUARE,
    ARROW_UP_RIGHT_SQUARE_FILL,
    ARROW_UP_RIGHT_VIDEO,
    ARROW_UP_RIGHT_VIDEO_FILL,
    ARROW_UP_SQUARE,
    ARROW_UP_SQUARE_FILL,
    ARROW_UP_TO_LINE,
    ARROW_UP_TO_LINE_CIRCLE,
    ARROW_UP_TO_LINE_CIRCLE_FILL,
    ARROW_UP_TO_LINE_COMPACT,
    ARROW_UTURN_BACKWARD,
    ARROW_UTURN_BACKWARD_CIRCLE,
    ARROW_UTURN_BACKWARD_CIRCLE_BADGE_ELLIPSIS,
    ARROW_UTURN_BACKWARD_CIRCLE_FILL,
    ARROW_UTURN_BACKWARD_SQUARE,
    ARROW_UTURN_BACKWARD_SQUARE_FILL,
    ARROW_UTURN_DOWN,
    ARROW_UTURN_DOWN_CIRCLE,
    ARROW_UTURN_DOWN_CIRCLE_FILL,
    ARROW_UTURN_DOWN_SQUARE,
    ARROW_UTURN_DOWN_SQUARE_FILL,
    ARROW_UTURN_FORWARD,
    ARROW_UTURN_FORWARD_CIRCLE,
    ARROW_UTURN_FORWARD_CIRCLE_FILL,
    ARROW_UTURN_FORWARD_SQUARE,
    ARROW_UTURN_FORWARD_SQUARE_FILL,
    ARROW_UTURN_LEFT,
    ARROW_UTURN_LEFT_CIRCLE,
    ARROW_UTURN_LEFT_CIRCLE_BADGE_ELLIPSIS,
    ARROW_UTURN_LEFT_CIRCLE_FILL,
    ARROW_UTURN_LEFT_SQUARE,
    ARROW_UTURN_LEFT_SQUARE_FILL,
    ARROW_UTURN_RIGHT,
    ARROW_UTURN_RIGHT_CIRCLE,
    ARROW_UTURN_RIGHT_CIRCLE_FILL,
    ARROW_UTURN_RIGHT_SQUARE,
    ARROW_UTURN_RIGHT_SQUARE_FILL,
    ARROW_UTURN_UP,
    ARROW_UTURN_UP_CIRCLE,
    ARROW_UTURN_UP_CIRCLE_FILL,
    ARROW_UTURN_UP_SQUARE,
    ARROW_UTURN_UP_SQUARE_FILL,
    ARROWSHAPE_BACKWARD,
    ARROWSHAPE_BACKWARD_FILL,
    ARROWSHAPE_BOUNCE_FORWARD,
    ARROWSHAPE_BOUNCE_FORWARD_FILL,
    ARROWSHAPE_BOUNCE_RIGHT,
    ARROWSHAPE_BOUNCE_RIGHT_FILL,
    ARROWSHAPE_FORWARD,
    ARROWSHAPE_FORWARD_FILL,
    ARROWSHAPE_LEFT,
    ARROWSHAPE_LEFT_FILL,
    ARROWSHAPE_RIGHT,
    ARROWSHAPE_RIGHT_FILL,
    ARROWSHAPE_TURN_UP_BACKWARD,
    ARROWSHAPE_TURN_UP_BACKWARD_2,
    ARROWSHAPE_TURN_UP_BACKWARD_2_CIRCLE,
    ARROWSHAPE_TURN_UP_BACKWARD_2_CIRCLE_FILL,
    ARROWSHAPE_TURN_UP_BACKWARD_2_FILL,
    ARROWSHAPE_TURN_UP_BACKWARD_BADGE_CLOCK,
    ARROWSHAPE_TURN_UP_BACKWARD_BADGE_CLOCK_FILL,
    ARROWSHAPE_TURN_UP_BACKWARD_CIRCLE,
    ARROWSHAPE_TURN_UP_BACKWARD_CIRCLE_FILL,
    ARROWSHAPE_TURN_UP_BACKWARD_FILL,
    ARROWSHAPE_TURN_UP_FORWARD,
    ARROWSHAPE_TURN_UP_FORWARD_CIRCLE,
    ARROWSHAPE_TURN_UP_FORWARD_CIRCLE_FILL,
    ARROWSHAPE_TURN_UP_FORWARD_FILL,
    ARROWSHAPE_TURN_UP_LEFT,
    ARROWSHAPE_TURN_UP_LEFT_2,
    ARROWSHAPE_TURN_UP_LEFT_2_CIRCLE,
    ARROWSHAPE_TURN_UP_LEFT_2_CIRCLE_FILL,
    ARROWSHAPE_TURN_UP_LEFT_2_FILL,
    ARROWSHAPE_TURN_UP_LEFT_CIRCLE,
    ARROWSHAPE_TURN_UP_LEFT_CIRCLE_FILL,
    ARROWSHAPE_TURN_UP_LEFT_FILL,
    ARROWSHAPE_TURN_UP_RIGHT,
    ARROWSHAPE_TURN_UP_RIGHT_CIRCLE,
    ARROWSHAPE_TURN_UP_RIGHT_CIRCLE_FILL,
    ARROWSHAPE_TURN_UP_RIGHT_FILL,
    ARROWSHAPE_ZIGZAG_FORWARD,
    ARROWSHAPE_ZIGZAG_FORWARD_FILL,
    ARROWSHAPE_ZIGZAG_RIGHT,
    ARROWSHAPE_ZIGZAG_RIGHT_FILL,
    ARROWTRIANGLE_BACKWARD,
    ARROWTRIANGLE_BACKWARD_CIRCLE,
    ARROWTRIANGLE_BACKWARD_CIRCLE_FILL,
    ARROWTRIANGLE_BACKWARD_FILL,
    ARROWTRIANGLE_BACKWARD_SQUARE,
    ARROWTRIANGLE_BACKWARD_SQUARE_FILL,
    ARROWTRIANGLE_DOWN,
    ARROWTRIANGLE_DOWN_CIRCLE,
    ARROWTRIANGLE_DOWN_CIRCLE_FILL,
    ARROWTRIANGLE_DOWN_FILL,
    ARROWTRIANGLE_DOWN_SQUARE,
    ARROWTRIANGLE_DOWN_SQUARE_FILL,
    ARROWTRIANGLE_FORWARD,
    ARROWTRIANGLE_FORWARD_CIRCLE,
    ARROWTRIANGLE_FORWARD_CIRCLE_FILL,
    ARROWTRIANGLE_FORWARD_FILL,
    ARROWTRIANGLE_FORWARD_SQUARE,
    ARROWTRIANGLE_FORWARD_SQUARE_FILL,
    ARROWTRIANGLE_LEFT,
    ARROWTRIANGLE_LEFT_AND_LINE_VERTICAL_AND_ARROWTRIANGLE_RIGHT,
    ARROWTRIANGLE_LEFT_AND_LINE_VERTICAL_AND_ARROWTRIANGLE_RIGHT_FILL,
    ARROWTRIANGLE_LEFT_CIRCLE,
    ARROWTRIANGLE_LEFT_CIRCLE_FILL,
    ARROWTRIANGLE_LEFT_FILL,
    ARROWTRIANGLE_LEFT_SQUARE,
    ARROWTRIANGLE_LEFT_SQUARE_FILL,
    ARROWTRIANGLE_RIGHT,
    ARROWTRIANGLE_RIGHT_AND_LINE_VERTICAL_AND_ARROWTRIANGLE_LEFT,
    ARROWTRIANGLE_RIGHT_AND_LINE_VERTICAL_AND_ARROWTRIANGLE_LEFT_FILL,
    ARROWTRIANGLE_RIGHT_CIRCLE,
    ARROWTRIANGLE_RIGHT_CIRCLE_FILL,
    ARROWTRIANGLE_RIGHT_FILL,
    ARROWTRIANGLE_RIGHT_SQUARE,
    ARROWTRIANGLE_RIGHT_SQUARE_FILL,
    ARROWTRIANGLE_UP,
    ARROWTRIANGLE_UP_CIRCLE,
    ARROWTRIANGLE_UP_CIRCLE_FILL,
    ARROWTRIANGLE_UP_FILL,
    ARROWTRIANGLE_UP_SQUARE,
    ARROWTRIANGLE_UP_SQUARE_FILL,
    ASPECTRATIO,
    ASPECTRATIO_FILL,
    ASTERISK,
    ASTERISK_CIRCLE,
    ASTERISK_CIRCLE_FILL,
    AT,
    AT_BADGE_MINUS,
    AT_BADGE_PLUS,
    AT_CIRCLE,
    AT_CIRCLE_FILL,
    ATOM,
    AUSTRALSIGN,
    AUSTRALSIGN_CIRCLE,
    AUSTRALSIGN_CIRCLE_FILL,
    AUSTRALSIGN_SQUARE,
    AUSTRALSIGN_SQUARE_FILL,
    AV_REMOTE,
    AV_REMOTE_FILL,
    B_CIRCLE,
    B_CIRCLE_FILL,
    B_SQUARE,
    B_SQUARE_FILL,
    BACKPACK,
    BACKPACK_FILL,
    BACKWARD,
    BACKWARD_CIRCLE,
    BACKWARD_CIRCLE_FILL,
    BACKWARD_END,
    BACKWARD_END_ALT,
    BACKWARD_END_ALT_FILL,
    BACKWARD_END_CIRCLE,
    BACKWARD_END_CIRCLE_FILL,
    BACKWARD_END_FILL,
    BACKWARD_FILL,
    BACKWARD_FRAME,
    BACKWARD_FRAME_FILL,
    BADGE_PLUS_RADIOWAVES_FORWARD,
    BADGE_PLUS_RADIOWAVES_RIGHT,
    BAG,
    BAG_BADGE_MINUS,
    BAG_BADGE_PLUS,
    BAG_BADGE_QUESTIONMARK,
    BAG_CIRCLE,
    BAG_CIRCLE_FILL,
    BAG_FILL,
    BAG_FILL_BADGE_MINUS,
    BAG_FILL_BADGE_PLUS,
    BAG_FILL_BADGE_QUESTIONMARK,
    BAHTSIGN,
    BAHTSIGN_CIRCLE,
    BAHTSIGN_CIRCLE_FILL,
    BAHTSIGN_SQUARE,
    BAHTSIGN_SQUARE_FILL,
    BALLOON,
    BALLOON_2,
    BALLOON_2_FILL,
    BALLOON_FILL,
    BANDAGE,
    BANDAGE_FILL,
    BANKNOTE,
    BANKNOTE_FILL,
    BARCODE,
    BARCODE_VIEWFINDER,
    BAROMETER,
    BASEBALL,
    BASEBALL_CIRCLE,
    BASEBALL_CIRCLE_FILL,
    BASEBALL_DIAMOND_BASES,
    BASEBALL_FILL,
    BASKET,
    BASKET_FILL,
    BASKETBALL,
    BASKETBALL_CIRCLE,
    BASKETBALL_CIRCLE_FILL,
    BASKETBALL_FILL,
    BATHTUB,
    BATHTUB_FILL,
    BATTERY_0,
    BATTERY_100,
    BATTERY_100_BOLT,
    BATTERY_100_CIRCLE,
    BATTERY_100_CIRCLE_FILL,
    BATTERY_25,
    BATTERY_50,
    BATTERY_75,
    BEACH_UMBRELLA,
    BEACH_UMBRELLA_FILL,
    BEATS_EARPHONES,
    BEATS_FIT_PRO,
    BEATS_FIT_PRO_CHARGINGCASE,
    BEATS_FIT_PRO_CHARGINGCASE_FILL,
    BEATS_FIT_PRO_LEFT,
    BEATS_FIT_PRO_RIGHT,
    BEATS_HEADPHONES,
    BEATS_POWERBEATS,
    BEATS_POWERBEATS3,
    BEATS_POWERBEATSPRO,
    BEATS_POWERBEATSPRO_CHARGINGCASE,
    BEATS_POWERBEATSPRO_CHARGINGCASE_FILL,
    BEATS_POWERBEATSPRO_LEFT,
    BEATS_POWERBEATSPRO_RIGHT,
    BEATS_STUDIOBUD_LEFT,
    BEATS_STUDIOBUD_RIGHT,
    BEATS_STUDIOBUDS,
    BEATS_STUDIOBUDS_CHARGINGCASE,
    BEATS_STUDIOBUDS_CHARGINGCASE_FILL,
    BED_DOUBLE,
    BED_DOUBLE_CIRCLE,
    BED_DOUBLE_CIRCLE_FILL,
    BED_DOUBLE_FILL,
    BELL,
    BELL_AND_WAVEFORM,
    BELL_AND_WAVEFORM_FILL,
    BELL_AND_WAVES_LEFT_AND_RIGHT,
    BELL_AND_WAVES_LEFT_AND_RIGHT_FILL,
    BELL_BADGE,
    BELL_BADGE_CIRCLE,
    BELL_BADGE_CIRCLE_FILL,
    BELL_BADGE_FILL,
    BELL_CIRCLE,
    BELL_CIRCLE_FILL,
    BELL_FILL,
    BELL_SLASH,
    BELL_SLASH_CIRCLE,
    BELL_SLASH_CIRCLE_FILL,
    BELL_SLASH_FILL,
    BELL_SQUARE,
    BELL_SQUARE_FILL,
    BICYCLE,
    BICYCLE_CIRCLE,
    BICYCLE_CIRCLE_FILL,
    BINOCULARS,
    BINOCULARS_FILL,
    BIRD,
    BIRD_FILL,
    BIRTHDAY_CAKE,
    BIRTHDAY_CAKE_FILL,
    BITCOINSIGN,
    BITCOINSIGN_CIRCLE,
    BITCOINSIGN_CIRCLE_FILL,
    BITCOINSIGN_SQUARE,
    BITCOINSIGN_SQUARE_FILL,
    BLINDS_HORIZONTAL_CLOSED,
    BLINDS_HORIZONTAL_OPEN,
    BLINDS_VERTICAL_CLOSED,
    BLINDS_VERTICAL_OPEN,
    BOLD,
    BOLD_ITALIC_UNDERLINE,
    BOLD_UNDERLINE,
    BOLT,
    BOLT_BADGE_A,
    BOLT_BADGE_A_FILL,
    BOLT_BADGE_CLOCK,
    BOLT_BADGE_CLOCK_FILL,
    BOLT_BATTERYBLOCK,
    BOLT_BATTERYBLOCK_FILL,
    BOLT_CAR,
    BOLT_CAR_CIRCLE,
    BOLT_CAR_CIRCLE_FILL,
    BOLT_CAR_FILL,
    BOLT_CIRCLE,
    BOLT_CIRCLE_FILL,
    BOLT_FILL,
    BOLT_HEART,
    BOLT_HEART_FILL,
    BOLT_HORIZONTAL,
    BOLT_HORIZONTAL_CIRCLE,
    BOLT_HORIZONTAL_CIRCLE_FILL,
    BOLT_HORIZONTAL_FILL,
    BOLT_HORIZONTAL_ICLOUD,
    BOLT_HORIZONTAL_ICLOUD_FILL,
    BOLT_RING_CLOSED,
    BOLT_SHIELD,
    BOLT_SHIELD_FILL,
    BOLT_SLASH,
    BOLT_SLASH_CIRCLE,
    BOLT_SLASH_CIRCLE_FILL,
    BOLT_SLASH_FILL,
    BOLT_SQUARE,
    BOLT_SQUARE_FILL,
    BONJOUR,
    BOOK,
    BOOK_CIRCLE,
    BOOK_CIRCLE_FILL,
    BOOK_CLOSED,
    BOOK_CLOSED_CIRCLE,
    BOOK_CLOSED_CIRCLE_FILL,
    BOOK_CLOSED_FILL,
    BOOK_FILL,
    BOOKMARK,
    BOOKMARK_CIRCLE,
    BOOKMARK_CIRCLE_FILL,
    BOOKMARK_FILL,
    BOOKMARK_SLASH,
    BOOKMARK_SLASH_FILL,
    BOOKMARK_SQUARE,
    BOOKMARK_SQUARE_FILL,
    BOOKS_VERTICAL,
    BOOKS_VERTICAL_CIRCLE,
    BOOKS_VERTICAL_CIRCLE_FILL,
    BOOKS_VERTICAL_FILL,
    BOX_TRUCK,
    BOX_TRUCK_BADGE_CLOCK,
    BOX_TRUCK_BADGE_CLOCK_FILL,
    BOX_TRUCK_FILL,
    BRAIN,
    BRAIN_HEAD_PROFILE,
    BRAZILIANREALSIGN,
    BRAZILIANREALSIGN_CIRCLE,
    BRAZILIANREALSIGN_CIRCLE_FILL,
    BRAZILIANREALSIGN_SQUARE,
    BRAZILIANREALSIGN_SQUARE_FILL,
    BRIEFCASE,
    BRIEFCASE_CIRCLE,
    BRIEFCASE_CIRCLE_FILL,
    BRIEFCASE_FILL,
    BUBBLE_LEFT,
    BUBBLE_LEFT_AND_BUBBLE_RIGHT,
    BUBBLE_LEFT_AND_BUBBLE_RIGHT_FILL,
    BUBBLE_LEFT_AND_EXCLAMATIONMARK_BUBBLE_RIGHT,
    BUBBLE_LEFT_AND_EXCLAMATIONMARK_BUBBLE_RIGHT_FILL,
    BUBBLE_LEFT_CIRCLE,
    BUBBLE_LEFT_CIRCLE_FILL,
    BUBBLE_LEFT_FILL,
    BUBBLE_MIDDLE_BOTTOM,
    BUBBLE_MIDDLE_BOTTOM_FILL,
    BUBBLE_MIDDLE_TOP,
    BUBBLE_MIDDLE_TOP_FILL,
    BUBBLE_RIGHT,
    BUBBLE_RIGHT_CIRCLE,
    BUBBLE_RIGHT_CIRCLE_FILL,
    BUBBLE_RIGHT_FILL,
    BUBBLES_AND_SPARKLES,
    BUBBLES_AND_SPARKLES_FILL,
    BUILDING,
    BUILDING_2,
    BUILDING_2_CROP_CIRCLE,
    BUILDING_2_CROP_CIRCLE_FILL,
    BUILDING_2_FILL,
    BUILDING_COLUMNS,
    BUILDING_COLUMNS_CIRCLE,
    BUILDING_COLUMNS_CIRCLE_FILL,
    BUILDING_COLUMNS_FILL,
    BUILDING_FILL,
    BURN,
    BURST,
    BURST_FILL,
    BUS,
    BUS_DOUBLEDECKER,
    BUS_DOUBLEDECKER_FILL,
    BUS_FILL,
    BUTTON_PROGRAMMABLE,
    BUTTON_PROGRAMMABLE_SQUARE,
    BUTTON_PROGRAMMABLE_SQUARE_FILL,
    C_CIRCLE,
    C_CIRCLE_FILL,
    C_SQUARE,
    C_SQUARE_FILL,
    CABINET,
    CABINET_FILL,
    CABLE_CONNECTOR,
    CABLE_CONNECTOR_HORIZONTAL,
    CABLECAR,
    CABLECAR_FILL,
    CALENDAR,
    CALENDAR_BADGE_CLOCK,
    CALENDAR_BADGE_EXCLAMATIONMARK,
    CALENDAR_BADGE_MINUS,
    CALENDAR_BADGE_PLUS,
    CALENDAR_CIRCLE,
    CALENDAR_CIRCLE_FILL,
    CALENDAR_DAY_TIMELINE_LEADING,
    CALENDAR_DAY_TIMELINE_LEFT,
    CALENDAR_DAY_TIMELINE_RIGHT,
    CALENDAR_DAY_TIMELINE_TRAILING,
    CAMERA,
    CAMERA_APERTURE,
    CAMERA_BADGE_ELLIPSIS,
    CAMERA_CIRCLE,
    CAMERA_CIRCLE_FILL,
    CAMERA_FILL,
    CAMERA_FILL_BADGE_ELLIPSIS,
    CAMERA_FILTERS,
    CAMERA_MACRO,
    CAMERA_MACRO_CIRCLE,
    CAMERA_MACRO_CIRCLE_FILL,
    CAMERA_METERING_CENTER_WEIGHTED,
    CAMERA_METERING_CENTER_WEIGHTED_AVERAGE,
    CAMERA_METERING_MATRIX,
    CAMERA_METERING_MULTISPOT,
    CAMERA_METERING_NONE,
    CAMERA_METERING_PARTIAL,
    CAMERA_METERING_SPOT,
    CAMERA_METERING_UNKNOWN,
    CAMERA_ON_RECTANGLE,
    CAMERA_ON_RECTANGLE_FILL,
    CAMERA_SHUTTER_BUTTON,
    CAMERA_SHUTTER_BUTTON_FILL,
    CAMERA_VIEWFINDER,
    CANDYBARPHONE,
    CAPSLOCK,
    CAPSLOCK_FILL,
    CAPSULE,
    CAPSULE_BOTTOMHALF_FILLED,
    CAPSULE_FILL,
    CAPSULE_INSET_FILLED,
    CAPSULE_LEFTHALF_FILLED,
    CAPSULE_PORTRAIT,
    CAPSULE_PORTRAIT_BOTTOMHALF_FILLED,
    CAPSULE_PORTRAIT_FILL,
    CAPSULE_PORTRAIT_INSET_FILLED,
    CAPSULE_PORTRAIT_LEFTHALF_FILLED,
    CAPSULE_PORTRAIT_RIGHTHALF_FILLED,
    CAPSULE_PORTRAIT_TOPHALF_FILLED,
    CAPSULE_RIGHTHALF_FILLED,
    CAPSULE_TOPHALF_FILLED,
    CAPTIONS_BUBBLE,
    CAPTIONS_BUBBLE_FILL,
    CAR,
    CAR_2,
    CAR_2_FILL,
    CAR_CIRCLE,
    CAR_CIRCLE_FILL,
    CAR_FERRY,
    CAR_FERRY_FILL,
    CAR_FILL,
    CARBON_DIOXIDE_CLOUD,
    CARBON_DIOXIDE_CLOUD_FILL,
    CARBON_MONOXIDE_CLOUD,
    CARBON_MONOXIDE_CLOUD_FILL,
    CARROT,
    CARROT_FILL,
    CART,
    CART_BADGE_MINUS,
    CART_BADGE_PLUS,
    CART_BADGE_QUESTIONMARK,
    CART_CIRCLE,
    CART_CIRCLE_FILL,
    CART_FILL,
    CART_FILL_BADGE_MINUS,
    CART_FILL_BADGE_PLUS,
    CART_FILL_BADGE_QUESTIONMARK,
    CASE,
    CASE_FILL,
    CEDISIGN,
    CEDISIGN_CIRCLE,
    CEDISIGN_CIRCLE_FILL,
    CEDISIGN_SQUARE,
    CEDISIGN_SQUARE_FILL,
    CELLULARBARS,
    CENTSIGN,
    CENTSIGN_CIRCLE,
    CENTSIGN_CIRCLE_FILL,
    CENTSIGN_SQUARE,
    CENTSIGN_SQUARE_FILL,
    CHAIR,
    CHAIR_FILL,
    CHAIR_LOUNGE,
    CHAIR_LOUNGE_FILL,
    CHANDELIER,
    CHANDELIER_FILL,
    CHARACTER,
    CHARACTER_BOOK_CLOSED,
    CHARACTER_BOOK_CLOSED_FILL,
    CHARACTER_BUBBLE,
    CHARACTER_BUBBLE_FILL,
    CHARACTER_CURSOR_IBEAM,
    CHARACTER_DUPLOYAN,
    CHARACTER_PHONETIC,
    CHARACTER_SUTTON,
    CHARACTER_TEXTBOX,
    CHART_BAR,
    CHART_BAR_DOC_HORIZONTAL,
    CHART_BAR_DOC_HORIZONTAL_FILL,
    CHART_BAR_FILL,
    CHART_BAR_XAXIS,
    CHART_LINE_DOWNTREND_XYAXIS,
    CHART_LINE_DOWNTREND_XYAXIS_CIRCLE,
    CHART_LINE_DOWNTREND_XYAXIS_CIRCLE_FILL,
    CHART_LINE_FLATTREND_XYAXIS,
    CHART_LINE_FLATTREND_XYAXIS_CIRCLE,
    CHART_LINE_FLATTREND_XYAXIS_CIRCLE_FILL,
    CHART_LINE_UPTREND_XYAXIS,
    CHART_LINE_UPTREND_XYAXIS_CIRCLE,
    CHART_LINE_UPTREND_XYAXIS_CIRCLE_FILL,
    CHART_PIE,
    CHART_PIE_FILL,
    CHART_XYAXIS_LINE,
    CHECKERBOARD_RECTANGLE,
    CHECKERBOARD_SHIELD,
    CHECKLIST,
    CHECKLIST_CHECKED,
    CHECKLIST_UNCHECKED,
    CHECKMARK,
    CHECKMARK_BUBBLE,
    CHECKMARK_BUBBLE_FILL,
    CHECKMARK_CIRCLE,
    CHECKMARK_CIRCLE_BADGE_QUESTIONMARK,
    CHECKMARK_CIRCLE_BADGE_QUESTIONMARK_FILL,
    CHECKMARK_CIRCLE_BADGE_XMARK,
    CHECKMARK_CIRCLE_BADGE_XMARK_FILL,
    CHECKMARK_CIRCLE_FILL,
    CHECKMARK_CIRCLE_TRIANGLEBADGE_EXCLAMATIONMARK,
    CHECKMARK_DIAMOND,
    CHECKMARK_DIAMOND_FILL,
    CHECKMARK_ICLOUD,
    CHECKMARK_ICLOUD_FILL,
    CHECKMARK_MESSAGE,
    CHECKMARK_MESSAGE_FILL,
    CHECKMARK_RECTANGLE,
    CHECKMARK_RECTANGLE_FILL,
    CHECKMARK_RECTANGLE_PORTRAIT,
    CHECKMARK_RECTANGLE_PORTRAIT_FILL,
    CHECKMARK_SEAL,
    CHECKMARK_SEAL_FILL,
    CHECKMARK_SHIELD,
    CHECKMARK_SHIELD_FILL,
    CHECKMARK_SQUARE,
    CHECKMARK_SQUARE_FILL,
    CHEVRON_BACKWARD,
    CHEVRON_BACKWARD_2,
    CHEVRON_BACKWARD_CIRCLE,
    CHEVRON_BACKWARD_CIRCLE_FILL,
    CHEVRON_BACKWARD_SQUARE,
    CHEVRON_BACKWARD_SQUARE_FILL,
    CHEVRON_BACKWARD_TO_LINE,
    CHEVRON_COMPACT_DOWN,
    CHEVRON_COMPACT_LEFT,
    CHEVRON_COMPACT_RIGHT,
    CHEVRON_COMPACT_UP,
    CHEVRON_DOWN,
    CHEVRON_DOWN_CIRCLE,
    CHEVRON_DOWN_CIRCLE_FILL,
    CHEVRON_DOWN_SQUARE,
    CHEVRON_DOWN_SQUARE_FILL,
    CHEVRON_FORWARD,
    CHEVRON_FORWARD_2,
    CHEVRON_FORWARD_CIRCLE,
    CHEVRON_FORWARD_CIRCLE_FILL,
    CHEVRON_FORWARD_SQUARE,
    CHEVRON_FORWARD_SQUARE_FILL,
    CHEVRON_FORWARD_TO_LINE,
    CHEVRON_LEFT,
    CHEVRON_LEFT_2,
    CHEVRON_LEFT_CIRCLE,
    CHEVRON_LEFT_CIRCLE_FILL,
    CHEVRON_LEFT_FORWARDSLASH_CHEVRON_RIGHT,
    CHEVRON_LEFT_SQUARE,
    CHEVRON_LEFT_SQUARE_FILL,
    CHEVRON_LEFT_TO_LINE,
    CHEVRON_RIGHT,
    CHEVRON_RIGHT_2,
    CHEVRON_RIGHT_CIRCLE,
    CHEVRON_RIGHT_CIRCLE_FILL,
    CHEVRON_RIGHT_SQUARE,
    CHEVRON_RIGHT_SQUARE_FILL,
    CHEVRON_RIGHT_TO_LINE,
    CHEVRON_UP,
    CHEVRON_UP_CHEVRON_DOWN,
    CHEVRON_UP_CIRCLE,
    CHEVRON_UP_CIRCLE_FILL,
    CHEVRON_UP_SQUARE,
    CHEVRON_UP_SQUARE_FILL,
    CIRCLE,
    CIRCLE_AND_LINE_HORIZONTAL,
    CIRCLE_AND_LINE_HORIZONTAL_FILL,
    CIRCLE_BOTTOMHALF_FILLED,
    CIRCLE_CIRCLE,
    CIRCLE_CIRCLE_FILL,
    CIRCLE_DASHED,
    CIRCLE_DASHED_INSET_FILLED,
    CIRCLE_DASHED_RECTANGLE,
    CIRCLE_DOTTED,
    CIRCLE_FILL,
    CIRCLE_FILLED_PATTERN_DIAGONALLINE_RECTANGLE,
    CIRCLE_GRID_2X1,
    CIRCLE_GRID_2X1_FILL,
    CIRCLE_GRID_2X1_LEFT_FILLED,
    CIRCLE_GRID_2X1_RIGHT_FILLED,
    CIRCLE_GRID_2X2,
    CIRCLE_GRID_2X2_FILL,
    CIRCLE_GRID_3X3,
    CIRCLE_GRID_3X3_CIRCLE,
    CIRCLE_GRID_3X3_CIRCLE_FILL,
    CIRCLE_GRID_3X3_FILL,
    CIRCLE_GRID_CROSS,
    CIRCLE_GRID_CROSS_DOWN_FILLED,
    CIRCLE_GRID_CROSS_FILL,
    CIRCLE_GRID_CROSS_LEFT_FILLED,
    CIRCLE_GRID_CROSS_RIGHT_FILLED,
    CIRCLE_GRID_CROSS_UP_FILLED,
    CIRCLE_HEXAGONGRID,
    CIRCLE_HEXAGONGRID_CIRCLE,
    CIRCLE_HEXAGONGRID_CIRCLE_FILL,
    CIRCLE_HEXAGONGRID_FILL,
    CIRCLE_HEXAGONPATH,
    CIRCLE_HEXAGONPATH_FILL,
    CIRCLE_INSET_FILLED,
    CIRCLE_LEFTHALF_FILLED,
    CIRCLE_RECTANGLE_DASHED,
    CIRCLE_RECTANGLE_FILLED_PATTERN_DIAGONALLINE,
    CIRCLE_RIGHTHALF_FILLED,
    CIRCLE_SLASH,
    CIRCLE_SLASH_FILL,
    CIRCLE_SQUARE,
    CIRCLE_SQUARE_FILL,
    CIRCLE_TOPHALF_FILLED,
    CIRCLEBADGE,
    CIRCLEBADGE_2,
    CIRCLEBADGE_2_FILL,
    CIRCLEBADGE_FILL,
    CLEAR,
    CLEAR_FILL,
    CLIPBOARD,
    CLIPBOARD_FILL,
    CLOCK,
    CLOCK_ARROW_2_CIRCLEPATH,
    CLOCK_ARROW_CIRCLEPATH,
    CLOCK_BADGE,
    CLOCK_BADGE_CHECKMARK,
    CLOCK_BADGE_CHECKMARK_FILL,
    CLOCK_BADGE_EXCLAMATIONMARK,
    CLOCK_BADGE_EXCLAMATIONMARK_FILL,
    CLOCK_BADGE_FILL,
    CLOCK_BADGE_QUESTIONMARK,
    CLOCK_BADGE_QUESTIONMARK_FILL,
    CLOCK_BADGE_XMARK,
    CLOCK_BADGE_XMARK_FILL,
    CLOCK_CIRCLE,
    CLOCK_CIRCLE_FILL,
    CLOCK_FILL,
    CLOUD,
    CLOUD_BOLT,
    CLOUD_BOLT_CIRCLE,
    CLOUD_BOLT_CIRCLE_FILL,
    CLOUD_BOLT_FILL,
    CLOUD_BOLT_RAIN,
    CLOUD_BOLT_RAIN_CIRCLE,
    CLOUD_BOLT_RAIN_CIRCLE_FILL,
    CLOUD_BOLT_RAIN_FILL,
    CLOUD_CIRCLE,
    CLOUD_CIRCLE_FILL,
    CLOUD_DRIZZLE,
    CLOUD_DRIZZLE_CIRCLE,
    CLOUD_DRIZZLE_CIRCLE_FILL,
    CLOUD_DRIZZLE_FILL,
    CLOUD_FILL,
    CLOUD_FOG,
    CLOUD_FOG_CIRCLE,
    CLOUD_FOG_CIRCLE_FILL,
    CLOUD_FOG_FILL,
    CLOUD_HAIL,
    CLOUD_HAIL_CIRCLE,
    CLOUD_HAIL_CIRCLE_FILL,
    CLOUD_HAIL_FILL,
    CLOUD_HEAVYRAIN,
    CLOUD_HEAVYRAIN_CIRCLE,
    CLOUD_HEAVYRAIN_CIRCLE_FILL,
    CLOUD_HEAVYRAIN_FILL,
    CLOUD_MOON,
    CLOUD_MOON_BOLT,
    CLOUD_MOON_BOLT_CIRCLE,
    CLOUD_MOON_BOLT_CIRCLE_FILL,
    CLOUD_MOON_BOLT_FILL,
    CLOUD_MOON_CIRCLE,
    CLOUD_MOON_CIRCLE_FILL,
    CLOUD_MOON_FILL,
    CLOUD_MOON_RAIN,
    CLOUD_MOON_RAIN_CIRCLE,
    CLOUD_MOON_RAIN_CIRCLE_FILL,
    CLOUD_MOON_RAIN_FILL,
    CLOUD_RAIN,
    CLOUD_RAIN_CIRCLE,
    CLOUD_RAIN_CIRCLE_FILL,
    CLOUD_RAIN_FILL,
    CLOUD_SLEET,
    CLOUD_SLEET_CIRCLE,
    CLOUD_SLEET_CIRCLE_FILL,
    CLOUD_SLEET_FILL,
    CLOUD_SNOW,
    CLOUD_SNOW_CIRCLE,
    CLOUD_SNOW_CIRCLE_FILL,
    CLOUD_SNOW_FILL,
    CLOUD_SUN,
    CLOUD_SUN_BOLT,
    CLOUD_SUN_BOLT_CIRCLE,
    CLOUD_SUN_BOLT_CIRCLE_FILL,
    CLOUD_SUN_BOLT_FILL,
    CLOUD_SUN_CIRCLE,
    CLOUD_SUN_CIRCLE_FILL,
    CLOUD_SUN_FILL,
    CLOUD_SUN_RAIN,
    CLOUD_SUN_RAIN_CIRCLE,
    CLOUD_SUN_RAIN_CIRCLE_FILL,
    CLOUD_SUN_RAIN_FILL,
    COLONCURRENCYSIGN,
    COLONCURRENCYSIGN_CIRCLE,
    COLONCURRENCYSIGN_CIRCLE_FILL,
    COLONCURRENCYSIGN_SQUARE,
    COLONCURRENCYSIGN_SQUARE_FILL,
    COMB,
    COMB_FILL,
    COMMAND,
    COMMAND_CIRCLE,
    COMMAND_CIRCLE_FILL,
    COMMAND_SQUARE,
    COMMAND_SQUARE_FILL,
    COMPASS_DRAWING,
    COMPUTERMOUSE,
    COMPUTERMOUSE_FILL,
    CONE,
    CONE_FILL,
    CONTACT_SENSOR,
    CONTACT_SENSOR_FILL,
    CONTEXTUALMENU_AND_CURSORARROW,
    CONTROL,
    COOKTOP,
    COOKTOP_FILL,
    CPU,
    CPU_FILL,
    CREDITCARD,
    CREDITCARD_AND_123,
    CREDITCARD_CIRCLE,
    CREDITCARD_CIRCLE_FILL,
    CREDITCARD_FILL,
    CREDITCARD_TRIANGLEBADGE_EXCLAMATIONMARK,
    CREDITCARD_VIEWFINDER,
    CRICKET_BALL,
    CRICKET_BALL_CIRCLE,
    CRICKET_BALL_CIRCLE_FILL,
    CRICKET_BALL_FILL,
    CROP,
    CROP_ROTATE,
    CROSS,
    CROSS_CASE,
    CROSS_CASE_FILL,
    CROSS_CIRCLE,
    CROSS_CIRCLE_FILL,
    CROSS_FILL,
    CROSS_VIAL,
    CROSS_VIAL_FILL,
    CROWN,
    CROWN_FILL,
    CRUZEIROSIGN,
    CRUZEIROSIGN_CIRCLE,
    CRUZEIROSIGN_CIRCLE_FILL,
    CRUZEIROSIGN_SQUARE,
    CRUZEIROSIGN_SQUARE_FILL,
    CUBE,
    CUBE_FILL,
    CUBE_TRANSPARENT,
    CUBE_TRANSPARENT_FILL,
    CUP_AND_SAUCER,
    CUP_AND_SAUCER_FILL,
    CURLYBRACES,
    CURLYBRACES_SQUARE,
    CURLYBRACES_SQUARE_FILL,
    CURSORARROW,
    CURSORARROW_AND_SQUARE_ON_SQUARE_DASHED,
    CURSORARROW_CLICK,
    CURSORARROW_CLICK_2,
    CURSORARROW_CLICK_BADGE_CLOCK,
    CURSORARROW_MOTIONLINES,
    CURSORARROW_MOTIONLINES_CLICK,
    CURSORARROW_RAYS,
    CURSORARROW_SQUARE,
    CURSORARROW_SQUARE_FILL,
    CURTAINS_CLOSED,
    CURTAINS_OPEN,
    CYLINDER,
    CYLINDER_FILL,
    CYLINDER_SPLIT_1X2,
    CYLINDER_SPLIT_1X2_FILL,
    D_CIRCLE,
    D_CIRCLE_FILL,
    D_SQUARE,
    D_SQUARE_FILL,
    DECREASE_INDENT,
    DECREASE_QUOTELEVEL,
    DEHUMIDIFIER,
    DEHUMIDIFIER_FILL,
    DELETE_BACKWARD,
    DELETE_BACKWARD_FILL,
    DELETE_FORWARD,
    DELETE_FORWARD_FILL,
    DELETE_LEFT,
    DELETE_LEFT_FILL,
    DELETE_RIGHT,
    DELETE_RIGHT_FILL,
    DESKCLOCK,
    DESKCLOCK_FILL,
    DESKTOPCOMPUTER,
    DESKTOPCOMPUTER_AND_ARROW_DOWN,
    DESKTOPCOMPUTER_TRIANGLEBADGE_EXCLAMATIONMARK,
    DESKVIEW,
    DESKVIEW_FILL,
    DIAL_HIGH,
    DIAL_HIGH_FILL,
    DIAL_LOW,
    DIAL_LOW_FILL,
    DIAL_MAX,
    DIAL_MAX_FILL,
    DIAL_MEDIUM,
    DIAL_MEDIUM_FILL,
    DIAL_MIN,
    DIAL_MIN_FILL,
    DIAMOND,
    DIAMOND_BOTTOMHALF_FILLED,
    DIAMOND_CIRCLE,
    DIAMOND_CIRCLE_FILL,
    DIAMOND_FILL,
    DIAMOND_INSET_FILLED,
    DIAMOND_LEFTHALF_FILLED,
    DIAMOND_RIGHTHALF_FILLED,
    DIAMOND_TOPHALF_FILLED,
    DICE,
    DICE_FILL,
    DIE_FACE_1,
    DIE_FACE_1_FILL,
    DIE_FACE_2,
    DIE_FACE_2_FILL,
    DIE_FACE_3,
    DIE_FACE_3_FILL,
    DIE_FACE_4,
    DIE_FACE_4_FILL,
    DIE_FACE_5,
    DIE_FACE_5_FILL,
    DIE_FACE_6,
    DIE_FACE_6_FILL,
    DIGITALCROWN_ARROW_CLOCKWISE,
    DIGITALCROWN_ARROW_CLOCKWISE_FILL,
    DIGITALCROWN_ARROW_COUNTERCLOCKWISE,
    DIGITALCROWN_ARROW_COUNTERCLOCKWISE_FILL,
    DIGITALCROWN_HORIZONTAL_ARROW_CLOCKWISE,
    DIGITALCROWN_HORIZONTAL_ARROW_CLOCKWISE_FILL,
    DIGITALCROWN_HORIZONTAL_ARROW_COUNTERCLOCKWISE,
    DIGITALCROWN_HORIZONTAL_ARROW_COUNTERCLOCKWISE_FILL,
    DIGITALCROWN_HORIZONTAL_PRESS,
    DIGITALCROWN_HORIZONTAL_PRESS_FILL,
    DIGITALCROWN_PRESS,
    DIGITALCROWN_PRESS_FILL,
    DIRECTCURRENT,
    DISHWASHER,
    DISHWASHER_FILL,
    DISPLAY,
    DISPLAY_2,
    DISPLAY_AND_ARROW_DOWN,
    DISPLAY_TRIANGLEBADGE_EXCLAMATIONMARK,
    DISTRIBUTE_HORIZONTAL_CENTER,
    DISTRIBUTE_HORIZONTAL_CENTER_FILL,
    DISTRIBUTE_HORIZONTAL_LEFT,
    DISTRIBUTE_HORIZONTAL_LEFT_FILL,
    DISTRIBUTE_HORIZONTAL_RIGHT,
    DISTRIBUTE_HORIZONTAL_RIGHT_FILL,
    DISTRIBUTE_VERTICAL_BOTTOM,
    DISTRIBUTE_VERTICAL_BOTTOM_FILL,
    DISTRIBUTE_VERTICAL_CENTER,
    DISTRIBUTE_VERTICAL_CENTER_FILL,
    DISTRIBUTE_VERTICAL_TOP,
    DISTRIBUTE_VERTICAL_TOP_FILL,
    DIVIDE,
    DIVIDE_CIRCLE,
    DIVIDE_CIRCLE_FILL,
    DIVIDE_SQUARE,
    DIVIDE_SQUARE_FILL,
    DOC,
    DOC_APPEND,
    DOC_APPEND_FILL,
    DOC_BADGE_ARROW_UP,
    DOC_BADGE_ARROW_UP_FILL,
    DOC_BADGE_ELLIPSIS,
    DOC_BADGE_GEARSHAPE,
    DOC_BADGE_GEARSHAPE_FILL,
    DOC_BADGE_PLUS,
    DOC_CIRCLE,
    DOC_CIRCLE_FILL,
    DOC_FILL,
    DOC_FILL_BADGE_ELLIPSIS,
    DOC_FILL_BADGE_PLUS,
    DOC_ON_CLIPBOARD,
    DOC_ON_CLIPBOARD_FILL,
    DOC_ON_DOC,
    DOC_ON_DOC_FILL,
    DOC_PLAINTEXT,
    DOC_PLAINTEXT_FILL,
    DOC_RICHTEXT,
    DOC_RICHTEXT_FILL,
    DOC_TEXT,
    DOC_TEXT_BELOW_ECG,
    DOC_TEXT_BELOW_ECG_FILL,
    DOC_TEXT_FILL,
    DOC_TEXT_IMAGE,
    DOC_TEXT_IMAGE_FILL,
    DOC_TEXT_MAGNIFYINGGLASS,
    DOC_VIEWFINDER,
    DOC_VIEWFINDER_FILL,
    DOC_ZIPPER,
    DOCK_ARROW_DOWN_RECTANGLE,
    DOCK_ARROW_UP_RECTANGLE,
    DOCK_RECTANGLE,
    DOLLARSIGN,
    DOLLARSIGN_ARROW_CIRCLEPATH,
    DOLLARSIGN_CIRCLE,
    DOLLARSIGN_CIRCLE_FILL,
    DOLLARSIGN_SQUARE,
    DOLLARSIGN_SQUARE_FILL,
    DONGSIGN,
    DONGSIGN_CIRCLE,
    DONGSIGN_CIRCLE_FILL,
    DONGSIGN_SQUARE,
    DONGSIGN_SQUARE_FILL,
    DOOR_FRENCH_CLOSED,
    DOOR_FRENCH_OPEN,
    DOOR_GARAGE_CLOSED,
    DOOR_GARAGE_CLOSED_TRIANGLEBADGE_EXCLAMATIONMARK,
    DOOR_GARAGE_DOUBLE_BAY_CLOSED,
    DOOR_GARAGE_DOUBLE_BAY_CLOSED_TRIANGLEBADGE_EXCLAMATIONMARK,
    DOOR_GARAGE_DOUBLE_BAY_OPEN,
    DOOR_GARAGE_DOUBLE_BAY_OPEN_TRIANGLEBADGE_EXCLAMATIONMARK,
    DOOR_GARAGE_OPEN,
    DOOR_GARAGE_OPEN_TRIANGLEBADGE_EXCLAMATIONMARK,
    DOOR_LEFT_HAND_CLOSED,
    DOOR_LEFT_HAND_OPEN,
    DOOR_RIGHT_HAND_CLOSED,
    DOOR_RIGHT_HAND_OPEN,
    DOOR_SLIDING_LEFT_HAND_CLOSED,
    DOOR_SLIDING_LEFT_HAND_OPEN,
    DOOR_SLIDING_RIGHT_HAND_CLOSED,
    DOOR_SLIDING_RIGHT_HAND_OPEN,
    DOT_ARROWTRIANGLES_UP_RIGHT_DOWN_LEFT_CIRCLE,
    DOT_CIRCLE_AND_CURSORARROW,
    DOT_CIRCLE_AND_HAND_POINT_UP_LEFT_FILL,
    DOT_CIRCLE_VIEWFINDER,
    DOT_RADIOWAVES_FORWARD,
    DOT_RADIOWAVES_LEFT_AND_RIGHT,
    DOT_RADIOWAVES_RIGHT,
    DOT_RADIOWAVES_UP_FORWARD,
    DOT_SQUARE,
    DOT_SQUARE_FILL,
    DOT_SQUARESHAPE,
    DOT_SQUARESHAPE_FILL,
    DOT_SQUARESHAPE_SPLIT_2X2,
    DOT_VIEWFINDER,
    DOTS_AND_LINE_VERTICAL_AND_CURSORARROW_RECTANGLE,
    DPAD,
    DPAD_DOWN_FILLED,
    DPAD_FILL,
    DPAD_LEFT_FILLED,
    DPAD_RIGHT_FILLED,
    DPAD_UP_FILLED,
    DROP,
    DROP_CIRCLE,
    DROP_CIRCLE_FILL,
    DROP_DEGREESIGN,
    DROP_DEGREESIGN_FILL,
    DROP_DEGREESIGN_SLASH,
    DROP_DEGREESIGN_SLASH_FILL,
    DROP_FILL,
    DROP_KEYPAD_RECTANGLE,
    DROP_KEYPAD_RECTANGLE_FILL,
    DROP_TRIANGLE,
    DROP_TRIANGLE_FILL,
    DRYER,
    DRYER_FILL,
    DUMBBELL,
    DUMBBELL_FILL,
    E_CIRCLE,
    E_CIRCLE_FILL,
    E_SQUARE,
    E_SQUARE_FILL,
    EAR,
    EAR_AND_WAVEFORM,
    EAR_BADGE_CHECKMARK,
    EAR_FILL,
    EAR_TRIANGLEBADGE_EXCLAMATIONMARK,
    EARBUDS,
    EARBUDS_CASE,
    EARBUDS_CASE_FILL,
    EARPODS,
    EJECT,
    EJECT_CIRCLE,
    EJECT_CIRCLE_FILL,
    EJECT_FILL,
    ELLIPSIS,
    ELLIPSIS_BUBBLE,
    ELLIPSIS_BUBBLE_FILL,
    ELLIPSIS_CIRCLE,
    ELLIPSIS_CIRCLE_FILL,
    ELLIPSIS_CURLYBRACES,
    ELLIPSIS_MESSAGE,
    ELLIPSIS_MESSAGE_FILL,
    ELLIPSIS_RECTANGLE,
    ELLIPSIS_RECTANGLE_FILL,
    ELLIPSIS_VERTICAL_BUBBLE,
    ELLIPSIS_VERTICAL_BUBBLE_FILL,
    ENTRY_LEVER_KEYPAD,
    ENTRY_LEVER_KEYPAD_FILL,
    ENTRY_LEVER_KEYPAD_TRIANGLEBADGE_EXCLAMATIONMARK,
    ENTRY_LEVER_KEYPAD_TRIANGLEBADGE_EXCLAMATIONMARK_FILL,
    ENVELOPE,
    ENVELOPE_ARROW_TRIANGLE_BRANCH,
    ENVELOPE_ARROW_TRIANGLE_BRANCH_FILL,
    ENVELOPE_BADGE,
    ENVELOPE_BADGE_FILL,
    ENVELOPE_BADGE_SHIELD_HALF_FILLED,
    ENVELOPE_BADGE_SHIELD_HALF_FILLED_FILL,
    ENVELOPE_CIRCLE,
    ENVELOPE_CIRCLE_FILL,
    ENVELOPE_FILL,
    ENVELOPE_OPEN,
    ENVELOPE_OPEN_BADGE_CLOCK,
    ENVELOPE_OPEN_FILL,
    EQUAL,
    EQUAL_CIRCLE,
    EQUAL_CIRCLE_FILL,
    EQUAL_SQUARE,
    EQUAL_SQUARE_FILL,
    ERASER,
    ERASER_FILL,
    ERASER_LINE_DASHED,
    ERASER_LINE_DASHED_FILL,
    ESCAPE,
    ESIM,
    ESIM_FILL,
    EUROSIGN,
    EUROSIGN_CIRCLE,
    EUROSIGN_CIRCLE_FILL,
    EUROSIGN_SQUARE,
    EUROSIGN_SQUARE_FILL,
    EXCLAMATIONMARK,
    EXCLAMATIONMARK_2,
    EXCLAMATIONMARK_3,
    EXCLAMATIONMARK_APPLEWATCH,
    EXCLAMATIONMARK_ARROW_CIRCLEPATH,
    EXCLAMATIONMARK_ARROW_TRIANGLE_2_CIRCLEPATH,
    EXCLAMATIONMARK_BUBBLE,
    EXCLAMATIONMARK_BUBBLE_CIRCLE,
    EXCLAMATIONMARK_BUBBLE_CIRCLE_FILL,
    EXCLAMATIONMARK_BUBBLE_FILL,
    EXCLAMATIONMARK_CIRCLE,
    EXCLAMATIONMARK_CIRCLE_FILL,
    EXCLAMATIONMARK_ICLOUD,
    EXCLAMATIONMARK_ICLOUD_FILL,
    EXCLAMATIONMARK_LOCK,
    EXCLAMATIONMARK_LOCK_FILL,
    EXCLAMATIONMARK_OCTAGON,
    EXCLAMATIONMARK_OCTAGON_FILL,
    EXCLAMATIONMARK_QUESTIONMARK,
    EXCLAMATIONMARK_SHIELD,
    EXCLAMATIONMARK_SHIELD_FILL,
    EXCLAMATIONMARK_SQUARE,
    EXCLAMATIONMARK_SQUARE_FILL,
    EXCLAMATIONMARK_TRIANGLE,
    EXCLAMATIONMARK_TRIANGLE_FILL,
    EXTERNALDRIVE,
    EXTERNALDRIVE_BADGE_CHECKMARK,
    EXTERNALDRIVE_BADGE_EXCLAMATIONMARK,
    EXTERNALDRIVE_BADGE_ICLOUD,
    EXTERNALDRIVE_BADGE_MINUS,
    EXTERNALDRIVE_BADGE_PERSON_CROP,
    EXTERNALDRIVE_BADGE_PLUS,
    EXTERNALDRIVE_BADGE_QUESTIONMARK,
    EXTERNALDRIVE_BADGE_TIMEMACHINE,
    EXTERNALDRIVE_BADGE_WIFI,
    EXTERNALDRIVE_BADGE_XMARK,
    EXTERNALDRIVE_CONNECTED_TO_LINE_BELOW,
    EXTERNALDRIVE_CONNECTED_TO_LINE_BELOW_FILL,
    EXTERNALDRIVE_FILL,
    EXTERNALDRIVE_FILL_BADGE_CHECKMARK,
    EXTERNALDRIVE_FILL_BADGE_EXCLAMATIONMARK,
    EXTERNALDRIVE_FILL_BADGE_ICLOUD,
    EXTERNALDRIVE_FILL_BADGE_MINUS,
    EXTERNALDRIVE_FILL_BADGE_PERSON_CROP,
    EXTERNALDRIVE_FILL_BADGE_PLUS,
    EXTERNALDRIVE_FILL_BADGE_QUESTIONMARK,
    EXTERNALDRIVE_FILL_BADGE_TIMEMACHINE,
    EXTERNALDRIVE_FILL_BADGE_WIFI,
    EXTERNALDRIVE_FILL_BADGE_XMARK,
    EXTERNALDRIVE_FILL_TRIANGLEBADGE_EXCLAMATIONMARK,
    EXTERNALDRIVE_TRIANGLEBADGE_EXCLAMATIONMARK,
    EYE,
    EYE_CIRCLE,
    EYE_CIRCLE_FILL,
    EYE_FILL,
    EYE_SLASH,
    EYE_SLASH_CIRCLE,
    EYE_SLASH_CIRCLE_FILL,
    EYE_SLASH_FILL,
    EYE_SQUARE,
    EYE_SQUARE_FILL,
    EYE_TRIANGLEBADGE_EXCLAMATIONMARK,
    EYE_TRIANGLEBADGE_EXCLAMATIONMARK_FILL,
    EYEBROW,
    EYEDROPPER,
    EYEDROPPER_FULL,
    EYEDROPPER_HALFFULL,
    EYEGLASSES,
    EYES,
    EYES_INVERSE,
    F_CIRCLE,
    F_CIRCLE_FILL,
    F_CURSIVE,
    F_CURSIVE_CIRCLE,
    F_CURSIVE_CIRCLE_FILL,
    F_SQUARE,
    F_SQUARE_FILL,
    FACE_DASHED,
    FACE_DASHED_FILL,
    FACE_SMILING,
    FACE_SMILING_FILL,
    FACEID,
    FACEMASK,
    FACEMASK_FILL,
    FAN_AND_LIGHT_CEILING,
    FAN_AND_LIGHT_CEILING_FILL,
    FAN_CEILING,
    FAN_CEILING_FILL,
    FAN_DESK,
    FAN_DESK_FILL,
    FAN_FLOOR,
    FAN_FLOOR_FILL,
    FAN_OSCILLATION,
    FAN_OSCILLATION_FILL,
    FANBLADES,
    FANBLADES_FILL,
    FAXMACHINE,
    FAXMACHINE_FILL,
    FERRY,
    FERRY_FILL,
    FIBRECHANNEL,
    FIGURE_2_AND_CHILD_HOLDINGHANDS,
    FIGURE_2_ARMS_OPEN,
    FIGURE_AMERICAN_FOOTBALL,
    FIGURE_AND_CHILD_HOLDINGHANDS,
    FIGURE_ARCHERY,
    FIGURE_ARMS_OPEN,
    FIGURE_AUSTRALIAN_FOOTBALL,
    FIGURE_BADMINTON,
    FIGURE_BARRE,
    FIGURE_BASEBALL,
    FIGURE_BASKETBALL,
    FIGURE_BOWLING,
    FIGURE_BOXING,
    FIGURE_CLIMBING,
    FIGURE_COOLDOWN,
    FIGURE_CORE_TRAINING,
    FIGURE_CRICKET,
    FIGURE_CROSS_TRAINING,
    FIGURE_CURLING,
    FIGURE_DANCE,
    FIGURE_DISC_SPORTS,
    FIGURE_DRESS_LINE_VERTICAL_FIGURE,
    FIGURE_ELLIPTICAL,
    FIGURE_EQUESTRIAN_SPORTS,
    FIGURE_FALL,
    FIGURE_FALL_CIRCLE,
    FIGURE_FALL_CIRCLE_FILL,
    FIGURE_FENCING,
    FIGURE_FISHING,
    FIGURE_FLEXIBILITY,
    FIGURE_GOLF,
    FIGURE_GYMNASTICS,
    FIGURE_HAND_CYCLING,
    FIGURE_HANDBALL,
    FIGURE_HIGHINTENSITY_INTERVALTRAINING,
    FIGURE_HIKING,
    FIGURE_HOCKEY,
    FIGURE_HUNTING,
    FIGURE_INDOOR_CYCLE,
    FIGURE_JUMPROPE,
    FIGURE_KICKBOXING,
    FIGURE_LACROSSE,
    FIGURE_MARTIAL_ARTS,
    FIGURE_MIND_AND_BODY,
    FIGURE_MIXED_CARDIO,
    FIGURE_OPEN_WATER_SWIM,
    FIGURE_OUTDOOR_CYCLE,
    FIGURE_PICKLEBALL,
    FIGURE_PILATES,
    FIGURE_PLAY,
    FIGURE_POOL_SWIM,
    FIGURE_RACQUETBALL,
    FIGURE_ROLL,
    FIGURE_ROLL_RUNNINGPACE,
    FIGURE_ROLLING,
    FIGURE_ROWER,
    FIGURE_RUGBY,
    FIGURE_RUN,
    FIGURE_RUN_CIRCLE,
    FIGURE_RUN_CIRCLE_FILL,
    FIGURE_SAILING,
    FIGURE_SKATING,
    FIGURE_SKIING_CROSSCOUNTRY,
    FIGURE_SKIING_DOWNHILL,
    FIGURE_SNOWBOARDING,
    FIGURE_SOCCER,
    FIGURE_SOCIALDANCE,
    FIGURE_SOFTBALL,
    FIGURE_SQUASH,
    FIGURE_STAIR_STEPPER,
    FIGURE_STAIRS,
    FIGURE_STAND,
    FIGURE_STAND_LINE_DOTTED_FIGURE_STAND,
    FIGURE_STEP_TRAINING,
    FIGURE_STRENGTHTRAINING_FUNCTIONAL,
    FIGURE_STRENGTHTRAINING_TRADITIONAL,
    FIGURE_SURFING,
    FIGURE_TABLE_TENNIS,
    FIGURE_TAICHI,
    FIGURE_TENNIS,
    FIGURE_TRACK_AND_FIELD,
    FIGURE_VOLLEYBALL,
    FIGURE_WALK,
    FIGURE_WALK_ARRIVAL,
    FIGURE_WALK_CIRCLE,
    FIGURE_WALK_CIRCLE_FILL,
    FIGURE_WALK_DEPARTURE,
    FIGURE_WALK_DIAMOND,
    FIGURE_WALK_DIAMOND_FILL,
    FIGURE_WALK_MOTION,
    FIGURE_WATER_FITNESS,
    FIGURE_WATERPOLO,
    FIGURE_WAVE,
    FIGURE_WAVE_CIRCLE,
    FIGURE_WAVE_CIRCLE_FILL,
    FIGURE_WRESTLING,
    FIGURE_YOGA,
    FILEMENU_AND_CURSORARROW,
    FILEMENU_AND_SELECTION,
    FILM,
    FILM_CIRCLE,
    FILM_CIRCLE_FILL,
    FILM_FILL,
    FILM_STACK,
    FILM_STACK_FILL,
    FIREPLACE,
    FIREPLACE_FILL,
    FIREWALL,
    FIREWALL_FILL,
    FISH,
    FISH_FILL,
    FLAG,
    FLAG_2_CROSSED,
    FLAG_2_CROSSED_CIRCLE,
    FLAG_2_CROSSED_CIRCLE_FILL,
    FLAG_2_CROSSED_FILL,
    FLAG_AND_FLAG_FILLED_CROSSED,
    FLAG_BADGE_ELLIPSIS,
    FLAG_BADGE_ELLIPSIS_FILL,
    FLAG_CHECKERED,
    FLAG_CHECKERED_2_CROSSED,
    FLAG_CIRCLE,
    FLAG_CIRCLE_FILL,
    FLAG_FILL,
    FLAG_FILLED_AND_FLAG_CROSSED,
    FLAG_SLASH,
    FLAG_SLASH_CIRCLE,
    FLAG_SLASH_CIRCLE_FILL,
    FLAG_SLASH_FILL,
    FLAG_SQUARE,
    FLAG_SQUARE_FILL,
    FLAME,
    FLAME_CIRCLE,
    FLAME_CIRCLE_FILL,
    FLAME_FILL,
    FLASHLIGHT_OFF_FILL,
    FLASHLIGHT_ON_FILL,
    FLEURON,
    FLEURON_FILL,
    FLIPPHONE,
    FLORINSIGN,
    FLORINSIGN_CIRCLE,
    FLORINSIGN_CIRCLE_FILL,
    FLORINSIGN_SQUARE,
    FLORINSIGN_SQUARE_FILL,
    FLOWCHART,
    FLOWCHART_FILL,
    FN,
    FOLDER,
    FOLDER_BADGE_GEARSHAPE,
    FOLDER_BADGE_MINUS,
    FOLDER_BADGE_PERSON_CROP,
    FOLDER_BADGE_PLUS,
    FOLDER_BADGE_QUESTIONMARK,
    FOLDER_CIRCLE,
    FOLDER_CIRCLE_FILL,
    FOLDER_FILL,
    FOLDER_FILL_BADGE_GEARSHAPE,
    FOLDER_FILL_BADGE_MINUS,
    FOLDER_FILL_BADGE_PERSON_CROP,
    FOLDER_FILL_BADGE_PLUS,
    FOLDER_FILL_BADGE_QUESTIONMARK,
    FOOTBALL,
    FOOTBALL_CIRCLE,
    FOOTBALL_CIRCLE_FILL,
    FOOTBALL_FILL,
    FORK_KNIFE,
    FORK_KNIFE_CIRCLE,
    FORK_KNIFE_CIRCLE_FILL,
    FORWARD,
    FORWARD_CIRCLE,
    FORWARD_CIRCLE_FILL,
    FORWARD_END,
    FORWARD_END_ALT,
    FORWARD_END_ALT_FILL,
    FORWARD_END_CIRCLE,
    FORWARD_END_CIRCLE_FILL,
    FORWARD_END_FILL,
    FORWARD_FILL,
    FORWARD_FRAME,
    FORWARD_FRAME_FILL,
    FOSSIL_SHELL,
    FOSSIL_SHELL_FILL,
    FRANCSIGN,
    FRANCSIGN_CIRCLE,
    FRANCSIGN_CIRCLE_FILL,
    FRANCSIGN_SQUARE,
    FRANCSIGN_SQUARE_FILL,
    FRYING_PAN,
    FRYING_PAN_FILL,
    FUELPUMP,
    FUELPUMP_CIRCLE,
    FUELPUMP_CIRCLE_FILL,
    FUELPUMP_FILL,
    FUNCTION,
    FX,
    G_CIRCLE,
    G_CIRCLE_FILL,
    G_SQUARE,
    G_SQUARE_FILL,
    GAMECONTROLLER,
    GAMECONTROLLER_FILL,
    GAUGE,
    GAUGE_BADGE_MINUS,
    GAUGE_BADGE_PLUS,
    GAUGE_HIGH,
    GAUGE_LOW,
    GAUGE_MEDIUM,
    GAUGE_MEDIUM_BADGE_MINUS,
    GAUGE_MEDIUM_BADGE_PLUS,
    GEAR,
    GEAR_BADGE,
    GEAR_BADGE_CHECKMARK,
    GEAR_BADGE_QUESTIONMARK,
    GEAR_BADGE_XMARK,
    GEAR_CIRCLE,
    GEAR_CIRCLE_FILL,
    GEARSHAPE,
    GEARSHAPE_2,
    GEARSHAPE_2_FILL,
    GEARSHAPE_ARROW_TRIANGLE_2_CIRCLEPATH,
    GEARSHAPE_CIRCLE,
    GEARSHAPE_CIRCLE_FILL,
    GEARSHAPE_FILL,
    GIFT,
    GIFT_CIRCLE,
    GIFT_CIRCLE_FILL,
    GIFT_FILL,
    GIFTCARD,
    GIFTCARD_FILL,
    GLOBE,
    GLOBE_AMERICAS,
    GLOBE_AMERICAS_FILL,
    GLOBE_ASIA_AUSTRALIA,
    GLOBE_ASIA_AUSTRALIA_FILL,
    GLOBE_BADGE_CHEVRON_BACKWARD,
    GLOBE_CENTRAL_SOUTH_ASIA,
    GLOBE_CENTRAL_SOUTH_ASIA_FILL,
    GLOBE_DESK,
    GLOBE_DESK_FILL,
    GLOBE_EUROPE_AFRICA,
    GLOBE_EUROPE_AFRICA_FILL,
    GOBACKWARD,
    GOBACKWARD_10,
    GOBACKWARD_15,
    GOBACKWARD_30,
    GOBACKWARD_45,
    GOBACKWARD_5,
    GOBACKWARD_60,
    GOBACKWARD_75,
    GOBACKWARD_90,
    GOBACKWARD_MINUS,
    GOFORWARD,
    GOFORWARD_10,
    GOFORWARD_15,
    GOFORWARD_30,
    GOFORWARD_45,
    GOFORWARD_5,
    GOFORWARD_60,
    GOFORWARD_75,
    GOFORWARD_90,
    GOFORWARD_PLUS,
    GRADUATIONCAP,
    GRADUATIONCAP_CIRCLE,
    GRADUATIONCAP_CIRCLE_FILL,
    GRADUATIONCAP_FILL,
    GREATERTHAN,
    GREATERTHAN_CIRCLE,
    GREATERTHAN_CIRCLE_FILL,
    GREATERTHAN_SQUARE,
    GREATERTHAN_SQUARE_FILL,
    GREETINGCARD,
    GREETINGCARD_FILL,
    GRID,
    GRID_CIRCLE,
    GRID_CIRCLE_FILL,
    GUARANISIGN,
    GUARANISIGN_CIRCLE,
    GUARANISIGN_CIRCLE_FILL,
    GUARANISIGN_SQUARE,
    GUARANISIGN_SQUARE_FILL,
    GUITARS,
    GUITARS_FILL,
    GYROSCOPE,
    H_CIRCLE,
    H_CIRCLE_FILL,
    H_SQUARE,
    H_SQUARE_FILL,
    H_SQUARE_ON_SQUARE,
    H_SQUARE_ON_SQUARE_FILL,
    HAMMER,
    HAMMER_CIRCLE,
    HAMMER_CIRCLE_FILL,
    HAMMER_FILL,
    HAND_DRAW,
    HAND_DRAW_FILL,
    HAND_POINT_DOWN,
    HAND_POINT_DOWN_FILL,
    HAND_POINT_LEFT,
    HAND_POINT_LEFT_FILL,
    HAND_POINT_RIGHT,
    HAND_POINT_RIGHT_FILL,
    HAND_POINT_UP,
    HAND_POINT_UP_BRAILLE,
    HAND_POINT_UP_BRAILLE_FILL,
    HAND_POINT_UP_FILL,
    HAND_POINT_UP_LEFT,
    HAND_POINT_UP_LEFT_FILL,
    HAND_RAISED,
    HAND_RAISED_CIRCLE,
    HAND_RAISED_CIRCLE_FILL,
    HAND_RAISED_FILL,
    HAND_RAISED_FINGERS_SPREAD,
    HAND_RAISED_FINGERS_SPREAD_FILL,
    HAND_RAISED_SLASH,
    HAND_RAISED_SLASH_FILL,
    HAND_RAISED_SQUARE,
    HAND_RAISED_SQUARE_FILL,
    HAND_RAISED_SQUARE_ON_SQUARE,
    HAND_RAISED_SQUARE_ON_SQUARE_FILL,
    HAND_TAP,
    HAND_TAP_FILL,
    HAND_THUMBSDOWN,
    HAND_THUMBSDOWN_CIRCLE,
    HAND_THUMBSDOWN_CIRCLE_FILL,
    HAND_THUMBSDOWN_FILL,
    HAND_THUMBSUP,
    HAND_THUMBSUP_CIRCLE,
    HAND_THUMBSUP_CIRCLE_FILL,
    HAND_THUMBSUP_FILL,
    HAND_WAVE,
    HAND_WAVE_FILL,
    HANDS_CLAP,
    HANDS_CLAP_FILL,
    HANDS_SPARKLES,
    HANDS_SPARKLES_FILL,
    HARE,
    HARE_FILL,
    HEADPHONES,
    HEADPHONES_CIRCLE,
    HEADPHONES_CIRCLE_FILL,
    HEARINGDEVICE_AND_SIGNAL_METER,
    HEARINGDEVICE_AND_SIGNAL_METER_FILL,
    HEARINGDEVICE_EAR,
    HEARINGDEVICE_EAR_FILL,
    HEART,
    HEART_CIRCLE,
    HEART_CIRCLE_FILL,
    HEART_FILL,
    HEART_RECTANGLE,
    HEART_RECTANGLE_FILL,
    HEART_SLASH,
    HEART_SLASH_CIRCLE,
    HEART_SLASH_CIRCLE_FILL,
    HEART_SLASH_FILL,
    HEART_SQUARE,
    HEART_SQUARE_FILL,
    HEART_TEXT_SQUARE,
    HEART_TEXT_SQUARE_FILL,
    HEATER_VERTICAL,
    HEATER_VERTICAL_FILL,
    HELM,
    HEXAGON,
    HEXAGON_BOTTOMHALF_FILLED,
    HEXAGON_FILL,
    HEXAGON_LEFTHALF_FILLED,
    HEXAGON_RIGHTHALF_FILLED,
    HEXAGON_TOPHALF_FILLED,
    HIFIRECEIVER,
    HIFIRECEIVER_FILL,
    HIFISPEAKER,
    HIFISPEAKER_2,
    HIFISPEAKER_2_FILL,
    HIFISPEAKER_AND_APPLETV,
    HIFISPEAKER_AND_APPLETV_FILL,
    HIFISPEAKER_AND_HOMEPOD,
    HIFISPEAKER_AND_HOMEPOD_FILL,
    HIFISPEAKER_AND_HOMEPODMINI,
    HIFISPEAKER_AND_HOMEPODMINI_FILL,
    HIFISPEAKER_FILL,
    HIGHLIGHTER,
    HOCKEY_PUCK,
    HOCKEY_PUCK_CIRCLE,
    HOCKEY_PUCK_CIRCLE_FILL,
    HOCKEY_PUCK_FILL,
    HOMEKIT,
    HOMEPOD,
    HOMEPOD_2,
    HOMEPOD_2_FILL,
    HOMEPOD_AND_APPLETV,
    HOMEPOD_AND_APPLETV_FILL,
    HOMEPOD_AND_HOMEPODMINI,
    HOMEPOD_AND_HOMEPODMINI_FILL,
    HOMEPOD_FILL,
    HOMEPODMINI,
    HOMEPODMINI_2,
    HOMEPODMINI_2_FILL,
    HOMEPODMINI_AND_APPLETV,
    HOMEPODMINI_AND_APPLETV_FILL,
    HOMEPODMINI_FILL,
    HOURGLASS,
    HOURGLASS_BADGE_PLUS,
    HOURGLASS_BOTTOMHALF_FILLED,
    HOURGLASS_CIRCLE,
    HOURGLASS_CIRCLE_FILL,
    HOURGLASS_TOPHALF_FILLED,
    HOUSE,
    HOUSE_CIRCLE,
    HOUSE_CIRCLE_FILL,
    HOUSE_FILL,
    HRYVNIASIGN,
    HRYVNIASIGN_CIRCLE,
    HRYVNIASIGN_CIRCLE_FILL,
    HRYVNIASIGN_SQUARE,
    HRYVNIASIGN_SQUARE_FILL,
    HUMIDIFIER,
    HUMIDIFIER_AND_DROPLETS,
    HUMIDIFIER_AND_DROPLETS_FILL,
    HUMIDIFIER_FILL,
    HUMIDITY,
    HUMIDITY_FILL,
    HURRICANE,
    HURRICANE_CIRCLE,
    HURRICANE_CIRCLE_FILL,
    I_CIRCLE,
    I_CIRCLE_FILL,
    I_SQUARE,
    I_SQUARE_FILL,
    ICLOUD,
    ICLOUD_AND_ARROW_DOWN,
    ICLOUD_AND_ARROW_DOWN_FILL,
    ICLOUD_AND_ARROW_UP,
    ICLOUD_AND_ARROW_UP_FILL,
    ICLOUD_CIRCLE,
    ICLOUD_CIRCLE_FILL,
    ICLOUD_FILL,
    ICLOUD_SLASH,
    ICLOUD_SLASH_FILL,
    ICLOUD_SQUARE,
    ICLOUD_SQUARE_FILL,
    INCREASE_INDENT,
    INCREASE_QUOTELEVEL,
    INDIANRUPEESIGN,
    INDIANRUPEESIGN_CIRCLE,
    INDIANRUPEESIGN_CIRCLE_FILL,
    INDIANRUPEESIGN_SQUARE,
    INDIANRUPEESIGN_SQUARE_FILL,
    INFINITY,
    INFINITY_CIRCLE,
    INFINITY_CIRCLE_FILL,
    INFO,
    INFO_BUBBLE,
    INFO_BUBBLE_FILL,
    INFO_CIRCLE,
    INFO_CIRCLE_FILL,
    INFO_SQUARE,
    INFO_SQUARE_FILL,
    INTERNALDRIVE,
    INTERNALDRIVE_FILL,
    IPAD,
    IPAD_AND_ARROW_FORWARD,
    IPAD_AND_IPHONE,
    IPAD_BADGE_PLAY,
    IPAD_HOMEBUTTON,
    IPAD_HOMEBUTTON_BADGE_PLAY,
    IPAD_HOMEBUTTON_LANDSCAPE,
    IPAD_HOMEBUTTON_LANDSCAPE_BADGE_PLAY,
    IPAD_LANDSCAPE,
    IPAD_LANDSCAPE_BADGE_PLAY,
    IPAD_REAR_CAMERA,
    IPHONE,
    IPHONE_AND_ARROW_FORWARD,
    IPHONE_BADGE_PLAY,
    IPHONE_CIRCLE,
    IPHONE_CIRCLE_FILL,
    IPHONE_HOMEBUTTON,
    IPHONE_HOMEBUTTON_BADGE_PLAY,
    IPHONE_HOMEBUTTON_CIRCLE,
    IPHONE_HOMEBUTTON_CIRCLE_FILL,
    IPHONE_HOMEBUTTON_LANDSCAPE,
    IPHONE_HOMEBUTTON_RADIOWAVES_LEFT_AND_RIGHT,
    IPHONE_HOMEBUTTON_RADIOWAVES_LEFT_AND_RIGHT_CIRCLE,
    IPHONE_HOMEBUTTON_RADIOWAVES_LEFT_AND_RIGHT_CIRCLE_FILL,
    IPHONE_HOMEBUTTON_SLASH,
    IPHONE_HOMEBUTTON_SLASH_CIRCLE,
    IPHONE_HOMEBUTTON_SLASH_CIRCLE_FILL,
    IPHONE_LANDSCAPE,
    IPHONE_RADIOWAVES_LEFT_AND_RIGHT,
    IPHONE_RADIOWAVES_LEFT_AND_RIGHT_CIRCLE,
    IPHONE_RADIOWAVES_LEFT_AND_RIGHT_CIRCLE_FILL,
    IPHONE_REAR_CAMERA,
    IPHONE_SLASH,
    IPHONE_SLASH_CIRCLE,
    IPHONE_SLASH_CIRCLE_FILL,
    IPHONE_SMARTBATTERYCASE_GEN1,
    IPHONE_SMARTBATTERYCASE_GEN2,
    IPOD,
    IPODSHUFFLE_GEN1,
    IPODSHUFFLE_GEN2,
    IPODSHUFFLE_GEN3,
    IPODSHUFFLE_GEN4,
    IPODTOUCH,
    IPODTOUCH_LANDSCAPE,
    IPODTOUCH_SLASH,
    ITALIC,
    IVFLUID_BAG,
    IVFLUID_BAG_FILL,
    J_CIRCLE,
    J_CIRCLE_FILL,
    J_SQUARE,
    J_SQUARE_FILL,
    J_SQUARE_ON_SQUARE,
    J_SQUARE_ON_SQUARE_FILL,
    K,
    K_CIRCLE,
    K_CIRCLE_FILL,
    K_SQUARE,
    K_SQUARE_FILL,
    KEY,
    KEY_FILL,
    KEY_ICLOUD,
    KEY_ICLOUD_FILL,
    KEY_VIEWFINDER,
    KEYBOARD,
    KEYBOARD_BADGE_ELLIPSIS,
    KEYBOARD_BADGE_ELLIPSIS_FILL,
    KEYBOARD_BADGE_EYE,
    KEYBOARD_BADGE_EYE_FILL,
    KEYBOARD_CHEVRON_COMPACT_DOWN,
    KEYBOARD_CHEVRON_COMPACT_DOWN_FILL,
    KEYBOARD_CHEVRON_COMPACT_LEFT,
    KEYBOARD_CHEVRON_COMPACT_LEFT_FILL,
    KEYBOARD_FILL,
    KEYBOARD_MACWINDOW,
    KEYBOARD_ONEHANDED_LEFT,
    KEYBOARD_ONEHANDED_LEFT_FILL,
    KEYBOARD_ONEHANDED_RIGHT,
    KEYBOARD_ONEHANDED_RIGHT_FILL,
    KIPSIGN,
    KIPSIGN_CIRCLE,
    KIPSIGN_CIRCLE_FILL,
    KIPSIGN_SQUARE,
    KIPSIGN_SQUARE_FILL,
    L_CIRCLE,
    L_CIRCLE_FILL,
    L_JOYSTICK,
    L_JOYSTICK_FILL,
    L_JOYSTICK_PRESS_DOWN,
    L_JOYSTICK_PRESS_DOWN_FILL,
    L_JOYSTICK_TILT_DOWN,
    L_JOYSTICK_TILT_DOWN_FILL,
    L_JOYSTICK_TILT_LEFT,
    L_JOYSTICK_TILT_LEFT_FILL,
    L_JOYSTICK_TILT_RIGHT,
    L_JOYSTICK_TILT_RIGHT_FILL,
    L_JOYSTICK_TILT_UP,
    L_JOYSTICK_TILT_UP_FILL,
    L_RECTANGLE_ROUNDEDBOTTOM,
    L_RECTANGLE_ROUNDEDBOTTOM_FILL,
    L_SQUARE,
    L_SQUARE_FILL,
    L1_RECTANGLE_ROUNDEDBOTTOM,
    L1_RECTANGLE_ROUNDEDBOTTOM_FILL,
    L2_RECTANGLE_ROUNDEDTOP,
    L2_RECTANGLE_ROUNDEDTOP_FILL,
    LADYBUG,
    LADYBUG_FILL,
    LAMP_CEILING,
    LAMP_CEILING_FILL,
    LAMP_CEILING_INVERSE,
    LAMP_DESK,
    LAMP_DESK_FILL,
    LAMP_FLOOR,
    LAMP_FLOOR_FILL,
    LAMP_TABLE,
    LAMP_TABLE_FILL,
    LANYARDCARD,
    LANYARDCARD_FILL,
    LAPTOPCOMPUTER,
    LAPTOPCOMPUTER_AND_ARROW_DOWN,
    LAPTOPCOMPUTER_AND_IPAD,
    LAPTOPCOMPUTER_AND_IPHONE,
    LAPTOPCOMPUTER_TRIANGLEBADGE_EXCLAMATIONMARK,
    LARISIGN,
    LARISIGN_CIRCLE,
    LARISIGN_CIRCLE_FILL,
    LARISIGN_SQUARE,
    LARISIGN_SQUARE_FILL,
    LASSO,
    LASSO_AND_SPARKLES,
    LATCH_2_CASE,
    LATCH_2_CASE_FILL,
    LAUREL_LEADING,
    LAUREL_TRAILING,
    LB_RECTANGLE_ROUNDEDBOTTOM,
    LB_RECTANGLE_ROUNDEDBOTTOM_FILL,
    LEAF,
    LEAF_ARROW_TRIANGLE_CIRCLEPATH,
    LEAF_CIRCLE,
    LEAF_CIRCLE_FILL,
    LEAF_FILL,
    LESSTHAN,
    LESSTHAN_CIRCLE,
    LESSTHAN_CIRCLE_FILL,
    LESSTHAN_SQUARE,
    LESSTHAN_SQUARE_FILL,
    LEVEL,
    LEVEL_FILL,
    LIFEPRESERVER,
    LIFEPRESERVER_FILL,
    LIGHT_BEACON_MAX,
    LIGHT_BEACON_MAX_FILL,
    LIGHT_BEACON_MIN,
    LIGHT_BEACON_MIN_FILL,
    LIGHT_CYLINDRICAL_CEILING,
    LIGHT_CYLINDRICAL_CEILING_FILL,
    LIGHT_CYLINDRICAL_CEILING_INVERSE,
    LIGHT_MAX,
    LIGHT_MIN,
    LIGHT_PANEL,
    LIGHT_PANEL_FILL,
    LIGHT_RECESSED,
    LIGHT_RECESSED_3,
    LIGHT_RECESSED_3_FILL,
    LIGHT_RECESSED_3_INVERSE,
    LIGHT_RECESSED_FILL,
    LIGHT_RECESSED_INVERSE,
    LIGHT_RIBBON,
    LIGHT_RIBBON_FILL,
    LIGHT_STRIP_2,
    LIGHT_STRIP_2_FILL,
    LIGHTBULB,
    LIGHTBULB_2,
    LIGHTBULB_2_FILL,
    LIGHTBULB_CIRCLE,
    LIGHTBULB_CIRCLE_FILL,
    LIGHTBULB_FILL,
    LIGHTBULB_LED,
    LIGHTBULB_LED_FILL,
    LIGHTBULB_LED_WIDE,
    LIGHTBULB_LED_WIDE_FILL,
    LIGHTBULB_SLASH,
    LIGHTBULB_SLASH_FILL,
    LIGHTSWITCH_OFF,
    LIGHTSWITCH_OFF_FILL,
    LIGHTSWITCH_OFF_SQUARE,
    LIGHTSWITCH_OFF_SQUARE_FILL,
    LIGHTSWITCH_ON,
    LIGHTSWITCH_ON_FILL,
    LIGHTSWITCH_ON_SQUARE,
    LIGHTSWITCH_ON_SQUARE_FILL,
    LINE_2_HORIZONTAL_DECREASE_CIRCLE,
    LINE_2_HORIZONTAL_DECREASE_CIRCLE_FILL,
    LINE_3_CROSSED_SWIRL_CIRCLE,
    LINE_3_CROSSED_SWIRL_CIRCLE_FILL,
    LINE_3_HORIZONTAL,
    LINE_3_HORIZONTAL_CIRCLE,
    LINE_3_HORIZONTAL_CIRCLE_FILL,
    LINE_3_HORIZONTAL_DECREASE,
    LINE_3_HORIZONTAL_DECREASE_CIRCLE,
    LINE_3_HORIZONTAL_DECREASE_CIRCLE_FILL,
    LINE_DIAGONAL,
    LINE_DIAGONAL_ARROW,
    LINE_HORIZONTAL_STAR_FILL_LINE_HORIZONTAL,
    LINES_MEASUREMENT_HORIZONTAL,
    LINEWEIGHT,
    LINK,
    LINK_BADGE_PLUS,
    LINK_CIRCLE,
    LINK_CIRCLE_FILL,
    LINK_ICLOUD,
    LINK_ICLOUD_FILL,
    LIRASIGN,
    LIRASIGN_CIRCLE,
    LIRASIGN_CIRCLE_FILL,
    LIRASIGN_SQUARE,
    LIRASIGN_SQUARE_FILL,
    LIST_AND_FILM,
    LIST_BULLET,
    LIST_BULLET_BELOW_RECTANGLE,
    LIST_BULLET_CIRCLE,
    LIST_BULLET_CIRCLE_FILL,
    LIST_BULLET_CLIPBOARD,
    LIST_BULLET_CLIPBOARD_FILL,
    LIST_BULLET_INDENT,
    LIST_BULLET_RECTANGLE,
    LIST_BULLET_RECTANGLE_FILL,
    LIST_BULLET_RECTANGLE_PORTRAIT,
    LIST_BULLET_RECTANGLE_PORTRAIT_FILL,
    LIST_CLIPBOARD,
    LIST_CLIPBOARD_FILL,
    LIST_DASH,
    LIST_DASH_HEADER_RECTANGLE,
    LIST_NUMBER,
    LIST_STAR,
    LIST_TRIANGLE,
    LIVEPHOTO,
    LIVEPHOTO_BADGE_A,
    LIVEPHOTO_PLAY,
    LIVEPHOTO_SLASH,
    LIZARD,
    LIZARD_FILL,
    LOCATION,
    LOCATION_CIRCLE,
    LOCATION_CIRCLE_FILL,
    LOCATION_FILL,
    LOCATION_FILL_VIEWFINDER,
    LOCATION_MAGNIFYINGGLASS,
    LOCATION_NORTH,
    LOCATION_NORTH_CIRCLE,
    LOCATION_NORTH_CIRCLE_FILL,
    LOCATION_NORTH_FILL,
    LOCATION_NORTH_LINE,
    LOCATION_NORTH_LINE_FILL,
    LOCATION_SLASH,
    LOCATION_SLASH_CIRCLE,
    LOCATION_SLASH_CIRCLE_FILL,
    LOCATION_SLASH_FILL,
    LOCATION_SQUARE,
    LOCATION_SQUARE_FILL,
    LOCATION_VIEWFINDER,
    LOCK,
    LOCK_APPLEWATCH,
    LOCK_CIRCLE,
    LOCK_CIRCLE_FILL,
    LOCK_DESKTOPCOMPUTER,
    LOCK_DISPLAY,
    LOCK_DOC,
    LOCK_DOC_FILL,
    LOCK_FILL,
    LOCK_ICLOUD,
    LOCK_ICLOUD_FILL,
    LOCK_IPAD,
    LOCK_IPHONE,
    LOCK_LAPTOPCOMPUTER,
    LOCK_OPEN,
    LOCK_OPEN_APPLEWATCH,
    LOCK_OPEN_DESKTOPCOMPUTER,
    LOCK_OPEN_DISPLAY,
    LOCK_OPEN_FILL,
    LOCK_OPEN_IPAD,
    LOCK_OPEN_IPHONE,
    LOCK_OPEN_LAPTOPCOMPUTER,
    LOCK_OPEN_TRIANGLEBADGE_EXCLAMATIONMARK,
    LOCK_OPEN_TRIANGLEBADGE_EXCLAMATIONMARK_FILL,
    LOCK_RECTANGLE,
    LOCK_RECTANGLE_FILL,
    LOCK_RECTANGLE_ON_RECTANGLE,
    LOCK_RECTANGLE_ON_RECTANGLE_FILL,
    LOCK_RECTANGLE_STACK,
    LOCK_RECTANGLE_STACK_FILL,
    LOCK_ROTATION,
    LOCK_ROTATION_OPEN,
    LOCK_SHIELD,
    LOCK_SHIELD_FILL,
    LOCK_SLASH,
    LOCK_SLASH_FILL,
    LOCK_SQUARE,
    LOCK_SQUARE_FILL,
    LOCK_SQUARE_STACK,
    LOCK_SQUARE_STACK_FILL,
    LOCK_TRIANGLEBADGE_EXCLAMATIONMARK,
    LOCK_TRIANGLEBADGE_EXCLAMATIONMARK_FILL,
    LOGO_PLAYSTATION,
    LOGO_XBOX,
    LOUPE,
    LT_RECTANGLE_ROUNDEDTOP,
    LT_RECTANGLE_ROUNDEDTOP_FILL,
    LUNGS,
    LUNGS_FILL,
    M_CIRCLE,
    M_CIRCLE_FILL,
    M_SQUARE,
    M_SQUARE_FILL,
    MACMINI,
    MACMINI_FILL,
    MACPRO_GEN1,
    MACPRO_GEN1_FILL,
    MACPRO_GEN2,
    MACPRO_GEN2_FILL,
    MACPRO_GEN3,
    MACPRO_GEN3_FILL,
    MACPRO_GEN3_SERVER,
    MACSTUDIO,
    MACSTUDIO_FILL,
    MACWINDOW,
    MACWINDOW_BADGE_PLUS,
    MACWINDOW_ON_RECTANGLE,
    MAGAZINE,
    MAGAZINE_FILL,
    MAGICMOUSE,
    MAGICMOUSE_FILL,
    MAGNIFYINGGLASS,
    MAGNIFYINGGLASS_CIRCLE,
    MAGNIFYINGGLASS_CIRCLE_FILL,
    MAGSAFE_BATTERYPACK,
    MAGSAFE_BATTERYPACK_FILL,
    MAIL,
    MAIL_AND_TEXT_MAGNIFYINGGLASS,
    MAIL_FILL,
    MAIL_STACK,
    MAIL_STACK_FILL,
    MANATSIGN,
    MANATSIGN_CIRCLE,
    MANATSIGN_CIRCLE_FILL,
    MANATSIGN_SQUARE,
    MANATSIGN_SQUARE_FILL,
    MAP,
    MAP_CIRCLE,
    MAP_CIRCLE_FILL,
    MAP_FILL,
    MAPPIN,
    MAPPIN_AND_ELLIPSE,
    MAPPIN_CIRCLE,
    MAPPIN_CIRCLE_FILL,
    MAPPIN_SLASH,
    MAPPIN_SLASH_CIRCLE,
    MAPPIN_SLASH_CIRCLE_FILL,
    MAPPIN_SQUARE,
    MAPPIN_SQUARE_FILL,
    MEDAL,
    MEDAL_FILL,
    MEDIASTICK,
    MEDICAL_THERMOMETER,
    MEDICAL_THERMOMETER_FILL,
    MEGAPHONE,
    MEGAPHONE_FILL,
    MEMORIES,
    MEMORIES_BADGE_MINUS,
    MEMORIES_BADGE_PLUS,
    MEMORYCHIP,
    MEMORYCHIP_FILL,
    MENUBAR_ARROW_DOWN_RECTANGLE,
    MENUBAR_ARROW_UP_RECTANGLE,
    MENUBAR_DOCK_RECTANGLE,
    MENUBAR_DOCK_RECTANGLE_BADGE_RECORD,
    MENUBAR_RECTANGLE,
    MENUCARD,
    MENUCARD_FILL,
    MESSAGE,
    MESSAGE_AND_WAVEFORM,
    MESSAGE_AND_WAVEFORM_FILL,
    MESSAGE_BADGE,
    MESSAGE_BADGE_CIRCLE,
    MESSAGE_BADGE_CIRCLE_FILL,
    MESSAGE_BADGE_FILL,
    MESSAGE_BADGE_FILLED_FILL,
    MESSAGE_CIRCLE,
    MESSAGE_CIRCLE_FILL,
    MESSAGE_FILL,
    METRONOME,
    METRONOME_FILL,
    MIC,
    MIC_AND_SIGNAL_METER,
    MIC_AND_SIGNAL_METER_FILL,
    MIC_BADGE_PLUS,
    MIC_BADGE_XMARK,
    MIC_CIRCLE,
    MIC_CIRCLE_FILL,
    MIC_FILL,
    MIC_FILL_BADGE_PLUS,
    MIC_FILL_BADGE_XMARK,
    MIC_SLASH,
    MIC_SLASH_CIRCLE,
    MIC_SLASH_CIRCLE_FILL,
    MIC_SLASH_FILL,
    MIC_SQUARE,
    MIC_SQUARE_FILL,
    MICROBE,
    MICROBE_CIRCLE,
    MICROBE_CIRCLE_FILL,
    MICROBE_FILL,
    MICROWAVE,
    MICROWAVE_FILL,
    MILLSIGN,
    MILLSIGN_CIRCLE,
    MILLSIGN_CIRCLE_FILL,
    MILLSIGN_SQUARE,
    MILLSIGN_SQUARE_FILL,
    MINUS,
    MINUS_CIRCLE,
    MINUS_CIRCLE_FILL,
    MINUS_DIAMOND,
    MINUS_DIAMOND_FILL,
    MINUS_FORWARDSLASH_PLUS,
    MINUS_MAGNIFYINGGLASS,
    MINUS_PLUS_BATTERYBLOCK,
    MINUS_PLUS_BATTERYBLOCK_FILL,
    MINUS_RECTANGLE,
    MINUS_RECTANGLE_FILL,
    MINUS_RECTANGLE_PORTRAIT,
    MINUS_RECTANGLE_PORTRAIT_FILL,
    MINUS_SQUARE,
    MINUS_SQUARE_FILL,
    MOON,
    MOON_CIRCLE,
    MOON_CIRCLE_FILL,
    MOON_FILL,
    MOON_HAZE,
    MOON_HAZE_CIRCLE,
    MOON_HAZE_CIRCLE_FILL,
    MOON_HAZE_FILL,
    MOON_STARS,
    MOON_STARS_CIRCLE,
    MOON_STARS_CIRCLE_FILL,
    MOON_STARS_FILL,
    MOON_ZZZ,
    MOON_ZZZ_FILL,
    MOONPHASE_FIRST_QUARTER,
    MOONPHASE_FIRST_QUARTER_INVERSE,
    MOONPHASE_FULL_MOON,
    MOONPHASE_FULL_MOON_INVERSE,
    MOONPHASE_LAST_QUARTER,
    MOONPHASE_LAST_QUARTER_INVERSE,
    MOONPHASE_NEW_MOON,
    MOONPHASE_NEW_MOON_INVERSE,
    MOONPHASE_WANING_CRESCENT,
    MOONPHASE_WANING_CRESCENT_INVERSE,
    MOONPHASE_WANING_GIBBOUS,
    MOONPHASE_WANING_GIBBOUS_INVERSE,
    MOONPHASE_WAXING_CRESCENT,
    MOONPHASE_WAXING_CRESCENT_INVERSE,
    MOONPHASE_WAXING_GIBBOUS,
    MOONPHASE_WAXING_GIBBOUS_INVERSE,
    MOSAIC,
    MOSAIC_FILL,
    MOUNT,
    MOUNT_FILL,
    MOUTH,
    MOUTH_FILL,
    MOVE_3D,
    MULTIPLY,
    MULTIPLY_CIRCLE,
    MULTIPLY_CIRCLE_FILL,
    MULTIPLY_SQUARE,
    MULTIPLY_SQUARE_FILL,
    MUSIC_MIC,
    MUSIC_MIC_CIRCLE,
    MUSIC_MIC_CIRCLE_FILL,
    MUSIC_NOTE,
    MUSIC_NOTE_HOUSE,
    MUSIC_NOTE_HOUSE_FILL,
    MUSIC_NOTE_LIST,
    MUSIC_NOTE_TV,
    MUSIC_NOTE_TV_FILL,
    MUSIC_QUARTERNOTE_3,
    MUSTACHE,
    MUSTACHE_FILL,
    N_CIRCLE,
    N_CIRCLE_FILL,
    N_SQUARE,
    N_SQUARE_FILL,
    NAIRASIGN,
    NAIRASIGN_CIRCLE,
    NAIRASIGN_CIRCLE_FILL,
    NAIRASIGN_SQUARE,
    NAIRASIGN_SQUARE_FILL,
    NETWORK,
    NETWORK_BADGE_SHIELD_HALF_FILLED,
    NEWSPAPER,
    NEWSPAPER_CIRCLE,
    NEWSPAPER_CIRCLE_FILL,
    NEWSPAPER_FILL,
    NOSE,
    NOSE_FILL,
    NOSIGN,
    NOTE,
    NOTE_TEXT,
    NOTE_TEXT_BADGE_PLUS,
    NUMBER,
    NUMBER_CIRCLE,
    NUMBER_CIRCLE_FILL,
    NUMBER_SQUARE,
    NUMBER_SQUARE_FILL,
    NUMBERSIGN,
    O_CIRCLE,
    O_CIRCLE_FILL,
    O_SQUARE,
    O_SQUARE_FILL,
    OAR_2_CROSSED,
    OCTAGON,
    OCTAGON_BOTTOMHALF_FILLED,
    OCTAGON_FILL,
    OCTAGON_LEFTHALF_FILLED,
    OCTAGON_RIGHTHALF_FILLED,
    OCTAGON_TOPHALF_FILLED,
    OPTICALDISC,
    OPTICALDISC_FILL,
    OPTICALDISCDRIVE,
    OPTICALDISCDRIVE_FILL,
    OPTION,
    OVAL,
    OVAL_BOTTOMHALF_FILLED,
    OVAL_FILL,
    OVAL_INSET_FILLED,
    OVAL_LEFTHALF_FILLED,
    OVAL_PORTRAIT,
    OVAL_PORTRAIT_BOTTOMHALF_FILLED,
    OVAL_PORTRAIT_FILL,
    OVAL_PORTRAIT_INSET_FILLED,
    OVAL_PORTRAIT_LEFTHALF_FILLED,
    OVAL_PORTRAIT_RIGHTHALF_FILLED,
    OVAL_PORTRAIT_TOPHALF_FILLED,
    OVAL_RIGHTHALF_FILLED,
    OVAL_TOPHALF_FILLED,
    OVEN,
    OVEN_FILL,
    P_CIRCLE,
    P_CIRCLE_FILL,
    P_SQUARE,
    P_SQUARE_FILL,
    PAINTBRUSH,
    PAINTBRUSH_FILL,
    PAINTBRUSH_POINTED,
    PAINTBRUSH_POINTED_FILL,
    PAINTPALETTE,
    PAINTPALETTE_FILL,
    PANO,
    PANO_FILL,
    PAPERCLIP,
    PAPERCLIP_BADGE_ELLIPSIS,
    PAPERCLIP_CIRCLE,
    PAPERCLIP_CIRCLE_FILL,
    PAPERPLANE,
    PAPERPLANE_CIRCLE,
    PAPERPLANE_CIRCLE_FILL,
    PAPERPLANE_FILL,
    PARAGRAPHSIGN,
    PARENTHESES,
    PARKINGSIGN,
    PARKINGSIGN_CIRCLE,
    PARKINGSIGN_CIRCLE_FILL,
    PARTY_POPPER,
    PARTY_POPPER_FILL,
    PAUSE,
    PAUSE_CIRCLE,
    PAUSE_CIRCLE_FILL,
    PAUSE_FILL,
    PAUSE_RECTANGLE,
    PAUSE_RECTANGLE_FILL,
    PAWPRINT,
    PAWPRINT_CIRCLE,
    PAWPRINT_CIRCLE_FILL,
    PAWPRINT_FILL,
    PC,
    PEACESIGN,
    PEDESTRIAN_GATE_CLOSED,
    PEDESTRIAN_GATE_OPEN,
    PENCIL,
    PENCIL_AND_OUTLINE,
    PENCIL_AND_RULER,
    PENCIL_AND_RULER_FILL,
    PENCIL_CIRCLE,
    PENCIL_CIRCLE_FILL,
    PENCIL_LINE,
    PENCIL_SLASH,
    PENCIL_TIP,
    PENCIL_TIP_CROP_CIRCLE,
    PENCIL_TIP_CROP_CIRCLE_BADGE_ARROW_FORWARD,
    PENCIL_TIP_CROP_CIRCLE_BADGE_MINUS,
    PENCIL_TIP_CROP_CIRCLE_BADGE_PLUS,
    PENTAGON,
    PENTAGON_BOTTOMHALF_FILLED,
    PENTAGON_FILL,
    PENTAGON_LEFTHALF_FILLED,
    PENTAGON_RIGHTHALF_FILLED,
    PENTAGON_TOPHALF_FILLED,
    PERCENT,
    PERSON,
    PERSON_2,
    PERSON_2_BADGE_GEARSHAPE,
    PERSON_2_BADGE_GEARSHAPE_FILL,
    PERSON_2_CIRCLE,
    PERSON_2_CIRCLE_FILL,
    PERSON_2_CROP_SQUARE_STACK,
    PERSON_2_CROP_SQUARE_STACK_FILL,
    PERSON_2_FILL,
    PERSON_2_GOBACKWARD,
    PERSON_2_WAVE_2,
    PERSON_2_WAVE_2_FILL,
    PERSON_3,
    PERSON_3_FILL,
    PERSON_3_SEQUENCE,
    PERSON_3_SEQUENCE_FILL,
    PERSON_AND_ARROW_LEFT_AND_ARROW_RIGHT,
    PERSON_BADGE_CLOCK,
    PERSON_BADGE_CLOCK_FILL,
    PERSON_BADGE_KEY,
    PERSON_BADGE_KEY_FILL,
    PERSON_BADGE_MINUS,
    PERSON_BADGE_PLUS,
    PERSON_BADGE_SHIELD_CHECKMARK,
    PERSON_BADGE_SHIELD_CHECKMARK_FILL,
    PERSON_BUST,
    PERSON_BUST_FILL,
    PERSON_CIRCLE,
    PERSON_CIRCLE_FILL,
    PERSON_CROP_ARTFRAME,
    PERSON_CROP_BACKGROUND_DOTTED,
    PERSON_CROP_CIRCLE,
    PERSON_CROP_CIRCLE_BADGE,
    PERSON_CROP_CIRCLE_BADGE_CHECKMARK,
    PERSON_CROP_CIRCLE_BADGE_CLOCK,
    PERSON_CROP_CIRCLE_BADGE_CLOCK_FILL,
    PERSON_CROP_CIRCLE_BADGE_EXCLAMATIONMARK,
    PERSON_CROP_CIRCLE_BADGE_EXCLAMATIONMARK_FILL,
    PERSON_CROP_CIRCLE_BADGE_FILL,
    PERSON_CROP_CIRCLE_BADGE_MINUS,
    PERSON_CROP_CIRCLE_BADGE_MOON,
    PERSON_CROP_CIRCLE_BADGE_MOON_FILL,
    PERSON_CROP_CIRCLE_BADGE_PLUS,
    PERSON_CROP_CIRCLE_BADGE_QUESTIONMARK,
    PERSON_CROP_CIRCLE_BADGE_QUESTIONMARK_FILL,
    PERSON_CROP_CIRCLE_BADGE_XMARK,
    PERSON_CROP_CIRCLE_FILL,
    PERSON_CROP_CIRCLE_FILL_BADGE_CHECKMARK,
    PERSON_CROP_CIRCLE_FILL_BADGE_MINUS,
    PERSON_CROP_CIRCLE_FILL_BADGE_PLUS,
    PERSON_CROP_CIRCLE_FILL_BADGE_XMARK,
    PERSON_CROP_RECTANGLE,
    PERSON_CROP_RECTANGLE_BADGE_PLUS,
    PERSON_CROP_RECTANGLE_BADGE_PLUS_FILL,
    PERSON_CROP_RECTANGLE_FILL,
    PERSON_CROP_RECTANGLE_STACK,
    PERSON_CROP_RECTANGLE_STACK_FILL,
    PERSON_CROP_SQUARE,
    PERSON_CROP_SQUARE_FILL,
    PERSON_CROP_SQUARE_FILLED_AND_AT_RECTANGLE,
    PERSON_CROP_SQUARE_FILLED_AND_AT_RECTANGLE_FILL,
    PERSON_FILL,
    PERSON_FILL_AND_ARROW_LEFT_AND_ARROW_RIGHT,
    PERSON_FILL_BADGE_MINUS,
    PERSON_FILL_BADGE_PLUS,
    PERSON_FILL_CHECKMARK,
    PERSON_FILL_QUESTIONMARK,
    PERSON_FILL_TURN_DOWN,
    PERSON_FILL_TURN_LEFT,
    PERSON_FILL_TURN_RIGHT,
    PERSON_FILL_VIEWFINDER,
    PERSON_FILL_XMARK,
    PERSON_ICLOUD,
    PERSON_ICLOUD_FILL,
    PERSON_LINE_DOTTED_PERSON,
    PERSON_LINE_DOTTED_PERSON_FILL,
    PERSON_TEXT_RECTANGLE,
    PERSON_TEXT_RECTANGLE_FILL,
    PERSON_WAVE_2,
    PERSON_WAVE_2_FILL,
    PERSONALHOTSPOT,
    PERSONALHOTSPOT_CIRCLE,
    PERSONALHOTSPOT_CIRCLE_FILL,
    PERSPECTIVE,
    PESETASIGN,
    PESETASIGN_CIRCLE,
    PESETASIGN_CIRCLE_FILL,
    PESETASIGN_SQUARE,
    PESETASIGN_SQUARE_FILL,
    PESOSIGN,
    PESOSIGN_CIRCLE,
    PESOSIGN_CIRCLE_FILL,
    PESOSIGN_SQUARE,
    PESOSIGN_SQUARE_FILL,
    PHONE,
    PHONE_AND_WAVEFORM,
    PHONE_AND_WAVEFORM_FILL,
    PHONE_ARROW_DOWN_LEFT,
    PHONE_ARROW_DOWN_LEFT_FILL,
    PHONE_ARROW_RIGHT,
    PHONE_ARROW_RIGHT_FILL,
    PHONE_ARROW_UP_RIGHT,
    PHONE_ARROW_UP_RIGHT_CIRCLE,
    PHONE_ARROW_UP_RIGHT_CIRCLE_FILL,
    PHONE_ARROW_UP_RIGHT_FILL,
    PHONE_BADGE_CHECKMARK,
    PHONE_BADGE_PLUS,
    PHONE_BUBBLE_LEFT,
    PHONE_BUBBLE_LEFT_FILL,
    PHONE_CIRCLE,
    PHONE_CIRCLE_FILL,
    PHONE_CONNECTION,
    PHONE_CONNECTION_FILL,
    PHONE_DOWN,
    PHONE_DOWN_CIRCLE,
    PHONE_DOWN_CIRCLE_FILL,
    PHONE_DOWN_FILL,
    PHONE_DOWN_WAVES_LEFT_AND_RIGHT,
    PHONE_FILL,
    PHONE_FILL_ARROW_DOWN_LEFT,
    PHONE_FILL_ARROW_RIGHT,
    PHONE_FILL_ARROW_UP_RIGHT,
    PHONE_FILL_BADGE_CHECKMARK,
    PHONE_FILL_BADGE_PLUS,
    PHONE_FILL_CONNECTION,
    PHOTO,
    PHOTO_ARTFRAME,
    PHOTO_CIRCLE,
    PHOTO_CIRCLE_FILL,
    PHOTO_FILL,
    PHOTO_FILL_ON_RECTANGLE_FILL,
    PHOTO_ON_RECTANGLE,
    PHOTO_ON_RECTANGLE_ANGLED,
    PHOTO_STACK,
    PHOTO_STACK_FILL,
    PHOTO_TV,
    PIANOKEYS,
    PIANOKEYS_INVERSE,
    PILL,
    PILL_CIRCLE,
    PILL_CIRCLE_FILL,
    PILL_FILL,
    PILLS,
    PILLS_CIRCLE,
    PILLS_CIRCLE_FILL,
    PILLS_FILL,
    PIN,
    PIN_CIRCLE,
    PIN_CIRCLE_FILL,
    PIN_FILL,
    PIN_SLASH,
    PIN_SLASH_FILL,
    PIN_SQUARE,
    PIN_SQUARE_FILL,
    PIP,
    PIP_ENTER,
    PIP_EXIT,
    PIP_FILL,
    PIP_REMOVE,
    PIP_SWAP,
    PIPE_AND_DROP,
    PIPE_AND_DROP_FILL,
    PLACEHOLDERTEXT_FILL,
    PLATTER_2_FILLED_IPAD,
    PLATTER_2_FILLED_IPAD_LANDSCAPE,
    PLATTER_2_FILLED_IPHONE,
    PLATTER_2_FILLED_IPHONE_LANDSCAPE,
    PLATTER_BOTTOM_APPLEWATCH_CASE,
    PLATTER_FILLED_BOTTOM_AND_ARROW_DOWN_IPHONE,
    PLATTER_FILLED_BOTTOM_APPLEWATCH_CASE,
    PLATTER_FILLED_BOTTOM_IPHONE,
    PLATTER_FILLED_TOP_AND_ARROW_UP_IPHONE,
    PLATTER_FILLED_TOP_APPLEWATCH_CASE,
    PLATTER_FILLED_TOP_IPHONE,
    PLATTER_TOP_APPLEWATCH_CASE,
    PLAY,
    PLAY_CIRCLE,
    PLAY_CIRCLE_FILL,
    PLAY_DESKTOPCOMPUTER,
    PLAY_DISPLAY,
    PLAY_FILL,
    PLAY_LAPTOPCOMPUTER,
    PLAY_RECTANGLE,
    PLAY_RECTANGLE_FILL,
    PLAY_RECTANGLE_ON_RECTANGLE,
    PLAY_RECTANGLE_ON_RECTANGLE_CIRCLE,
    PLAY_RECTANGLE_ON_RECTANGLE_CIRCLE_FILL,
    PLAY_RECTANGLE_ON_RECTANGLE_FILL,
    PLAY_SLASH,
    PLAY_SLASH_FILL,
    PLAY_SQUARE,
    PLAY_SQUARE_FILL,
    PLAY_TV,
    PLAY_TV_FILL,
    PLAYPAUSE,
    PLAYPAUSE_CIRCLE,
    PLAYPAUSE_CIRCLE_FILL,
    PLAYPAUSE_FILL,
    PLAYSTATION_LOGO,
    PLUS,
    PLUS_APP,
    PLUS_APP_FILL,
    PLUS_BUBBLE,
    PLUS_BUBBLE_FILL,
    PLUS_CIRCLE,
    PLUS_CIRCLE_FILL,
    PLUS_DIAMOND,
    PLUS_DIAMOND_FILL,
    PLUS_FORWARDSLASH_MINUS,
    PLUS_MAGNIFYINGGLASS,
    PLUS_MESSAGE,
    PLUS_MESSAGE_FILL,
    PLUS_RECTANGLE,
    PLUS_RECTANGLE_FILL,
    PLUS_RECTANGLE_FILL_ON_RECTANGLE_FILL,
    PLUS_RECTANGLE_ON_FOLDER,
    PLUS_RECTANGLE_ON_FOLDER_FILL,
    PLUS_RECTANGLE_ON_RECTANGLE,
    PLUS_RECTANGLE_PORTRAIT,
    PLUS_RECTANGLE_PORTRAIT_FILL,
    PLUS_SQUARE,
    PLUS_SQUARE_DASHED,
    PLUS_SQUARE_FILL,
    PLUS_SQUARE_FILL_ON_SQUARE_FILL,
    PLUS_SQUARE_ON_SQUARE,
    PLUS_VIEWFINDER,
    PLUSMINUS,
    PLUSMINUS_CIRCLE,
    PLUSMINUS_CIRCLE_FILL,
    POINT_3_CONNECTED_TRIANGLEPATH_DOTTED,
    POINT_3_FILLED_CONNECTED_TRIANGLEPATH_DOTTED,
    POINT_FILLED_TOPLEFT_DOWN_CURVEDTO_POINT_BOTTOMRIGHT_UP,
    POINT_TOPLEFT_DOWN_CURVEDTO_POINT_BOTTOMRIGHT_UP,
    POINT_TOPLEFT_DOWN_CURVEDTO_POINT_BOTTOMRIGHT_UP_FILL,
    POINT_TOPLEFT_DOWN_CURVEDTO_POINT_FILLED_BOTTOMRIGHT_UP,
    POPCORN,
    POPCORN_CIRCLE,
    POPCORN_CIRCLE_FILL,
    POPCORN_FILL,
    POWER,
    POWER_CIRCLE,
    POWER_CIRCLE_FILL,
    POWER_DOTTED,
    POWEROFF,
    POWERON,
    POWEROUTLET_STRIP,
    POWEROUTLET_STRIP_FILL,
    POWEROUTLET_TYPE_A,
    POWEROUTLET_TYPE_A_FILL,
    POWEROUTLET_TYPE_A_SQUARE,
    POWEROUTLET_TYPE_A_SQUARE_FILL,
    POWEROUTLET_TYPE_B,
    POWEROUTLET_TYPE_B_FILL,
    POWEROUTLET_TYPE_B_SQUARE,
    POWEROUTLET_TYPE_B_SQUARE_FILL,
    POWEROUTLET_TYPE_C,
    POWEROUTLET_TYPE_C_FILL,
    POWEROUTLET_TYPE_C_SQUARE,
    POWEROUTLET_TYPE_C_SQUARE_FILL,
    POWEROUTLET_TYPE_D,
    POWEROUTLET_TYPE_D_FILL,
    POWEROUTLET_TYPE_D_SQUARE,
    POWEROUTLET_TYPE_D_SQUARE_FILL,
    POWEROUTLET_TYPE_E,
    POWEROUTLET_TYPE_E_FILL,
    POWEROUTLET_TYPE_E_SQUARE,
    POWEROUTLET_TYPE_E_SQUARE_FILL,
    POWEROUTLET_TYPE_F,
    POWEROUTLET_TYPE_F_FILL,
    POWEROUTLET_TYPE_F_SQUARE,
    POWEROUTLET_TYPE_F_SQUARE_FILL,
    POWEROUTLET_TYPE_G,
    POWEROUTLET_TYPE_G_FILL,
    POWEROUTLET_TYPE_G_SQUARE,
    POWEROUTLET_TYPE_G_SQUARE_FILL,
    POWEROUTLET_TYPE_H,
    POWEROUTLET_TYPE_H_FILL,
    POWEROUTLET_TYPE_H_SQUARE,
    POWEROUTLET_TYPE_H_SQUARE_FILL,
    POWEROUTLET_TYPE_I,
    POWEROUTLET_TYPE_I_FILL,
    POWEROUTLET_TYPE_I_SQUARE,
    POWEROUTLET_TYPE_I_SQUARE_FILL,
    POWEROUTLET_TYPE_J,
    POWEROUTLET_TYPE_J_FILL,
    POWEROUTLET_TYPE_J_SQUARE,
    POWEROUTLET_TYPE_J_SQUARE_FILL,
    POWEROUTLET_TYPE_K,
    POWEROUTLET_TYPE_K_FILL,
    POWEROUTLET_TYPE_K_SQUARE,
    POWEROUTLET_TYPE_K_SQUARE_FILL,
    POWEROUTLET_TYPE_L,
    POWEROUTLET_TYPE_L_FILL,
    POWEROUTLET_TYPE_L_SQUARE,
    POWEROUTLET_TYPE_L_SQUARE_FILL,
    POWEROUTLET_TYPE_M,
    POWEROUTLET_TYPE_M_FILL,
    POWEROUTLET_TYPE_M_SQUARE,
    POWEROUTLET_TYPE_M_SQUARE_FILL,
    POWEROUTLET_TYPE_N,
    POWEROUTLET_TYPE_N_FILL,
    POWEROUTLET_TYPE_N_SQUARE,
    POWEROUTLET_TYPE_N_SQUARE_FILL,
    POWEROUTLET_TYPE_O,
    POWEROUTLET_TYPE_O_FILL,
    POWEROUTLET_TYPE_O_SQUARE,
    POWEROUTLET_TYPE_O_SQUARE_FILL,
    POWERPLUG,
    POWERPLUG_FILL,
    POWERSLEEP,
    PRINTER,
    PRINTER_DOTMATRIX,
    PRINTER_DOTMATRIX_FILL,
    PRINTER_DOTMATRIX_FILLED_AND_PAPER,
    PRINTER_FILL,
    PRINTER_FILLED_AND_PAPER,
    PROJECTIVE,
    PURCHASED,
    PURCHASED_CIRCLE,
    PURCHASED_CIRCLE_FILL,
    PUZZLEPIECE,
    PUZZLEPIECE_EXTENSION,
    PUZZLEPIECE_EXTENSION_FILL,
    PUZZLEPIECE_FILL,
    PYRAMID,
    PYRAMID_FILL,
    Q_CIRCLE,
    Q_CIRCLE_FILL,
    Q_SQUARE,
    Q_SQUARE_FILL,
    QRCODE,
    QRCODE_VIEWFINDER,
    QUESTIONMARK,
    QUESTIONMARK_APP,
    QUESTIONMARK_APP_DASHED,
    QUESTIONMARK_APP_FILL,
    QUESTIONMARK_BUBBLE,
    QUESTIONMARK_BUBBLE_FILL,
    QUESTIONMARK_CIRCLE,
    QUESTIONMARK_CIRCLE_FILL,
    QUESTIONMARK_DIAMOND,
    QUESTIONMARK_DIAMOND_FILL,
    QUESTIONMARK_FOLDER,
    QUESTIONMARK_FOLDER_FILL,
    QUESTIONMARK_SQUARE,
    QUESTIONMARK_SQUARE_DASHED,
    QUESTIONMARK_SQUARE_FILL,
    QUESTIONMARK_VIDEO,
    QUESTIONMARK_VIDEO_FILL,
    QUOTE_BUBBLE,
    QUOTE_BUBBLE_FILL,
    QUOTE_CLOSING,
    QUOTE_OPENING,
    QUOTELEVEL,
    R_CIRCLE,
    R_CIRCLE_FILL,
    R_JOYSTICK,
    R_JOYSTICK_FILL,
    R_JOYSTICK_PRESS_DOWN,
    R_JOYSTICK_PRESS_DOWN_FILL,
    R_JOYSTICK_TILT_DOWN,
    R_JOYSTICK_TILT_DOWN_FILL,
    R_JOYSTICK_TILT_LEFT,
    R_JOYSTICK_TILT_LEFT_FILL,
    R_JOYSTICK_TILT_RIGHT,
    R_JOYSTICK_TILT_RIGHT_FILL,
    R_JOYSTICK_TILT_UP,
    R_JOYSTICK_TILT_UP_FILL,
    R_RECTANGLE_ROUNDEDBOTTOM,
    R_RECTANGLE_ROUNDEDBOTTOM_FILL,
    R_SQUARE,
    R_SQUARE_FILL,
    R_SQUARE_ON_SQUARE,
    R_SQUARE_ON_SQUARE_FILL,
    R1_RECTANGLE_ROUNDEDBOTTOM,
    R1_RECTANGLE_ROUNDEDBOTTOM_FILL,
    R2_RECTANGLE_ROUNDEDTOP,
    R2_RECTANGLE_ROUNDEDTOP_FILL,
    RADIO,
    RADIO_FILL,
    RAYS,
    RB_RECTANGLE_ROUNDEDBOTTOM,
    RB_RECTANGLE_ROUNDEDBOTTOM_FILL,
    RECORD_CIRCLE,
    RECORD_CIRCLE_FILL,
    RECORDINGTAPE,
    RECORDINGTAPE_CIRCLE,
    RECORDINGTAPE_CIRCLE_FILL,
    RECTANGLE,
    RECTANGLE_2_SWAP,
    RECTANGLE_3_GROUP,
    RECTANGLE_3_GROUP_BUBBLE_LEFT,
    RECTANGLE_3_GROUP_BUBBLE_LEFT_FILL,
    RECTANGLE_3_GROUP_FILL,
    RECTANGLE_AND_ARROW_UP_RIGHT_AND_ARROW_DOWN_LEFT,
    RECTANGLE_AND_ARROW_UP_RIGHT_AND_ARROW_DOWN_LEFT_SLASH,
    RECTANGLE_AND_HAND_POINT_UP_LEFT,
    RECTANGLE_AND_HAND_POINT_UP_LEFT_FILL,
    RECTANGLE_AND_HAND_POINT_UP_LEFT_FILLED,
    RECTANGLE_AND_PAPERCLIP,
    RECTANGLE_AND_PENCIL_AND_ELLIPSIS,
    RECTANGLE_AND_TEXT_MAGNIFYINGGLASS,
    RECTANGLE_ARROWTRIANGLE_2_INWARD,
    RECTANGLE_ARROWTRIANGLE_2_OUTWARD,
    RECTANGLE_BADGE_CHECKMARK,
    RECTANGLE_BADGE_MINUS,
    RECTANGLE_BADGE_PERSON_CROP,
    RECTANGLE_BADGE_PLUS,
    RECTANGLE_BADGE_XMARK,
    RECTANGLE_BOTTOMHALF_FILLED,
    RECTANGLE_BOTTOMHALF_INSET_FILLED,
    RECTANGLE_BOTTOMTHIRD_INSET_FILLED,
    RECTANGLE_CENTER_INSET_FILLED,
    RECTANGLE_CENTER_INSET_FILLED_BADGE_PLUS,
    RECTANGLE_COMPRESS_VERTICAL,
    RECTANGLE_CONNECTED_TO_LINE_BELOW,
    RECTANGLE_DASHED,
    RECTANGLE_DASHED_AND_PAPERCLIP,
    RECTANGLE_DASHED_BADGE_RECORD,
    RECTANGLE_EXPAND_VERTICAL,
    RECTANGLE_FILL,
    RECTANGLE_FILL_BADGE_CHECKMARK,
    RECTANGLE_FILL_BADGE_MINUS,
    RECTANGLE_FILL_BADGE_PERSON_CROP,
    RECTANGLE_FILL_BADGE_PLUS,
    RECTANGLE_FILL_BADGE_XMARK,
    RECTANGLE_FILL_ON_RECTANGLE_ANGLED_FILL,
    RECTANGLE_FILL_ON_RECTANGLE_FILL,
    RECTANGLE_FILLED_AND_HAND_POINT_UP_LEFT,
    RECTANGLE_GRID_1X2,
    RECTANGLE_GRID_1X2_FILL,
    RECTANGLE_GRID_2X2,
    RECTANGLE_GRID_2X2_FILL,
    RECTANGLE_GRID_3X2,
    RECTANGLE_GRID_3X2_FILL,
    RECTANGLE_INSET_BOTTOMLEADING_FILLED,
    RECTANGLE_INSET_BOTTOMLEFT_FILLED,
    RECTANGLE_INSET_BOTTOMRIGHT_FILLED,
    RECTANGLE_INSET_BOTTOMTRAILING_FILLED,
    RECTANGLE_INSET_FILLED,
    RECTANGLE_INSET_FILLED_AND_PERSON_FILLED,
    RECTANGLE_INSET_FILLED_ON_RECTANGLE,
    RECTANGLE_INSET_TOPLEADING_FILLED,
    RECTANGLE_INSET_TOPLEFT_FILLED,
    RECTANGLE_INSET_TOPRIGHT_FILLED,
    RECTANGLE_INSET_TOPTRAILING_FILLED,
    RECTANGLE_LEADINGHALF_FILLED,
    RECTANGLE_LEADINGHALF_INSET_FILLED,
    RECTANGLE_LEADINGHALF_INSET_FILLED_ARROW_LEADING,
    RECTANGLE_LEADINGTHIRD_INSET_FILLED,
    RECTANGLE_LEFTHALF_FILLED,
    RECTANGLE_LEFTHALF_INSET_FILLED,
    RECTANGLE_LEFTHALF_INSET_FILLED_ARROW_LEFT,
    RECTANGLE_LEFTTHIRD_INSET_FILLED,
    RECTANGLE_ON_RECTANGLE,
    RECTANGLE_ON_RECTANGLE_ANGLED,
    RECTANGLE_ON_RECTANGLE_CIRCLE,
    RECTANGLE_ON_RECTANGLE_CIRCLE_FILL,
    RECTANGLE_ON_RECTANGLE_SLASH,
    RECTANGLE_ON_RECTANGLE_SLASH_CIRCLE,
    RECTANGLE_ON_RECTANGLE_SLASH_CIRCLE_FILL,
    RECTANGLE_ON_RECTANGLE_SLASH_FILL,
    RECTANGLE_ON_RECTANGLE_SQUARE,
    RECTANGLE_ON_RECTANGLE_SQUARE_FILL,
    RECTANGLE_PORTRAIT,
    RECTANGLE_PORTRAIT_AND_ARROW_FORWARD,
    RECTANGLE_PORTRAIT_AND_ARROW_FORWARD_FILL,
    RECTANGLE_PORTRAIT_AND_ARROW_RIGHT,
    RECTANGLE_PORTRAIT_AND_ARROW_RIGHT_FILL,
    RECTANGLE_PORTRAIT_ARROWTRIANGLE_2_INWARD,
    RECTANGLE_PORTRAIT_ARROWTRIANGLE_2_OUTWARD,
    RECTANGLE_PORTRAIT_BOTTOMHALF_FILLED,
    RECTANGLE_PORTRAIT_BOTTOMHALF_INSET_FILLED,
    RECTANGLE_PORTRAIT_BOTTOMLEADING_INSET_FILLED,
    RECTANGLE_PORTRAIT_BOTTOMLEFT_INSET_FILLED,
    RECTANGLE_PORTRAIT_BOTTOMRIGHT_INSET_FILLED,
    RECTANGLE_PORTRAIT_BOTTOMTHIRD_INSET_FILLED,
    RECTANGLE_PORTRAIT_BOTTOMTRAILING_INSET_FILLED,
    RECTANGLE_PORTRAIT_CENTER_INSET_FILLED,
    RECTANGLE_PORTRAIT_FILL,
    RECTANGLE_PORTRAIT_INSET_FILLED,
    RECTANGLE_PORTRAIT_LEADINGHALF_INSET_FILLED,
    RECTANGLE_PORTRAIT_LEADINGTHIRD_INSET_FILLED,
    RECTANGLE_PORTRAIT_LEFTHALF_FILLED,
    RECTANGLE_PORTRAIT_LEFTHALF_INSET_FILLED,
    RECTANGLE_PORTRAIT_LEFTTHIRD_INSET_FILLED,
    RECTANGLE_PORTRAIT_ON_RECTANGLE_PORTRAIT,
    RECTANGLE_PORTRAIT_ON_RECTANGLE_PORTRAIT_ANGLED,
    RECTANGLE_PORTRAIT_ON_RECTANGLE_PORTRAIT_ANGLED_FILL,
    RECTANGLE_PORTRAIT_ON_RECTANGLE_PORTRAIT_FILL,
    RECTANGLE_PORTRAIT_ON_RECTANGLE_PORTRAIT_SLASH,
    RECTANGLE_PORTRAIT_ON_RECTANGLE_PORTRAIT_SLASH_FILL,
    RECTANGLE_PORTRAIT_RIGHTHALF_FILLED,
    RECTANGLE_PORTRAIT_RIGHTHALF_INSET_FILLED,
    RECTANGLE_PORTRAIT_RIGHTTHIRD_INSET_FILLED,
    RECTANGLE_PORTRAIT_SLASH,
    RECTANGLE_PORTRAIT_SLASH_FILL,
    RECTANGLE_PORTRAIT_SPLIT_2X1,
    RECTANGLE_PORTRAIT_SPLIT_2X1_FILL,
    RECTANGLE_PORTRAIT_SPLIT_2X1_SLASH,
    RECTANGLE_PORTRAIT_SPLIT_2X1_SLASH_FILL,
    RECTANGLE_PORTRAIT_TOPHALF_FILLED,
    RECTANGLE_PORTRAIT_TOPHALF_INSET_FILLED,
    RECTANGLE_PORTRAIT_TOPLEADING_INSET_FILLED,
    RECTANGLE_PORTRAIT_TOPLEFT_INSET_FILLED,
    RECTANGLE_PORTRAIT_TOPRIGHT_INSET_FILLED,
    RECTANGLE_PORTRAIT_TOPTHIRD_INSET_FILLED,
    RECTANGLE_PORTRAIT_TOPTRAILING_INSET_FILLED,
    RECTANGLE_PORTRAIT_TRAILINGHALF_INSET_FILLED,
    RECTANGLE_PORTRAIT_TRAILINGTHIRD_INSET_FILLED,
    RECTANGLE_RIGHTHALF_FILLED,
    RECTANGLE_RIGHTHALF_INSET_FILLED,
    RECTANGLE_RIGHTHALF_INSET_FILLED_ARROW_RIGHT,
    RECTANGLE_RIGHTTHIRD_INSET_FILLED,
    RECTANGLE_ROUNDEDBOTTOM,
    RECTANGLE_ROUNDEDBOTTOM_FILL,
    RECTANGLE_ROUNDEDTOP,
    RECTANGLE_ROUNDEDTOP_FILL,
    RECTANGLE_SLASH,
    RECTANGLE_SLASH_FILL,
    RECTANGLE_SPLIT_1X2,
    RECTANGLE_SPLIT_1X2_FILL,
    RECTANGLE_SPLIT_2X1,
    RECTANGLE_SPLIT_2X1_FILL,
    RECTANGLE_SPLIT_2X1_SLASH,
    RECTANGLE_SPLIT_2X1_SLASH_FILL,
    RECTANGLE_SPLIT_2X2,
    RECTANGLE_SPLIT_2X2_FILL,
    RECTANGLE_SPLIT_3X1,
    RECTANGLE_SPLIT_3X1_FILL,
    RECTANGLE_SPLIT_3X3,
    RECTANGLE_SPLIT_3X3_FILL,
    RECTANGLE_STACK,
    RECTANGLE_STACK_BADGE_MINUS,
    RECTANGLE_STACK_BADGE_PERSON_CROP,
    RECTANGLE_STACK_BADGE_PERSON_CROP_FILL,
    RECTANGLE_STACK_BADGE_PLAY,
    RECTANGLE_STACK_BADGE_PLAY_FILL,
    RECTANGLE_STACK_BADGE_PLUS,
    RECTANGLE_STACK_FILL,
    RECTANGLE_STACK_FILL_BADGE_MINUS,
    RECTANGLE_STACK_FILL_BADGE_PLUS,
    RECTANGLE_TOPHALF_FILLED,
    RECTANGLE_TOPHALF_INSET_FILLED,
    RECTANGLE_TOPTHIRD_INSET_FILLED,
    RECTANGLE_TRAILINGHALF_FILLED,
    RECTANGLE_TRAILINGHALF_INSET_FILLED,
    RECTANGLE_TRAILINGHALF_INSET_FILLED_ARROW_TRAILING,
    RECTANGLE_TRAILINGTHIRD_INSET_FILLED,
    REFRIGERATOR,
    REFRIGERATOR_FILL,
    REPEAT,
    REPEAT_1,
    REPEAT_1_CIRCLE,
    REPEAT_1_CIRCLE_FILL,
    REPEAT_CIRCLE,
    REPEAT_CIRCLE_FILL,
    RESTART,
    RESTART_CIRCLE,
    RESTART_CIRCLE_FILL,
    RETURN,
    RETURN_LEFT,
    RETURN_RIGHT,
    RHOMBUS,
    RHOMBUS_FILL,
    ROLLER_SHADE_CLOSED,
    ROLLER_SHADE_OPEN,
    ROMAN_SHADE_CLOSED,
    ROMAN_SHADE_OPEN,
    ROSETTE,
    ROTATE_3D,
    ROTATE_LEFT,
    ROTATE_LEFT_FILL,
    ROTATE_RIGHT,
    ROTATE_RIGHT_FILL,
    RT_RECTANGLE_ROUNDEDTOP,
    RT_RECTANGLE_ROUNDEDTOP_FILL,
    RUBLESIGN,
    RUBLESIGN_CIRCLE,
    RUBLESIGN_CIRCLE_FILL,
    RUBLESIGN_SQUARE,
    RUBLESIGN_SQUARE_FILL,
    RULER,
    RULER_FILL,
    RUPEESIGN,
    RUPEESIGN_CIRCLE,
    RUPEESIGN_CIRCLE_FILL,
    RUPEESIGN_SQUARE,
    RUPEESIGN_SQUARE_FILL,
    S_CIRCLE,
    S_CIRCLE_FILL,
    S_SQUARE,
    S_SQUARE_FILL,
    SAFARI,
    SAFARI_FILL,
    SAILBOAT,
    SAILBOAT_FILL,
    SCALE_3D,
    SCALEMASS,
    SCALEMASS_FILL,
    SCANNER,
    SCANNER_FILL,
    SCISSORS,
    SCISSORS_BADGE_ELLIPSIS,
    SCISSORS_CIRCLE,
    SCISSORS_CIRCLE_FILL,
    SCOOTER,
    SCOPE,
    SCREWDRIVER,
    SCREWDRIVER_FILL,
    SCRIBBLE,
    SCRIBBLE_VARIABLE,
    SCROLL,
    SCROLL_FILL,
    SDCARD,
    SDCARD_FILL,
    SEAL,
    SEAL_FILL,
    SELECTION_PIN_IN_OUT,
    SENSOR,
    SENSOR_FILL,
    SENSOR_TAG_RADIOWAVES_FORWARD,
    SENSOR_TAG_RADIOWAVES_FORWARD_FILL,
    SERVER_RACK,
    SHADOW,
    SHARED_WITH_YOU,
    SHARED_WITH_YOU_SLASH,
    SHAREPLAY,
    SHAREPLAY_SLASH,
    SHAZAM_LOGO,
    SHAZAM_LOGO_FILL,
    SHEKELSIGN,
    SHEKELSIGN_CIRCLE,
    SHEKELSIGN_CIRCLE_FILL,
    SHEKELSIGN_SQUARE,
    SHEKELSIGN_SQUARE_FILL,
    SHIELD,
    SHIELD_FILL,
    SHIELD_LEFTHALF_FILLED,
    SHIELD_LEFTHALF_FILLED_SLASH,
    SHIELD_RIGHTHALF_FILLED,
    SHIELD_SLASH,
    SHIELD_SLASH_FILL,
    SHIFT,
    SHIFT_FILL,
    SHIPPINGBOX,
    SHIPPINGBOX_AND_ARROW_BACKWARD,
    SHIPPINGBOX_AND_ARROW_BACKWARD_FILL,
    SHIPPINGBOX_CIRCLE,
    SHIPPINGBOX_CIRCLE_FILL,
    SHIPPINGBOX_FILL,
    SHOEPRINTS_FILL,
    SHOWER,
    SHOWER_FILL,
    SHOWER_HANDHELD,
    SHOWER_HANDHELD_FILL,
    SHOWER_SIDEJET,
    SHOWER_SIDEJET_FILL,
    SHUFFLE,
    SHUFFLE_CIRCLE,
    SHUFFLE_CIRCLE_FILL,
    SIDEBAR_LEADING,
    SIDEBAR_LEFT,
    SIDEBAR_RIGHT,
    SIDEBAR_SQUARES_LEADING,
    SIDEBAR_SQUARES_LEFT,
    SIDEBAR_SQUARES_RIGHT,
    SIDEBAR_SQUARES_TRAILING,
    SIDEBAR_TRAILING,
    SIGNATURE,
    SIGNPOST_LEFT,
    SIGNPOST_LEFT_FILL,
    SIGNPOST_RIGHT,
    SIGNPOST_RIGHT_FILL,
    SIMCARD,
    SIMCARD_2,
    SIMCARD_2_FILL,
    SIMCARD_FILL,
    SINK,
    SINK_FILL,
    SKEW,
    SLASH_CIRCLE,
    SLASH_CIRCLE_FILL,
    SLEEP,
    SLEEP_CIRCLE,
    SLEEP_CIRCLE_FILL,
    SLIDER_HORIZONTAL_2_GOBACKWARD,
    SLIDER_HORIZONTAL_2_RECTANGLE_AND_ARROW_TRIANGLE_2_CIRCLEPATH,
    SLIDER_HORIZONTAL_2_SQUARE_BADGE_ARROW_DOWN,
    SLIDER_HORIZONTAL_2_SQUARE_ON_SQUARE,
    SLIDER_HORIZONTAL_3,
    SLIDER_HORIZONTAL_BELOW_RECTANGLE,
    SLIDER_HORIZONTAL_BELOW_SQUARE_AND_SQUARE_FILLED,
    SLIDER_HORIZONTAL_BELOW_SQUARE_FILLED_AND_SQUARE,
    SLIDER_VERTICAL_3,
    SLOWMO,
    SMALLCIRCLE_CIRCLE,
    SMALLCIRCLE_CIRCLE_FILL,
    SMALLCIRCLE_FILLED_CIRCLE,
    SMALLCIRCLE_FILLED_CIRCLE_FILL,
    SMOKE,
    SMOKE_CIRCLE,
    SMOKE_CIRCLE_FILL,
    SMOKE_FILL,
    SNOWFLAKE,
    SNOWFLAKE_CIRCLE,
    SNOWFLAKE_CIRCLE_FILL,
    SOCCERBALL,
    SOCCERBALL_CIRCLE,
    SOCCERBALL_CIRCLE_FILL,
    SOCCERBALL_CIRCLE_FILL_INVERSE,
    SOCCERBALL_CIRCLE_INVERSE,
    SOCCERBALL_INVERSE,
    SOFA,
    SOFA_FILL,
    SPACE,
    SPARKLE,
    SPARKLE_MAGNIFYINGGLASS,
    SPARKLES,
    SPARKLES_RECTANGLE_STACK,
    SPARKLES_RECTANGLE_STACK_FILL,
    SPARKLES_SQUARE_FILLED_ON_SQUARE,
    SPARKLES_TV,
    SPARKLES_TV_FILL,
    SPEAKER,
    SPEAKER_BADGE_EXCLAMATIONMARK,
    SPEAKER_BADGE_EXCLAMATIONMARK_FILL,
    SPEAKER_CIRCLE,
    SPEAKER_CIRCLE_FILL,
    SPEAKER_FILL,
    SPEAKER_MINUS,
    SPEAKER_MINUS_FILL,
    SPEAKER_PLUS,
    SPEAKER_PLUS_FILL,
    SPEAKER_SLASH,
    SPEAKER_SLASH_CIRCLE,
    SPEAKER_SLASH_CIRCLE_FILL,
    SPEAKER_SLASH_FILL,
    SPEAKER_SQUARE,
    SPEAKER_SQUARE_FILL,
    SPEAKER_WAVE_1,
    SPEAKER_WAVE_1_FILL,
    SPEAKER_WAVE_2,
    SPEAKER_WAVE_2_BUBBLE_LEFT,
    SPEAKER_WAVE_2_BUBBLE_LEFT_FILL,
    SPEAKER_WAVE_2_CIRCLE,
    SPEAKER_WAVE_2_CIRCLE_FILL,
    SPEAKER_WAVE_2_FILL,
    SPEAKER_WAVE_3,
    SPEAKER_WAVE_3_FILL,
    SPEAKER_ZZZ,
    SPEAKER_ZZZ_FILL,
    SPEEDOMETER,
    SPIGOT,
    SPIGOT_FILL,
    SPORTSCOURT,
    SPORTSCOURT_CIRCLE,
    SPORTSCOURT_CIRCLE_FILL,
    SPORTSCOURT_FILL,
    SPRINKLER,
    SPRINKLER_AND_DROPLETS,
    SPRINKLER_AND_DROPLETS_FILL,
    SPRINKLER_FILL,
    SQUARE,
    SQUARE_2_LAYERS_3D,
    SQUARE_2_LAYERS_3D_BOTTOM_FILLED,
    SQUARE_2_LAYERS_3D_TOP_FILLED,
    SQUARE_2_STACK_3D,
    SQUARE_2_STACK_3D_BOTTOM_FILLED,
    SQUARE_2_STACK_3D_TOP_FILLED,
    SQUARE_3_LAYERS_3D,
    SQUARE_3_LAYERS_3D_BOTTOM_FILLED,
    SQUARE_3_LAYERS_3D_DOWN_BACKWARD,
    SQUARE_3_LAYERS_3D_DOWN_FORWARD,
    SQUARE_3_LAYERS_3D_DOWN_LEFT,
    SQUARE_3_LAYERS_3D_DOWN_LEFT_SLASH,
    SQUARE_3_LAYERS_3D_DOWN_RIGHT,
    SQUARE_3_LAYERS_3D_DOWN_RIGHT_SLASH,
    SQUARE_3_LAYERS_3D_MIDDLE_FILLED,
    SQUARE_3_LAYERS_3D_SLASH,
    SQUARE_3_LAYERS_3D_TOP_FILLED,
    SQUARE_3_STACK_3D,
    SQUARE_3_STACK_3D_BOTTOM_FILLED,
    SQUARE_3_STACK_3D_MIDDLE_FILLED,
    SQUARE_3_STACK_3D_SLASH,
    SQUARE_3_STACK_3D_TOP_FILLED,
    SQUARE_AND_ARROW_DOWN,
    SQUARE_AND_ARROW_DOWN_FILL,
    SQUARE_AND_ARROW_DOWN_ON_SQUARE,
    SQUARE_AND_ARROW_DOWN_ON_SQUARE_FILL,
    SQUARE_AND_ARROW_UP,
    SQUARE_AND_ARROW_UP_CIRCLE,
    SQUARE_AND_ARROW_UP_CIRCLE_FILL,
    SQUARE_AND_ARROW_UP_FILL,
    SQUARE_AND_ARROW_UP_ON_SQUARE,
    SQUARE_AND_ARROW_UP_ON_SQUARE_FILL,
    SQUARE_AND_ARROW_UP_TRIANGLEBADGE_EXCLAMATIONMARK,
    SQUARE_AND_AT_RECTANGLE,
    SQUARE_AND_AT_RECTANGLE_FILL,
    SQUARE_AND_LINE_VERTICAL_AND_SQUARE,
    SQUARE_AND_LINE_VERTICAL_AND_SQUARE_FILLED,
    SQUARE_AND_PENCIL,
    SQUARE_AND_PENCIL_CIRCLE,
    SQUARE_AND_PENCIL_CIRCLE_FILL,
    SQUARE_BOTTOMHALF_FILLED,
    SQUARE_BOTTOMTHIRD_INSET_FILLED,
    SQUARE_CIRCLE,
    SQUARE_CIRCLE_FILL,
    SQUARE_DASHED,
    SQUARE_DASHED_INSET_FILLED,
    SQUARE_DOTTED,
    SQUARE_FILL,
    SQUARE_FILL_AND_LINE_VERTICAL_AND_SQUARE_FILL,
    SQUARE_FILL_ON_CIRCLE_FILL,
    SQUARE_FILL_ON_SQUARE_FILL,
    SQUARE_FILL_TEXT_GRID_1X2,
    SQUARE_FILLED_AND_LINE_VERTICAL_AND_SQUARE,
    SQUARE_FILLED_ON_SQUARE,
    SQUARE_GRID_2X2,
    SQUARE_GRID_2X2_FILL,
    SQUARE_GRID_3X1_BELOW_LINE_GRID_1X2,
    SQUARE_GRID_3X1_BELOW_LINE_GRID_1X2_FILL,
    SQUARE_GRID_3X1_FOLDER_BADGE_PLUS,
    SQUARE_GRID_3X1_FOLDER_FILL_BADGE_PLUS,
    SQUARE_GRID_3X2,
    SQUARE_GRID_3X2_FILL,
    SQUARE_GRID_3X3,
    SQUARE_GRID_3X3_BOTTOMLEFT_FILLED,
    SQUARE_GRID_3X3_BOTTOMMIDDLE_FILLED,
    SQUARE_GRID_3X3_BOTTOMRIGHT_FILLED,
    SQUARE_GRID_3X3_FILL,
    SQUARE_GRID_3X3_MIDDLE_FILLED,
    SQUARE_GRID_3X3_MIDDLELEFT_FILLED,
    SQUARE_GRID_3X3_MIDDLERIGHT_FILLED,
    SQUARE_GRID_3X3_SQUARE,
    SQUARE_GRID_3X3_TOPLEFT_FILLED,
    SQUARE_GRID_3X3_TOPMIDDLE_FILLED,
    SQUARE_GRID_3X3_TOPRIGHT_FILLED,
    SQUARE_GRID_4X3_FILL,
    SQUARE_INSET_FILLED,
    SQUARE_LEADINGTHIRD_INSET_FILLED,
    SQUARE_LEFTHALF_FILLED,
    SQUARE_LEFTTHIRD_INSET_FILLED,
    SQUARE_ON_CIRCLE,
    SQUARE_ON_SQUARE,
    SQUARE_ON_SQUARE_BADGE_PERSON_CROP,
    SQUARE_ON_SQUARE_BADGE_PERSON_CROP_FILL,
    SQUARE_ON_SQUARE_DASHED,
    SQUARE_ON_SQUARE_INTERSECTION_DASHED,
    SQUARE_ON_SQUARE_SQUARESHAPE_CONTROLHANDLES,
    SQUARE_RIGHTHALF_FILLED,
    SQUARE_RIGHTTHIRD_INSET_FILLED,
    SQUARE_SLASH,
    SQUARE_SLASH_FILL,
    SQUARE_SPLIT_1X2,
    SQUARE_SPLIT_1X2_FILL,
    SQUARE_SPLIT_2X1,
    SQUARE_SPLIT_2X1_FILL,
    SQUARE_SPLIT_2X2,
    SQUARE_SPLIT_2X2_FILL,
    SQUARE_SPLIT_BOTTOMRIGHTQUARTER,
    SQUARE_SPLIT_BOTTOMRIGHTQUARTER_FILL,
    SQUARE_SPLIT_DIAGONAL,
    SQUARE_SPLIT_DIAGONAL_2X2,
    SQUARE_SPLIT_DIAGONAL_2X2_FILL,
    SQUARE_SPLIT_DIAGONAL_FILL,
    SQUARE_STACK,
    SQUARE_STACK_3D_DOWN_FORWARD,
    SQUARE_STACK_3D_DOWN_FORWARD_FILL,
    SQUARE_STACK_3D_DOWN_RIGHT,
    SQUARE_STACK_3D_DOWN_RIGHT_FILL,
    SQUARE_STACK_3D_FORWARD_DOTTEDLINE,
    SQUARE_STACK_3D_FORWARD_DOTTEDLINE_FILL,
    SQUARE_STACK_3D_UP,
    SQUARE_STACK_3D_UP_BADGE_A,
    SQUARE_STACK_3D_UP_BADGE_A_FILL,
    SQUARE_STACK_3D_UP_FILL,
    SQUARE_STACK_3D_UP_SLASH,
    SQUARE_STACK_3D_UP_SLASH_FILL,
    SQUARE_STACK_FILL,
    SQUARE_TEXT_SQUARE,
    SQUARE_TEXT_SQUARE_FILL,
    SQUARE_TOPHALF_FILLED,
    SQUARE_TOPTHIRD_INSET_FILLED,
    SQUARE_TRAILINGTHIRD_INSET_FILLED,
    SQUARES_BELOW_RECTANGLE,
    SQUARES_LEADING_RECTANGLE,
    SQUARESHAPE,
    SQUARESHAPE_CONTROLHANDLES_ON_SQUARESHAPE_CONTROLHANDLES,
    SQUARESHAPE_DASHED_SQUARESHAPE,
    SQUARESHAPE_DOTTED_SPLIT_2X2,
    SQUARESHAPE_FILL,
    SQUARESHAPE_SPLIT_2X2,
    SQUARESHAPE_SPLIT_2X2_DOTTED,
    SQUARESHAPE_SPLIT_3X3,
    SQUARESHAPE_SQUARESHAPE_DASHED,
    STAIRS,
    STAR,
    STAR_BUBBLE,
    STAR_BUBBLE_FILL,
    STAR_CIRCLE,
    STAR_CIRCLE_FILL,
    STAR_FILL,
    STAR_LEADINGHALF_FILLED,
    STAR_SLASH,
    STAR_SLASH_FILL,
    STAR_SQUARE,
    STAR_SQUARE_FILL,
    STAR_SQUARE_ON_SQUARE,
    STAR_SQUARE_ON_SQUARE_FILL,
    STAROFLIFE,
    STAROFLIFE_CIRCLE,
    STAROFLIFE_CIRCLE_FILL,
    STAROFLIFE_FILL,
    STERLINGSIGN,
    STERLINGSIGN_CIRCLE,
    STERLINGSIGN_CIRCLE_FILL,
    STERLINGSIGN_SQUARE,
    STERLINGSIGN_SQUARE_FILL,
    STETHOSCOPE,
    STETHOSCOPE_CIRCLE,
    STETHOSCOPE_CIRCLE_FILL,
    STOP,
    STOP_CIRCLE,
    STOP_CIRCLE_FILL,
    STOP_FILL,
    STOPWATCH,
    STOPWATCH_FILL,
    STOVE,
    STOVE_FILL,
    STRIKETHROUGH,
    STUDENTDESK,
    SUIT_CLUB,
    SUIT_CLUB_FILL,
    SUIT_DIAMOND,
    SUIT_DIAMOND_FILL,
    SUIT_HEART,
    SUIT_HEART_FILL,
    SUIT_SPADE,
    SUIT_SPADE_FILL,
    SUITCASE,
    SUITCASE_CART,
    SUITCASE_CART_FILL,
    SUITCASE_FILL,
    SUM,
    SUN_AND_HORIZON,
    SUN_AND_HORIZON_CIRCLE,
    SUN_AND_HORIZON_CIRCLE_FILL,
    SUN_AND_HORIZON_FILL,
    SUN_DUST,
    SUN_DUST_CIRCLE,
    SUN_DUST_CIRCLE_FILL,
    SUN_DUST_FILL,
    SUN_HAZE,
    SUN_HAZE_CIRCLE,
    SUN_HAZE_CIRCLE_FILL,
    SUN_HAZE_FILL,
    SUN_MAX,
    SUN_MAX_CIRCLE,
    SUN_MAX_CIRCLE_FILL,
    SUN_MAX_FILL,
    SUN_MAX_TRIANGLEBADGE_EXCLAMATIONMARK,
    SUN_MAX_TRIANGLEBADGE_EXCLAMATIONMARK_FILL,
    SUN_MIN,
    SUN_MIN_FILL,
    SUNRISE,
    SUNRISE_CIRCLE,
    SUNRISE_CIRCLE_FILL,
    SUNRISE_FILL,
    SUNSET,
    SUNSET_CIRCLE,
    SUNSET_CIRCLE_FILL,
    SUNSET_FILL,
    SWATCHPALETTE,
    SWATCHPALETTE_FILL,
    SWIFT,
    SWITCH_2,
    SWITCH_PROGRAMMABLE,
    SWITCH_PROGRAMMABLE_FILL,
    SWITCH_PROGRAMMABLE_SQUARE,
    SWITCH_PROGRAMMABLE_SQUARE_FILL,
    SYRINGE,
    SYRINGE_FILL,
    T_CIRCLE,
    T_CIRCLE_FILL,
    T_SQUARE,
    T_SQUARE_FILL,
    TABLE_FURNITURE,
    TABLE_FURNITURE_FILL,
    TABLECELLS,
    TABLECELLS_BADGE_ELLIPSIS,
    TABLECELLS_FILL,
    TABLECELLS_FILL_BADGE_ELLIPSIS,
    TAG,
    TAG_CIRCLE,
    TAG_CIRCLE_FILL,
    TAG_FILL,
    TAG_SLASH,
    TAG_SLASH_FILL,
    TAG_SQUARE,
    TAG_SQUARE_FILL,
    TAKEOUTBAG_AND_CUP_AND_STRAW,
    TAKEOUTBAG_AND_CUP_AND_STRAW_FILL,
    TARGET,
    TEDDYBEAR,
    TEDDYBEAR_FILL,
    TELETYPE,
    TELETYPE_ANSWER,
    TELETYPE_ANSWER_CIRCLE,
    TELETYPE_ANSWER_CIRCLE_FILL,
    TELETYPE_CIRCLE,
    TELETYPE_CIRCLE_FILL,
    TENGESIGN,
    TENGESIGN_CIRCLE,
    TENGESIGN_CIRCLE_FILL,
    TENGESIGN_SQUARE,
    TENGESIGN_SQUARE_FILL,
    TENNIS_RACKET,
    TENNIS_RACKET_CIRCLE,
    TENNIS_RACKET_CIRCLE_FILL,
    TENNISBALL,
    TENNISBALL_CIRCLE,
    TENNISBALL_CIRCLE_FILL,
    TENNISBALL_FILL,
    TENT,
    TENT_FILL,
    TERMINAL,
    TERMINAL_FILL,
    TESTTUBE_2,
    TEXT_ALIGNCENTER,
    TEXT_ALIGNLEFT,
    TEXT_ALIGNRIGHT,
    TEXT_AND_COMMAND_MACWINDOW,
    TEXT_APPEND,
    TEXT_BADGE_CHECKMARK,
    TEXT_BADGE_MINUS,
    TEXT_BADGE_PLUS,
    TEXT_BADGE_STAR,
    TEXT_BADGE_XMARK,
    TEXT_BELOW_PHOTO,
    TEXT_BELOW_PHOTO_FILL,
    TEXT_BOOK_CLOSED,
    TEXT_BOOK_CLOSED_FILL,
    TEXT_BUBBLE,
    TEXT_BUBBLE_FILL,
    TEXT_INSERT,
    TEXT_JUSTIFY,
    TEXT_JUSTIFY_LEADING,
    TEXT_JUSTIFY_LEFT,
    TEXT_JUSTIFY_RIGHT,
    TEXT_JUSTIFY_TRAILING,
    TEXT_LINE_FIRST_AND_ARROWTRIANGLE_FORWARD,
    TEXT_LINE_LAST_AND_ARROWTRIANGLE_FORWARD,
    TEXT_MAGNIFYINGGLASS,
    TEXT_QUOTE,
    TEXT_REDACTION,
    TEXT_VIEWFINDER,
    TEXT_WORD_SPACING,
    TEXTFORMAT,
    TEXTFORMAT_12,
    TEXTFORMAT_123,
    TEXTFORMAT_ABC,
    TEXTFORMAT_ABC_DOTTEDUNDERLINE,
    TEXTFORMAT_ALT,
    TEXTFORMAT_SIZE,
    TEXTFORMAT_SIZE_LARGER,
    TEXTFORMAT_SIZE_SMALLER,
    TEXTFORMAT_SUBSCRIPT,
    TEXTFORMAT_SUPERSCRIPT,
    THEATERMASK_AND_PAINTBRUSH,
    THEATERMASK_AND_PAINTBRUSH_FILL,
    THEATERMASKS,
    THEATERMASKS_CIRCLE,
    THEATERMASKS_CIRCLE_FILL,
    THEATERMASKS_FILL,
    THERMOMETER,
    THERMOMETER_HIGH,
    THERMOMETER_LOW,
    THERMOMETER_MEDIUM,
    THERMOMETER_MEDIUM_SLASH,
    THERMOMETER_SNOWFLAKE,
    THERMOMETER_SNOWFLAKE_CIRCLE,
    THERMOMETER_SNOWFLAKE_CIRCLE_FILL,
    THERMOMETER_SUN,
    THERMOMETER_SUN_CIRCLE,
    THERMOMETER_SUN_CIRCLE_FILL,
    THERMOMETER_SUN_FILL,
    TICKET,
    TICKET_FILL,
    TIMELAPSE,
    TIMELINE_SELECTION,
    TIMER,
    TIMER_CIRCLE,
    TIMER_CIRCLE_FILL,
    TIMER_SQUARE,
    TOGGLEPOWER,
    TOILET,
    TOILET_FILL,
    TORNADO,
    TORNADO_CIRCLE,
    TORNADO_CIRCLE_FILL,
    TORTOISE,
    TORTOISE_FILL,
    TORUS,
    TOUCHID,
    TRAIN_SIDE_FRONT_CAR,
    TRAIN_SIDE_MIDDLE_CAR,
    TRAIN_SIDE_REAR_CAR,
    TRAM,
    TRAM_CIRCLE,
    TRAM_CIRCLE_FILL,
    TRAM_FILL,
    TRAM_FILL_TUNNEL,
    TRAPEZOID_AND_LINE_HORIZONTAL,
    TRAPEZOID_AND_LINE_HORIZONTAL_FILL,
    TRAPEZOID_AND_LINE_VERTICAL,
    TRAPEZOID_AND_LINE_VERTICAL_FILL,
    TRASH,
    TRASH_CIRCLE,
    TRASH_CIRCLE_FILL,
    TRASH_FILL,
    TRASH_SLASH,
    TRASH_SLASH_CIRCLE,
    TRASH_SLASH_CIRCLE_FILL,
    TRASH_SLASH_FILL,
    TRASH_SLASH_SQUARE,
    TRASH_SLASH_SQUARE_FILL,
    TRASH_SQUARE,
    TRASH_SQUARE_FILL,
    TRAY,
    TRAY_2,
    TRAY_2_FILL,
    TRAY_AND_ARROW_DOWN,
    TRAY_AND_ARROW_DOWN_FILL,
    TRAY_AND_ARROW_UP,
    TRAY_AND_ARROW_UP_FILL,
    TRAY_CIRCLE,
    TRAY_CIRCLE_FILL,
    TRAY_FILL,
    TRAY_FULL,
    TRAY_FULL_FILL,
    TRIANGLE,
    TRIANGLE_BOTTOMHALF_FILLED,
    TRIANGLE_CIRCLE,
    TRIANGLE_CIRCLE_FILL,
    TRIANGLE_FILL,
    TRIANGLE_INSET_FILLED,
    TRIANGLE_LEFTHALF_FILLED,
    TRIANGLE_RIGHTHALF_FILLED,
    TRIANGLE_TOPHALF_FILLED,
    TROPHY,
    TROPHY_CIRCLE,
    TROPHY_CIRCLE_FILL,
    TROPHY_FILL,
    TROPICALSTORM,
    TROPICALSTORM_CIRCLE,
    TROPICALSTORM_CIRCLE_FILL,
    TSHIRT,
    TSHIRT_FILL,
    TUGRIKSIGN,
    TUGRIKSIGN_CIRCLE,
    TUGRIKSIGN_CIRCLE_FILL,
    TUGRIKSIGN_SQUARE,
    TUGRIKSIGN_SQUARE_FILL,
    TUNINGFORK,
    TURKISHLIRASIGN,
    TURKISHLIRASIGN_CIRCLE,
    TURKISHLIRASIGN_CIRCLE_FILL,
    TURKISHLIRASIGN_SQUARE,
    TURKISHLIRASIGN_SQUARE_FILL,
    TV,
    TV_AND_HIFISPEAKER_FILL,
    TV_AND_MEDIABOX,
    TV_CIRCLE,
    TV_CIRCLE_FILL,
    TV_FILL,
    TV_INSET_FILLED,
    U_CIRCLE,
    U_CIRCLE_FILL,
    U_SQUARE,
    U_SQUARE_FILL,
    UIWINDOW_SPLIT_2X1,
    UMBRELLA,
    UMBRELLA_FILL,
    UMBRELLA_PERCENT,
    UMBRELLA_PERCENT_FILL,
    UNDERLINE,
    V_CIRCLE,
    V_CIRCLE_FILL,
    V_SQUARE,
    V_SQUARE_FILL,
    VIAL_VIEWFINDER,
    VIDEO,
    VIDEO_AND_WAVEFORM,
    VIDEO_AND_WAVEFORM_FILL,
    VIDEO_BADGE_CHECKMARK,
    VIDEO_BADGE_ELLIPSIS,
    VIDEO_BADGE_PLUS,
    VIDEO_BUBBLE_LEFT,
    VIDEO_BUBBLE_LEFT_FILL,
    VIDEO_CIRCLE,
    VIDEO_CIRCLE_FILL,
    VIDEO_DOORBELL,
    VIDEO_DOORBELL_FILL,
    VIDEO_FILL,
    VIDEO_FILL_BADGE_CHECKMARK,
    VIDEO_FILL_BADGE_ELLIPSIS,
    VIDEO_FILL_BADGE_PLUS,
    VIDEO_SLASH,
    VIDEO_SLASH_FILL,
    VIDEO_SQUARE,
    VIDEO_SQUARE_FILL,
    VIDEOPROJECTOR,
    VIDEOPROJECTOR_FILL,
    VIEW_2D,
    VIEW_3D,
    VIEWFINDER,
    VIEWFINDER_CIRCLE,
    VIEWFINDER_CIRCLE_FILL,
    VOLLEYBALL,
    VOLLEYBALL_CIRCLE,
    VOLLEYBALL_CIRCLE_FILL,
    VOLLEYBALL_FILL,
    W_CIRCLE,
    W_CIRCLE_FILL,
    W_SQUARE,
    W_SQUARE_FILL,
    WAKE,
    WAKE_CIRCLE,
    WAKE_CIRCLE_FILL,
    WALLET_PASS,
    WALLET_PASS_FILL,
    WAND_AND_RAYS,
    WAND_AND_RAYS_INVERSE,
    WAND_AND_STARS,
    WAND_AND_STARS_INVERSE,
    WASHER,
    WASHER_FILL,
    WATCHFACE_APPLEWATCH_CASE,
    WATER_WAVES,
    WATER_WAVES_AND_ARROW_DOWN,
    WATER_WAVES_AND_ARROW_DOWN_TRIANGLEBADGE_EXCLAMATIONMARK,
    WATER_WAVES_AND_ARROW_UP,
    WATER_WAVES_SLASH,
    WAVE_3_BACKWARD,
    WAVE_3_BACKWARD_CIRCLE,
    WAVE_3_BACKWARD_CIRCLE_FILL,
    WAVE_3_FORWARD,
    WAVE_3_FORWARD_CIRCLE,
    WAVE_3_FORWARD_CIRCLE_FILL,
    WAVE_3_LEFT,
    WAVE_3_LEFT_CIRCLE,
    WAVE_3_LEFT_CIRCLE_FILL,
    WAVE_3_RIGHT,
    WAVE_3_RIGHT_CIRCLE,
    WAVE_3_RIGHT_CIRCLE_FILL,
    WAVEFORM,
    WAVEFORM_AND_MAGNIFYINGGLASS,
    WAVEFORM_AND_MIC,
    WAVEFORM_BADGE_EXCLAMATIONMARK,
    WAVEFORM_BADGE_MINUS,
    WAVEFORM_BADGE_PLUS,
    WAVEFORM_CIRCLE,
    WAVEFORM_CIRCLE_FILL,
    WAVEFORM_PATH,
    WAVEFORM_PATH_BADGE_MINUS,
    WAVEFORM_PATH_BADGE_PLUS,
    WAVEFORM_PATH_ECG,
    WAVEFORM_PATH_ECG_RECTANGLE,
    WAVEFORM_PATH_ECG_RECTANGLE_FILL,
    WAVEFORM_SLASH,
    WEB_CAMERA,
    WEB_CAMERA_FILL,
    WIFI,
    WIFI_CIRCLE,
    WIFI_CIRCLE_FILL,
    WIFI_EXCLAMATIONMARK,
    WIFI_ROUTER,
    WIFI_ROUTER_FILL,
    WIFI_SLASH,
    WIFI_SQUARE,
    WIFI_SQUARE_FILL,
    WIND,
    WIND_CIRCLE,
    WIND_CIRCLE_FILL,
    WIND_SNOW,
    WIND_SNOW_CIRCLE,
    WIND_SNOW_CIRCLE_FILL,
    WINDOW_AWNING,
    WINDOW_AWNING_CLOSED,
    WINDOW_CASEMENT,
    WINDOW_CASEMENT_CLOSED,
    WINDOW_CEILING,
    WINDOW_CEILING_CLOSED,
    WINDOW_HORIZONTAL,
    WINDOW_HORIZONTAL_CLOSED,
    WINDOW_SHADE_CLOSED,
    WINDOW_SHADE_OPEN,
    WINDOW_VERTICAL_CLOSED,
    WINDOW_VERTICAL_OPEN,
    WINEGLASS,
    WINEGLASS_FILL,
    WONSIGN,
    WONSIGN_CIRCLE,
    WONSIGN_CIRCLE_FILL,
    WONSIGN_SQUARE,
    WONSIGN_SQUARE_FILL,
    WRENCH,
    WRENCH_ADJUSTABLE,
    WRENCH_ADJUSTABLE_FILL,
    WRENCH_AND_SCREWDRIVER,
    WRENCH_AND_SCREWDRIVER_FILL,
    WRENCH_FILL,
    X_CIRCLE,
    X_CIRCLE_FILL,
    X_SQUARE,
    X_SQUARE_FILL,
    X_SQUAREROOT,
    XBOX_LOGO,
    XMARK,
    XMARK_APP,
    XMARK_APP_FILL,
    XMARK_BIN,
    XMARK_BIN_CIRCLE,
    XMARK_BIN_CIRCLE_FILL,
    XMARK_BIN_FILL,
    XMARK_CIRCLE,
    XMARK_CIRCLE_FILL,
    XMARK_DIAMOND,
    XMARK_DIAMOND_FILL,
    XMARK_ICLOUD,
    XMARK_ICLOUD_FILL,
    XMARK_OCTAGON,
    XMARK_OCTAGON_FILL,
    XMARK_RECTANGLE,
    XMARK_RECTANGLE_FILL,
    XMARK_RECTANGLE_PORTRAIT,
    XMARK_RECTANGLE_PORTRAIT_FILL,
    XMARK_SEAL,
    XMARK_SEAL_FILL,
    XMARK_SHIELD,
    XMARK_SHIELD_FILL,
    XMARK_SQUARE,
    XMARK_SQUARE_FILL,
    XSERVE,
    Y_CIRCLE,
    Y_CIRCLE_FILL,
    Y_SQUARE,
    Y_SQUARE_FILL,
    YENSIGN,
    YENSIGN_CIRCLE,
    YENSIGN_CIRCLE_FILL,
    YENSIGN_SQUARE,
    YENSIGN_SQUARE_FILL,
    Z_CIRCLE,
    Z_CIRCLE_FILL,
    Z_SQUARE,
    Z_SQUARE_FILL,
    ZL_RECTANGLE_ROUNDEDTOP,
    ZL_RECTANGLE_ROUNDEDTOP_FILL,
    ZR_RECTANGLE_ROUNDEDTOP,
    ZR_RECTANGLE_ROUNDEDTOP_FILL,
    ZZZ,
];


mod ffi;
