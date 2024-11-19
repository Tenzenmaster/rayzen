pub use raylib_sys::{MouseCursor, KeyboardKey, MouseButton, Quaternion, TraceLogLevel, Transform, Vector2, Vector3, Vector4};
use std::ffi::c_uchar;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl From<Color> for raylib_sys::Color {
    fn from(color: Color) -> Self {
        Self {
            r: color.r as c_uchar,
            g: color.g as c_uchar,
            b: color.b as c_uchar,
            a: color.a as c_uchar,
        }
    }
}

impl From<raylib_sys::Color> for Color {
    fn from(color: raylib_sys::Color) -> Self {
        Self {
            r: color.r as u8,
            g: color.g as u8,
            b: color.b as u8,
            a: color.a as u8,
        }
    }
}

impl Color {
    pub const BLACK: Self = Self { r: 0, g: 0, b: 0, a: 255 };
    pub const BLANK: Self = Self { r: 0, g: 0, b: 0, a: 0 };
    pub const RAYWHITE: Self = Self { r: 245, g: 245, b: 245, a: 255 };
    pub const RED: Self = Self { r: 255, g: 0, b: 0, a: 255 };
    pub const WHITE: Self = Self { r: 255, g: 255, b: 255, a: 255 };
}
