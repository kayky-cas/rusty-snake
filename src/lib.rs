use js_sys::Math;
use wasm_bindgen::prelude::*;

mod snake;

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn main() {}

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
