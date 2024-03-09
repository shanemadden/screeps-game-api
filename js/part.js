module.exports.part_num_for_bodypart = function(const_map, bodypart) {
    return const_map[bodypart.type]
}

module.exports.part_array_num_to_str = function(const_map, body_num_array) {
    // this is a Uint8Array and its map can't produce strings as-is,
    // spread it first so the map can result in an array with constant strings
    return [...body_num_array].map((v) => const_map.get(v));
}
