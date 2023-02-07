use std::{cell::RefCell, rc::Rc};

use snake::{Direction, Pos, SnakeGame};
use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlElement};

mod snake;

#[wasm_bindgen]
extern "C" {
    fn setInterval(closure: &Closure<dyn FnMut()>, millis: u32) -> f64;
    fn cancelInterval(token: f64);

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
        // Construct a new closure.
        let closure = Closure::new(f);

        // Pass the closure to JS, to run every n milliseconds.
        let token = setInterval(&closure, millis);

        Interval { closure, token }
    }
}

// When the Interval is destroyed, cancel its `setInterval` timer.
impl Drop for Interval {
    fn drop(&mut self) {
        cancelInterval(self.token);
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    let width = 15;
    let height = 15;

    let game_ref = Rc::new(RefCell::new(SnakeGame::new(width, height)));

    let game = game_ref.clone();
    let keypress_event = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
        let direction = event.code().parse::<Direction>();

        if let Ok(direction) = direction {
            game.borrow_mut().direction = direction;
        }
    });

    window()
        .unwrap_throw()
        .add_event_listener_with_callback("keypress", keypress_event.as_ref().unchecked_ref())
        .unwrap_throw();

    let game = game_ref.clone();
    Interval::new(100, move || {
        let document = window().unwrap_throw().document().unwrap_throw();

        let board = document
            .get_element_by_id("board")
            .unwrap_throw()
            .dyn_into::<HtmlElement>()
            .unwrap_throw();

        board.style().set_property("display", "grid").unwrap_throw();

        board
            .style()
            .set_property(
                "grid-template",
                &format!(
                    "repeat({}, auto) / repeat({}, auto)",
                    game.borrow().width,
                    game.borrow().height
                ),
            )
            .unwrap_throw();

        board.set_text_content(Some(""));

        for x in 0..game.borrow().width {
            for y in 0..game.borrow().height {
                let cell = document
                    .create_element("div")
                    .unwrap_throw()
                    .dyn_into::<HtmlElement>()
                    .unwrap_throw();

                let mut text = "⬜️";

                let pos = Pos(x, y);

                if pos == game.borrow().food {
                    text = "🍎";
                } else if game.borrow().snake.get(0).unwrap_throw().clone() == pos {
                    text = "🤓"
                } else if game.borrow().snake.contains(&pos) {
                    text = "🟨"
                }

                cell.set_inner_text(text);

                board.append_child(&cell).unwrap_throw();
            }
        }

        game.borrow_mut().tick();
    });
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
