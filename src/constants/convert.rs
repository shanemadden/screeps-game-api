use crate::{
    constants::{Part, PartInt},
    objects::BodyPart,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/part.js")]
extern "C" {
    pub fn part_num_to_str(num: PartInt) -> Part;
    pub fn part_str_to_num(str: Part) -> PartInt;
    pub fn part_num_for_bodypart(body_part: &BodyPart) -> PartInt;
}
