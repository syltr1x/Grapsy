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
  const filePath = document.querySelector('#file_path');
  if (filePath.value.length < 1) {
    texto.innerHTML = `La ruta del archivo no puede estar vacia.`;
    return 1
  }
  texto.innerHTML = `descomprimiendo: ${filePath.value}...`;
  invoke('descomprimir', { directorio: filePath.value })
  texto.innerHTML = `${filePath.value} Se descomprimio correctamente.`;
}
function send_file() {
  const filePath = document.querySelector('#file_path');
  const remotePath = document.querySelector('#remote_path');
  if (filePath.value.length < 1) {
    texto.innerHTML = `La ruta del archivo no puede estar vacia.`;
    return 1
  }
  texto.innerHTML = `enviando: ${filePath.value}...`;
  invoke('enviar', { archivoLocal: filePath.value , archivoRemoto: remotePath.value})
  texto.innerHTML = `${filePath.value} Se envio correctamente.`;
}
function receive_file() {
  const filePath = document.querySelector('#file_path');
  const remotePath = document.querySelector('#remote_path');
  if (filePath.value.length < 1) {
    texto.innerHTML = `La ruta del archivo no puede estar vacia.`;
    return 1
  }
  texto.innerHTML = `recibiendo: ${filePath.value}...`;
  invoke('recibir', { archivoLocal: filePath.value, archivoRemoto: remotePath.value })
  texto.innerHTML = `${filePath.value} Se recibio correctamente.`;
}
