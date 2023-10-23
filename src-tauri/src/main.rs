// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate font_kit;
use font_kit::source::SystemSource;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command


#[tauri::command]
fn get_fonts() -> String {
    let source = SystemSource::new();
    let fonts = source.all_fonts().unwrap();
    let mut font_list: Vec<String> = Vec::new();

    for font in fonts {
        if let Ok(font) = font.load() {
            font_list.push(font.full_name());
        }
    }
    let partial_font_list = &font_list[1..10];
    return partial_font_list.to_vec().join(", ");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_fonts])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
