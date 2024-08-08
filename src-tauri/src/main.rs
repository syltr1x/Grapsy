#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod cian;
#[tauri::command]
fn comprimir(directorio: &str) {
    println!("Directorio: {}", directorio);
    let _unused = cian::compress_file(&directorio);
}
#[tauri::command]
fn descomprimir(directorio: &str) {
    let _unused = cian::decompress_file(&directorio);
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![comprimir, descomprimir])
    .run(tauri::generate_context!())
    .expect("failed to run app");
}
