use crate::colors::*;
use crate::tetrominos::*;

use macroquad::input::{is_key_pressed, KeyCode};
use macroquad::shapes::draw_rectangle;
use rand::{thread_rng, Rng};

const WIDTH: u8 = 10;
const HEIGHT: u8 = 22;
const GRID_SIZE: f32 = 30.;

enum GamePhase {
    Play,
}

#[derive(Copy, Clone)]
struct PieceState {
    index: u8,
    offset_row: i16,
    offset_col: i16,
    rot: u8,
}

impl PieceState {
    fn new() -> Self {
        PieceState {
            index: thread_rng().gen_range(0..7),
            offset_row: -3,
            offset_col: 4,
            rot: 0,
        }
    }
}

pub struct Tetris {
    board: [u8; (WIDTH * HEIGHT) as usize],
    piece: PieceState,
    phase: GamePhase,
}

impl Tetris {
    pub fn new() -> Self {
        Tetris {
            board: [0; (WIDTH * HEIGHT) as usize],
            piece: PieceState::new(),
            phase: GamePhase::Play,
        }
    }

    pub fn update_game(&mut self) {
        match &self.phase {
            GamePhase::Play => self.update_game_play(),
        }
    }

    fn merge_piece(&mut self) {
        let tetromino = TETROMINOS[self.piece.index as usize];
        for row in 0..tetromino.sides {
            for col in 0..tetromino.sides {
                let value = tetromino.get_value(row, col, self.piece.rot);

                if value > 0 {
                    let row = row as i16 + self.piece.offset_row;
                    let col = col as i16 + self.piece.offset_col;
                    self.board[(row * WIDTH as i16 + col) as usize] = value;
                }
            }
        }
    }

    fn soft_drop(&mut self) {
        self.piece.offset_row += 1;

        if !self.check_piece_valid(self.piece) {
            self.piece.offset_row -= 1;
            self.merge_piece();
            self.piece = PieceState::new();
        }
    }

    fn update_game_play(&mut self) {
        self.soft_drop();

        let mut piece = self.piece;

        if is_key_pressed(KeyCode::Left) {
            piece.offset_col -= 1;
        }
        if is_key_pressed(KeyCode::Right) {
            piece.offset_col += 1;
        }
        if is_key_pressed(KeyCode::Up) {
            piece.rot = (piece.rot + 1) % 4;
        }

        if is_key_pressed(KeyCode::Down) {
            piece.offset_row += 1;
        }

        if self.check_piece_valid(piece) {
            self.piece = piece;
        }
    }

    fn board_value(&self, row: i16, col: i16) -> u8 {
        let x = (row * WIDTH as i16) as i16 + col;

        if x < 0 {
            return 0;
        }

        self.board[x as usize]
    }

    fn check_piece_valid(&mut self, piece: PieceState) -> bool {
        let tetromino = TETROMINOS[piece.index as usize];
        for row in 0..tetromino.sides {
            for col in 0..tetromino.sides {
                let value = tetromino.get_value(row, col, piece.rot);

                if value > 0 {
                    let board_row = piece.offset_row + row as i16;
                    let board_col = piece.offset_col + col as i16;

                    if board_row >= HEIGHT as i16 {
                        return false;
                    }
                    if board_col >= WIDTH as i16 {
                        return false;
                    }
                    if board_col < 0 {
                        return false;
                    }
                    if self.board_value(board_row, board_col) > 0 {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn draw_cell(row: i16, col: i16, value: u8) {
        let base_color = BASE_COLORS[value as usize];
        let light_color = LIGHT_COLORS[value as usize];
        let dark_color = DARK_COLORS[value as usize];

        let edge = GRID_SIZE / 8.;

        let x = col as f32 * GRID_SIZE;
        let y = row as f32 * GRID_SIZE;

        draw_rectangle(x, y, GRID_SIZE, GRID_SIZE, dark_color);
        draw_rectangle(x + edge, y, GRID_SIZE - edge, GRID_SIZE - edge, light_color);
        draw_rectangle(
            x + edge,
            y + edge,
            GRID_SIZE - edge * 2.,
            GRID_SIZE - edge * 2.,
            base_color,
        );
    }

    fn draw_piece(&self) {
        let tetromino = TETROMINOS[self.piece.index as usize];
        for row in 0..tetromino.sides {
            for col in 0..tetromino.sides {
                let value = tetromino.get_value(row, col, self.piece.rot);
                if value > 0 {
                    Tetris::draw_cell(
                        row as i16 + self.piece.offset_row,
                        col as i16 + self.piece.offset_col,
                        value,
                    );
                }
            }
        }
    }

    fn draw_board(&self) {
        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                let value = self.board_value(row as i16, col as i16);
                if value > 0 {
                    Tetris::draw_cell(row as i16, col as i16, value);
                }
            }
        }
    }

    pub fn render_game(&mut self) {
        self.draw_piece();
        self.draw_board();
    }
}
