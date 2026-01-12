// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use rand::Rng;
mod romaji;
mod names;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn generate_username(length: usize) -> String {
    let mut rng = rand::rng();
    let mut username = String::with_capacity(length);

    static ASCII_LOWER: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    for _ in 0..length {
        let c = ASCII_LOWER[rng.random_range(0..ASCII_LOWER.len())];
        username.push(c);
    }

    username
}

#[tauri::command]
fn generate_romaji_username(length: usize) -> String {
    let mut rng = rand::rng();
    let mut username = String::with_capacity(length);

    let alphabets: [&[&str]; 2] = [
        &romaji::ROMAJI_ASCII_LOWER,
        &romaji::ROMAJI_YOON_ASCII_LOWER
    ];

    for _ in 0..length {
        let alphabet = alphabets[rng.random_range(0..alphabets.len())];
        let c = alphabet[rng.random_range(0..alphabet.len())];
        username.push_str(c);
    }

    username
}

#[tauri::command]
fn generate_random_names(length: usize) -> String {
    let mut rng = rand::rng();
    let mut username = String::with_capacity(length);

    let alphabets: [&[&str]; 1] = [
        &names::ENGLISH_MASCULINE_GIVEN_NAMES,
    ];

    for _ in 0..length {
        let alphabet = alphabets[rng.random_range(0..alphabets.len())];
        let c = alphabet[rng.random_range(0..alphabet.len())];
        username.push_str(c);
    }

    username
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            generate_username,
            generate_romaji_username,
            generate_random_names
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
