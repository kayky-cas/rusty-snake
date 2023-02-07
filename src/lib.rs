use std::{cell::RefCell, rc::Rc};

use snake::{Direction, Pos, SnakeGame};
use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlElement, Window};

mod snake;

#[wasm_bindgen]
extern "C" {
    fn setInterval(closure: &Closure<dyn FnMut()>, millis: u32) -> f64;

}

fn game_tick(game: Rc<RefCell<SnakeGame>>, millis: i32) {
    let tick = Closure::new(move || {
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

                let pos = Pos(x, y);

                let mut text = "⬜️";

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

    window()
        .unwrap_throw()
        .set_interval_with_callback_and_timeout_and_arguments_0(
            tick.as_ref().unchecked_ref(),
            millis,
        )
        .unwrap_throw();

    tick.forget();
}

fn keypress_event(game: Rc<RefCell<SnakeGame>>) {
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

    keypress_event.forget();
}

#[wasm_bindgen(start)]
pub fn main() {
    let width = 15;
    let height = 15;

    let game = Rc::new(RefCell::new(SnakeGame::new(width, height)));

    keypress_event(game.clone());
    game_tick(game.clone(), 100);
}
