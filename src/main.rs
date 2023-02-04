use std::collections::VecDeque;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Pos(u32, u32);

struct SnakeGame {
    widht: u32,
    height: u32,
    snake: VecDeque<Pos>,
    food: Pos,
    direction: Direction,
}

impl SnakeGame {
    fn new(widht: u32, height: u32) -> Self {
        let half_with = widht / 2;
        let half_height = height / 2;

        let head = Pos(half_with, half_height);

        return Self {
            widht,
            height,
            snake: vec![head].into(),
            food: Pos(0, 0),
            direction: Direction::Right,
        };
    }
}

fn main() {
    println!("Hello, world!");
}
