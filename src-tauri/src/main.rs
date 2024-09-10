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
fn receive_file(archivo_local: &str, archivo_remoto: &str) {
    let downloaded_file = cian::receive_file(&archivo_local, &archivo_remoto).unwrap();
    let _ = cian::decompress_file(&downloaded_file.to_owned());
}
#[tauri::command]
fn read_config() -> Result<String, String> {
    match cian::read_config_json() {
        Ok(config) => Ok(config),
        Err(e) => Err(format!("Error reading config: {}", e)),
    }
}
#[tauri::command]
fn write_config(user: &str, host: &str, port: &str, local_folder: &str, remote_folder: &str) {
    let _ = cian::write_config(user, host, port, local_folder, remote_folder);
}
#[tauri::command]
fn send_key(desc: &str, user: &str, password: &str, address: &str, port: &str) {
    let _ = cian::send_key(desc, user, password, address, port);
}
#[tauri::command]
fn get_server_info() -> Result<String, String>{
    match cian::server_info() {
        Ok(server) => Ok(server),
        Err(e) => Err(format!("Error reading config: {}", e)),
    }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![send_file, receive_file, read_config, write_config, send_key, get_server_info])
    .run(tauri::generate_context!())
    .expect("failed to run app");
}
