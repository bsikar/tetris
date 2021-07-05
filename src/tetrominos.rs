#[derive(Copy, Clone)]
pub struct Tetromino<'a> {
    pub data: &'a [u8],
    pub sides: u8,
}

impl<'a> Tetromino<'a> {
    const fn new(data: &[u8], sides: u8) -> Tetromino {
        Tetromino { data, sides }
    }

    pub fn get_value(&self, row: u8, col: u8, rot: u8) -> u8 {
        let sides = self.sides;

        match rot {
            0 => self.data[(row * sides + col) as usize],
            1 => self.data[((sides - col - 1) * sides + row) as usize],
            2 => self.data[((sides - row - 1) * sides + (sides - col - 1)) as usize],
            3 => self.data[(col * sides + (sides - row - 1)) as usize],
            _ => 0,
        }
    }
}

const TETROMINO_1: [u8; 16] = [0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0];
const TETROMINO_2: [u8; 4] = [2; 4];
const TETROMINO_3: [u8; 9] = [0, 0, 0, 3, 3, 3, 0, 3, 0];
const TETROMINO_4: [u8; 9] = [0, 4, 4, 4, 4, 0, 0, 0, 0];
const TETROMINO_5: [u8; 9] = [5, 5, 0, 0, 5, 5, 0, 0, 0];
const TETROMINO_6: [u8; 9] = [6, 0, 0, 6, 6, 6, 0, 0, 0];
const TETROMINO_7: [u8; 9] = [0, 0, 7, 7, 7, 7, 0, 0, 0];

pub const TETROMINOS: &[Tetromino] = &[
    Tetromino::new(&TETROMINO_1, 4),
    Tetromino::new(&TETROMINO_2, 2),
    Tetromino::new(&TETROMINO_3, 3),
    Tetromino::new(&TETROMINO_4, 3),
    Tetromino::new(&TETROMINO_5, 3),
    Tetromino::new(&TETROMINO_6, 3),
    Tetromino::new(&TETROMINO_7, 3),
];
