import customtkinter as ctk
import platform, subprocess as sp, json, os

# Commands Definition
global config
commands = open('commands.json', 'r').read()
commands = json.loads(commands)

# Config Creation
def read_config():
    cFile = open('config.json', 'r')
    config = json.loads(cFile.read())
    cFile.close()
    return config
try:
    read_config()
except FileNotFoundError:
    cFile = open('config.json', 'w')
    cFile.write('{\n    "local_folder":"./",\n    "remote_folder":"/",\n    "server_user":"user",\n    "server_ip":"0.0.0.0",\n    "server_port":"22"\n}')
    cFile.close()
    read_config()

config = read_config()

# Main Application
app = ctk.CTk()
app.title("Cian Server")
app.geometry('380x380')

tabs = ctk.CTkTabview(app)
tabs.grid(row=0, column=0, padx=15, pady=15)
tabs.add('Enviar')
tabs.add('Recibir')
tabs.add('Configuracion')

# Send and Receive Function
def file_sender(local, remote):
    sp.Popen(comandos["sender"].replace('{port}', config["server_port"]).replace('{local_path}',
        f'{config["local_folder"]}/{local}').replace('{user}', config["server_user"])
        .replace('{remote_ip}', config["server_ip"]).replace('{remote_path}'
        ,f'{config["remote_folder"]}/{remote}')
        , shell=True)
def file_receiver(remote, local):
    sp.Popen(comandos["receiver"].replace('{user}', config["server_user"]).replace('{remote_ip}'
        ,config["server_ip"]).replace('{port}', config["server_port"]).replace('{remote_path}'
        ,f'{config["remote_folder"]}/{remote}').replace('{local_path}'
        ,f'{config["local_folder"]}/{local}'), shell=True)

files = os.listdir(config["local_folder"])
if config["local_folder"] == './':
    files.remove('commands.json')
    files.remove('config.json')
    files.remove('main.py')

send_options = ctk.CTkComboBox(tabs.tab('Enviar'), values=files)
send_options.grid(row=0, column=0)
send_entry = ctk.CTkEntry(tabs.tab('Enviar'), placeholder_text="Ruta de destino")
send_entry.grid(row=0, column=1)

send_btn = ctk.CTkButton(tabs.tab('Enviar'), text="ENVIAR", command=lambda:(file_sender(send_options.get(), send_entry.get())))
send_btn.grid(row=1, column=0, padx=5, pady=10, columnspan=2, sticky='new')

receive_r_entry = ctk.CTkEntry(tabs.tab('Recibir'), placeholder_text="Ruta remota")
receive_r_entry.grid(row=0, column=0)
receive_l_entry = ctk.CTkEntry(tabs.tab('Recibir'), placeholder_text="Ruta Local")
receive_l_entry.grid(row=0, column=1)

receive_btn = ctk.CTkButton(tabs.tab('Recibir'), text="PEDIR", command=lambda:(file_receiver(receive_r_entry.get(), receive_l_entry.get())))
receive_btn.grid(row=1, column=0, padx=5, pady=10, columnspan=2, sticky='new')

# Config Tab
def save_config(local, remote, user, ip, port):
    if not local != '': local = config["local_folder"]
    if not remote != '': remote = config["remote_folder"]
    if not user != '': user = config["server_user"]
    if not ip != '': ip = config["server_ip"]
    if not port != '': port = config["server_port"]
    cFile = open('config.json', 'w')
    cFile.write('{\n'+f'    "local_folder":"{local}",\n    "remote_folder":"{remote}",\n    "server_user":"{user}",\n    "server_ip":"{ip}",\n    "server_port":"{port}"'+'\n}')
    cFile.close()

local_entry = ctk.CTkEntry(tabs.tab('Configuracion'), placeholder_text="Directorio Local")
local_entry.grid(row=0, column=0)
remote_entry = ctk.CTkEntry(tabs.tab('Configuracion'), placeholder_text="Directorio Remoto")
remote_entry.grid(row=0, column=1)
user_entry = ctk.CTkEntry(tabs.tab('Configuracion'), placeholder_text="Usuario del Servidor")
user_entry.grid(row=1, column=0)
ip_entry = ctk.CTkEntry(tabs.tab('Configuracion'), placeholder_text="Ip del Servidor")
ip_entry.grid(row=1, column=1)
port_entry = ctk.CTkEntry(tabs.tab('Configuracion'), placeholder_text="Puerto del Servidor")
port_entry.grid(row=2, column=0)
config_sub_btn = ctk.CTkButton(tabs.tab('Configuracion'), text="Aplicar Configuracion", command=lambda:(save_config(local_entry.get(),
        remote_entry.get(), user_entry.get(), ip_entry.get(), port_entry.get())))
config_sub_btn.grid(row=2, column=1)

# System Detection
system = platform.system()
if system == 'Windows':
    comandos = commands["windows"] 
elif system == 'Linux':
    comandos = commands["linux"]

app.mainloop()