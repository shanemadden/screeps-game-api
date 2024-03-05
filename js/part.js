// keep in sync with Part in constants/small_enums.rs
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

export function part_num_for_bodypart(bodypart) {
    return PART_STR_TO_NUM[bodypart.type]
}

export function part_array_num_to_str(body_num_array) {
    console.log(typeof body_num_array)
    console.log(typeof [1, 2]);
    let r = body_num_array.map(part_num_to_str);
    console.log([1, 2].map(part_num_to_str));
    console.log("passed part array ints", body_num_array);
    console.log("passed part array mapped", body_num_array.map(part_num_to_str));
    console.log("hardcoded array ints", [0, 0]);
    console.log("hardcoded array mapped", [0, 0].map(part_num_to_str));
    //console.log([0, 0] == body_num_array);
    return r
}
