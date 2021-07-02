use macroquad::color::Color;
use macroquad::color_u8;

pub const BASE_COLORS: [Color; 8] = [
    color_u8!(0x28, 0x28, 0x28, 0xFF),
    color_u8!(0x2D, 0x99, 0x99, 0xFF),
    color_u8!(0x99, 0x99, 0x2D, 0xFF),
    color_u8!(0x99, 0x2D, 0x99, 0xFF),
    color_u8!(0x2D, 0x99, 0x51, 0xFF),
    color_u8!(0x99, 0x2D, 0x2D, 0xFF),
    color_u8!(0x2D, 0x63, 0x99, 0xFF),
    color_u8!(0x99, 0x63, 0x2D, 0xFF),
];

pub const LIGHT_COLORS: [Color; 8] = [
    color_u8!(0x28, 0x28, 0x28, 0xFF),
    color_u8!(0x44, 0xE5, 0xE5, 0xFF),
    color_u8!(0xE5, 0xE5, 0x44, 0xFF),
    color_u8!(0xE5, 0x44, 0xE5, 0xFF),
    color_u8!(0x44, 0xE5, 0x7A, 0xFF),
    color_u8!(0xE5, 0x44, 0x44, 0xFF),
    color_u8!(0x44, 0x95, 0xE5, 0xFF),
    color_u8!(0xE5, 0x95, 0x44, 0xFF),
];

pub const DARK_COLORS: [Color; 8] = [
    color_u8!(0x28, 0x28, 0x28, 0xFF),
    color_u8!(0x1E, 0x66, 0x66, 0xFF),
    color_u8!(0x66, 0x66, 0x1E, 0xFF),
    color_u8!(0x66, 0x1E, 0x66, 0xFF),
    color_u8!(0x1E, 0x66, 0x36, 0xFF),
    color_u8!(0x66, 0x1E, 0x1E, 0xFF),
    color_u8!(0x1E, 0x42, 0x66, 0xFF),
    color_u8!(0x66, 0x42, 0x1E, 0xFF),
];
