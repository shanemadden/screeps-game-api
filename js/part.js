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

export function part_num_for_bodypart(bodypart) {
    return PART_STR_TO_NUM[bodypart.type]
}
