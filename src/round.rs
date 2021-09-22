use super::*;

pub enum RoundType {
    First,
    Normal,
    Last
}

// Preforme 1 round of encription
pub fn encript_round(input: &State, round_key: &[u8; 16], buffer: &mut State, sbox: &[u8; 256],t: RoundType) {
    match t {
        RoundType::First => {
            add_key(&input, round_key, buffer);
        },
        RoundType::Normal => {
            let mut b1 = [0_u8;16];
            sub_bytes(input,&mut b1,sbox);
            let mut b2 = [0_u8;16];
            shift_rows(&b1,&mut b2);
            let mut b3 = [0_u8;16];
            mix_col(&b2,&mut b3);
            add_key(&b3, round_key, buffer);
        }
        RoundType::Last => {
            let mut b1 = [0_u8;16];
            sub_bytes(input,&mut b1,sbox);
            let mut b2 = [0_u8;16];
            shift_rows(&b1,&mut b2);
            add_key(&b2, round_key, buffer);
        }
    }
}

// NOTE Sbox should be an inverted sbox
// Fist/Last switch in decription
pub fn decript_round(input: &State, round_key: &[u8; 16], buffer: &mut State, sbox: &[u8; 256],t: RoundType) {
    match t {
        RoundType::Last => {
            add_key(input,round_key,buffer)
        },
        RoundType::Normal => {
            let mut b1 = [0_u8;16];
            add_key(input,round_key,&mut b1);
            let mut b2 = [0_u8;16];
            inverse_mix_col(&b1,&mut b2);
            let mut b3 = [0_u8;16];
            inverse_shift_rows(&b2, &mut b3);
            sub_bytes(&b3,buffer,sbox);
       }
        RoundType::First => {
            let mut b1 = [0_u8;16];
            add_key(input,round_key,&mut b1);
            let mut b2 = [0_u8;16];
            inverse_shift_rows(&b1, &mut b2);
            sub_bytes(&b2,buffer,sbox);
        }
    }
}

#[test]
fn sane_inverse() {
    let key = b"0123456789ABCDEF";
    let msg = b"thisisacode-----";
    use crate::make_sub_box;
    use crate::invert_sub_box;
    let sbox = make_sub_box();
    let mut coded = [0_u8;16];
    let mut decoded = [0_u8;16];
    encript_round(msg,key,&mut coded,&sbox,RoundType::Normal);
    decript_round(&coded,key,&mut decoded,&invert_sub_box(&sbox),RoundType::Normal);
    assert!(&coded != msg);
    assert_eq!(&decoded, msg);
}

#[test]
fn sane_inverse_on_first_round() {
    let key = b"0123456789ABCDEF";
    let msg = b"thisisacode-----";
    use crate::make_sub_box;
    use crate::invert_sub_box;
    let sbox = make_sub_box();
    let mut coded = [0_u8;16];
    let mut decoded = [0_u8;16];
    encript_round(msg,key,&mut coded,&sbox,RoundType::First);
    decript_round(&coded,key,&mut decoded,&invert_sub_box(&sbox),RoundType::Last);
    assert!(&coded != msg);
    assert_eq!(&decoded, msg);
}

#[test]
fn sane_inverse_on_last_round() {
    let key = b"0123456789ABCDEF";
    let msg = b"thisisacode-----";
    use crate::make_sub_box;
    use crate::invert_sub_box;
    let sbox = make_sub_box();
    let mut coded = [0_u8;16];
    let mut decoded = [0_u8;16];
    encript_round(msg, key, &mut coded, &sbox, RoundType::Last);
    decript_round(&coded, key, &mut decoded, &invert_sub_box(&sbox), RoundType::First);
    assert!(&coded != msg);
    assert_eq!(&decoded, msg);
}
