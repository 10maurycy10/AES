use crate::State;

// Substitution BOX, a lookup table to optimyze the substitution step
pub type Sbox = [u8; 256];

// shift 8 bits left
fn rot_l8(x: u8, shift: isize) -> u8 {
    // check that input is in bounds
    assert!(shift < 8);
    assert!(shift > -8);
    // actualy do it
    ((x) << (shift)) | ((x) >> (8 - (shift)))
}

// code adapeded from https://en.wikipedia.org/wiki/Rijndael_S-box
pub fn make_sub_box() -> Sbox {
    let mut sbox: Sbox = [0_u8; 256];

    let mut p: u8 = 1;
    let mut q: u8 = 1;
    loop {
        let x = match p & 0x80 {
            0 => 0,
            _ => 0x1B,
        };

        p = p ^ (p << 1) ^ x;

        q ^= q << 1;
        q ^= q << 2;
        q ^= q << 4;
        q ^= match q & 0x80 {
            0 => 0,
            _ => 0x09,
        };

        let transformed = q ^ rot_l8(q, 1) ^ rot_l8(q, 2) ^ rot_l8(q, 3) ^ rot_l8(q, 4);

        sbox[p as usize] = transformed ^ 0x63;

        // stop if p is one
        if p == 1 {
            break;
        }
    }

    //zero is specal
    sbox[0] = 0x63;

    return sbox;
}

// we need a way to undo the sbox.
pub fn invert_sub_box(sbox: &Sbox) -> Sbox {
    // buffer for inverted sbox
    let mut isbox = [0_u8; 256];

    // i is the location in the sbox, v is the value at that location
    for (i, v) in sbox.iter().enumerate() {
        isbox[*v as usize] = i as u8;
    }

    return isbox;
}

pub fn sub_bytes(input: &State, result_buffer: &mut State, sbox: &Sbox) {
    for (i, v) in input.iter().enumerate() {
        result_buffer[i] = sbox[*v as usize];
    }
}

#[test]
fn test_sbox() {
    // generate a sbox and inverse
    let sbox = make_sub_box();
    let isbox = invert_sub_box(&sbox);
    // some test input
    let input = b"0123456789ABCDEF";
    // some buffers
    let mut crypt_buffer = [0; 16];
    let mut decrypt_buffer = [0; 16];
    // sbox and de-sbox data
    sub_bytes(input, &mut crypt_buffer, &sbox);
    sub_bytes(&crypt_buffer, &mut decrypt_buffer, &isbox);
    // make sure the inverse sbox is actaly inverse of the sbox
    assert_eq!(decrypt_buffer, *input);
}
