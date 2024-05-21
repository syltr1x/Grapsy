import customtkinter as ctk
import platform, subprocess as sp, json, os
from tkinterdnd2 import TkinterDnD, DND_FILES

# Commands Definition
global comandos
commands = open('commands.json', 'r').read()
commands = json.loads(commands)

# Config Creation
def set_config():
    global config
    cFile = open('config.json', 'r')
    config = json.loads(cFile.read())
    cFile.close()
try:
    set_config()
except FileNotFoundError:
    cFile = open('config.json', 'w')
    cFile.write('{\n    "local_folder":"./",\n    "remote_folder":"/",\n    "server_user":"user",\n    "server_ip":"0.0.0.0",\n    "server_port":"22"\n}')
    cFile.close()
    set_config()

# Sender Frame
class SenderFrame(ctk.CTkFrame):
    def __init__(self, master, **kwargs):
        super().__init__(master, **kwargs)
        self.file_list = []

        self.rowconfigure(0, weight=1)
        self.columnconfigure(0, weight=0)

        self.send_files = TkinterDnD.CTkFrame(self)
        self.send_files.grid(row=0, column=0, padx=5, pady=5, sticky='nsew')
        self.send_files.rowconfigure(1, weight=1)
        self.send_files.columnconfigure(0, weight=1)

        self.text_widget = ctk.CTkLabel(self.send_files, text="Arrastra aquí los archivos")
        self.text_widget.pack(expand=True)
        self.text_widget.drop_target_register(DND_FILES)
        self.text_widget.dnd_bind('<<Drop>>', self.drop)

        self.send_options = ctk.CTkComboBox(self, values=[])
        self.send_options.grid(row=1, column=0)
        self.send_entry = ctk.CTkEntry(self, placeholder_text="Ruta de destino")
        self.send_entry.grid(row=1, column=0)

        self.send_btn = ctk.CTkButton(self, text="ENVIAR", command=lambda:(self.file_sender(self.file_list, self.send_entry.get())))
        self.send_btn.grid(row=1, column=0, padx=5, pady=10, columnspan=2, sticky='new')

    def drop(self, event):
        if event.data.count('{') >= 1:
            closing_bracket_index = event.data.find('}')
            split_index = closing_bracket_index + 1
            while split_index < len(event.data) and event.data[split_index] == ' ':
                split_index += 1
            part1 = event.data[:split_index].strip()
            part2 = event.data[split_index:].strip()
            files = [part1, part2]
            self.file_list = [i.replace('}', '"').replace('{', '"') for i in files if len(i) > 0]
        else:
            self.file_list = event.data.split()
        self.file_paths = '\n'.join(self.file_list)
        self.text_widget.configure(state='disabled', text=self.file_paths)

    def file_sender(self, local, remote):
        if local == []: return 0
        local = " ".join(local)
        sp.Popen(comandos["sender"].replace('{port}', config["server_port"]).replace('{local_path}',
        local).replace('{user}', config["server_user"])
        .replace('{remote_ip}', config["server_ip"]).replace('{remote_path}'
        ,f'{config["remote_folder"]}/{remote}')
        , shell=True)

# Receiver Frame
class ReceiverFrame(ctk.CTkFrame):
    def __init__(self, master, **kwargs):
        super().__init__(master, **kwargs)

        self.remote_entry = ctk.CTkEntry(self, placeholder_text="Ruta remota")
        self.remote_entry.grid(row=0, column=0)
        self.local_entry = ctk.CTkEntry(self, placeholder_text="Ruta Local")
        self.local_entry.grid(row=0, column=1)

        self.receive_btn = ctk.CTkButton(self, text="PEDIR", command=lambda:(self.file_receiver(self.remote_entry.get(), self.local_entry.get())))
        self.receive_btn.grid(row=1, column=0, padx=5, pady=10, columnspan=2, sticky='new')

    def file_receiver(remote, local):
        sp.Popen(comandos["receiver"].replace('{user}', config["server_user"]).replace('{remote_ip}'
        ,config["server_ip"]).replace('{port}', config["server_port"]).replace('{remote_path}'
        ,f'{config["remote_folder"]}/{remote}').replace('{local_path}'
        ,f'{config["local_folder"]}/{local}'), shell=True)

class ConfigFrame(ctk.CTkFrame):
    def __init__(self, master, **kwargs):
        super().__init__(master, **kwargs)

        self.local_label = ctk.CTkLabel(self, text="Direcotrio Local")
        self.local_label.grid(row=0, column=0, padx=5, pady=5)
        self.local_entry = ctk.CTkEntry(self, placeholder_text=config["local_folder"])
        self.local_entry.grid(row=0, column=1, padx=5, pady=5)
        
        self.remote_label = ctk.CTkLabel(self, text="Directorio Remoto")
        self.remote_label.grid(row=1, column=0, padx=5, pady=5)
        self.remote_entry = ctk.CTkEntry(self, placeholder_text=config["remote_folder"])
        self.remote_entry.grid(row=1, column=1, padx=5, pady=5)
        
        self.user_label = ctk.CTkLabel(self, text="Usuario del Servidor")
        self.user_label.grid(row=2, column=0, padx=5, pady=5)
        self.user_entry = ctk.CTkEntry(self, placeholder_text=config["server_user"])
        self.user_entry.grid(row=2, column=1, padx=5, pady=5)

        self.ip_label = ctk.CTkLabel(self, text="Ip del Servidor")
        self.ip_label.grid(row=3, column=0, padx=5, pady=5)
        self.ip_entry = ctk.CTkEntry(self, placeholder_text=config["server_ip"])
        self.ip_entry.grid(row=3, column=1, padx=5, pady=5)

        self.port_label = ctk.CTkLabel(self, text="Puerto del Servidor")
        self.port_label.grid(row=4, column=0, padx=5, pady=5)
        self.port_entry = ctk.CTkEntry(self, placeholder_text=config["server_port"])
        self.port_entry.grid(row=4, column=1, padx=5, pady=5)

        self.config_sub_btn = ctk.CTkButton(self, text="Aplicar Configuracion", command=lambda:(self.save_config(self.local_entry.get(),
                self.remote_entry.get(), self.user_entry.get(), self.ip_entry.get(), self.port_entry.get())))
        self.config_sub_btn.grid(row=5, column=1, padx=5, pady=5)

    def save_config(self, local, remote, user, ip, port):
        if not local != '': local = config["local_folder"]
        if not remote != '': remote = config["remote_folder"]
        if not user != '': user = config["server_user"]
        if not ip != '': ip = config["server_ip"]
        if not port != '': port = config["server_port"]
        cFile = open('config.json', 'w')
        cFile.write('{\n'+f'    "local_folder":"{local}",\n    "remote_folder":"{remote}",\n    "server_user":"{user}",\n    "server_ip":"{ip}",\n    "server_port":"{port}"'+'\n}')
        cFile.close()
        set_config()
        self.local_entry.configure(placeholder_text=config["local_folder"])
        self.remote_entry.configure(placeholder_text=config["remote_folder"])
        self.user_entry.configure(placeholder_text=config["server_user"])
        self.ip_entry.configure(placeholder_text=config["server_ip"])
        self.port_entry.configure(placeholder_text=config["server_port"])

# Main Application
class App(ctk.CTk):
    def __init__(self):
        super().__init__()

        self.title("Cian Server")
        self.geometry('480x480')

        self.navigation_frame = ctk.CTkFrame(self)
        self.navigation_frame.grid(row=0, column=0, padx=(10, 5), pady=15, sticky="nsew")
        self.navigation_frame.grid_rowconfigure(4, weight=1)

        self.navigation_frame_label = ctk.CTkLabel(self.navigation_frame, text="NAVIGATION",
                compound="left", font=ctk.CTkFont(size=15, weight="bold"))
        self.navigation_frame_label.grid(row=0, column=0, padx=20, pady=20)

        self.sender_button = ctk.CTkButton(self.navigation_frame, corner_radius=0, height=40, border_spacing=10, text="Subir Archivos",
                fg_color="transparent", text_color=("gray10", "gray90"), hover_color=("gray70", "gray30"),
                anchor="w", command=self.sender_button_event)
        self.sender_button.grid(row=1, column=0, sticky="ew")

        self.receiver_button = ctk.CTkButton(self.navigation_frame, corner_radius=0, height=40, border_spacing=10, text="Descargar Archivos",
                fg_color="transparent", text_color=("gray10", "gray90"), hover_color=("gray70", "gray30"),
                anchor="w", command=self.receiver_button_event)
        self.receiver_button.grid(row=2, column=0, sticky="ew")

        self.config_button = ctk.CTkButton(self.navigation_frame, corner_radius=0, height=40, border_spacing=10, text="Configuracion",
                fg_color="transparent", text_color=("gray10", "gray90"), hover_color=("gray70", "gray30"),
                anchor="w", command=self.config_button_event)
        self.config_button.grid(row=3, column=0, sticky="ew", pady=(0, 5))

        self.sender_frame = SenderFrame(self)
        self.second_frame = ReceiverFrame(self)
        self.config_frame = ConfigFrame(self)

        # select default frame
        self.select_frame_by_name("sender")

    def select_frame_by_name(self, name):
        self.sender_button.configure(fg_color=("gray75", "gray25") if name == "sender" else "transparent")
        self.receiver_button.configure(fg_color=("gray75", "gray25") if name == "receiver" else "transparent")
        self.config_button.configure(fg_color=("gray75", "gray25") if name == "config" else "transparent")

        if name == "sender": self.sender_frame.grid(row=0, column=1, padx=(5, 15), pady=15, sticky="nsew")
        else: self.sender_frame.grid_forget()
        if name == "receiver": self.second_frame.grid(row=0, column=1, padx=(5, 15), pady=15, sticky="nsew")
        else: self.second_frame.grid_forget()
        if name == "config": self.config_frame.grid(row=0, column=1, padx=(5, 15), pady=15, sticky="nsew")
        else: self.config_frame.grid_forget()

    def sender_button_event(self):
        self.select_frame_by_name("sender")
    def receiver_button_event(self):
        self.select_frame_by_name("receiver")
    def config_button_event(self):
        self.select_frame_by_name("config")

set_config()
system = platform.system()
if system == 'Windows':
    comandos = commands["windows"] 
elif system == 'Linux':
    comandos = commands["linux"]

app = App()
app.mainloop()