use std::{collections::VecDeque, str::FromStr};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "KeyS" | "ArrowDown" => Ok(Direction::Down),
            "KeyW" | "ArrowUp" => Ok(Direction::Up),
            "KeyA" | "ArrowLeft" => Ok(Direction::Left),
            "KeyD" | "ArrowRight" => Ok(Direction::Right),
            _ => anyhow::bail!("Invalid key code!"),
        }
    }
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

fn main() -> anyhow::Result<()> {
    Ok(())
}
