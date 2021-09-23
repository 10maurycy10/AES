use super::{State,RoundType};
use super::encript_round;
use super::decript_round;
use super::expand_key;

pub fn encript_block(mes: &State, key: &[u8; 32], sbox: &[u8; 256]) -> State {
    let expanded_key = expand_key(key,sbox);
    
    // this bit is fine
    // split the _long_ expanded key into round keys
    let mut split_key = [[0_u8; 16]; 15];
    for i in 0..15 {
        for e in 0..16 {
            split_key[i][e] = expanded_key[i*16+e];
        }
    }
        
    let mut b1 = [0_u8; 16];
    let mut b2 = [0_u8; 16];
    
    encript_round(mes,&split_key[0],&mut b1,sbox,RoundType::First); // first round
    
    encript_round(&b1,&split_key[1],&mut b2,sbox,RoundType::Normal);
    encript_round(&b2,&split_key[2],&mut b1,sbox,RoundType::Normal);
    encript_round(&b1,&split_key[3],&mut b2,sbox,RoundType::Normal);
    encript_round(&b2,&split_key[4],&mut b1,sbox,RoundType::Normal);
    encript_round(&b1,&split_key[5],&mut b2,sbox,RoundType::Normal);
    encript_round(&b2,&split_key[6],&mut b1,sbox,RoundType::Normal);
    encript_round(&b1,&split_key[7],&mut b2,sbox,RoundType::Normal);
    encript_round(&b2,&split_key[8],&mut b1,sbox,RoundType::Normal);
    encript_round(&b1,&split_key[9],&mut b2,sbox,RoundType::Normal);
    encript_round(&b2,&split_key[10],&mut b1,sbox,RoundType::Normal);
    encript_round(&b1,&split_key[11],&mut b2,sbox,RoundType::Normal);
    encript_round(&b2,&split_key[12],&mut b1,sbox,RoundType::Normal);
    encript_round(&b1,&split_key[13],&mut b2,sbox,RoundType::Normal);
    
    encript_round(&mut b2,&split_key[14],&mut b1,sbox,RoundType::Last);

    return b1
}

pub fn decript_block(mes: &State, key: &[u8; 32], sbox: &[u8; 256], isbox: &[u8; 256]) -> State {
    let expanded_key = expand_key(key,sbox);
    
    // split the _long_ expanded key into round keys
    let mut split_key = [[0_u8; 16]; 15];
    for i in 0..15 {
        for e in 0..16 {
            split_key[i][e] = expanded_key[i*16+e];
        }
    }
    
    let mut b1 = [0_u8; 16];
    let mut b2 = [0_u8; 16];
    
    decript_round(mes,&split_key[14],&mut b2,isbox,RoundType::First);
    
    decript_round(&b2,&split_key[13],&mut b1,isbox,RoundType::Normal);
    decript_round(&b1,&split_key[12],&mut b2,isbox,RoundType::Normal);
    decript_round(&b2,&split_key[11],&mut b1,isbox,RoundType::Normal);
    decript_round(&b1,&split_key[10],&mut b2,isbox,RoundType::Normal);
    decript_round(&b2,&split_key[9],&mut b1,isbox,RoundType::Normal);
    decript_round(&b1,&split_key[8],&mut b2,isbox,RoundType::Normal);
    decript_round(&b2,&split_key[7],&mut b1,isbox,RoundType::Normal);
    decript_round(&b1,&split_key[6],&mut b2,isbox,RoundType::Normal);
    decript_round(&b2,&split_key[5],&mut b1,isbox,RoundType::Normal);
    decript_round(&b1,&split_key[4],&mut b2,isbox,RoundType::Normal);
    decript_round(&b2,&split_key[3],&mut b1,isbox,RoundType::Normal);
    decript_round(&b1,&split_key[2],&mut b2,isbox,RoundType::Normal);
    decript_round(&b2,&split_key[1],&mut b1,isbox,RoundType::Normal);
    
    decript_round(&b1,&split_key[0],&mut b2,isbox,RoundType::Last);

    return b2
}

#[test]
fn sane_decript() {
    use crate::make_sub_box;
    use crate::invert_sub_box;
    let sbox = make_sub_box();
    let isbox = invert_sub_box(&sbox);
    let coded = encript_block(b"thisisacoded----",b"BADKEY----------BADKEY----------",&sbox);
    let decoded = decript_block(&coded,b"BADKEY----------BADKEY----------",&sbox,&isbox);
    assert_eq!(b"thisisacoded----",&decoded);
}

fn reverse_state(input: &State) -> State {
    let mut buffer = [0_u8; 16];
    for i in 0..16 {
        buffer[15-i] = input[i]
    }
    return buffer
}

#[test]
fn can_revese_state() {
    assert_eq!(reverse_state(&[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]),[16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1]);
}

#[test]
fn kat0 () {
    use crate::make_sub_box;
    let key = [0_u8; 32];
    let plain = [0x01, 0x47, 0x30, 0xf8, 0x0a, 0xc6, 0x25, 0xfe, 0x84, 0xf0, 0x26, 0xc6, 0x0b, 0xfd, 0x54, 0x7d];
    let coded = encript_block(&plain,&key,&make_sub_box());
    assert_eq!(coded,[0x5c, 0x9d, 0x84, 0x4e, 0xd4, 0x6f, 0x98, 0x85, 0x08, 0x5e, 0x5d, 0x6a, 0x4f, 0x94, 0xc7, 0xd7])
}
