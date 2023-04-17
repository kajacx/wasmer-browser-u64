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
