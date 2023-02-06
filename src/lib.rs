use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn setInterval(closure: &Closure<dyn FnMut()>, millis: u32) -> f64;
    fn cancelInterval(token: f64);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Interval {
    closure: Closure<dyn FnMut()>,
    token: f64,
}

impl Interval {
    pub fn new<F: 'static>(millis: u32, f: F) -> Interval
    where
        F: FnMut()
    {
        let closure = Closure::new(f);

        let token = setInterval(&closure, millis);

        Interval { closure, token }
    }
}

impl Drop for Interval {
    fn drop(&mut self) {
        cancelInterval(self.token);
    }
}

use std::{collections::VecDeque, ops::Add, str::FromStr};
#[derive(Debug, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
struct Pos(i32, i32);

impl Add<&Pos> for Pos {
    type Output = Pos;

    fn add(self, rhs: &Pos) -> Self::Output {
        return Pos(self.0 + rhs.0, self.1 + rhs.1);
    }
}

#[derive(Clone)]
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
            direction: Direction::Left,
        };
    }

    fn gen_rand_food(&mut self) {}

    fn walk(&mut self) {
        let head = self.snake.iter().last().unwrap();

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

        self.snake.push_back(new_head);
        self.snake.pop_front();
    }


    fn print_snake(&self) {
    }


    fn tick(&mut self) {
        self.print_snake();
        self.walk();
        log(format!("{:?}", self.snake.get(0)).as_ref());
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let width = 30;
    let height = 30;

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let game =  document.create_element("div")?;

    game.set_class_name("game");

    for x in 0..width {
        for y in 0..height  {
            let cell = document.create_element("div")?;
            cell.set_class_name("cell");
            cell.set_id(format!("{}x{}-cell", x, y).as_ref());
            cell.set_text_content(Some(&"⬜️"));

            game.append_child(&cell)?;
        }
    }

    body.append_child(&game)?;

    let mut snake_game = SnakeGame::new(width, height);
    Interval::new(100, move || {
       // snake_game.tick();

        document.query_selector(".cell")
					.unwrap()
                    .iter()
                    .for_each(|x| {
                        x.set_text_content(Some(&""));
                    });

        snake_game.snake.iter().for_each(|s| {
            document.get_element_by_id(format!("{}x{}-cell", s.0, s.1).as_ref()).unwrap().set_text_content(Some(&"🟩"));
        });
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::Pos;

    #[test]
    fn change_positon_increase() {
        let pos1 = Pos(1, 1);
        let pos2 = Pos(1, 1);

        assert_eq!(pos1 + &pos2, Pos(2, 2));
    }

    #[test]
    fn change_positon_decrease() {
        let pos1 = Pos(1, 1);
        let pos2 = Pos(-1, 1);

        assert_eq!(pos1 + &pos2, Pos(0, 2));
    }
}
