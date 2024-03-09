module.exports.part_num_for_bodypart = function(part_str_to_num_map, bodypart) {
    return part_str_to_num_map.get(bodypart.type)
}

module.exports.part_array_num_to_str = function(part_num_to_str_map, body_num_array) {
    // this is a Uint8Array and its map can't produce strings as-is,
    // spread it first so the map can result in an array with constant strings
    return [...body_num_array].map((v) => part_num_to_str_map.get(v));
}
