pub use raylib_sys::{MouseCursor, KeyboardKey, MouseButton, Quaternion, TraceLogLevel, Transform, Vector2, Vector3, Vector4};
use std::ffi::{c_int, c_uchar, CString};

// Color

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

// Window

pub fn init_window(width: i32, height: i32, title: &str) {
    unsafe {
        #[allow(temporary_cstring_as_ptr)]
        raylib_sys::InitWindow(
            width as c_int,
            height as c_int,
            CString::new(title).unwrap().as_ptr(),
        )
    }
}

pub fn close_window() {
    unsafe { raylib_sys::CloseWindow() }
}

pub fn window_should_close() -> bool {
    unsafe { raylib_sys::WindowShouldClose() }
}

pub fn is_window_ready() -> bool {
    unsafe { raylib_sys::IsWindowReady() }
}

pub fn is_window_fullscreen() -> bool {
    unsafe { raylib_sys::IsWindowFullscreen() }
}

pub fn is_window_hidden() -> bool {
    unsafe { raylib_sys::IsWindowHidden() }
}

pub fn is_window_minimized() -> bool {
    unsafe { raylib_sys::IsWindowMinimized() }
}

pub fn is_window_maximized() -> bool {
    unsafe { raylib_sys::IsWindowMaximized() }
}

pub fn is_window_focused() -> bool {
    unsafe { raylib_sys::IsWindowFocused() }
}

pub fn is_window_resized() -> bool {
    unsafe { raylib_sys::IsWindowResized() }
}

// Drawing

pub fn clear_background(color: impl Into<raylib_sys::Color>) {
    unsafe { raylib_sys::ClearBackground(color.into()) }
}

pub fn begin_drawing() {
    unsafe { raylib_sys::BeginDrawing() }
}

pub fn end_drawing() {
    unsafe { raylib_sys::EndDrawing() }
}

// Cursor

pub fn show_cursor() {
    unsafe { raylib_sys::ShowCursor() }
}

pub fn hide_cursor() {
    unsafe { raylib_sys::HideCursor() }
}

pub fn is_cursor_hidden() -> bool {
    unsafe { raylib_sys::IsCursorHidden() }
}

pub fn enable_cursor() {
    unsafe { raylib_sys::EnableCursor() }
}

pub fn disable_cursor() {
    unsafe { raylib_sys::DisableCursor() }
}

pub fn is_cursor_on_screen() -> bool {
    unsafe { raylib_sys::IsCursorOnScreen() }
}

// Timing

pub fn set_target_fps(fps: i32) {
    unsafe { raylib_sys::SetTargetFPS(fps as c_int) }
}

// Utility

pub fn set_trace_log_level(log_level: raylib_sys::TraceLogLevel) {
    unsafe { raylib_sys::SetTraceLogLevel(log_level as c_int) }
}

// Keyboard

pub fn is_key_pressed(key: crate::KeyboardKey) -> bool {
    unsafe { raylib_sys::IsKeyPressed(key as c_int) }
}

pub fn is_key_pressed_repeat(key: crate::KeyboardKey) -> bool {
    unsafe { raylib_sys::IsKeyPressedRepeat(key as c_int) }
}

pub fn is_key_down(key: crate::KeyboardKey) -> bool {
    unsafe { raylib_sys::IsKeyDown(key as c_int) }
}

pub fn is_key_released(key: crate::KeyboardKey) -> bool {
    unsafe { raylib_sys::IsKeyReleased(key as c_int) }
}

pub fn is_key_up(key: crate::KeyboardKey) -> bool {
    unsafe { raylib_sys::IsKeyUp(key as c_int) }
}

pub fn get_key_pressed() -> crate::KeyboardKey {
    unsafe { key_from_i32(raylib_sys::GetKeyPressed()).unwrap() }
}

pub fn get_char_pressed() -> Option<char> {
    unsafe {
        match char::from_u32(raylib_sys::GetCharPressed() as u32) {
            Some(c) if c == '\0' => None,
            Some(c) => Some(c),
            None => unreachable!(),
        }
    }
}

pub fn set_exit_key(key: crate::KeyboardKey) {
    unsafe { raylib_sys::SetExitKey(key as c_int) }
}

// Mouse

pub fn is_mouse_button_pressed(button: crate::MouseButton) -> bool {
    unsafe { raylib_sys::IsMouseButtonPressed(button as c_int) }
}

pub fn is_mouse_button_down(button: crate::MouseButton) -> bool {
    unsafe { raylib_sys::IsMouseButtonDown(button as c_int) }
}

pub fn is_mouse_button_released(button: crate::MouseButton) -> bool {
    unsafe { raylib_sys::IsMouseButtonReleased(button as c_int) }
}

pub fn is_mouse_button_up(button: crate::MouseButton) -> bool {
    unsafe { raylib_sys::IsMouseButtonUp(button as c_int) }
}

pub fn get_mouse_x() -> i32 {
    unsafe { raylib_sys::GetMouseX() as i32 }
}

pub fn get_mouse_y() -> i32 {
    unsafe { raylib_sys::GetMouseY() as i32 }
}

pub fn get_mouse_position() -> crate::Vector2 {
    unsafe { raylib_sys::GetMousePosition() }
}

pub fn get_mouse_delta() -> crate::Vector2 {
    unsafe { raylib_sys::GetMouseDelta() }
}

pub fn set_mouse_position(x: i32, y: i32) {
    unsafe { raylib_sys::SetMousePosition(x as c_int, y as c_int) }
}

pub fn set_mouse_offset(offset_x: i32, offset_y: i32) {
    unsafe { raylib_sys::SetMouseOffset(offset_x as c_int, offset_y as c_int) }
}

pub fn set_mouse_scale(scale_x: i32, scale_y: i32) {
    unsafe { raylib_sys::SetMouseOffset(scale_x as c_int, scale_y as c_int) }
}

pub fn get_mouse_wheel_move() -> f32 {
    unsafe { raylib_sys::GetMouseWheelMove() }
}

pub fn get_mouse_wheel_move_v() -> crate::Vector2 {
    unsafe { raylib_sys::GetMouseWheelMoveV() }
}

pub fn set_mouse_cursor(cursor: crate::MouseCursor) {
    unsafe { raylib_sys::SetMouseCursor(cursor as c_int) }
}

// Private

fn key_from_i32(key: i32) -> Option<crate::KeyboardKey> {
    use crate::KeyboardKey::*;
    match key {
        39 => Some(KEY_APOSTROPHE),
        44 => Some(KEY_COMMA),
        45 => Some(KEY_MINUS),
        46 => Some(KEY_PERIOD),
        47 => Some(KEY_SLASH),
        48 => Some(KEY_ZERO),
        49 => Some(KEY_ONE),
        50 => Some(KEY_TWO),
        51 => Some(KEY_THREE),
        52 => Some(KEY_FOUR),
        53 => Some(KEY_FIVE),
        54 => Some(KEY_SIX),
        55 => Some(KEY_SEVEN),
        56 => Some(KEY_EIGHT),
        57 => Some(KEY_NINE),
        59 => Some(KEY_SEMICOLON),
        61 => Some(KEY_EQUAL),
        65 => Some(KEY_A),
        66 => Some(KEY_B),
        67 => Some(KEY_C),
        68 => Some(KEY_D),
        69 => Some(KEY_E),
        70 => Some(KEY_F),
        71 => Some(KEY_G),
        72 => Some(KEY_H),
        73 => Some(KEY_I),
        74 => Some(KEY_J),
        75 => Some(KEY_K),
        76 => Some(KEY_L),
        77 => Some(KEY_M),
        78 => Some(KEY_N),
        79 => Some(KEY_O),
        80 => Some(KEY_P),
        81 => Some(KEY_Q),
        82 => Some(KEY_R),
        83 => Some(KEY_S),
        84 => Some(KEY_T),
        85 => Some(KEY_U),
        86 => Some(KEY_V),
        87 => Some(KEY_W),
        88 => Some(KEY_X),
        89 => Some(KEY_Y),
        90 => Some(KEY_Z),
        32 => Some(KEY_SPACE),
        256 => Some(KEY_ESCAPE),
        257 => Some(KEY_ENTER),
        258 => Some(KEY_TAB),
        259 => Some(KEY_BACKSPACE),
        260 => Some(KEY_INSERT),
        261 => Some(KEY_DELETE),
        262 => Some(KEY_RIGHT),
        263 => Some(KEY_LEFT),
        264 => Some(KEY_DOWN),
        265 => Some(KEY_UP),
        266 => Some(KEY_PAGE_UP),
        267 => Some(KEY_PAGE_DOWN),
        268 => Some(KEY_HOME),
        269 => Some(KEY_END),
        280 => Some(KEY_CAPS_LOCK),
        281 => Some(KEY_SCROLL_LOCK),
        282 => Some(KEY_NUM_LOCK),
        283 => Some(KEY_PRINT_SCREEN),
        284 => Some(KEY_PAUSE),
        290 => Some(KEY_F1),
        291 => Some(KEY_F2),
        292 => Some(KEY_F3),
        293 => Some(KEY_F4),
        294 => Some(KEY_F5),
        295 => Some(KEY_F6),
        296 => Some(KEY_F7),
        297 => Some(KEY_F8),
        298 => Some(KEY_F9),
        299 => Some(KEY_F10),
        300 => Some(KEY_F11),
        301 => Some(KEY_F12),
        340 => Some(KEY_LEFT_SHIFT),
        341 => Some(KEY_LEFT_CONTROL),
        342 => Some(KEY_LEFT_ALT),
        343 => Some(KEY_LEFT_SUPER),
        344 => Some(KEY_RIGHT_SHIFT),
        345 => Some(KEY_RIGHT_CONTROL),
        346 => Some(KEY_RIGHT_ALT),
        347 => Some(KEY_RIGHT_SUPER),
        348 => Some(KEY_KB_MENU),
        91 => Some(KEY_LEFT_BRACKET),
        92 => Some(KEY_BACKSLASH),
        93 => Some(KEY_RIGHT_BRACKET),
        96 => Some(KEY_GRAVE),
        320 => Some(KEY_KP_0),
        321 => Some(KEY_KP_1),
        322 => Some(KEY_KP_2),
        323 => Some(KEY_KP_3),
        324 => Some(KEY_KP_4),
        325 => Some(KEY_KP_5),
        326 => Some(KEY_KP_6),
        327 => Some(KEY_KP_7),
        328 => Some(KEY_KP_8),
        329 => Some(KEY_KP_9),
        330 => Some(KEY_KP_DECIMAL),
        331 => Some(KEY_KP_DIVIDE),
        332 => Some(KEY_KP_MULTIPLY),
        333 => Some(KEY_KP_SUBTRACT),
        334 => Some(KEY_KP_ADD),
        335 => Some(KEY_KP_ENTER),
        336 => Some(KEY_KP_EQUAL),
        _ => None,
    }
}

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

// Text
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
