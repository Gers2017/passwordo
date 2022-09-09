use std::collections::HashSet;

// https://en.wikipedia.org/wiki/Password_strength#Entropy_as_a_measure_of_password_strength
// R = pool of unique characters
// log2(R^Length)
pub fn bits_entropy(unique_symbols: usize, pool_length: usize) -> u32 {
    let pool_length = pool_length as f64;
    (pool_length.powf(unique_symbols as f64)).log2() as u32
}

pub fn get_password_strength(text: &String, pool_length: usize) -> u32 {
    let unique_symbols = get_unique_chars_count(text);
    bits_entropy(unique_symbols, pool_length)
}

fn get_unique_chars_count(text: &String) -> usize {
    let mut set: HashSet<char> = HashSet::new();
    let chars: Vec<char> = text.chars().into_iter().collect();
    for item in chars {
        set.insert(item);
    }

    set.len()
}
