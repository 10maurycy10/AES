// 2d array of bytes (represented as array of 32 bytes for speed)
pub type State = [u8; 16];

pub fn get_from_state(x: usize, y: usize, state: &State) -> &u8 {
    &state[x+y*4]
}

pub fn get_mut_from_state(x: usize, y: usize, state: &mut State) -> &mut u8 {
    state.get_mut(x+y*4).unwrap()
}

mod sbox;
use sbox::*;

mod shiftrows;
use shiftrows::*;

mod addkey;
use addkey::*;

mod keygen;
use keygen::*;

mod mixcol;
use mixcol::*;

fn main() {

    let mut buffer = [255; 16];
    mix_col(&[
        0,1,2,3,
        4,1,6,7,
        1,1,10,11,
        1,1,14,1
    ],&mut buffer);
    println!("{:?}",buffer);
}
