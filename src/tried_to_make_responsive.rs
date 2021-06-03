#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use ::rand::{self, Rng};
use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    let font = load_ttf_font_from_bytes(include_bytes!("../assets/tetris-atari.ttf")).unwrap();
    let gray_block = Texture2D::from_file_with_format(
        include_bytes!("../assets/gray_block.png"),
        Some(ImageFormat::Png),
    );
    let cyan_block = Texture2D::from_file_with_format(
        include_bytes!("../assets/cyan_block.png"),
        Some(ImageFormat::Png),
    );

    loop {
        draw_boarder(gray_block);
        //draw_text(font);
        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Tetris".to_owned(),
        window_width: 1400,
        window_height: 800,
        //window_resizable: false,
        ..Default::default()
    }
}

fn draw_text(font: Font) {
    let block_size;
    if screen_width() < screen_height() {
        block_size = screen_width() / 20.;
    } else {
        block_size = screen_height() / 20.;
    }

    //let font_size = (((screen_width() / 2.) - (6. * block_size)) / 2.) as u16;
    let font_size = (screen_width() / block_size) as u16;

    draw_text_ex(
        "NEXT",
        //(measure_text("NEXT", Some(font), font_size, 1.).width / 1.5) + 2.,
        measure_text("NEXT", Some(font), font_size, 1.).width,
        (2. * block_size) + 2.,
        TextParams {
            font_size,
            font,
            color: LIGHTGRAY,
            ..Default::default()
        },
    );
    /*
    draw_text_ex(
        "NEXT",
        measure_text("NEXT", Some(font), font_size, 1.).width / 1.5,
        2. * block_size,
        TextParams {
            font_size,
            font,
            ..Default::default()
        },
    );
    */
}

fn draw_boarder(block: Texture2D) {
    let block_size;
    if screen_width() < screen_height() {
        block_size = screen_width() / 20.;
    } else {
        block_size = screen_height() / 20.;
    }

    for i in 0..20 {
        draw_texture_ex(
            block,
            (screen_width() / 2.) + (5. * block_size),
            i as f32 * block_size,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(block_size, block_size)),
                ..Default::default()
            },
        );
        draw_texture_ex(
            block,
            (screen_width() / 2.) - (6. * block_size),
            i as f32 * block_size,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(block_size, block_size)),
                ..Default::default()
            },
        );
    }
}

/*
    draw_text_ex(
        "SCORE",
        200.0,
        200.0,
        TextParams {
            font_size: 200,
            font,
            ..Default::default()
        },
    );
    draw_texture_ex(
        block,
        screen_width() / 2.,
        screen_height() / 2.,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(500., 500.)),
            ..Default::default()
        },
    );
*/
