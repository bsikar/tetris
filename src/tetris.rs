use crate::colors::*;
use crate::tetrominos::*;
use macroquad::color_u8;
use macroquad::prelude::Color;
use macroquad::rand;

use macroquad::input::{is_key_down, is_key_pressed, KeyCode};
use macroquad::shapes::{draw_rectangle, draw_rectangle_lines};
use macroquad::time::get_time;

const WIDTH: u8 = 10;
const HEIGHT: u8 = 22;
const GRID_SIZE: f32 = 30.;
const DELAYS: [f64; 10] = [0.25, 0.22, 0.20, 0.18, 0.17, 0.16, 0.15, 0.14, 0.13, 0.125];

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
            index: rand::gen_range(0, 6),
            offset_row: -3,
            offset_col: 4,
            rot: 0,
        }
    }
}

pub struct Tetris {
    board: [u8; (WIDTH * HEIGHT) as usize],
    piece: PieceState,
    tld: f64,
    num_removed: u8,
    level: u8,
    dt: f64,
}

impl Tetris {
    pub fn new() -> Self {
        rand::srand(macroquad::miniquad::date::now() as _);
        Tetris {
            board: [0; (WIDTH * HEIGHT) as usize],
            piece: PieceState::new(),
            tld: get_time(),
            num_removed: 0,
            level: 0,
            dt: get_time(),
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
                    let index = (row * WIDTH as i16 + col) as usize;

                    if index < self.board.len() {
                        self.board[(row * WIDTH as i16 + col) as usize] = value;
                    } else {
                        *self = Tetris::new();
                    }
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

    fn hard_drop(&mut self) {
        self.piece = self.get_lowest();
    }

    fn is_row_filled(&mut self) -> Option<usize> {
        let x = [true; WIDTH as usize];
        (0..(HEIGHT as usize)).find(|&n| self
                .board
                .iter()
                .skip(n * WIDTH as usize)
                .take(WIDTH as usize)
                .map(|x| *x != 0)
                .collect::<Vec<_>>()
                == x)
    }

    pub fn update_game(&mut self) {
        if is_key_pressed(KeyCode::Space) {
            self.hard_drop();
            self.merge_piece();
            self.piece = PieceState::new();
        }

        let mut piece = self.piece;

        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
            piece.rot = (piece.rot + 1) % 4;
        }
        if (is_key_down(KeyCode::Left) || is_key_down(KeyCode::A)) && self.dt + 0.1 < get_time() {
            piece.offset_col -= 1;
            self.dt = get_time();
        }
        if (is_key_down(KeyCode::Right) || is_key_down(KeyCode::D)) && self.dt + 0.1 < get_time() {
            piece.offset_col += 1;
            self.dt = get_time();
        }
        if self.check_piece_valid(piece) {
            self.piece = piece;
        }

        if self.tld + DELAYS[self.level as usize] < get_time() {
            if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
                piece.offset_row += 1;
            }
            if self.check_piece_valid(piece) {
                self.piece = piece;
            }
            self.soft_drop();

            self.tld = get_time();
        }

        self.clear_lines();
    }

    fn clear_lines(&mut self) {
        if let Some(row) = self.is_row_filled() {
            self.num_removed += 1;
            self.num_removed %= 10;
            if self.num_removed == 0 {
                self.level += 1;
            }
            for n in 0..(WIDTH as usize) {
                self.board[(row * WIDTH as usize) + n] = 0;
            }
            for row in (1..=row).rev() {
                for n in (0..(WIDTH as usize)).rev() {
                    self.board[(row * WIDTH as usize) + n] =
                        self.board[((row - 1) * WIDTH as usize) + n];
                }
            }
        }
    }

    fn board_value(&self, row: i16, col: i16) -> u8 {
        let x = (row * WIDTH as i16) + col;

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

    fn draw_cell(row: i16, col: i16, value: u8, filled: bool) {
        let base_color = BASE_COLORS[value as usize];
        let light_color = LIGHT_COLORS[value as usize];
        let dark_color = DARK_COLORS[value as usize];

        let edge = GRID_SIZE / 8.;

        let x = col as f32 * GRID_SIZE + 50.0;
        let y = row as f32 * GRID_SIZE + 50.0;

        if filled {
            draw_rectangle(x, y, GRID_SIZE, GRID_SIZE, dark_color);
            draw_rectangle(x + edge, y, GRID_SIZE - edge, GRID_SIZE - edge, light_color);
            draw_rectangle(
                x + edge,
                y + edge,
                GRID_SIZE - edge * 2.,
                GRID_SIZE - edge * 2.,
                base_color,
            );
        } else {
            draw_rectangle_lines(x, y, GRID_SIZE, GRID_SIZE, edge, dark_color);
        }
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
                        true,
                    );
                }
            }
        }
    }

    fn get_lowest(&mut self) -> PieceState {
        let mut piece1 = self.piece;
        for i in 0..HEIGHT {
            let piece2 = PieceState {
                offset_row: i as i16,
                ..self.piece
            };

            if self.check_piece_valid(piece2) {
                piece1 = piece2;
            } else {
                break;
            }
        }
        piece1
    }

    fn draw_ghost(&mut self) {
        let piece = self.get_lowest();

        let tetromino = TETROMINOS[piece.index as usize];
        for row in 0..tetromino.sides {
            for col in 0..tetromino.sides {
                let value = tetromino.get_value(row, col, piece.rot);
                if value > 0 {
                    Tetris::draw_cell(
                        row as i16 + piece.offset_row,
                        col as i16 + piece.offset_col,
                        value,
                        false,
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
                    Tetris::draw_cell(row as i16, col as i16, value, true);
                }
            }
        }
    }

    pub fn render_game(&mut self) {
        // draw an outer boarder
        let color = color_u8!(0x28, 0x28, 0x28, 0xFF);
        draw_rectangle(50.0, 0.0, 300.0, 710.0, color);
        self.draw_ghost();
        self.draw_piece();
        self.draw_board();
    }
}
