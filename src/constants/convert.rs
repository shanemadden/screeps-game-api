use js_sys::{Array, Map};
use wasm_bindgen::prelude::*;

use crate::{constants::Part, objects::BodyPart};

thread_local! {
  static PART_NUM_TO_STR_MAP: js_sys::Map = {
    let map = js_sys::Map::new();
    map
      .set(&JsValue::from(Part::Move as u8), &JsValue::from_str("move"))
      .set(&JsValue::from(Part::Work as u8), &JsValue::from_str("work"))
      .set(&JsValue::from(Part::Carry as u8), &JsValue::from_str("carry"))
      .set(&JsValue::from(Part::Attack as u8), &JsValue::from_str("attack"))
      .set(&JsValue::from(Part::RangedAttack as u8), &JsValue::from_str("ranged_attack"))
      .set(&JsValue::from(Part::Tough as u8), &JsValue::from_str("tough"))
      .set(&JsValue::from(Part::Heal as u8), &JsValue::from_str("heal"))
      .set(&JsValue::from(Part::Claim as u8), &JsValue::from_str("claim"))
  };
}

#[wasm_bindgen(module = "/js/part.js")]
extern "C" {
    pub(crate) fn part_num_for_bodypart(map: &Map, body_part: &BodyPart) -> Part;
    pub(crate) fn part_array_num_to_str(map: &Map, part_array: &[u8]) -> Array;
}

#[cfg(test)]
mod test {
    use wasm_bindgen_test::*;

    use super::{part_array_num_to_str, PART_NUM_TO_STR_MAP};

    #[wasm_bindgen_test]
    pub fn parts_to_array() {
        // work, carry, move, move
        PART_NUM_TO_STR_MAP.with(|map| {
            let body = [1, 2, 0, 0];
            let array = part_array_num_to_str(&map, &body);
            assert_eq!(array.length(), 4);
            assert_eq!(array.get(0), "work");
            assert_eq!(array.get(1), "carry");
            assert_eq!(array.get(2), "move");
            assert_eq!(array.get(3), "move");
        })
    }
}
