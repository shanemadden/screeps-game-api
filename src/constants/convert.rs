use js_sys::Array;
use wasm_bindgen::prelude::*;

use crate::{constants::Part, objects::BodyPart};

#[wasm_bindgen(module = "/js/part.js")]
extern "C" {
    pub(crate) fn part_num_for_bodypart(body_part: &BodyPart) -> Part;
    pub(crate) fn part_array_num_to_str(part_array: &[u8]) -> Array;
}

#[cfg(test)]
mod test {
    use wasm_bindgen_test::*;

    use super::part_array_num_to_str;

    #[wasm_bindgen_test]
    pub fn parts_to_array() {
        // work, carry, move
        let body = [2, 1, 0];
        let array = part_array_num_to_str(&body);
        assert_eq!(array.get(0), "work");
        
    }
}
