// build-pass

use spirv_std as _;

#[spirv(fragment)]
pub fn main(#[spirv(flat)] i: i32) {
    while i < 10 {
        if i == 0 {
            break;
        }
        if i == 1 {
            break;
        }
    }
}
