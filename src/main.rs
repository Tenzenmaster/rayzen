use rayzen as rz;

fn main() {
    rz::set_trace_log_level(rz::TraceLogLevel::LOG_WARNING);
    rz::init_window(1920, 1080, "Helloooouuu");
    rz::set_target_fps(60);

    while !rz::window_should_close() {
        rz::begin_drawing();
        {
            rz::clear_background(rz::Color::RAYWHITE);
            rz::draw_text("Hi there", 24, 24, 40, rz::Color::BLACK);
            rz::draw_line(64, 64, 256, 64, rz::Color::RED);
        }
        rz::end_drawing();
    }
}
