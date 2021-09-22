// operations in 8 bit https://en.wikipedia.org/wiki/GF(2)

pub fn gadd(a: u8, b: u8) -> u8 {
    a ^ b
}

pub fn gmul(mut a: u8, mut b: u8) -> u8 {
    let mut p: u8 = 0;
    let mut hi_bit_set: u8;
    for _i in 0..8 {
        if (b & 1) == 1 {
            p ^= a;
        }
        hi_bit_set = a & 0x80;
        a <<= 1;
        if hi_bit_set == 0x80 {
            a ^= 0x1b;
        }
        b >>= 1;
    }
    return p;
}
