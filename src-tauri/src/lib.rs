// Modules
mod commands;
mod core;
mod system;
mod ml;
mod network;
mod security;
mod utils;

use commands::{
    greet::greet,
    system_commands::{get_system_info},
    ml_commands::{list_available_models, load_model},
    network_commands::{discover_network_nodes, connect_to_flower_server},
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_system_info,
            list_available_models,
            load_model,
            discover_network_nodes,
            connect_to_flower_server
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
