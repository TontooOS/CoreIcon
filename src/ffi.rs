//! C FFI exports for CoreIcon.

use std::ffi::CStr;
use std::os::raw::c_char;

/// RGBA color passed across the FFI boundary.
#[repr(C)]
pub struct CoreIconColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

/// The framework version as a static C string.
#[no_mangle]
pub extern "C" fn tontoo_coreicon_version() -> *const c_char {
    concat!(env!("CARGO_PKG_VERSION"), "\0").as_ptr() as *const c_char
}

/// Parse a hex color string (`#rrggbb` / `#rrggbbaa`) into RGBA components.
///
/// Returns 0 on success and writes to `out`; -1 on null arguments; -2 on
/// invalid UTF-8; -3 on an unparsable color.
///
/// # Safety
///
/// `hex` must be NUL-terminated; `out` must be a valid pointer.
#[no_mangle]
pub unsafe extern "C" fn tontoo_coreicon_color_from_hex(
    hex: *const c_char,
    out: *mut CoreIconColor,
) -> i32 {
    if hex.is_null() || out.is_null() {
        return -1;
    }
    let hex = match CStr::from_ptr(hex).to_str() {
        Ok(s) => s,
        Err(_) => return -2,
    };
    match crate::Color::from_hex(hex) {
        Some(color) => {
            *out = CoreIconColor {
                r: color.r,
                g: color.g,
                b: color.b,
                a: color.a,
            };
            0
        }
        None => -3,
    }
}

/// Free a string returned by this library.
///
/// # Safety
///
/// `s` must be a pointer returned by this API or null.
#[no_mangle]
pub unsafe extern "C" fn tontoo_coreicon_string_free(s: *mut c_char) {
    if !s.is_null() {
        drop(std::ffi::CString::from_raw(s));
    }
}
