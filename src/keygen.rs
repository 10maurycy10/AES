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

#[rustfmt::skip]
#[test]

fn expand_all_zeros() {
    use crate::make_sub_box;
    let expanded_key = expand_key(&[0_u8; 32], &make_sub_box());
    // I CAN HAS PROTEST CASE!!
    assert_eq!(expanded_key,[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x62, 0x63, 0x63, 0x63, 0x62, 0x63, 0x63, 0x63, 0x62, 0x63, 0x63, 0x63, 0x62, 0x63, 0x63, 0x63, 0xaa, 0xfb, 0xfb, 0xfb, 0xaa, 0xfb, 0xfb, 0xfb, 0xaa, 0xfb, 0xfb, 0xfb, 0xaa, 0xfb, 0xfb, 0xfb, 0x6f, 0x6c, 0x6c, 0xcf, 0x0d, 0x0f, 0x0f, 0xac, 0x6f, 0x6c, 0x6c, 0xcf, 0x0d, 0x0f, 0x0f, 0xac, 0x7d, 0x8d, 0x8d, 0x6a, 0xd7, 0x76, 0x76, 0x91, 0x7d, 0x8d, 0x8d, 0x6a, 0xd7, 0x76, 0x76, 0x91, 0x53, 0x54, 0xed, 0xc1, 0x5e, 0x5b, 0xe2, 0x6d, 0x31, 0x37, 0x8e, 0xa2, 0x3c, 0x38, 0x81, 0x0e, 0x96, 0x8a, 0x81, 0xc1, 0x41, 0xfc, 0xf7, 0x50, 0x3c, 0x71, 0x7a, 0x3a, 0xeb, 0x07, 0x0c, 0xab, 0x9e, 0xaa, 0x8f, 0x28, 0xc0, 0xf1, 0x6d, 0x45, 0xf1, 0xc6, 0xe3, 0xe7, 0xcd, 0xfe, 0x62, 0xe9, 0x2b, 0x31, 0x2b, 0xdf, 0x6a, 0xcd, 0xdc, 0x8f, 0x56, 0xbc, 0xa6, 0xb5, 0xbd, 0xbb, 0xaa, 0x1e, 0x64, 0x06, 0xfd, 0x52, 0xa4, 0xf7, 0x90, 0x17, 0x55, 0x31, 0x73, 0xf0, 0x98, 0xcf, 0x11, 0x19, 0x6d, 0xbb, 0xa9, 0x0b, 0x07, 0x76, 0x75, 0x84, 0x51, 0xca, 0xd3, 0x31, 0xec, 0x71, 0x79, 0x2f, 0xe7, 0xb0, 0xe8, 0x9c, 0x43, 0x47, 0x78, 0x8b, 0x16, 0x76, 0x0b, 0x7b, 0x8e, 0xb9, 0x1a, 0x62, 0x74, 0xed, 0x0b, 0xa1, 0x73, 0x9b, 0x7e, 0x25, 0x22, 0x51, 0xad, 0x14, 0xce, 0x20, 0xd4, 0x3b, 0x10, 0xf8, 0x0a, 0x17, 0x53, 0xbf, 0x72, 0x9c, 0x45, 0xc9, 0x79, 0xe7, 0xcb, 0x70, 0x63, 0x85])
}
