const { appWindow, invoke } = window.__TAURI__.tauri;
const { open } = window.__TAURI__.dialog;

// --- Pre load Functions ---
const menuButton = document.querySelector('#toggle');
const menu = document.querySelector('#sidebar');

let menu_status = localStorage.getItem("menu_status");
if (menu_status && menu_status === "open") {
  menu.classList.remove('close');
  menuButton.style.rotate = '180deg'
} else {
  menu.classList.add('close');
  menuButton.style.rotate = '0deg'
}
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
addEventListener('DOMContentLoaded', () => {
  const body = document.querySelector('body');
  body.classList.remove('charge')
})
// --- Toggle Sidebar ---
menuButton.addEventListener('click', () => {
  if (menu.classList.contains('close')) {
    menu.classList.remove('close');
    menuButton.style.rotate = '180deg';
    localStorage.setItem("menu_status", "open")
  } else {
    menu.classList.add('close')
    menuButton.style.rotate = '0deg';
    localStorage.setItem("menu_status", "close")
  }
})

// --- Toggle Dark Mode ---
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

// --- Toggle Password Visibility ---
const show_button = document.querySelector('#show_icon');
if (show_button !== null) {
  const password_field = document.querySelector('#password');

  show_button.addEventListener('click', () => {
    if (show_button.classList.contains('fa-eye')) {
      show_button.classList.remove('fa-eye')
      show_button.classList.add('fa-eye-slash')
      password_field.setAttribute('type', 'text')
    } else {
      show_button.classList.remove('fa-eye-slash')
      show_button.classList.add('fa-eye')
      password_field.setAttribute('type', 'password')
    }
  })
}

// --- Select Files ---
const filesButton = document.querySelector('#file_button');
const sendButton = document.querySelector('#send_files');
let filesList = null;

// Open file explorer to select file/s
if (filesButton !== null) {
  filesButton.addEventListener('click', async () => {
    const selectedFilePath = await open({
      multiple: true
    });

    if (selectedFilePath) {
      processFile(selectedFilePath);
      filesList = selectedFilePath;
      sendButton.disabled = false;
    }
  });
}

// Process every file (write and store path)
function processFile(files) {
  document.querySelector('#preview').innerHTML = "";
  Object.keys(files).forEach(key => {
    console.log(`File path: ${files[key]}`);
    const fileData = `<div class="file-container">
      <div class="status">
        <span class="status-text"><b>${key}: </b> ${files[key]}...</span>
      </div>
    </div>`;
    const html = document.querySelector('#preview');
    html.innerHTML = fileData + html.innerHTML;
  });
}

// Button logic if no files selected
sendButton.addEventListener('mouseenter', (e) => {
  e.preventDefault()
  if (sendButton.disabled) {
    sendButton.innerHTML = '<i class="fa-solid fa-ban"></i> Please select files first';
  }
})
sendButton.addEventListener('mouseleave', (e) => {
  e.preventDefault()
  sendButton.innerHTML = '<i class="fa-solid fa-arrow-up"></i> Upload Files';
})
// Send files (if they're selected)
sendButton.addEventListener('click', () => {
  const remotePath = document.querySelector('#remote_path');
  if (filesList === null) {alert('Error: No files to upload \n Please select files first')}
  Object.keys(filesList.forEach(key => {
    invoke('enviar', { archivoLocal: filesList[key] , archivoRemoto: remotePath.value})
  }))
})

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
