#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use ::rand::{
    self,
    distributions::{Distribution, Standard},
    Rng,
};
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

#[derive(Debug, Copy, Clone)]
enum Tetromino {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl Tetromino {
    fn color(&self) -> Color {
        match *self {
            Tetromino::I => SKYBLUE,
            Tetromino::J => DARKBLUE,
            Tetromino::L => ORANGE,
            Tetromino::O => YELLOW,
            Tetromino::S => GREEN,
            Tetromino::T => PURPLE,
            Tetromino::Z => RED,
        }
    }
    fn texture(&self, block: Block) -> Texture2D {
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
}

impl Distribution<Tetromino> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Tetromino {
        match rng.gen_range(1..=7) {
            _ => Tetromino::S,
            1 => Tetromino::I,
            2 => Tetromino::J,
            3 => Tetromino::L,
            4 => Tetromino::O,
            5 => Tetromino::S,
            6 => Tetromino::T,
            7 => Tetromino::Z,
            _ => unreachable!(),
        }
    }
}

struct Game {
    font: Font,
    score: u32,
    lines: u32,
    level: u8,
    next_tetromino: Tetromino,
    current_tetromino: Tetromino,
    //current_tetromino_rotation: u8,
    block: Block,
    board: [[Option<Tetromino>; 10]; 23],
    //active_board: [[Option<Tetromino>; 10]; 23],
    time_since_move: f64,
}

impl Game {
    fn generate_board(current_tetromino: Tetromino) -> [[Option<Tetromino>; 10]; 23] {
        let mut board = [[None; 10]; 23];

        match current_tetromino {
            Tetromino::I => {
                for i in 3..=6 {
                    board[1][i] = Some(current_tetromino);
                }
            }
            Tetromino::J => {
                board[0][3] = Some(current_tetromino);
                for i in 3..=5 {
                    board[1][i] = Some(current_tetromino);
                }
            }
            Tetromino::L => {
                board[0][5] = Some(current_tetromino);
                for i in 3..=5 {
                    board[1][i] = Some(current_tetromino);
                }
            }
            Tetromino::O => {
                board[0][4] = Some(current_tetromino);
                board[0][5] = Some(current_tetromino);
                board[1][4] = Some(current_tetromino);
                board[1][5] = Some(current_tetromino);
            }
            Tetromino::S => {
                board[0][4] = Some(current_tetromino);
                board[0][5] = Some(current_tetromino);
                board[1][3] = Some(current_tetromino);
                board[1][4] = Some(current_tetromino);
            }
            Tetromino::T => {
                board[0][4] = Some(current_tetromino);
                for i in 3..6 {
                    board[1][i] = Some(current_tetromino);
                }
            }
            Tetromino::Z => {
                board[0][3] = Some(current_tetromino);
                board[0][4] = Some(current_tetromino);
                board[1][4] = Some(current_tetromino);
                board[1][5] = Some(current_tetromino);
            }
        }

        board
    }

    fn new() -> Game {
        let current_tetromino: Tetromino = rand::random();
        let cyan_block = Texture2D::from_file_with_format(
            include_bytes!("../assets/cyan_block.png"),
            Some(ImageFormat::Png),
        );
        let gray_block = Texture2D::from_file_with_format(
            include_bytes!("../assets/gray_block.png"),
            Some(ImageFormat::Png),
        );

        Game {
            font: load_ttf_font_from_bytes(include_bytes!("../assets/tetris-atari.ttf")).unwrap(),
            score: 0,
            lines: 0,
            level: 1,
            next_tetromino: rand::random(),
            current_tetromino,
            //current_tetromino_rotation: 0,
            time_since_move: 0.,
            board: Game::generate_board(current_tetromino),
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
    fn draw_tetromino(block: Block, tetromino: Tetromino, x: f32, y: f32) {
        match tetromino {
            Tetromino::I => {
                for i in -1..3 {
                    draw_texture_ex(
                        block.i,
                        x - (i as f32 * BLOCK_SIZE),
                        y + BLOCK_SIZE,
                        tetromino.color(),
                        DrawTextureParams {
                            dest_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                            ..Default::default()
                        },
                    );
                }
            }
            Tetromino::J => {
                for i in 0..1 {
                    draw_texture_ex(
                        block.j,
                        x - (2. * BLOCK_SIZE),
                        y,
                        tetromino.color(),
                        DrawTextureParams {
                            dest_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                            ..Default::default()
                        },
                    );
                }
                for i in 0..3 {
                    draw_texture_ex(
                        block.j,
                        x - (i as f32 * BLOCK_SIZE),
                        y + BLOCK_SIZE,
                        tetromino.color(),
                        DrawTextureParams {
                            dest_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                            ..Default::default()
                        },
                    );
                }
            }
            Tetromino::L => {
                for i in 0..2 {
                    draw_texture_ex(
                        block.l,
                        x,
                        y + (i as f32 * BLOCK_SIZE),
                        tetromino.color(),
                        DrawTextureParams {
                            dest_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                            ..Default::default()
                        },
                    );
                }
                for i in 1..3 {
                    draw_texture_ex(
                        block.l,
                        x - (i as f32 * BLOCK_SIZE),
                        y + BLOCK_SIZE,
                        tetromino.color(),
                        DrawTextureParams {
                            dest_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                            ..Default::default()
                        },
                    );
                }
            }
            Tetromino::O => {
                for i in 0..2 {
                    draw_texture_ex(
                        block.o,
                        x,
                        y + (i as f32 * BLOCK_SIZE),
                        tetromino.color(),
                        DrawTextureParams {
                            dest_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                            ..Default::default()
                        },
                    );
                }
                for i in 0..2 {
                    draw_texture_ex(
                        block.o,
                        x - BLOCK_SIZE,
                        y + (i as f32 * BLOCK_SIZE),
                        tetromino.color(),
                        DrawTextureParams {
                            dest_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                            ..Default::default()
                        },
                    );
                }
            }
            Tetromino::S => {
                for i in 1..3 {
                    draw_texture_ex(
                        block.s,
                        x - (i as f32 * BLOCK_SIZE),
                        y + BLOCK_SIZE,
                        tetromino.color(),
                        DrawTextureParams {
                            dest_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                            ..Default::default()
                        },
                    );
                }
                for i in -1..1 {
                    draw_texture_ex(
                        block.s,
                        x + (i as f32 * BLOCK_SIZE),
                        y,
                        tetromino.color(),
                        DrawTextureParams {
                            dest_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                            ..Default::default()
                        },
                    );
                }
            }
            Tetromino::T => {
                for i in 0..1 {
                    draw_texture_ex(
                        block.t,
                        x - BLOCK_SIZE,
                        y,
                        tetromino.color(),
                        DrawTextureParams {
                            dest_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                            ..Default::default()
                        },
                    );
                }
                for i in 0..3 {
                    draw_texture_ex(
                        block.t,
                        x - (i as f32 * BLOCK_SIZE),
                        y + BLOCK_SIZE,
                        tetromino.color(),
                        DrawTextureParams {
                            dest_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                            ..Default::default()
                        },
                    );
                }
            }
            Tetromino::Z => {
                for i in 1..3 {
                    draw_texture_ex(
                        block.z,
                        x - (i as f32 * BLOCK_SIZE),
                        y,
                        tetromino.color(),
                        DrawTextureParams {
                            dest_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                            ..Default::default()
                        },
                    );
                }
                for i in -1..1 {
                    draw_texture_ex(
                        block.z,
                        x + (i as f32 * BLOCK_SIZE),
                        y + BLOCK_SIZE,
                        tetromino.color(),
                        DrawTextureParams {
                            dest_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                            ..Default::default()
                        },
                    );
                }
            }
        }
    }
    fn draw_text(&self) {
        let font_size = 35;

        draw_text_ex(
            "NEXT",
            measure_text("NEXT", Some(self.font), font_size, 1.).width + 4.,
            (2. * BLOCK_SIZE) + 2.,
            TextParams {
                font_size,
                font: self.font,
                color: LIGHTGRAY,
                ..Default::default()
            },
        );
        draw_text_ex(
            "NEXT",
            measure_text("NEXT", Some(self.font), font_size, 1.).width,
            2. * BLOCK_SIZE,
            TextParams {
                font_size,
                font: self.font,
                ..Default::default()
            },
        );

        Game::draw_tetromino(
            self.block,
            self.next_tetromino,
            measure_text("NEXT", Some(self.font), font_size, 1.).width * 1.4,
            3. * BLOCK_SIZE,
        );

        draw_text_ex(
            "SCORE",
            1004.,
            2. * BLOCK_SIZE,
            TextParams {
                font_size,
                font: self.font,
                color: LIGHTGRAY,
                ..Default::default()
            },
        );
        draw_text_ex(
            "SCORE",
            1000.,
            2. * BLOCK_SIZE,
            TextParams {
                font_size,
                font: self.font,
                ..Default::default()
            },
        );
        draw_text_ex(
            self.score.to_string().as_str(),
            1004.,
            3. * BLOCK_SIZE,
            TextParams {
                font_size,
                font: self.font,
                color: BEIGE,
                ..Default::default()
            },
        );
        draw_text_ex(
            self.score.to_string().as_str(),
            1000.,
            3. * BLOCK_SIZE,
            TextParams {
                font_size,
                font: self.font,
                color: GOLD,
                ..Default::default()
            },
        );
        draw_text_ex(
            "LINES",
            1004.,
            5. * BLOCK_SIZE,
            TextParams {
                font_size,
                font: self.font,
                color: LIGHTGRAY,
                ..Default::default()
            },
        );
        draw_text_ex(
            "LINES",
            1000.,
            5. * BLOCK_SIZE,
            TextParams {
                font_size,
                font: self.font,
                ..Default::default()
            },
        );
        draw_text_ex(
            self.lines.to_string().as_str(),
            1004.,
            6. * BLOCK_SIZE,
            TextParams {
                font_size,
                font: self.font,
                color: BROWN,
                ..Default::default()
            },
        );
        draw_text_ex(
            self.lines.to_string().as_str(),
            1000.,
            6. * BLOCK_SIZE,
            TextParams {
                font_size,
                font: self.font,
                color: ORANGE,
                ..Default::default()
            },
        );
        draw_text_ex(
            "LEVEL",
            1004.,
            8. * BLOCK_SIZE,
            TextParams {
                font_size,
                font: self.font,
                color: LIGHTGRAY,
                ..Default::default()
            },
        );
        draw_text_ex(
            "LEVEL",
            1000.,
            8. * BLOCK_SIZE,
            TextParams {
                font_size,
                font: self.font,
                ..Default::default()
            },
        );
        draw_text_ex(
            self.level.to_string().as_str(),
            1004.,
            9. * BLOCK_SIZE,
            TextParams {
                font_size,
                font: self.font,
                color: MAROON,
                ..Default::default()
            },
        );
        draw_text_ex(
            self.level.to_string().as_str(),
            1000.,
            9. * BLOCK_SIZE,
            TextParams {
                font_size,
                font: self.font,
                color: RED,
                ..Default::default()
            },
        );
    }

    fn draw_boarder(&self) {
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
    }
    fn mv(&mut self) {
        if get_time() > self.time_since_move + 0.3 {
            self.time_since_move = get_time();
            let before = self.board;

            for (y, seg) in before.iter().enumerate().rev() {
                if y < 22 {
                    for (x, seg) in seg.iter().enumerate() {
                        match *seg {
                            Some(Tetromino::I) | Some(Tetromino::J) | Some(Tetromino::L)
                            | Some(Tetromino::O) | Some(Tetromino::T) => {
                                if let None = self.board[y + 1][x] {
                                    self.board[y + 1][x] = self.board[y][x];
                                    self.board[y][x] = None;
                                }
                            }
                            Some(Tetromino::S) => {
                                if x > 0 {
                                    if let None = self.board[y + 1][x] {
                                        self.board[y + 1][x] = self.board[y][x];
                                        self.board[y][x] = None;
                                    }
                                }
                            }
                            Some(Tetromino::Z) => {
                                if x < 9 {
                                    if let None = self.board[y + 1][x] {
                                        if let None = self.board[y + 1][x + 1] {
                                            self.board[y + 1][x] = self.board[y][x];
                                            self.board[y][x] = None;
                                        }
                                    }
                                }
                            }
                            None => {}
                        }
                    }
                }
            }
        }

        for (y, seg) in self.board.iter().enumerate() {
            for (x, seg) in seg.iter().enumerate() {
                match *seg {
                    Some(_) => {
                        draw_texture_ex(
                            self.current_tetromino.texture(self.block),
                            ((screen_width() / 2.) - (5. * BLOCK_SIZE)) + (x as f32 * BLOCK_SIZE),
                            (-3. * BLOCK_SIZE) + (y as f32 * BLOCK_SIZE),
                            self.current_tetromino.color(),
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

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();

    game.next_tetromino = rand::random();
    loop {
        game.draw_boarder();
        game.draw_text();
        game.mv();

        next_frame().await
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
