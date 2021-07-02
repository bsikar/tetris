mod colors;
mod tetris;
mod tetrominos;

use macroquad::input::{is_key_pressed, KeyCode};
use macroquad::window::{next_frame, Conf};

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = tetris::Tetris::new();

    while !is_key_pressed(KeyCode::Escape) {
        game.update_game();
        game.render_game();

        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Tetris".to_owned(),
        window_width: 300,
        window_height: 660,
        window_resizable: false,
        ..Default::default()
    }
}
