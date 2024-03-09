// keep in sync with Part in constants/small_enums.rs

// for test, wonder how to get rid of..
const MOVE = global.MOVE || "move";
const WORK = global.WORK || "work";
const CARRY = global.CARRY || "carry";
const ATTACK = global.ATTACK || "attack";
const RANGED_ATTACK = global.RANGED_ATTACK || "ranged_attack";
const TOUGH = global.TOUGH || "tough";
const HEAL = global.HEAL || "heal";
const CLAIM = global.CLAIM || "claim";

const PART_STR_TO_NUM = {
    MOVE: 0,
    WORK: 1,
    CARRY: 2,
    ATTACK: 3,
    RANGED_ATTACK: 4,
    TOUGH: 5,
    HEAL: 6,
    CLAIM: 7,
};

const PART_NUM_TO_STR = {
    '0': MOVE,
    '1': WORK,
    '2': CARRY,
    '3': ATTACK,
    '4': RANGED_ATTACK,
    '5': TOUGH,
    '6': HEAL,
    '7': CLAIM,
};

function part_num_to_str(num) {
    return PART_NUM_TO_STR[num]
}

module.exports.part_num_for_bodypart = function(bodypart) {
    return PART_STR_TO_NUM[bodypart.type]
}

module.exports.part_array_num_to_str = function(body_num_array) {
    // this is a Uint8Array and its map can't produce strings as-is,
    // spread it first so the map can result in an array with constant strings
    return [...body_num_array].map(part_num_to_str)
}
