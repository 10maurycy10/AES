use super::gmul;

fn rotate(x: &mut [u8; 4]) {
    let a = x[0];
    for i in 0..3 {
        x[i] = x[i + 1]
    }
    x[3] = a;
}

// Calculate the rcon used in key expansion
fn rcon(mut x: u8) -> u8 {
    let mut c = 1_u8;
    if x == 0 {
        return 0;
    }
    while x != 1 {
        c = gmul(c, 2);
        x -= 1;
    }
    return c;
}

// Do scrambaling
fn schedule_core(input: &mut [u8; 4], i: u8, sbox: &[u8; 256]) {
    // Rotate the input 8 bits to the left
    rotate(input);
    // Apply Rijndael's s-box on all 4 bytes
    for a in 0..4 {
        input[a] = sbox[input[a as usize] as usize];
    }
    // On just the first byte, add 2^i to the byte
    input[0] ^= rcon(i);
}

pub fn expand_key(input: &[u8; 32], sbox: &[u8; 256]) -> [u8; 240] {
    let mut t = [0_u8; 4];
    let mut c = 32_u8;
    let mut i: u8 = 1;
    let mut _a: u8;

    let mut buffer = [0_u8; 240];

    for i in 0..32 {
        buffer[i] = input[i]
    }

    while c < 240 {
        // Copy the temporary variable over
        for a in 0..4 {
            t[a as usize] = buffer[a as usize + c as usize - 4];
        }
        // Every eight sets, do a complex calculation
        if c % 32 == 0 {
            schedule_core(&mut t, i, sbox);
            i += 1;
        }
        // For 256-bit keys, we add an extra sbox to the calculation
        if c % 32 == 16 {
            for a in 0..4 {
                t[a as usize] = sbox[t[a as usize] as usize];
            }
        }
        for a in 0..4 {
            buffer[c as usize] = buffer[c as usize - 32] ^ t[a as usize];
            c += 1;
        }
    }

    return buffer;
}

#[test]

fn expand_all_zeros() {
    use crate::make_sub_box;
    let expanded_key = expand_key(&[0_u8; 32], &make_sub_box());
    // I CAN HAS PROTEST CASE!!
    #[rustfmt::skip]
    assert_eq!(expanded_key,[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 98, 99, 99, 99, 98, 99, 99, 99, 98, 99, 99, 99, 98, 99, 99, 99, 170, 251, 251, 251, 170, 251, 251, 251, 170, 251, 251, 251, 170, 251, 251, 251, 111, 108, 108, 207, 13, 15, 15, 172, 111, 108, 108, 207, 13, 15, 15, 172, 125, 141, 141, 106, 215, 118, 118, 145, 125, 141, 141, 106, 215, 118, 118, 145, 83, 84, 237, 193, 94, 91, 226, 109, 49, 55, 142, 162, 60, 56, 129, 14, 150, 138, 129, 193, 65, 252, 247, 80, 60, 113, 122, 58, 235, 7, 12, 171, 158, 170, 143, 40, 192, 241, 109, 69, 241, 198, 227, 231, 205, 254, 98, 233, 43, 49, 43, 223, 106, 205, 220, 143, 86, 188, 166, 181, 189, 187, 170, 30, 100, 6, 253, 82, 164, 247, 144, 23, 85, 49, 115, 240, 152, 207, 17, 25, 109, 187, 169, 11, 7, 118, 117, 132, 81, 202, 211, 49, 236, 113, 121, 47, 231, 176, 232, 156, 67, 71, 120, 139, 22, 118, 11, 123, 142, 185, 26, 98, 116, 237, 11, 161, 115, 155, 126, 37, 34, 81, 173, 20, 206, 32, 212, 59, 16, 248, 10, 23, 83, 191, 114, 156, 69, 201, 121, 231, 203, 112, 99, 133])
}
