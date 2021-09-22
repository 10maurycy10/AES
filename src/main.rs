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

fn main()  {
    let sbox = make_sub_box();
    //let isbox = invert_sub_box(&sbox);
    
    // Stuff to include 256 bits from a file
    use std::convert::TryInto;
    let key: Vec<u8> = include_bytes!("../keyfile")[0..32].iter().map(|x| *x).collect();
    let key: [u8; 32] = key.try_into().unwrap();

    // Do some coding
    let coded_sample = encript_block(b"ThisIsACodeBlock",&key,&sbox);
    
    // Stuff to include N bits from a file
    let secret_message: Vec<u8> = include_bytes!("../puzzle").iter().map(|x| *x).collect();
    
    
    // this sucks
    for i in 0..(secret_message.len()/16 + 1) {
        // extract a block from the message
        let mut buffer = [0_u8; 16];
        for e in 0..16 {
            let p = e+i*16;
            if p < secret_message.len() {
                buffer[e] = secret_message[p]
            }
        }
        // ECB MODE!!!!!!!!!
        let puzzle = encript_block(&buffer,&key,&sbox);
        println!("CODE BLOCK {}: {:x?}",i,puzzle);
    }
    
    println!("{:x?} is 'ThisIsACodeBlock'", coded_sample);
}
