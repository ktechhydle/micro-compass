#![no_std]

use core::f32::consts::PI;
use libm::atan2f;
use rtt_target::rprintln;

const NORTH: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 1],
    [1, 1, 0, 0, 1],
    [1, 0, 1, 0, 1],
    [1, 0, 0, 1, 1],
    [1, 0, 0, 0, 1],
];
const NORTH_EAST: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 1],
    [0, 0, 0, 1, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
];
const NORTH_WEST: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 0],
    [0, 1, 0, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
];
const SOUTH: [[u8; 5]; 5] = [
    [1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0],
    [1, 1, 1, 1, 1],
    [0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1],
];
const SOUTH_EAST: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 0, 1, 0],
    [0, 0, 0, 0, 1],
];
const SOUTH_WEST: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 1, 0, 0, 0],
    [1, 0, 0, 0, 0],
];
const EAST: [[u8; 5]; 5] = [
    [1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0],
    [1, 1, 1, 1, 0],
    [1, 0, 0, 0, 0],
    [1, 1, 1, 1, 1],
];
const WEST: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 1, 0, 1],
    [0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0],
];

pub enum Direction {
    North([[u8; 5]; 5]),
    NorthEast([[u8; 5]; 5]),
    NorthWest([[u8; 5]; 5]),
    South([[u8; 5]; 5]),
    SouthEast([[u8; 5]; 5]),
    SouthWest([[u8; 5]; 5]),
    East([[u8; 5]; 5]),
    West([[u8; 5]; 5]),
}

pub fn calculate_direction(x: i32, y: i32) -> Direction {
    let theta = atan2f(y as f32, x as f32);
    let mut degrees = theta * (180_f32 / PI);

    // normalize angles (0 - 360)
    degrees = (degrees + 90.0) % 360.0;

    if degrees < 0.0 {
        degrees += 360.0;
    }

    rprintln!("{}", degrees);

    if degrees >= 337.5 || degrees < 22.5 {
        return Direction::North(NORTH);
    } else if degrees >= 22.5 && degrees < 67.5 {
        return Direction::NorthEast(NORTH_EAST);
    } else if degrees >= 67.5 && degrees < 112.5 {
        return Direction::East(EAST);
    } else if degrees >= 112.5 && degrees < 157.5 {
        return Direction::SouthEast(SOUTH_EAST);
    } else if degrees >= 157.5 && degrees < 202.5 {
        return Direction::South(SOUTH);
    } else if degrees >= 202.5 && degrees < 247.5 {
        return Direction::SouthWest(SOUTH_WEST);
    } else if degrees >= 247.5 && degrees < 292.5 {
        return Direction::West(WEST);
    } else {
        // 292.5 - 337.5
        return Direction::NorthWest(NORTH_WEST);
    }
}
