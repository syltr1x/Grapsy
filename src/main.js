const { invoke } = window.__TAURI__.tauri;
const { open } = window.__TAURI__.dialog;
const { readDir } = window.__TAURI__.fs;

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
const foldersButton = document.querySelector('#folder_button');
const sendButton = document.querySelector('#send_files');
let filesList = null;

// Open file explorer to select file/s
if (filesButton !== null) {
  // Process every file (write and store path)
  function processFile(files) {
    document.querySelector('#preview').innerHTML = "";
    Object.keys(files).forEach(key => {
      const fileData = `<div class="file-container">
      <div class="status">
        <span class="status-text"><b>${key}: </b> ${files[key]}...</span>
      </div>
      </div>`;
      const html = document.querySelector('#preview');
      html.innerHTML = fileData + html.innerHTML;
    });
  }

  // Process files
  filesButton.addEventListener('click', async () => {
    const selectedFilePath = await open({
      multiple: true
    })

    if (selectedFilePath) {
      processFile(selectedFilePath);
      filesList = selectedFilePath;
      sendButton.disabled = false;
    }
  });

  // Process folders
  foldersButton.addEventListener('click', async() => {
    const selectedFolderPath = await open({
      directory: true,
      multiple: true
    })

    if (selectedFolderPath) {
      processFile(selectedFolderPath);
      filesList = selectedFolderPath;
      sendButton.disabled = false;
    }
  });

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
  sendButton.addEventListener('click', async() => {
    const remotePath = document.querySelector('#remote_path');
    if (filesList === null) {alert('Error: No files to upload \n Please select files first')}
    for (let file in filesList) {
      let response = await invoke('send_file', { archivoLocal: filesList[file] , archivoRemoto: remotePath.value})
      alert(response)
    }
  })
}

// --- Get server info (for info.html)
const storageBar = document.querySelector('#storage_bar')
if (storageBar != null) {
  async function get_server_info() {
    const response = await invoke('get_server_info')
    let server
    try {
      server = JSON.parse(response)
    } catch {
      alert(response)
      return 1
    }

    if (server.address != "0.0.0.0") {
      document.querySelector('#address').innerHTML = `Address: ${server.address}:${server.port}`;
    }
    if (server.status) {
      document.querySelector('#status').innerHTML = "Server status: On";
    }
    if (server.authenticated) {
      let storage_width = Math.round(server.storage.used_size*100/server.storage.total_size)
      document.querySelector('#storage').innerHTML = `Server storage: ${server.storage.used_size}GB/${server.storage.total_size}GB`;
      document.querySelector('#storage_bar').style.setProperty('--storage-width', `${storage_width}%`);
      document.querySelector('#key').innerHTML = "Server key status: Authenticated";
      document.querySelector('#warn').hidden = true;
      document.querySelector('#rd-create').hidden = true;
    }
  }

  const checkKey = await invoke('check_rsa_key')
  if (checkKey) {
    get_server_info()
  }
}

// --- Download file from server ---
const receiveButton = document.querySelector('#receive_file')
if (receiveButton != undefined) {
  receiveButton.addEventListener('click', async() => {
    const filePath = document.querySelector('#file_path');
    const remotePath = document.querySelector('#remote_path');
    if (filePath.value.length < 1) {
      alert(`La ruta del archivo no puede estar vacia.`);
      return 1
    }
    let res = await invoke('receive_file', { archivoLocal: filePath.value, archivoRemoto: remotePath.value })
    alert(res)
  })
}

// --- Update config ---
const updconfigButton = document.querySelector('#write_config')
if (updconfigButton != undefined) {
  async function fill_values() {
    const user_field = document.querySelector('#server_user');
    const host_field = document.querySelector('#server_address');
    const port_field = document.querySelector('#server_port');
    const local_field = document.querySelector('#local_path');
    const remote_field = document.querySelector('#remote_path');

    const data = JSON.parse(await invoke('read_config'))

    user_field.value = data.user;
    host_field.value = data.host;
    port_field.value = data.port;
    local_field.value = data.local_path;
    remote_field.value = data.remote_path;
  }
  addEventListener('DOMContentLoaded', () => {
    fill_values()
  })
  updconfigButton.addEventListener('click', async() => {
    const user = document.querySelector('#server_user').value;
    const address = document.querySelector('#server_address').value;
    const port = document.querySelector('#server_port').value;
    const localFolder = document.querySelector('#local_path').value;
    const remoteFolder = document.querySelector('#remote_path').value;

    let res = await invoke('write_config', {user: user, host: address, port: port, localFolder: localFolder, remoteFolder: remoteFolder})
    alert(res)
  })
}
// -- Authenticate user in server ---
const newkeyButton = document.querySelector('#create_key')
if (newkeyButton != undefined) {
  // Charge configured values
  async function fill_values() {
    const user_field = document.querySelector('#username');
    const host_field = document.querySelector('#address');
    const port_field = document.querySelector('#port');
    const data = JSON.parse(await invoke('read_config'))

    user_field.value = data.user;
    host_field.value = data.host;
    port_field.value = data.port;
  }
  addEventListener('DOMContentLoaded', () => {
    fill_values()
  })

  newkeyButton.addEventListener('click', async() => {
    const description = document.querySelector('#description').value;
    const username = document.querySelector('#username').value;
    const password = document.querySelector('#password').value;
    const address = document.querySelector('#address').value;
    const port = document.querySelector('#port').value;

    let res = await invoke('send_key', { desc: description, user: username, password: password, address: address, port: port})
    alert(res)
  })
}
