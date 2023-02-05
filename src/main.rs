use std::{collections::VecDeque, ops::Add, str::FromStr};

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

struct Pos(i32, i32);

impl Add<&Pos> for Pos {
    type Output = Pos;

    fn add(self, rhs: &Pos) -> Self::Output {
        return Pos(self.0 + rhs.0, self.1 + rhs.1);
    }
}

struct SnakeGame {
    widht: i32,
    height: i32,
    snake: VecDeque<Pos>,
    food: Pos,
    direction: Direction,
}

impl SnakeGame {
    pub fn new(widht: i32, height: i32) -> Self {
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

    fn gen_rand_food(&mut self) {}

    fn walk(&mut self) {
        let head = self.snake.iter().last().unwrap();

        let new_head = match &self.direction {
            Direction::Up => Pos(1, 0),
            Direction::Down => Pos(-1, 0),
            Direction::Left => Pos(0, -1),
            Direction::Right => Pos(0, 1),
        } + head;

        self.snake.push_back(new_head);
        self.snake.pop_front();
    }

    fn tick(&mut self) {
        self.gen_rand_food();
        self.walk();
    }
}

fn main() -> anyhow::Result<()> {
    Ok(())
}
