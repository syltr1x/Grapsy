#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod cian;
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
    cian::send_file(&archivo_local, &archivo_remoto);
}
#[tauri::command]
fn recibir(archivo_local: &str, archivo_remoto: &str) {
    let _ = cian::receive_file(&archivo_local, &archivo_remoto);
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![comprimir, descomprimir, enviar, recibir])
    .run(tauri::generate_context!())
    .expect("failed to run app");
}
