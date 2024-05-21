## Powerhsell

[!] Abrir powershell como administrador (INSTALACION NECESARIA)
> Add-WindowsCapability -Online -Name OpenSSH.Client*

-------------------------- ENVIAR --------------------------
[1] Comando para enviar archivo
> scp -P {port} {local_file_path} user@remote_ip:{remote_file_path}
------------------------- RECIBIR -------------------------
[1] Comando para recibir archivo
> scp -P {port} user@remote_ip:{remote_file_path} {local_file_path}

## Linux
------------------------- ENVIAR ---------------------------
[1] Comando para enviar archivo
> cat {file} | ssh -p {port} user@remote_ip "cat > {dest_file_path}"
------------------------ RECIBIR ---------------------------
[1] Comando para recibir un archivo
> ssh user@remote_ip -p {port} "cat {remote_file_path}" > {dest_file_path}
