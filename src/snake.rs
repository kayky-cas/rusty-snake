use js_sys::Math;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

use std::{collections::VecDeque, ops::Add, str::FromStr};

#[derive(Debug, Clone)]
pub enum Direction {
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

#[derive(Debug, PartialEq, Clone)]
pub struct Pos(pub i32, pub i32);

impl Add<&Pos> for Pos {
    type Output = Pos;

    fn add(self, rhs: &Pos) -> Self::Output {
        return Pos(self.0 + rhs.0, self.1 + rhs.1);
    }
}

#[derive(Clone)]
pub struct SnakeGame {
    pub widht: i32,
    pub height: i32,
    pub snake: VecDeque<Pos>,
    pub food: Pos,
    pub direction: Direction,
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
            direction: Direction::Left,
        };
    }

    fn gen_rand_food(&mut self) {
        loop {
            let x = Math::floor(Math::random() * self.widht as f64) as i32;
            let y = Math::floor(Math::random() * self.widht as f64) as i32;

            let food_pos = Pos(x, y);

            if !self.snake.contains(&food_pos) {
                self.food = food_pos;
                return;
            }
        }
    }

    fn eat(&mut self) {
        let head = self.snake.get(0).unwrap().clone();

        if head == self.food {
            self.gen_rand_food();

            let new_head = match &self.direction {
                Direction::Up => Pos(1, 0),
                Direction::Down => Pos(-1, 0),
                Direction::Left => Pos(0, -1),
                Direction::Right => Pos(0, 1),
            } + &head;

            self.snake.push_front(new_head);
        }
    }

    fn walk(&mut self) {
        let head = self.snake.get(0).unwrap();

        let mut new_head = match &self.direction {
            Direction::Up => Pos(1, 0),
            Direction::Down => Pos(-1, 0),
            Direction::Left => Pos(0, -1),
            Direction::Right => Pos(0, 1),
        } + head;

        if new_head.0 >= self.widht || new_head.0 < 0 {
            new_head.0 = new_head.0.rem_euclid(self.widht);
        }

        if new_head.1 >= self.height || new_head.1 < 0 {
            new_head.1 = new_head.1.rem_euclid(self.height);
        }

        self.snake.push_front(new_head);
        self.snake.pop_back();
    }

    pub fn tick(&mut self) {
        self.walk();
        self.eat();
        log(format!("{:?}", self.snake.get(0)).as_ref());
    }
}
