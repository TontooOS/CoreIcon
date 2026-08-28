// TintMatrix — Apple-style color matrix recoloring (RB::Layer tint rows).
//
// Artwork can stay neutral; the color lives in a 4x5 matrix (16 coefficients
// plus per-row offsets). One matrix replaces recolored asset variants.

use crate::Color;
use image::{Rgba, RgbaImage};

/// A 4x5 color transformation matrix.
///
/// Each output channel is the dot product of a row with the input vector
/// `[r, g, b, a, 1]`, clamped to `0.0..=1.0`:
///
/// ```text
/// out = M * [r, g, b, a, 1]^T
/// ```
///
/// Rows 0..2 map RGB, row 3 maps alpha. The fifth column is an additive
/// offset. This mirrors the per-layer tint matrices of Apple's IconRendering
/// `RB::Layer` kernel (`tintMatrixRow0..tintMatrixRow3`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TintMatrix(pub [[f32; 5]; 4]);

pub const LUMA_R: f32 = 0.213;
pub const LUMA_G: f32 = 0.715;
pub const LUMA_B: f32 = 0.072;

impl TintMatrix {
    /// Pass-through matrix.
    pub fn identity() -> Self {
        Self([
            [1.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0, 0.0],
        ])
    }

    /// Build from raw rows `[r, g, b, a, offset]`.
    pub const fn from_rows(rows: [[f32; 5]; 4]) -> Self { Self(rows) }

    /// Replace RGB with a constant color, keep source alpha (template tint).
    pub fn solid(color: Color) -> Self {
        let mut m = Self::identity();
        m.0[0] = [0.0, 0.0, 0.0, 0.0, color.r];
        m.0[1] = [0.0, 0.0, 0.0, 0.0, color.g];
        m.0[2] = [0.0, 0.0, 0.0, 0.0, color.b];
        m
    }

    /// Modulate RGB by a color (`out.rgb = src.rgb * color.rgb`).
    pub fn multiply(color: Color) -> Self {
        let mut m = Self::identity();
        m.0[0][0] = color.r;
        m.0[1][1] = color.g;
        m.0[2][2] = color.b;
        m
    }

    /// Apple accent tint: `out.rgb = accent.rgb * (k * luma + (1-k) * value)`.
    ///
    /// Presets the artwork's shading while pulling every colored pixel toward
    /// the accent hue. `keep_value` is the `k` split; Apple uses `0.2`
    /// luma / `0.8` value for dock icon tints.
    pub fn luma_tint(accent: Color, keep_value: f32) -> Self {
        let k = keep_value.clamp(0.0, 1.0);
        let l = 1.0 - k;
        let mut m = Self([[0.0; 5]; 4]);
        for (row, coeff) in m.0.iter_mut().zip([accent.r, accent.g, accent.b]) {
            row[0] = coeff * (k + l * LUMA_R);
            row[1] = coeff * l * LUMA_G;
            row[2] = coeff * l * LUMA_B;
            row[3] = 0.0;
            row[4] = 0.0;
        }
        m.0[3] = [0.0, 0.0, 0.0, 1.0, 0.0];
        m
    }

    /// Desaturate by `strength` (`0.0` = original, `1.0` = gray).
    pub fn grayscale(strength: f32) -> Self { Self::saturate(1.0 - strength.clamp(0.0, 1.0)) }

    /// Scale saturation around the luminance axis (`1.0` = unchanged).
    pub fn saturate(s: f32) -> Self {
        let mut m = Self::identity();
        let rows = [
            [LUMA_R + s * (1.0 - LUMA_R), LUMA_G * (1.0 - s), LUMA_B * (1.0 - s)],
            [LUMA_R * (1.0 - s), LUMA_G + s * (1.0 - LUMA_G), LUMA_B * (1.0 - s)],
            [LUMA_R * (1.0 - s), LUMA_G * (1.0 - s), LUMA_B + s * (1.0 - LUMA_B)],
        ];
        for i in 0..3 { m.0[i][0..3].copy_from_slice(&rows[i]); }
        m
    }

    /// Rotate hue by `degrees`. Alpha and overall luminance are preserved.
    pub fn hue_rotate(degrees: f32) -> Self {
        let rad = degrees.to_radians();
        let (sin, cos) = rad.sin_cos();
        let mut m = Self::identity();
        let rows = [
            [
                LUMA_R + cos * (1.0 - LUMA_R) - sin * LUMA_R,
                LUMA_G - cos * LUMA_G - sin * LUMA_G,
                LUMA_B - cos * LUMA_B + sin * (1.0 - LUMA_B),
            ],
            [
                LUMA_R - cos * LUMA_R + sin * 0.143,
                LUMA_G + cos * (1.0 - LUMA_G) + sin * 0.140,
                LUMA_B - cos * LUMA_B - sin * LUMA_B,
            ],
            [
                LUMA_R - cos * LUMA_R - sin * (1.0 - LUMA_R),
                LUMA_G - cos * LUMA_G + sin * LUMA_G,
                LUMA_B + cos * (1.0 - LUMA_B) + sin * LUMA_B,
            ],
        ];
        for i in 0..3 { m.0[i][0..3].copy_from_slice(&rows[i]); }
        m
    }

    /// Multiply RGB by `factor` (`1.0` = unchanged).
    pub fn brightness(factor: f32) -> Self {
        let mut m = Self::identity();
        for row in m.0.iter_mut().take(3) {
            for c in row.iter_mut().take(3) { *c *= factor; }
        }
        m
    }

    /// Contrast pivot at mid-gray (`1.0` = unchanged).
    pub fn contrast(factor: f32) -> Self {
        let mut m = Self::identity();
        for row in m.0.iter_mut().take(3) {
            for c in row.iter_mut().take(3) { *c *= factor; }
            row[4] = 0.5 - 0.5 * factor;
        }
        m
    }

    /// Invert RGB by `strength` (`0.0` = original, `1.0` = fully inverted).
    pub fn invert(strength: f32) -> Self {
        let k = strength.clamp(0.0, 1.0);
        let mut m = Self::identity();
        for row in m.0.iter_mut().take(3) {
            for c in row.iter_mut().take(3) { *c = *c * (1.0 - 2.0 * k); }
            row[4] = k;
        }
        m
    }

    /// Scale the alpha channel by `factor` (`1.0` = unchanged).
    pub fn fade(factor: f32) -> Self {
        let mut m = Self::identity();
        m.0[3][3] = factor.max(0.0);
        m
    }

    /// Compose two matrices: `self.then(other)` applies `self` first.
    pub fn then(&self, other: &TintMatrix) -> TintMatrix {
        let mut out = [[0.0f32; 5]; 4];
        for (i, orow) in out.iter_mut().enumerate() {
            for j in 0..5 {
                let mut acc = 0.0;
                for k in 0..4 {
                    acc += self.0[i][k] * other.0[k][j];
                }
                // j == 4 picks up the constant column via the implicit 1.
                if j == 4 { acc += self.0[i][4]; }
                orow[j] = acc;
            }
        }
        TintMatrix(out)
    }

    /// Apply to straight-alpha RGBA components in `0.0..=1.0`.
    pub fn apply(&self, r: f32, g: f32, b: f32, a: f32) -> (f32, f32, f32, f32) {
        let v = [r, g, b, a, 1.0];
        let ch = |row: &[f32; 5]| {
            row.iter().zip(v.iter()).map(|(m, x)| m * x).sum::<f32>().clamp(0.0, 1.0)
        };
        (ch(&self.0[0]), ch(&self.0[1]), ch(&self.0[2]), ch(&self.0[3]))
    }

    /// Apply the matrix to a whole image (straight alpha, in a new buffer).
    /// Fully transparent pixels pass through untouched.
    pub fn apply_image(&self, src: &RgbaImage) -> RgbaImage {
        let mut out = RgbaImage::new(src.width(), src.height());
        for (x, y, pixel) in src.enumerate_pixels() {
            let p = *pixel;
            if p[3] == 0 {
                out.put_pixel(x, y, p);
                continue;
            }
            let (r, g, b, a) = self.apply(
                p[0] as f32 / 255.0,
                p[1] as f32 / 255.0,
                p[2] as f32 / 255.0,
                p[3] as f32 / 255.0,
            );
            out.put_pixel(x, y, Rgba([
                (r * 255.0).round().clamp(0.0, 255.0) as u8,
                (g * 255.0).round().clamp(0.0, 255.0) as u8,
                (b * 255.0).round().clamp(0.0, 255.0) as u8,
                (a * 255.0).round().clamp(0.0, 255.0) as u8,
            ]));
        }
        out
    }
}

impl Default for TintMatrix {
    fn default() -> Self { Self::identity() }
}
