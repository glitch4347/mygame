use macroquad::prelude::*;


#[macroquad::main("Snake")]
async fn main() {

    let mut i = 0;
    loop {
        clear_background(LIGHTGRAY);

        let game_size = screen_width().min(screen_height());
        let offset_x = (screen_width() - game_size) / 2. + 10.;
        let offset_y = (screen_height() - game_size) / 2. + 10.;

        draw_rectangle(offset_x, offset_y, game_size - 20., game_size - 20., RED);

        draw_line(
            offset_x + 16  as f32,
            offset_y,
            offset_x + 16 as f32,
            screen_height() - offset_y,
            2.,
            LIGHTGRAY,
        );

        draw_circle(screen_width() / 2., i as _, 20 as _, GREEN);

        i += 1;

        next_frame().await
    }
}
