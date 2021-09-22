use super::*;

pub enum RoundType {
    First,
    Normal,
    Last
}

pub fn encript_round(input: &State, round_key: &[u8; 16], sbox: &[u8; 256],t: RoundType) -> State {
    match t {
        RoundType::First => {
            let mut b1 = [0_u8;16];
            add_key(&input, round_key, &mut b1);
            b1
        },
        RoundType::Normal => {
            let mut b1 = [0_u8;16];
            sub_bytes(input,&mut b1,sbox);
            let mut b2 = [0_u8;16];
            shift_rows(&b1,&mut b2);
            let mut b3 = [0_u8;16];
            mix_col(&b2,&mut b3);
            let mut b4 = [0_u8;16];
            add_key(&b3, round_key, &mut b4);
            b4
        }
        RoundType::Last => {
            let mut b1 = [0_u8;16];
            sub_bytes(input,&mut b1,sbox);
            let mut b2 = [0_u8;16];
            shift_rows(&b1,&mut b2);
            let mut b3 = [0_u8;16];
            add_key(&b2, round_key, &mut b3);
            b3
        }
    }
}
