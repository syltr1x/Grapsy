## Powerhsell

[!] Abrir powershell como administrador (INSTALACION NECESARIA)
> Add-WindowsCapability -Online -Name OpenSSH.Client*

-------------------------- ENVIAR --------------------------
[1] Comando para enviar archivo
> scp -P {port} {local_file_path} user@remote_ip:{remote_file_path}
[2] Ejemplo de comando utilizado
> scp -P 18542 test.txt cian@0.tcp.sa.ngrok.io:/home/cian/server_test/file.txt
------------------------- RECIBIR -------------------------
[1] Comando para recibir archivo
> scp -P {port} user@remote_ip:{remote_file_path} {local_file_path}
[2] Ejemplo de comando utilizado
> scp -P 18542 cian@0.tcp.sa.ngrok.io:/home/cian/Descargas/pruebas.md recibido


## Linux
------------------------- ENVIAR ---------------------------
[1] Comando para enviar archivo
> cat {file} | ssh -p {port} user@remote_ip "cat > {dest_file_path}"
[2] Ejemplo de comando Utilizado
> cat saludos | ssh -p 18542 cian@0.tcp.sa.ngrok.io "cat > /home/cian/server_test/file.txt" 

------------------------ RECIBIR ---------------------------
[1] Comando para recibir un archivo
> ssh user@remote_ip -p {port} "cat {remote_file_path}" > {dest_file_path}
[2] Ejemplo de comando utilizado
> ssh cian@0.tcp.sa.ngrok.io -p 18542 "cat /home/cian/Descargas/pruebas.md" > recibido