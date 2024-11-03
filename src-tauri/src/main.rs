#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod cian;

// Commands
#[tauri::command]
fn send_file(archivo_local: &str, archivo_remoto: &str) -> Result<String, String> {
    let compressed_file = cian::compress_file(&archivo_local).unwrap();
    match cian::send_file(&compressed_file, &archivo_remoto) {
        Ok(response) => Ok(response),
        Err(_e) => Err("Error".to_string()),
    }
}
#[tauri::command]
fn get_content_folder(remote_folder: &str) -> Result<String, String> {
    match cian::get_content_folder(remote_folder) {
        Ok(res) => Ok(res),
        Err(e) => Err(format!("Error getting folder info: {}", e)),
    }
}
#[tauri::command]
fn receive_file(archivo_local: &str, archivo_remoto: &str) -> Result<String, String> {
    let downloaded_file = cian::receive_file(&archivo_local, &archivo_remoto).unwrap();
    match cian::decompress_file(&downloaded_file.to_owned()) {
        Ok(res) => Ok(res),
        Err(_e) => Err("Error".to_string()),
    }
}
#[tauri::command]
fn read_config() -> Result<String, String> {
    match cian::read_config_json() {
        Ok(config) => Ok(config),
        Err(e) => Err(format!("Error reading config: {}", e)),
    }
}
#[tauri::command]
fn write_config(user: &str, host: &str, port: &str, local_folder: &str, remote_folder: &str) -> Result<String, String> {
    match cian::write_config(user, host, port, local_folder, remote_folder) {
        Ok(res) => Ok(res),
        Err(e) => Err(format!("Error writing config: {}", e)),
    }
}
#[tauri::command]
fn send_key(desc: &str, user: &str, password: &str, address: &str, port: &str) -> Result<String, String> {
    match cian::send_key(desc, user, password, address, port) {
        Ok(res) => Ok(res),
        Err(e) => Err(format!("Error authenticating in server: {}", e)),
    }
}
#[tauri::command]
fn get_server_info() -> Result<String, String> {
    match cian::server_info() {
        Ok(server) => Ok(server),
        Err(e) => Err(format!("Error reading config: {}", e)),
    }
}
#[tauri::command]
fn check_rsa_key() -> Result<bool, bool> {
    let key_exist = cian::check_rsa_key().unwrap();
    Ok(key_exist)
}
#[tauri::command]
fn is_file(file_path: &str) -> Result<bool, bool> {
    let is_file = cian::validate_file_type(&file_path).unwrap();
    Ok(is_file)
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![send_file, receive_file, read_config, write_config, send_key, get_server_info, check_rsa_key, get_content_folder, is_file])
    .run(tauri::generate_context!())
    .expect("failed to run app");
}
