const { appWindow, invoke } = window.__TAURI__.tauri;

const texto = document.querySelector('#parrafo');

// Front Functions
const sourceFile = document.querySelector('#file_path');
const destFile = document.querySelector('#remote_path');
//sourceFile.addEventListener('input', () => {
//  destFile.value = sourceFile.value;
//})
// --- Show Sidebar ---
const menuButton = document.querySelector('#toggle');
const menu = document.querySelector('#sidebar');
menuButton.addEventListener('click', () => {
  if (menu.classList.contains('close')) {
    menu.classList.remove('close');
  } else {
    menu.classList.add('close')
  }
})

// --- Dark Mode ---
const contenido = document.querySelector('#content');
const darkModeButton = document.querySelector('#toggle_darkmode');

let dark_mode = localStorage.getItem("dark_mode");
if (dark_mode && dark_mode === "on") {
  darkModeButton.classList.add('fa-moon')
  darkModeButton.classList.add('active')
  menu.classList.add('dark')
} else {
  darkModeButton.classList.add('fa-sun')
}

darkModeButton.addEventListener('click', () => {
  darkModeButton.classList.toggle("active")
  if (darkModeButton.classList.contains('fa-moon')) {
    darkModeButton.classList.remove('fa-moon')
    darkModeButton.classList.add('fa-sun')
    menu.classList.remove('dark')
    localStorage.setItem("dark_mode", "off")
  } else {
    darkModeButton.classList.add('fa-moon')
    darkModeButton.classList.remove('fa-sun')
    menu.classList.add('dark')
    localStorage.setItem("dark_mode", "on")
  }
})

// Cian Back Functions
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
