use crate::{entropy::get_password_strength, password_info::PasswordInfo, POOL, SPECIAL};
use hex::encode;
use rand::prelude::*;

pub fn gen_schemed(
    hash_length: usize,
    is_inverted: bool,
    words: &mut Vec<String>,
    use_special: bool,
) -> PasswordInfo {
    let rng = &mut thread_rng();
    for _ in 0..10 {
        words.shuffle(rng);
    }

    let word = words.choose(rng).unwrap().to_owned();
    let nonsense = gen_nonsense(hash_length, use_special);
    let hash = nonsense.text;

    let pool_length = POOL.len() + (use_special as usize) * SPECIAL.len();
    let text = if is_inverted {
        format!("{}{}", hash, word)
    } else {
        format!("{}{}", word, hash)
    };

    let entropy = get_password_strength(&text, pool_length);

    PasswordInfo::new(text, entropy)
}

pub fn gen_nonsense(length: usize, use_special: bool) -> PasswordInfo {
    let rng = &mut thread_rng();
    let mut bytes: Vec<u8> = POOL.as_bytes().iter().cloned().collect();

    if use_special {
        bytes.extend(SPECIAL.as_bytes());
    }

    for _ in 0..10 {
        bytes.shuffle(rng);
    }

    let bucket: Vec<u8> = (0..length)
        .map(|_| bytes.choose(rng).unwrap().to_owned())
        .collect();

    let pool_length = POOL.len() + (use_special as usize) * SPECIAL.len();
    let text = String::from_utf8(bucket).unwrap_or(String::default());
    let entropy = get_password_strength(&text, pool_length);

    PasswordInfo::new(text, entropy)
}

pub fn gen_random_bytes_hex(size: usize) -> PasswordInfo {
    let text = encode(get_random_bytes(size));
    let entropy = get_password_strength(&text, 16);
    PasswordInfo::new(text, entropy)
}

fn get_random_bytes(size: usize) -> Vec<u8> {
    let bytes: Vec<u8> = (0..size).map(|_| random::<u8>()).collect();
    bytes
}
