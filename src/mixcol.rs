use super::gmul;
use super::State;

pub fn mix_col(a: &State, b: &mut State) {
    for c in 0..4 {
        b[0 + c]    = gmul(2, a[0 + c]) ^ gmul(3, a[4 + c]) ^ a[8 + c]          ^ a[12 + c];
        b[4 + c]    = a[0 + c]          ^ gmul(2, a[4 + c]) ^ gmul(3, a[8 + c]) ^ a[12 + c];
        b[8 + c]    = a[0 + c]          ^ a[4 + c]          ^ gmul(2, a[8 + c]) ^ gmul(3, a[12 + c]);
        b[12 + c]   = gmul(3, a[0 + c]) ^ a[4 + c]          ^ a[8 + c]          ^ gmul(2, a[12 + c]);
    }
}

// inverse from https://en.wikipedia.org/wiki/Rijndael_MixColumns

pub fn inverse_mix_col(a: &State, b: &mut State) {
    for c in 0..4 {
        b[0 + c] =
            gmul(14, a[0 + c]) ^ gmul(11, a[4 + c]) ^ gmul(13, a[8 + c]) ^ gmul(09, a[12 + c]);
        b[4 + c] =
            gmul(09, a[0 + c]) ^ gmul(14, a[4 + c]) ^ gmul(11, a[8 + c]) ^ gmul(13, a[12 + c]);
        b[8 + c] =
            gmul(13, a[0 + c]) ^ gmul(09, a[4 + c]) ^ gmul(14, a[8 + c]) ^ gmul(11, a[12 + c]);
        b[12 + c] =
            gmul(11, a[0 + c]) ^ gmul(13, a[4 + c]) ^ gmul(09, a[8 + c]) ^ gmul(14, a[12 + c]);
    }
}

#[test]
fn test_sane_inverse() {
    let mut buffer1 = [0_u8; 16];
    let mut buffer2 = [0_u8; 16];
    #[rustfmt::skip]
    let init = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];

    mix_col(&init, &mut buffer1);
    inverse_mix_col(&buffer1, &mut buffer2);

    assert_eq!(init, buffer2);
}

// vectors derived from https://www.samiam.org/mix-column.html
#[test]
fn test_vec() {
    let input = [
        0xdb, 0xf2, 0x01, 0xc6,
        0x13, 0x0a, 0x01, 0xc6,
        0x53, 0x22, 0x01, 0xc6,
        0x45, 0x5c, 0x01, 0xc6
    ];
    let output = [
        0x8e, 0x9f, 0x01, 0xc6,
        0x4d, 0xdc, 0x01, 0xc6,
        0xa1, 0x58, 0x01, 0xc6,
        0xbc, 0x9d, 0x01, 0xc6
    ];
    let mut buffer = [0_u8;16];
    mix_col(&input,&mut buffer);
    assert_eq!(buffer,output);
}
