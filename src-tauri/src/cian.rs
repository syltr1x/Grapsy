use dirs;
use std::process::Command;
use std::fs::{File, remove_file, write};
use std::io::{stdin, stdout, BufReader, BufWriter, Read, Result, Write, BufRead, ErrorKind, Error};
use brotli::{Decompressor, CompressorWriter};
use std::path::Path;

pub struct Config {
    user: String,
    host: String,
    port: u16,
    local_path: String,
    remote_path: String
}

pub fn read_config() -> std::io::Result<Config> {
    if Path::new("cian.conf").exists() {
        let file = File::open("cian.conf")?;
        let reader = BufReader::new(file);

        let mut user = String::new();
        let mut host = String::new();
        let mut port = String::new();
        let mut local_path = String::new();
        let mut remote_path = String::new();

        for line in reader.lines() {
            let line = line?;
            if line.starts_with("user=") {
                user = line[5..].to_string();
            } else if line.starts_with("host=") {
                host = line[5..].to_string();
            } else if line.starts_with("port=") {
                port = line[5..].to_string();
            } else if line.starts_with("local_path=") {
                local_path = line[11..].to_string();
            } else if line.starts_with("remote_path=") {
                remote_path = line[12..].to_string();
            }
        }

        Ok(Config {
            user,
            host,
            port:port.trim().parse().unwrap(),
            local_path,
            remote_path
        })
    } else {
        let mut file = File::create("cian.conf")?;
        let mut user = String::new();
        let mut host = String::new();
        let mut port = String::new();
        let mut local_path = String::new();
        let mut remote_path = String::new();

        print!("Server Username >> ");
        stdout().flush()?;
        stdin().read_line(&mut user)?;

        print!("Server Address >> ");
        stdout().flush()?;
        stdin().read_line(&mut host)?;

        print!("Server Port >> ");
        stdout().flush()?;
        stdin().read_line(&mut port)?;

        print!("Local Path (search here for files to send) >> ");
        stdout().flush()?;
        stdin().read_line(&mut local_path)?;

        print!("Remote path (send here all files) >> ");
        stdout().flush()?;
        stdin().read_line(&mut remote_path)?;

        if local_path.ends_with("/") {
            local_path.pop();
        }
        if remote_path.ends_with("/") {
            remote_path.pop();
        }

        let contenido = format!("user={}\nhost={}\nport={}\nlocal_path={}\nremote_path={}", user.trim(), host.trim(), port.trim(), local_path.trim(), remote_path.trim());
        file.write_all(contenido.as_bytes())?;

        Ok(Config {
            user,
            host,
            port:port.trim().parse().unwrap(),
            local_path,
            remote_path
        })
    }
}

pub fn compress_file(input_path: &str) -> Result<()> {
    let input_file = File::open(input_path)?;
    let mut input_reader = BufReader::new(input_file);
    
    let output_path = format!("{}.brotli", input_path);
    let output_file = File::create(&output_path)?;

    let mut output_writer = BufWriter::new(output_file);
    let mut compressor = CompressorWriter::new(&mut output_writer, 4096, 11, 22);

    let mut buffer = [0u8; 4096];
    input_reader.read(&mut buffer)?;
    compressor.write(&buffer)?;
    compressor.flush()?;
    Ok(())
}

pub fn decompress_file(input_path: &str) -> Result<()> {
    let input_file = File::open(input_path)?;
    let mut input_reader = BufReader::new(input_file);
    let output_path = input_path.strip_suffix(".brotli").unwrap_or(input_path);
    let output_file = File::create(output_path)?;
    let mut output_writer = BufWriter::new(output_file);

    let mut decompressor = Decompressor::new(&mut input_reader, 4096);

    let mut buffer = [0u8; 4096];
    loop {
        match decompressor.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => output_writer.write_all(&buffer[..n])?,
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

pub fn send_file_in_chunks(input_path: &str, remote_path: &str, remote_host: &str, user: &str, remote_port: u16) -> Result<()> {
    let local_path = format!("{}.brotli", input_path);
    let input_file = File::open(local_path)?;
    let mut reader = BufReader::new(input_file);

    let mut comm = Command::new("ssh")
        .arg(format!("-p {}", remote_port))
        .arg(format!("{}@{}", user, remote_host))
        .arg(format!("cat >> {}", remote_path))
        .stdin(std::process::Stdio::piped())
        .spawn()?;

    // Obtener el manejador de la entrada estándar del proceso SSH
    let stdin = comm.stdin.as_mut().ok_or_else(|| Error::new(ErrorKind::Other, "Failed to open stdin"))?;

    // Leer el archivo y escribir en el stdin del comando SSH
    let mut buffer = [0; 4096];
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        print!("{:02x} ", bytes_read);
        stdin.write_all(&buffer[..bytes_read])?;
    }
    comm.wait()?;
    Ok(())
}

pub fn receive_file(local_path: &str, remote_path: &str, remote_host: &str, user: &str, remote_port: u16) -> Result<()> {
    let status = Command::new("ssh")
        .arg(format!("{}@{}", user, remote_host))
        .arg(format!("-p {}", remote_port))
        .arg(format!("cat {}", remote_path)) // Ejecuta 'cat' en el servidor remoto
        .output()?; // Captura la salida del comando

    if status.status.success() {
        write(local_path, status.stdout)?;
    } else {
        eprintln!("Error: {}", String::from_utf8_lossy(&status.stderr));
        return Err(Error::new(ErrorKind::Other, "Failed to execute command"));
    }
    Ok(())
}
