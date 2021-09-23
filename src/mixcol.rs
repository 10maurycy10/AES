use super::gmul;
use super::State;

pub fn mix_col(a: &State, b: &mut State) {
    for c in 0..4 {
        let d = c*4;
        b[0 + d]    = gmul(2, a[0 + d]) ^ gmul(3, a[1 + d]) ^ a[2 + d]          ^ a[3 + d];
        b[1 + d]    = a[0 + d]          ^ gmul(2, a[1 + d]) ^ gmul(3, a[2 + d]) ^ a[3 + d];
        b[2 + d]    = a[0 + d]          ^ a[1 + d]          ^ gmul(2, a[2 + d]) ^ gmul(3, a[3 + d]);
        b[3 + d]    = gmul(3, a[0 + d]) ^ a[1 + d]          ^ a[2 + d]          ^ gmul(2, a[3 + d]);
    }
}

// inverse from https://en.wikipedia.org/wiki/Rijndael_MixColumns

pub fn inverse_mix_col(a: &State, b: &mut State) {
    for c in 0..4 {
        let d = c*4;
        b[0 + d] =
            gmul(14, a[0 + d]) ^ gmul(11, a[1 + d]) ^ gmul(13, a[2 + d]) ^ gmul(09, a[3 + d]);
        b[1 + d] =
            gmul(09, a[0 + d]) ^ gmul(14, a[1 + d]) ^ gmul(11, a[2 + d]) ^ gmul(13, a[3 + d]);
        b[2 + d] =
            gmul(13, a[0 + d]) ^ gmul(09, a[1 + d]) ^ gmul(14, a[2 + d]) ^ gmul(11, a[3 + d]);
        b[3 + d] =
            gmul(11, a[0 + d]) ^ gmul(13, a[1 + d]) ^ gmul(09, a[2 + d]) ^ gmul(14, a[3 + d]);
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

// utility to convert a 2 d state to the format rijndael uses
pub fn rotate_state(input: &State) -> State {
    let mut buffer = [0_u8; 16];
    for x in 0..4 {
        for y in 0..4 {
            buffer[y+x*4] = input[y*4+x];
        }
    }
    return buffer;
}

// vectors derived from https://www.samiam.org/mix-column.html
#[test]
fn test_vec() {
    let input = rotate_state(&[
        0xdb, 0xf2, 0x01, 0xc6,
        0x13, 0x0a, 0x01, 0xc6,
        0x53, 0x22, 0x01, 0xc6,
        0x45, 0x5c, 0x01, 0xc6
    ]);
    let output = rotate_state(&[
        0x8e, 0x9f, 0x01, 0xc6,
        0x4d, 0xdc, 0x01, 0xc6,
        0xa1, 0x58, 0x01, 0xc6,
        0xbc, 0x9d, 0x01, 0xc6
    ]);
    let mut buffer = [0_u8;16];
    mix_col(&input,&mut buffer);
    assert_eq!(buffer,output);
}
