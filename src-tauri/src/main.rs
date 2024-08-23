#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod cian;

// Commands
#[tauri::command]
fn comprimir(directorio: &str) {
    println!("Directorio: {}", directorio);
    let _ = cian::compress_file(&directorio);
}
#[tauri::command]
fn descomprimir(directorio: &str) {
    let _ = cian::decompress_file(&directorio);
}
#[tauri::command]
fn enviar(archivo_local: &str, archivo_remoto: &str) {
    println!("Enviando...");
    let _ = cian::send_file(&archivo_local, &archivo_remoto);
}
#[tauri::command]
fn recibir(archivo_local: &str, archivo_remoto: &str) {
    let _ = cian::receive_file(&archivo_local, &archivo_remoto);
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

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![comprimir, descomprimir, enviar, recibir, read_config, write_config, send_key])
    .run(tauri::generate_context!())
    .expect("failed to run app");
}
