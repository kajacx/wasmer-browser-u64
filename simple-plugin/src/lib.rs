#[no_mangle]
pub fn add_i32(left: i32, right: i32) -> i32 {
    left.wrapping_add(right)
}

#[no_mangle]
pub fn sub_i32(left: i32, right: i32) -> i32 {
    left.wrapping_sub(right)
}

#[no_mangle]
pub fn add_i64(left: i64, right: i64) -> i64 {
    left.wrapping_add(right)
}

#[no_mangle]
pub fn sub_i64(left: i64, right: i64) -> i64 {
    left.wrapping_sub(right)
}

#[no_mangle]
pub fn is_even_i64(value: i64) -> bool {
    value % 2 == 0
}

#[no_mangle]
pub fn combine_to_i64(upper: i32, lower: i32) -> i64 {
    (upper as i64 * 0x1_00_00_00_00) + lower as i64
}
