use core::f32;

use macroquad::prelude::*;

const SQUARES_SIZE: i16 = 32;


type Point = (i16, i16);

struct Game {
    game_size: f32,
    pixel_size: f32
}

impl Game {
    fn new() -> Game {
        let game_size = screen_width().min(screen_height());
        let pixel_size = game_size / SQUARES_SIZE as f32;
        return Game {
            game_size,
            pixel_size
        }
    }

    fn draw_pixel(&self, p: Point) {
        let x = p.0 as f32 * self.pixel_size;
        let y = p.1 as f32 * self.pixel_size;
        draw_rectangle(
            x, y,
            self.pixel_size, self.pixel_size, 
            BLUE
        );
    }
    
}

#[macroquad::main("Snake")]
async fn main() {

    loop {
        clear_background(LIGHTGRAY);
        let game = Game::new();
        game.draw_pixel((30, 31));
        next_frame().await
    }
}
