// 2d array of bytes (represented as array of 32 bytes for speed)
pub type State = [u8; 16];

pub fn get_from_state(x: usize, y: usize, state: &State) -> &u8 {
    &state[x + y * 4]
}

pub fn get_mut_from_state(x: usize, y: usize, state: &mut State) -> &mut u8 {
    state.get_mut(x + y * 4).unwrap()
}

mod g;
use g::*;

mod sbox;
pub use sbox::*;

mod shiftrows;
use shiftrows::*;

mod addkey;
use addkey::*;

mod keygen;
use keygen::*;

mod mixcol;
use mixcol::*;

mod round;
use round::*;

mod alg;
use alg::*;

pub fn ecb_encript(message: &Vec<u8>, key: &[u8; 32]) -> Vec<[u8; 16]> {
    let sbox = make_sub_box();
    let mut coded = Vec::new();
    for i in 0..(message.len()/16 + 1) {
        let mut buffer = [0_u8; 16];
        for e in 0..16 {
            let p = e+i*16;
            if p < message.len() {
                buffer[e] = message[p];
            }
        }
        // ECB MODE!!!!!!!!!
        let coded_block = encript_block(&buffer,key,&sbox);
        coded.push(coded_block)
    }
    coded
}

pub fn ecb_decript(message: &Vec<[u8; 16]>, key: &[u8; 32]) -> Vec<u8> {
    let sbox = make_sub_box();
    let isbox = invert_sub_box(&sbox);
    let mut coded = Vec::new();
    for i in 0..message.len() {
        let coded_block = decript_block(&message[i],key,&sbox,&isbox);
        coded.push(coded_block)
    }
    coded.into_iter().flatten().collect()
}

#[test]
fn have_sane_decript() {
    // null padded
    let original = b"This is a message that will take several block";
    let coded = ecb_encript(&original.to_vec(),b"JUNK KEY------------------------");
    let decoded = ecb_decript(&coded,b"JUNK KEY------------------------");
    assert_eq!(decoded[0..45],original[0..45]);
}