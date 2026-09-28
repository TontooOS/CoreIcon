//! Raster I/O helpers on top of CoreImage.
//!
//! All image file access in CoreIcon goes through these helpers so the
//! third-party `image` crate never appears in this crate directly. Pixel
//! buffers keep the shared [`coreimage::RgbaImage`] type, so conversion is
//! zero-copy via `TiImage::from_rgba` / `TiImage::into_rgba`.

use coreimage::{ImageFormat, RgbaImage, TiImage};
use std::path::Path;

/// Load an image file (format auto-detected) into an RGBA buffer.
pub(crate) fn load_rgba(path: impl AsRef<Path>) -> Result<RgbaImage, coreimage::ImageError> {
    Ok(TiImage::load(&path.as_ref().to_string_lossy())?.into_rgba())
}

/// Resize to exact dimensions (Lanczos3), stretching when aspects differ.
pub(crate) fn resize_exact(img: &RgbaImage, width: u32, height: u32) -> RgbaImage {
    TiImage::from_rgba(img.clone())
        .resize(width.max(1), height.max(1), coreimage::FilterType::Lanczos3)
        .into_rgba()
}

/// Aspect-preserving fit inside `width` x `height` (no padding), matching
/// the old `DynamicImage::resize` behavior used for 1024px icon sources.
pub(crate) fn resize_fit(img: &RgbaImage, width: u32, height: u32) -> RgbaImage {
    let (sw, sh) = (img.width().max(1), img.height().max(1));
    let scale = (width as f32 / sw as f32).min(height as f32 / sh as f32);
    let (nw, nh) = (
        ((sw as f32 * scale).round() as u32).max(1),
        ((sh as f32 * scale).round() as u32).max(1),
    );
    resize_exact(img, nw, nh)
}

/// Scale to fill `width` x `height`, then center-crop to the exact size
/// (matches the old `resize_to_fill` behavior for backgrounds and layers).
pub(crate) fn resize_fill(
    img: &RgbaImage,
    width: u32,
    height: u32,
) -> Result<RgbaImage, coreimage::ImageError> {
    Ok(TiImage::from_rgba(img.clone())
        .fit(width, height, coreimage::FitMode::Fill)?
        .into_rgba())
}

/// Save an RGBA buffer, inferring the format from the file extension.
/// JPEG uses quality 75 (the previous encoder default); lossless formats
/// ignore quality.
pub(crate) fn save_rgba(
    img: &RgbaImage,
    path: impl AsRef<Path>,
) -> Result<(), coreimage::ImageError> {
    let path = path.as_ref();
    let name = path.to_string_lossy();
    let format = ImageFormat::from_extension(&name)
        .ok_or_else(|| coreimage::ImageError::Unsupported(coreimage::tr("unknown_format")))?;
    let quality = if format == ImageFormat::Jpeg { 75 } else { 100 };
    TiImage::from_rgba(img.clone()).save(&name, format, quality)
}
