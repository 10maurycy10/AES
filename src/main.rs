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

fn main() {
    let sbox = make_sub_box();
    println!("{:x?}", expand_key(&[0; 32], &sbox));
}
