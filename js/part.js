// keep in sync with PartInt in constants/small_enums.rs
const PART_TABLE = {
    MOVE: 0,
    '0': MOVE,
    WORK: 1,
    '1': WORK,
    CARRY: 2,
    '2': CARRY,
    ATTACK: 3,
    '3': ATTACK,
    RANGED_ATTACK: 4,
    '4': RANGED_ATTACK,
    TOUGH: 5,
    '5': TOUGH,
    HEAL: 6,
    '6': HEAL,
    CLAIM: 7,
    '7': CLAIM,
};

export function part_num_to_str(num) {
    return PART_TABLE[num]
}

export function part_str_to_num(str) {
    return PART_TABLE[str]
}

export function part_num_for_bodypart(bodypart) {
    return part_str_to_num(bodypart.type)
}
