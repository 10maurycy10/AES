use super::State;

pub fn add_key(init: &State, key: &State, buffer: &mut State) {
    for i in 0..16 {
        buffer[i] = init[i] ^ key[i]
    }
}
