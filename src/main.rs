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
}

impl Distribution<Tetromino> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Tetromino {
        match rng.gen_range(1..=7) {
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
    block: Block,
}

impl Game {
    fn new() -> Game {
        Game {
            font: load_ttf_font_from_bytes(include_bytes!("../assets/tetris-atari.ttf")).unwrap(),
            score: 0,
            lines: 0,
            level: 1,
            next_tetromino: rand::random(),
            block: Block {
                i: Texture2D::from_file_with_format(
                    include_bytes!("../assets/cyan_block.png"),
                    Some(ImageFormat::Png),
                ),
                j: Texture2D::from_file_with_format(
                    include_bytes!("../assets/gray_block.png"),
                    Some(ImageFormat::Png),
                ),
                l: Texture2D::from_file_with_format(
                    include_bytes!("../assets/gray_block.png"),
                    Some(ImageFormat::Png),
                ),
                o: Texture2D::from_file_with_format(
                    include_bytes!("../assets/gray_block.png"),
                    Some(ImageFormat::Png),
                ),
                s: Texture2D::from_file_with_format(
                    include_bytes!("../assets/gray_block.png"),
                    Some(ImageFormat::Png),
                ),
                t: Texture2D::from_file_with_format(
                    include_bytes!("../assets/gray_block.png"),
                    Some(ImageFormat::Png),
                ),
                z: Texture2D::from_file_with_format(
                    include_bytes!("../assets/gray_block.png"),
                    Some(ImageFormat::Png),
                ),
                gray: Texture2D::from_file_with_format(
                    include_bytes!("../assets/gray_block.png"),
                    Some(ImageFormat::Png),
                ),
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
                        block.i,
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
            Tetromino::L => {
                for i in 0..2 {
                    draw_texture_ex(
                        block.i,
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
            Tetromino::O => {
                for i in 0..2 {
                    draw_texture_ex(
                        block.i,
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
                        block.i,
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
                for i in -1..1 {
                    draw_texture_ex(
                        block.i,
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
                        block.i,
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
            Tetromino::Z => {
                for i in 1..3 {
                    draw_texture_ex(
                        block.i,
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
                        block.i,
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
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();

    loop {
        let x = get_time();
        game.next_tetromino = rand::random();
        while x + 0.25 > get_time() {
            game.draw_boarder();
            game.draw_text();

            Game::draw_tetromino(
                game.block,
                game.next_tetromino,
                screen_width() / 2.,
                screen_height() / 2. - (3. * BLOCK_SIZE),
            );

            next_frame().await
        }
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
