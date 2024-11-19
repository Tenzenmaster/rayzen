use std::ffi::{c_int, CString};

pub fn draw_text(
    text: &str,
    pos_x: i32,
    pos_y: i32,
    font_size: i32,
    color: impl Into<raylib_sys::Color>,
) {
    unsafe {
        #[allow(temporary_cstring_as_ptr)]
        raylib_sys::DrawText(
            CString::new(text).unwrap().as_ptr(),
            pos_x as c_int,
            pos_y as c_int,
            font_size as c_int,
            color.into(),
        )
    }
}
