use std::path::PathBuf;
use chrono;

/// Format file size in human-readable format
pub fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

/// Format duration in human-readable format
pub fn format_duration(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;
    
    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}

/// Sanitize filename for cross-platform compatibility
pub fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .map(|c| match c {
            '<' | '>' | ':' | '"' | '|' | '?' | '*' | '\\' | '/' => '_',
            c if c.is_control() => '_',
            c => c,
        })
        .collect()
}

/// Ensure directory exists, create if it doesn't
pub fn ensure_directory_exists(path: &PathBuf) -> Result<(), std::io::Error> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

/// Generate a unique filename by appending a number if the file already exists
pub fn generate_unique_filename(base_path: &PathBuf, filename: &str) -> PathBuf {
    let mut path = base_path.join(filename);
    let mut counter = 1;
    
    while path.exists() {
        if let Some(stem) = std::path::Path::new(filename).file_stem() {
            if let Some(extension) = std::path::Path::new(filename).extension() {
                let new_filename = format!("{}_{}.{}", 
                    stem.to_string_lossy(), 
                    counter, 
                    extension.to_string_lossy()
                );
                path = base_path.join(new_filename);
            } else {
                let new_filename = format!("{}_{}", stem.to_string_lossy(), counter);
                path = base_path.join(new_filename);
            }
        }
        counter += 1;
    }
    
    path
}

/// Calculate SHA-256 hash of file
pub fn calculate_file_hash(file_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::Read;
    
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    
    // TODO: Implement actual SHA-256 hashing
    // For now, return a placeholder
    Ok(format!("sha256:{:x}", buffer.len()))
}

/// Validate email address format
pub fn is_valid_email(email: &str) -> bool {
    email.contains('@') && email.contains('.') && email.len() > 5
}

/// Generate a random string of given length
pub fn generate_random_string(length: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/// Convert memory from bytes to megabytes
pub fn bytes_to_mb(bytes: u64) -> u64 {
    bytes / (1024 * 1024)
}

/// Convert memory from megabytes to bytes
pub fn mb_to_bytes(mb: u64) -> u64 {
    mb * 1024 * 1024
}
