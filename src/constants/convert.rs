use crate::{constants::Part, objects::BodyPart};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/part.js")]
extern "C" {
    pub fn part_num_for_bodypart(body_part: &BodyPart) -> Part;
}
