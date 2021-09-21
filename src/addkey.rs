use super::State;

fn add_key(init: &State, key: &State, buffer: &mut State) {
    for (i, v) in init.iter().enumerate() {
        buffer[i] = init[i] ^ key[i]
    }
}
