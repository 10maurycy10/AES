use super::State;

pub fn shift_rows(inital :&State, buffer: &mut State) {

    // NOTE shifts in table are reversed.
    let table: [usize; 16] = [
        0,1,2,3, // no shift
        5,6,7,4, // right by 1
        10,11,8,9, // right by 2
        15,12,13,14, // right by 3 (left by 1)
    ];

    for (i, v) in inital.iter().enumerate() {
        buffer[table[i as usize] as usize] = *v
    }
    
}

pub fn inverse_shift_rows(inital :&State, buffer: &mut State) {

    // NOTE shifts in table are reversed.
    let table: [usize; 16] = [
        0,1,2,3,
        7,4,5,6,
        10,11,8,9,
        13,14,15,12,
    ];

    for (i, v) in inital.iter().enumerate() {
        buffer[table[i as usize] as usize] = *v
    }
    
}

#[test]
fn test_shift_row_inverse() {
    let init = [
        10,11,12,13,
        14,15,16,17,
        18,19,20,21,
        22,23,24,25
    ];
    
    let mut buffer = [0_u8;16];
    let mut buffer2 = [0_u8;16];

    shift_rows(&init,&mut buffer);
    inverse_shift_rows(&buffer,&mut buffer2);
    
    assert_eq!(buffer,buffer)
}

#[test]
fn test_shift_row() {
    let init = [
        10,11,12,13,
        14,15,16,17,
        18,19,20,21,
        22,23,24,25
    ];
    
    let mut buffer = [0_u8;16];

    shift_rows(&init,&mut buffer);
    
    assert_eq!(buffer, [
        10,11,12,13,
        17,14,15,16,
        20,21,18,19,
        23,24,25,22,        
    ])
}
