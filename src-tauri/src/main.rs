#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use passwordo::gen_methods::*;
use passwordo::password_info::PasswordInfo;

#[tauri::command]
fn generate_random_bytes_passwords(amount: usize, length: usize) -> Vec<PasswordInfo> {
    let res: Vec<PasswordInfo> = (0..amount).map(|_| gen_random_bytes_hex(length)).collect();
    res
}

#[tauri::command]
fn generate_nonsense_passwords(
    amount: usize,
    length: usize,
    use_special: bool,
) -> Vec<PasswordInfo> {
    let res: Vec<PasswordInfo> = (0..amount)
        .map(|_| gen_nonsense(length, use_special))
        .collect();
    res
}

#[tauri::command]
fn generate_schemed_passwords(
    amount: usize,
    hash_length: usize,
    is_inverted: bool,
    list_of_words: Vec<String>,
    use_special: bool,
) -> Vec<PasswordInfo> {
    if list_of_words.is_empty() {
        return vec![];
    }

    let mut words = list_of_words;

    let res: Vec<PasswordInfo> = (0..amount)
        .map(|_| gen_schemed(hash_length, is_inverted, &mut words, use_special))
        .collect();
    res
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            generate_nonsense_passwords,
            generate_schemed_passwords,
            generate_random_bytes_passwords
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
