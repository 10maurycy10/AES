use super::State;
use super::rotate_state;

pub fn shift_rows(inital: &State, buffer: &mut State) {
    for x in 0..4 {
        for y in 0..4 {
            let new_x = (y + x)%4;
            buffer[new_x*4+y] = inital[x*4+y];
        }
    }
}

pub fn inverse_shift_rows(inital: &State, buffer: &mut State) {
    for x in 0..4_usize {
        for y in 0..4_usize {
            let new_x = (x+y)%4;
            buffer[x*4+y] = inital[new_x*4+y];
        }
    }    
}

#[test]
fn test_shift_row_inverse() {
    #[rustfmt::skip]
    let init = [
        10,11,12,13,
        14,15,16,17,
        18,19,20,21,
        22,23,24,25
    ];

    let mut buffer = [0_u8; 16];
    let mut buffer2 = [0_u8; 16];

    shift_rows(&init, &mut buffer);
    inverse_shift_rows(&buffer, &mut buffer2);

    assert_eq!(buffer, buffer)
}

#[test]
fn test_shift_row() {
    let init = rotate_state(&[
        10, 11, 12, 13,
        14, 15, 16, 17,
        18, 19, 20, 21,
        22, 23, 24, 25,
    ]);

    let mut buffer = [0_u8; 16];

    shift_rows(&init, &mut buffer);

    assert_eq!(
        buffer,
        rotate_state(&[
        10, 11, 12, 13,
        17, 14, 15, 16,
        20, 21, 18, 19,
        23, 24, 25, 22,])
    )
}
