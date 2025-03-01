use macroquad::prelude::*;

#[macroquad::main("Demo")]
async fn main() {
    loop {
        clear_background(WHITE);
        let welcomeText = "Hello World!";
        let welcomeFontSize = 30;
        let welcomeTextSize = measure_text(
            &welcomeText,
            None,
            welcomeFontSize,
            1.0
        );
        draw_text(
            welcomeText,
            screen_width() / 2.0 - welcomeTextSize.width / 2.0,
            screen_height() / 2.0 - welcomeTextSize.height / 2.0,
            welcomeFontSize as f32,
            BLACK
        );
        next_frame().await;
    }
}
