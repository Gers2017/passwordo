#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rand::prelude::*;

const POOL: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const SPECIAL: &str = "*+-=?!<>#$%&?@^~_|";

fn get_words() -> Vec<String> {
    let words = vec!["lapras", "pikachu", "mewtwo", "eevee", "magmar", "bidoof"];
    words.iter().map(|it| it.to_string()).collect::<Vec<_>>()
}

fn gen_nonsense(length: usize, use_special: bool) -> String {
    let rng = &mut thread_rng();
    let mut bytes: Vec<u8> = POOL.as_bytes().iter().cloned().collect();
    if use_special {
        bytes.extend(SPECIAL.as_bytes());
    }

    bytes.shuffle(rng);

    let bucket: Vec<u8> = (0..length)
        .map(|_| {
            // bytes.choose(rng).unwrap().to_owned()
            let i = rng.gen_range(0..bytes.len());
            bytes[i]
        })
        .collect();

    String::from_utf8(bucket).unwrap_or(String::default())
}

fn gen_schemed(
    hash_length: usize,
    is_inverted: bool,
    words: &mut Vec<String>,
    use_special: bool,
) -> String {
    let rng = &mut thread_rng();
    words.shuffle(rng);

    let word = words.choose(rng).unwrap().to_owned();
    let hash = gen_nonsense(hash_length, use_special);

    if is_inverted {
        return format!("{}{}", hash, word);
    }

    format!("{}{}", word, hash)
}

fn get_random_bytes(size: usize) -> Vec<u8> {
    let bytes: Vec<u8> = (0..size).map(|_| random::<u8>()).collect();
    bytes
}

#[tauri::command]
fn generate_nonsense_passwords(
    length: usize,
    amount: usize,
    use_random_bytes: bool,
    use_special: bool,
) -> Vec<String> {
    let mut res: Vec<String> = Vec::with_capacity(amount);
    for _ in 0..amount {
        let value = match use_random_bytes {
            true => hex::encode(get_random_bytes(length)),
            false => gen_nonsense(length, use_special),
        };
        res.push(value);
    }

    res
}

#[tauri::command]
fn generate_schemed_passwords(
    hash_length: usize,
    amount: usize,
    is_inverted: bool,
    list_of_words: Vec<String>,
    use_special: bool,
) -> Vec<String> {
    let mut words: Vec<String> = if list_of_words.is_empty() {
        get_words()
    } else {
        list_of_words
    };

    let res: Vec<String> = (0..amount)
        .map(|_| gen_schemed(hash_length, is_inverted, &mut words, use_special))
        .collect();
    res
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            generate_nonsense_passwords,
            generate_schemed_passwords
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
