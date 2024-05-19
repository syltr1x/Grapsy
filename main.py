import customtkinter as ctk
import platform, subprocess as sp, json, os

# Commands Definition
global config
global comandos
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

# Sender Frame
class SenderFrame(ctk.CTkFrame):
    def __init__(self, master, **kwargs):
        super().__init__(master, **kwargs)

        files = os.listdir(config["local_folder"])
        if config["local_folder"] == './':
            files.remove('commands.json')
            files.remove('config.json')
            files.remove('main.py')

        self.send_options = ctk.CTkComboBox(self, values=files)
        self.send_options.grid(row=0, column=0)
        self.send_entry = ctk.CTkEntry(self, placeholder_text="Ruta de destino")
        self.send_entry.grid(row=0, column=1)

        self.send_btn = ctk.CTkButton(self, text="ENVIAR", command=lambda:(self.file_sender(self.send_options.get(), self.send_entry.get())))
        self.send_btn.grid(row=1, column=0, padx=5, pady=10, columnspan=2, sticky='new')

    def file_sender(local, remote):
        sp.Popen(comandos["sender"].replace('{port}', config["server_port"]).replace('{local_path}',
        f'{config["local_folder"]}/{local}').replace('{user}', config["server_user"])
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

        self.local_entry = ctk.CTkEntry(self, placeholder_text="Directorio Local")
        self.local_entry.grid(row=0, column=0)
        self.remote_entry = ctk.CTkEntry(self, placeholder_text="Directorio Remoto")
        self.remote_entry.grid(row=0, column=1)
        self.user_entry = ctk.CTkEntry(self, placeholder_text="Usuario del Servidor")
        self.user_entry.grid(row=1, column=0)
        self.ip_entry = ctk.CTkEntry(self, placeholder_text="Ip del Servidor")
        self.ip_entry.grid(row=1, column=1)
        self.port_entry = ctk.CTkEntry(self, placeholder_text="Puerto del Servidor")
        self.port_entry.grid(row=2, column=0)
        self.config_sub_btn = ctk.CTkButton(self, text="Aplicar Configuracion", command=lambda:(self.save_config(self.local_entry.get(),
                self.remote_entry.get(), self.user_entry.get(), self.ip_entry.get(), self.port_entry.get())))
        self.config_sub_btn.grid(row=2, column=1)

    def save_config(local, remote, user, ip, port):
        if not local != '': local = config["local_folder"]
        if not remote != '': remote = config["remote_folder"]
        if not user != '': user = config["server_user"]
        if not ip != '': ip = config["server_ip"]
        if not port != '': port = config["server_port"]
        cFile = open('config.json', 'w')
        cFile.write('{\n'+f'    "local_folder":"{local}",\n    "remote_folder":"{remote}",\n    "server_user":"{user}",\n    "server_ip":"{ip}",\n    "server_port":"{port}"'+'\n}')
        cFile.close()

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

system = platform.system()
if system == 'Windows':
    comandos = commands["windows"] 
elif system == 'Linux':
    comandos = commands["linux"]

app = App()
app.mainloop()