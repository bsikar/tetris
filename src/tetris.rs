use crate::tetromino::*;
use ::rand;
use macroquad::prelude::*;

const BLOCK_SIZE: f32 = 40.;

#[derive(Debug, Copy, Clone)]
struct Block {
    i: Texture2D,
    j: Texture2D,
    l: Texture2D,
    o: Texture2D,
    s: Texture2D,
    t: Texture2D,
    z: Texture2D,
    gray: Texture2D,
}

impl Block {
    fn texture_from_tetromino(&self, tetromino: Tetromino) -> Texture2D {
        use Tetromino::*;
        match tetromino {
            I => self.i,
            J => self.j,
            L => self.l,
            O => self.o,
            S => self.s,
            T => self.t,
            Z => self.z,
        }
    }
}
/*
    pub fn texture(&self, block: Tetris::Block) -> macroquad::texture::Texture2D {
        match *self {
            Tetromino::I => block.i,
            Tetromino::J => block.j,
            Tetromino::L => block.l,
            Tetromino::O => block.o,
            Tetromino::S => block.s,
            Tetromino::T => block.t,
            Tetromino::Z => block.z,
        }
    }

*/

pub struct Tetris {
    current_tetromino: Tetromino,
    //current_tetromino_rotation: u8,
    block: Block,
    board: [[Option<Color>; 10]; 23],
    //active_board: [[Option<Tetromino>; 10]; 23],
    //time_since_move: f64,
}

impl Tetris {
    pub fn new() -> Self {
        let current_tetromino: Tetromino = rand::random();
        let cyan_block = Texture2D::from_file_with_format(
            include_bytes!("../assets/cyan_block.png"),
            Some(ImageFormat::Png),
        );
        let gray_block = Texture2D::from_file_with_format(
            include_bytes!("../assets/gray_block.png"),
            Some(ImageFormat::Png),
        );

        Tetris {
            current_tetromino,
            //current_tetromino_rotation: 0,
            //time_since_move: 0.,
            board: Tetris::fill_board(&current_tetromino),
            block: Block {
                i: cyan_block,
                j: gray_block,
                l: gray_block,
                o: gray_block,
                s: gray_block,
                t: gray_block,
                z: gray_block,
                gray: gray_block,
            },
        }
    }

    fn fill_board(current_tetromino: &Tetromino) -> [[Option<Color>; 10]; 23] {
        let mut board = [[None; 10]; 23];
        let x = current_tetromino.get_shape()[0];
        for (i, x) in x.iter().enumerate() {
            for (o, x) in x.iter().enumerate() {
                board[i][5 + o] = *x;
            }
        }
        board
    }

    pub fn draw_boarder(&self) {
        for i in 0..20 {
            draw_texture_ex(
                self.block.gray,
                900.,
                i as f32 * BLOCK_SIZE,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                    ..Default::default()
                },
            );
            draw_texture_ex(
                self.block.gray,
                460.,
                i as f32 * BLOCK_SIZE,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                    ..Default::default()
                },
            );
        }

        for (y, seg) in self.board.iter().enumerate() {
            for (x, seg) in seg.iter().enumerate() {
                match *seg {
                    Some(color) => {
                        draw_texture_ex(
                            self.block.texture_from_tetromino(self.current_tetromino),
                            ((screen_width() / 2.) - (7. * BLOCK_SIZE)) + (x as f32 * BLOCK_SIZE),
                            (-3. * BLOCK_SIZE) + ((y + 5) as f32 * BLOCK_SIZE),
                            color,
                            DrawTextureParams {
                                dest_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                                ..Default::default()
                            },
                        );
                    }
                    _ => {}
                }
            }
        }
    }
}
