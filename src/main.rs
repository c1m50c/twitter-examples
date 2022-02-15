#[inline]
pub const fn rotate_right(x: u32, n: u32) -> u32 { (x >> n) | (x << u32::BITS - n) }

fn use_rotr() {
    let one_rotr: u32 = rotate_right(1, 3);

    // When compiled the expression that defines `one_rotr` will look like...

    let one_rotr: u32 = (1 >> 3) | (1 << u32::BITS - 3);
}

fn main() {  }