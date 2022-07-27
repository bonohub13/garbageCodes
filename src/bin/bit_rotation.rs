pub mod bit_rotation {
    pub const INT_BITS: i32 = 32;

    pub fn left(i: i32, j: i32) -> i32 {
        (i << j) | (i >> (INT_BITS - j))
    }

    pub fn left2(i: i32, j: i32) -> i32 {
        (i << (4 * j)) | (i >> (INT_BITS - j))
    }

    pub fn right(i: i32, j: i32) -> i32 {
        (i >> j) | (i << (INT_BITS - j))
    }
}

fn main() {
    let mut i = 1;

    println!("Init Value: {}", i);

    for _ in 0..3 {
        i = bit_rotation::left2(i, 1);

        println!("Value: {}", i);
    }
}
