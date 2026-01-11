// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use rand::Rng;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn generate_username(length: usize) -> String {
    let mut rng = rand::rng();
    let mut username = String::with_capacity(length);

    static ASCII_LOWER: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 
        'f', 'g', 'h', 'i', 'j', 
        'k', 'l', 'm', 'n', 'o',
        'p', 'q', 'r', 's', 't', 
        'u', 'v', 'w', 'x', 'y', 
        'z',
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

    static ROMAJI_ASCII_LOWER: [&str; 45] = [
        "a", "i", "u", "e", "o",
        "ka", "ki", "ku", "ke", "ko",
        "sa", "shi", "su", "se", "so",
        "ta", "chi", "tsu", "te", "to",
        "na", "ni", "nu", "ne", "no",
        "ha", "hi", "fu", "he", "ho",
        "ma", "mi", "mu", "me", "mo",
        "ya", "yu", "yo",
        "ra", "ri", "ru", "re", "ro",
        "wa", "wo",
    ];

    for _ in 0..length {
        let c = ROMAJI_ASCII_LOWER[rng.random_range(0..ROMAJI_ASCII_LOWER.len())];
        username.push_str(c);
    }

    username
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![generate_username])
        .invoke_handler(tauri::generate_handler![generate_romaji_username])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
