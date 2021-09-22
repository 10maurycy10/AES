use super::State;

pub fn add_key(init: &State, key: &State, buffer: &mut State) {
    for (i, _v) in init.iter().enumerate() {
        buffer[i] = init[i] ^ key[i]
    }
}
