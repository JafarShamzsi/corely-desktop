#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to AI Training Desktop!", name)
}
