use ::rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use macroquad::color::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Tetromino {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl Tetromino {
    pub fn get_shape(&self) -> [[[Option<Color>; 4]; 4]; 4] {
        use Tetromino::*;

        match *self {
            I => [
                [[None; 4], [Some(SKYBLUE); 4], [None; 4], [None; 4]],
                [[None, None, Some(SKYBLUE), None]; 4],
                [[None; 4], [None; 4], [Some(SKYBLUE); 4], [None; 4]],
                [[None, Some(SKYBLUE), None, None]; 4],
            ],
            J => [
                [
                    [None; 4],
                    [None, Some(DARKBLUE), None, None],
                    [None, Some(DARKBLUE), Some(DARKBLUE), Some(DARKBLUE)],
                    [None; 4],
                ],
                [
                    [None; 4],
                    [None, None, Some(DARKBLUE), Some(DARKBLUE)],
                    [None, None, Some(DARKBLUE), None],
                    [None, None, Some(DARKBLUE), None],
                ],
                [
                    [None; 4],
                    [None; 4],
                    [None, Some(DARKBLUE), Some(DARKBLUE), Some(DARKBLUE)],
                    [None, None, None, Some(DARKBLUE)],
                ],
                [
                    [None; 4],
                    [None, None, Some(DARKBLUE), None],
                    [None, None, Some(DARKBLUE), None],
                    [None, Some(DARKBLUE), Some(DARKBLUE), None],
                ],
            ],
            L => [
                [
                    [None; 4],
                    [None, None, None, Some(ORANGE)],
                    [None, Some(ORANGE), Some(ORANGE), Some(ORANGE)],
                    [None; 4],
                ],
                [
                    [None; 4],
                    [None, Some(ORANGE), None, None],
                    [None, Some(ORANGE), None, None],
                    [None, None, Some(ORANGE), Some(ORANGE)],
                ],
                [
                    [None; 4],
                    [None; 4],
                    [None, Some(ORANGE), Some(ORANGE), Some(ORANGE)],
                    [None, Some(ORANGE), None, None],
                ],
                [
                    [None; 4],
                    [Some(ORANGE), Some(ORANGE), None, None],
                    [None, None, Some(ORANGE), None],
                    [None, None, Some(ORANGE), None],
                ],
            ],
            O => {
                [[
                    [None; 4],
                    [None, Some(YELLOW), Some(YELLOW), None],
                    [None, Some(YELLOW), Some(YELLOW), None],
                    [None; 4],
                ]; 4]
            }
            S => [
                [
                    [None; 4],
                    [None, None, Some(GREEN), Some(GREEN)],
                    [None, Some(GREEN), Some(GREEN), None],
                    [None; 4],
                ],
                [
                    [None; 4],
                    [None, None, Some(GREEN), None],
                    [None, None, Some(GREEN), Some(GREEN)],
                    [None, None, None, Some(GREEN)],
                ],
                [
                    [None; 4],
                    [None; 4],
                    [None, None, Some(GREEN), Some(GREEN)],
                    [None, Some(GREEN), Some(GREEN), None],
                ],
                [
                    [None; 4],
                    [None, Some(GREEN), None, None],
                    [None, Some(GREEN), Some(GREEN), None],
                    [None, None, Some(GREEN), None],
                ],
            ],
            T => [
                [
                    [None; 4],
                    [None, None, Some(PURPLE), None],
                    [None, Some(PURPLE), Some(PURPLE), Some(PURPLE)],
                    [None; 4],
                ],
                [
                    [None; 4],
                    [None, None, Some(PURPLE), None],
                    [None, None, Some(PURPLE), Some(PURPLE)],
                    [None, None, Some(PURPLE), None],
                ],
                [
                    [None; 4],
                    [None; 4],
                    [None, Some(PURPLE), Some(PURPLE), Some(PURPLE)],
                    [None, None, Some(PURPLE), None],
                ],
                [
                    [None; 4],
                    [None, None, Some(PURPLE), None],
                    [None, Some(PURPLE), Some(PURPLE), None],
                    [None, None, Some(PURPLE), None],
                ],
            ],
            Z => [
                [
                    [None; 4],
                    [None, Some(RED), Some(RED), None],
                    [None, None, Some(RED), Some(RED)],
                    [None; 4],
                ],
                [
                    [None; 4],
                    [None, None, None, Some(RED)],
                    [None, None, Some(RED), Some(RED)],
                    [None, None, Some(RED), None],
                ],
                [
                    [None; 4],
                    [None; 4],
                    [None, Some(RED), Some(RED), None],
                    [None, None, Some(RED), Some(RED)],
                ],
                [
                    [None; 4],
                    [None, None, Some(RED), None],
                    [None, Some(RED), Some(RED), None],
                    [None, Some(RED), None, None],
                ],
            ],
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
