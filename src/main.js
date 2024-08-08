const { invoke } = window.__TAURI__.tauri;
const texto = document.querySelector('#parrafo');

function compress_file() {
  const filePath = document.querySelector('#file_path');
  if (filePath.value.length < 1) {
    texto.innerHTML = `La ruta del archivo no puede estar vacia.`;
    return 1
  }
  texto.innerHTML = `comprimiendo: ${filePath.value}...`;
  invoke('comprimir', { directorio: filePath.value })
  texto.innerHTML = `${filePath.value} Se comprimio correctamente.`;
}
function decompress_file() {
  if (filePath.value.length < 1) {
    texto.innerHTML = `La ruta del archivo no puede estar vacia.`;
    return 1
  }
  const filePath = document.querySelector('#file_path');
  texto.innerHTML = `descomprimiendo: ${filePath.value}...`;
  invoke('descomprimir', { directorio: filePath.value })
  texto.innerHTML = `${filePath.value} Se descomprimio correctamente.`;
}
function send_file() {
  texto.innerHTML = "Sorry. Not working yet :("
}
function receive_file() {
  texto.innerHTML = "Sorry. Not working yet :("
}
