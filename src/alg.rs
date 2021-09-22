use super::{State,RoundType};
use super::encript_round;
use super::decript_round;
use super::expand_key;

pub fn encript_block(mes: &State, key: &[u8; 32], sbox: &[u8; 256]) -> State {
    let expanded_key = expand_key(key,sbox);
    
    // split the _long_ expanded key into round keys
    let mut split_key = [[0_u8; 16]; 14];
    for i in 0..14 {
        for e in 0..16 {
            split_key[i][e] = expanded_key[i*16+e];
        }
    }
    
    let mut b1 = [0_u8; 16];
    let mut b2 = [0_u8; 16];
    
    encript_round(mes,&split_key[0],&mut b1,sbox,RoundType::First);
    
    encript_round(&mut b1,&split_key[1],&mut b2,sbox,RoundType::Normal);
    encript_round(&mut b2,&split_key[2],&mut b1,sbox,RoundType::Normal);
    encript_round(&mut b1,&split_key[3],&mut b2,sbox,RoundType::Normal);
    encript_round(&mut b2,&split_key[4],&mut b1,sbox,RoundType::Normal);
    encript_round(&mut b1,&split_key[5],&mut b2,sbox,RoundType::Normal);
    encript_round(&mut b2,&split_key[6],&mut b1,sbox,RoundType::Normal);
    encript_round(&mut b1,&split_key[7],&mut b2,sbox,RoundType::Normal);
    encript_round(&mut b2,&split_key[8],&mut b1,sbox,RoundType::Normal);
    encript_round(&mut b1,&split_key[9],&mut b2,sbox,RoundType::Normal);
    encript_round(&mut b2,&split_key[10],&mut b1,sbox,RoundType::Normal);
    encript_round(&mut b1,&split_key[11],&mut b2,sbox,RoundType::Normal);
    encript_round(&mut b2,&split_key[12],&mut b1,sbox,RoundType::Normal);
    
    encript_round(&mut b1,&split_key[13],&mut b2,sbox,RoundType::Last);

    return b2
}

pub fn decript_block(mes: &State, key: &[u8; 32], sbox: &[u8; 256], isbox: &[u8; 256]) -> State {
    let expanded_key = expand_key(key,sbox);
    
    // split the _long_ expanded key into round keys
    let mut split_key = [[0_u8; 16]; 14];
    for i in 0..14 {
        for e in 0..16 {
            split_key[i][e] = expanded_key[i*16+e];
        }
    }
    
    let mut b1 = [0_u8; 16];
    let mut b2 = [0_u8; 16];
    
    decript_round(mes,&split_key[13],&mut b1,isbox,RoundType::First);
    
    decript_round(&mut b1,&split_key[12],&mut b2,isbox,RoundType::Normal);
    decript_round(&mut b2,&split_key[11],&mut b1,isbox,RoundType::Normal);
    decript_round(&mut b1,&split_key[10],&mut b2,isbox,RoundType::Normal);
    decript_round(&mut b2,&split_key[9],&mut b1,isbox,RoundType::Normal);
    decript_round(&mut b1,&split_key[8],&mut b2,isbox,RoundType::Normal);
    decript_round(&mut b2,&split_key[7],&mut b1,isbox,RoundType::Normal);
    decript_round(&mut b1,&split_key[6],&mut b2,isbox,RoundType::Normal);
    decript_round(&mut b2,&split_key[5],&mut b1,isbox,RoundType::Normal);
    decript_round(&mut b1,&split_key[4],&mut b2,isbox,RoundType::Normal);
    decript_round(&mut b2,&split_key[3],&mut b1,isbox,RoundType::Normal);
    decript_round(&mut b1,&split_key[2],&mut b2,isbox,RoundType::Normal);
    decript_round(&mut b2,&split_key[1],&mut b1,isbox,RoundType::Normal);
    
    decript_round(&mut b1,&split_key[0],&mut b2,isbox,RoundType::Last);

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
