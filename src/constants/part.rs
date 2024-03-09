use enum_iterator::Sequence;
use js_sys::{Array, Map};
use num_derive::FromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};
use wasm_bindgen::prelude::*;

use crate::objects::BodyPart;

/// Translates body part type and `BODYPARTS_ALL` constants
#[wasm_bindgen]
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Hash,
    FromPrimitive,
    Serialize_repr,
    Deserialize_repr,
    Sequence,
)]
#[repr(u8)]
// keep integer representations in sync with js/part.js
pub enum Part {
    Move = 0,
    Work = 1,
    Carry = 2,
    Attack = 3,
    RangedAttack = 4,
    Tough = 5,
    Heal = 6,
    Claim = 7,
}

impl Part {
    /// Translates the `BODYPART_COST` constant.
    #[inline]
    pub const fn cost(self) -> u32 {
        match self {
            Part::Move => 50,
            Part::Work => 100,
            Part::Carry => 50,
            Part::Attack => 80,
            Part::RangedAttack => 150,
            Part::Tough => 10,
            Part::Heal => 250,
            Part::Claim => 600,
        }
    }
}

thread_local! {
    pub static PART_NUM_TO_STR_MAP: js_sys::Map = {
        js_sys::Map::new()
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
    fn bodypart_to_part_num(map: &Map, body_part: &BodyPart) -> Part;
    fn part_nums_to_str_array(map: &Map, part_array: &[u8]) -> Array;
}

#[cfg(test)]
mod test {
    use wasm_bindgen_test::*;

    use super::{part_nums_to_str_array, PART_NUM_TO_STR_MAP};

    #[wasm_bindgen_test]
    pub fn parts_to_array() {
        // work, carry, move, move
        PART_NUM_TO_STR_MAP.with(|map| {
            let body = [1, 2, 0, 0];
            let array = part_nums_to_str_array(&map, &body);
            assert_eq!(array.length(), 4);
            assert_eq!(array.get(0), "work");
            assert_eq!(array.get(1), "carry");
            assert_eq!(array.get(2), "move");
            assert_eq!(array.get(3), "move");
        })
    }
}
