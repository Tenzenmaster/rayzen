use std::ffi::c_int;

pub fn draw_pixel(pos_x: i32, pos_y: i32, color: impl Into<raylib_sys::Color>) {
    unsafe { raylib_sys::DrawPixel(pos_x as c_int, pos_y as c_int, color.into()) }
}

pub fn draw_pixel_v(position: crate::Vector2, color: impl Into<raylib_sys::Color>) {
    unsafe { raylib_sys::DrawPixelV(position, color.into()) }
}

pub fn draw_line(start_pos_x: i32, start_pos_y: i32, end_pos_x: i32, end_pos_y: i32, color: impl Into<raylib_sys::Color>) {
    unsafe { raylib_sys::DrawLine(start_pos_x as c_int, start_pos_y as c_int, end_pos_x as c_int, end_pos_y as c_int, color.into()) }
}

pub fn draw_line_v(start_pos: crate::Vector2, end_pos: crate::Vector2, color: impl Into<raylib_sys::Color>) {
    unsafe { raylib_sys::DrawLineV(start_pos, end_pos, color.into()) }
}
