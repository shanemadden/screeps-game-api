module.exports.object_id_into_uint8array = function(id, arr) {
    // passed array is exactly 16 bytes -- 12 for id, 3 blank, 1 for string pad length
    // initialize all of them so that we can assume they're initialized on the rust side
    const padding = id.length;

    for (let byte = 0; byte <= 12; byte++) {
        if (byte * 2 + padding < 23) {
            arr[byte] = 0;
        } else if (byte * 2 + padding === 23) {
            arr[byte] = parseInt(id.substr(byte * 2 - padding, 1), 16)
        } else {
            console.log("writing", byte)
            console.log(id.substr(byte * 2 - padding, 2))
            console.log(parseInt(id.substr(byte * 2 - padding, 2), 16))
            arr[byte] = parseInt(id.substr(byte * 2 - padding, 2), 16)
        }
    }
    
    arr[13] = 0;
    arr[14] = 0;
    arr[15] = 0;
    arr[16] = padding;
}

/*
object_id_into_uint8array = function(id, arr) {
    const padding = id.length;

    for (let byte = 0; byte <= 12; byte++) {
        if (byte * 2 + padding < 23) {
            arr[byte] = 0;
        } else if (byte * 2 + padding === 23) {
            arr[byte] = parseInt(id.substr(byte * 2 - padding, 1), 16)
        } else {
            console.log("writing", byte)
            console.log(id.substr(byte * 2 - padding, 2))
            console.log(parseInt(id.substr(byte * 2 - padding, 2), 16))
            arr[byte] = parseInt(id.substr(byte * 2 - padding, 2), 16)
        }
    }
    
    arr[13] = 0;
    arr[14] = 0;
    arr[15] = 0;
    arr[16] = padding;
}
let array = new Uint8Array(16)
object_id_into_uint8array("12", array)
console.log(array)
*/
