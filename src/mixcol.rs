use super::State;

// adapted from https://en.wikipedia.org/wiki/Rijndael_MixColumns
fn gmul(mut a: u8,mut b: u8) -> u8 { // Galois Field (256) Multiplication of two Bytes
    let mut p: u8 = 0;

    for counter in 0..8 {
        if ((b & 1) != 0) {
            p ^= a;
        }

        let hi_bit_set: bool = (a & 0x80) != 0;
        a <<= 1;
        if (hi_bit_set) {
            a ^= 0x1B; /* x^8 + x^4 + x^3 + x + 1 */
        }
        b >>= 1;
    }

    return p;
}

pub fn mix_col(a: &State,b: &mut State) {
    for c in 0..4 {
        b[0+c] = gmul(0x02, a[0+c]) ^ gmul(0x03, a[4+c]) ^ a[8+c]             ^ a[12+c];
        b[4+c] = a[0+c]             ^ gmul(0x02, a[4+c]) ^ gmul(0x03, a[8+c]) ^ a[12+c];
        b[8+c] = a[0+c]             ^ a[4+c]             ^ gmul(0x02, a[8+c]) ^ gmul(0x03, a[12+c]);
        b[12+c] = gmul(0x03, a[0+c])^ a[4+c]             ^ a[8+c]             ^ gmul(0x02, a[12+c]);
    }
}

// inverse from https://en.wikipedia.org/wiki/Rijndael_MixColumns

pub fn inverse_mix_col(a: &State,b: &mut State) {
    for c in 0..4 {
        b[0+c]  = gmul(14, a[0+c]) ^ gmul(11, a[4+c]) ^ gmul(13, a[8+c]) ^ gmul(09, a[12+c]);
        b[4+c]  = gmul(09, a[0+c]) ^ gmul(14, a[4+c]) ^ gmul(11, a[8+c]) ^ gmul(13, a[12+c]);
        b[8+c]  = gmul(13, a[0+c]) ^ gmul(09, a[4+c]) ^ gmul(14, a[8+c]) ^ gmul(11, a[12+c]);
        b[12+c] = gmul(11, a[0+c]) ^ gmul(13, a[4+c]) ^ gmul(09, a[8+c]) ^ gmul(14, a[12+c]);
    }
}

#[test]
fn test_sane_inverse() {
    let mut buffer1 = [0_u8;16];
    let mut buffer2 = [0_u8;16];
    let init = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
    
    mix_col(&init,&mut buffer1);
    inverse_mix_col(&buffer1,&mut buffer2);

    assert_eq!(init,buffer2);
}
