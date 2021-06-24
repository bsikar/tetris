mod tetris;
mod tetromino;

use macroquad::window::{next_frame, Conf};
use tetris::*;

#[macroquad::main(window_conf)]
async fn main() {
    let game = Tetris::new();

    loop {
        game.draw_boarder();

        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Tetris".to_owned(),
        window_width: 1400,
        window_height: 800,
        window_resizable: false,
        ..Default::default()
    }
}
