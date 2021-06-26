use macroquad::prelude::*;

use std::collections::LinkedList;

const SQUARES: i16 = 16;

type Point = (i16, i16);

struct Snake {
    head: Point,
    body: LinkedList<Point>,
    dir: Point,
}

#[macroquad::main("Snake")]
async fn main() {


    loop {
        clear_background(WHITE);
        let text = "Game Over. Press [enter] to play again.";
        let font_size = 30.;
        let text_size = measure_text(text, None, font_size as _, 1.0);

        draw_text(
            text,
            screen_width() / 2. - text_size.width / 2.,
            screen_height() / 2. - text_size.height / 2.,
            font_size,
            DARKGRAY,
        );

        next_frame().await
    }
}
