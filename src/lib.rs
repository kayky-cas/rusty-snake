mod snake;
use snake::{Pos, SnakeGame};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

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
        F: FnMut(),
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

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let width = 20;
    let height = 20;

    let mut snake_game = SnakeGame::new(width, height);

    Interval::new(500, move || {
        snake_game.tick();

        let document = web_sys::window()
            .expect("no global `window` exists")
            .document()
            .expect("should have a document on window");

        let body = document.body().expect("document should have a body");

        let game = document
            .get_element_by_id("game")
            .unwrap_throw()
            .dyn_into::<HtmlElement>()
            .unwrap_throw();

        game.set_text_content(Some(""));

        game.style().set_property("display", "grid").unwrap_throw();

        game.style()
            .set_property(
                "grid-template",
                &format!("repeat({}, auto) / repeat({}, auto)", width, height),
            )
            .unwrap_throw();

        for x in 0..width {
            for y in 0..height {
                let cell = document.create_element("div").unwrap_throw();
                cell.set_class_name("cell");
                cell.set_id(format!("{}x{}-cell", x, y).as_ref());

                let pos = Pos(x, y);

                cell.set_text_content({
                    if snake_game.snake.contains(&pos) {
                        Some("🤓")
                    } else {
                        Some(&"⬜️")
                    }
                });

                game.append_child(&cell).unwrap_throw();
            }
        }

        body.append_child(&game).unwrap_throw();
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::snake::Pos;

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
