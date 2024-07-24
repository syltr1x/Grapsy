use std::process::Command;
use std::fs::File;
use std::io::{self, stdin, stdout, BufReader, BufWriter, Read, Result, Write, BufRead};
use brotli::{Decompressor, CompressorWriter};
use std::path::{Path, PathBuf};

struct Config {
    user: String,
    host: String,
    port: u16,
    local_path: String,
    remote_path: String
}

fn read_config() -> std::io::Result<Config> {
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

fn brotli_path(input_path: &str) -> PathBuf {
    let input_path = Path::new(input_path);

    let mut output_path = if input_path.is_dir() {
        input_path.with_file_name(input_path.file_name().unwrap_or_default())
    } else {
        input_path.to_path_buf()
    };

    if output_path.ends_with("/") {
        output_path.pop();
    }

    output_path.set_extension("brotli");
    output_path
}

fn compress_file(input_path: &str) -> std::io::Result<()> {
    let input_file = File::open(input_path)?;
    let mut input_reader = BufReader::new(input_file);
    let output_file = File::create(brotli_path(input_path))?;
    let mut output_writer = BufWriter::new(output_file);

    // Configure the Brotli compressor
    let mut compressor = CompressorWriter::new(&mut output_writer, 4096, 11, 22);

    // Read data from the input file and write it to the compressor
    let mut buffer = Vec::new();
    input_reader.read_to_end(&mut buffer)?;
    compressor.write_all(&buffer)?;
    compressor.flush()?;

    Ok(())
}

fn decompress_file(input_path: &str) -> io::Result<()> {
    let input_file = File::open(input_path)?;
    let mut input_reader = io::BufReader::new(input_file);
    let output_path = input_path.strip_suffix(".brotli").unwrap_or(input_path);
    let output_file = File::create(output_path)?;
    let mut output_writer = io::BufWriter::new(output_file);

    // Crear un descompresor Brotli
    let mut decompressor = Decompressor::new(&mut input_reader, 4096);

    // Leer y escribir los datos descomprimidos
    let mut buffer = [0; 4096];
    loop {
        match decompressor.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => output_writer.write_all(&buffer[..n])?,
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

fn send_file_in_chunks(local_path: &str, remote_path: &str, remote_host: &str, user: &str, remote_port: u16) -> io::Result<()> {
    let file = File::open(brotli_path(local_path))?;
    let mut reader = BufReader::new(file);

    let mut comm = Command::new("ssh")
        .arg(format!("-p {}", remote_port))
        .arg(format!("{}@{}", user, remote_host))
        .arg(format!("cat >> {}", remote_path))
        .stdin(std::process::Stdio::piped())
        .spawn()?;

    // Obtener el manejador de la entrada estándar del proceso SSH
    let stdin = comm.stdin.as_mut().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to open stdin"))?;

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

fn receive_file(local_path: &str, remote_path: &str, remote_host: &str, user: &str, remote_port: u16) -> io::Result<()> {
    println!("ssh {}@{} -p {} cat {} > {}", user, remote_host, remote_port, remote_path, local_path);

    let status = Command::new("ssh")
        .arg(format!("{}@{}", user, remote_host))
        .arg(format!("-p {}", remote_port))
        .arg(format!("cat {}", remote_path)) // Ejecuta 'cat' en el servidor remoto
        .output()?; // Captura la salida del comando

    if status.status.success() {
        // Escribe la salida en el archivo local
        std::fs::write(local_path, status.stdout)?;
    } else {
        eprintln!("Error: {}", String::from_utf8_lossy(&status.stderr));
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to execute command"));
    }

    Ok(())

}

fn main() -> Result<()> {
    println!("\x1b[31m[0]\x1b[0m Exit\n\x1b[33m[1]\x1b[0m Download    \x1b[33m[2]\x1b[0m Upload\n\x1b[33m[3]\x1b[0m Config");
    let mut action = String::new();
    print!("Option >> ");
    stdout().flush()?;
    stdin().read_line(&mut action)?;
    let action = action.trim();

    if action == "1" {
        let mut remote_file = String::new();
        let mut input_path = String::new();
        let config = read_config()?;

        let status = Command::new("ssh")
            .arg(format!("{}@{}", config.user, config.host))
            .arg(format!("-p {}", config.port))
            .arg(format!("ls {}", config.remote_path.trim()))
            .status()?;
        if status.success() {
            print!("\nRemote file (without ext) >> ");
            stdout().flush()?;
            stdin().read_line(&mut remote_file)?;
            let remote_file = remote_file.trim();

            print!("Local file (with ext) >> ");
            stdout().flush()?;
            stdin().read_line(&mut input_path)?;
            let mut input_path = input_path.trim().to_owned();
            input_path = input_path+".brotli";

            let mut remote_path = format!("{}/{}", config.remote_path.trim().to_string(), remote_file);
            remote_path = remote_path+".brotli";

            let remote_host = &config.host;
            let remote_port = config.port;
            let user = &config.user;

            receive_file(&input_path, &remote_path, remote_host, user, remote_port)?;
            decompress_file(&input_path)?;
        }
    } else if action == "2" {
        let config = read_config()?;
        let remote_host = config.host;
        let remote_port = config.port;
        let mut input_path = String::new();
        let mut remote_path = String::new();

        if config.local_path == "" {
            print!("Local file path (with ext) >> ");
            stdout().flush()?;
            stdin().read_line(&mut input_path)?;
        } else {
            let mut file_name = String::new();
            print!("Local filename (with ext) >> ");
            stdout().flush()?;
            stdin().read_line(&mut file_name)?;
            input_path = format!("{}/{}", config.local_path, file_name);
        }

        if config.remote_path == "" {
            print!("Remote file path (without ext) >> ");
            stdout().flush()?;
            stdin().read_line(&mut remote_path)?;
            remote_path = remote_path+".brotli";
        } else {
            let mut file_name = String::new();
            print!("Remote filename (without ext) >> ");
            stdout().flush()?;
            stdin().read_line(&mut file_name)?;
            remote_path = format!("{}/{}.brotli", config.remote_path, file_name.trim());
        }

        let input_path = input_path.trim();
        let remote_path = remote_path.trim();

        compress_file(input_path)?;
        println!("File compressed successfully!");

        send_file_in_chunks(input_path, remote_path, &remote_host, &config.user, remote_port)?;
        println!("File sent succesfully!");
    } else if action == "3" {
        let config = read_config()?;
        println!("user: {}host:{}port:{}\nlocal:{}remote:{}", config.user, config.host, config.port, config.local_path, config.remote_path);
    } else {
        println!("[-] Err: {} is invalid option.", action);
    }
    Ok(())
}
