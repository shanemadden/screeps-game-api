use js_sys::Array;
use wasm_bindgen::prelude::*;

use crate::{constants::Part, objects::BodyPart};

#[wasm_bindgen(module = "/js/part.js")]
extern "C" {
    pub fn part_num_for_bodypart(body_part: &BodyPart) -> Part;
    pub fn part_array_num_to_str(part_array: Box<[Part]>) -> Array;
}
