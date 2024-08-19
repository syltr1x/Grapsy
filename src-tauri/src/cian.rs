use std::process::Command;
use std::fs::{File, write};
use std::io::{BufReader, Read, Result, Write, BufRead, ErrorKind, Error};
use zstd::{Encoder, Decoder};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    user: String,
    host: String,
    port: u16,
    local_path: String,
    remote_path: String
}

pub fn read_config() -> Result<String> {
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

    let config_json = serde_json::to_string(&Config {
        user,
        host,
        port:port.trim().parse().unwrap(),
        local_path,
        remote_path
    })?;

    Ok(config_json)
}
pub fn write_config(user: &str, host: &str, port: &str, local_folder: &str, remote_folder: &str) -> Result<String> {
    let mut file = File::create("cian.conf")?;
    file.write_all(format!("user={}\nhost={}\nport={}\nlocal_path={}\nremote_path={}",
        user, host, port, local_folder, remote_folder).as_bytes())?;

    Ok("".to_string())
}
pub fn compress_file(input_path: &str) -> Result<()> {
    let mut input_file = File::open(input_path)?;
    let output_file = File::create(format!("{}.zst", input_path))?;
    let mut encoder = Encoder::new(output_file, 0)?;
    
    let mut buffer = [0; 4096];
    while let Ok(bytes_read) = input_file.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        encoder.write_all(&buffer[..bytes_read])?;
    }
    
    encoder.finish()?;
    Ok(())
}
pub fn decompress_file(input_path: &str) -> Result<()> {
    let input_file = File::open(input_path)?;
    let output_path = input_path.strip_suffix(".zst").unwrap_or(input_path);
    let mut output_file = File::create(output_path)?;
    let mut decoder = Decoder::new(input_file)?;
    
    let mut buffer = [0; 4096];
    while let Ok(bytes_read) = decoder.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        output_file.write_all(&buffer[..bytes_read])?;
    }
    Ok(())
}

pub fn send_file(input_path: &str, remote_path: &str) -> Result<()> {
    let remote_port = "";
    let remote_host = "";
    let user = "";

    let local_path = input_path.trim();
    let input_file = File::open(local_path)?;
    let mut reader = BufReader::new(input_file);

    println!("ssh -p {} {}@{} cat >> {}", remote_port, user ,remote_host, remote_path.trim());
    let mut comm = Command::new("ssh")
        .arg(format!("-p {}", remote_port))
        .arg(format!("{}@{}", user, remote_host))
        .arg(format!("cat >> {}", remote_path.trim()))
        .stdin(std::process::Stdio::piped())
        .spawn()?;

    let stdin = comm.stdin.as_mut().ok_or_else(|| Error::new(ErrorKind::Other, "Failed to open stdin"))?;

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

pub fn receive_file(local_path: &str, remote_path: &str) -> Result<()> {
    let remote_port = "";
    let remote_host = "";
    let user = "";

    println!("ssh {}@{} -p {} cat >> {}", user, remote_host, remote_port, remote_path.trim());
    let status = Command::new("ssh")
        .arg(format!("{}@{}", user, remote_host))
        .arg(format!("-p {}", remote_port))
        .arg(format!("cat {}", remote_path.trim()))
        .output()?;

    if status.status.success() {
        write(local_path.trim(), status.stdout)?;
    } else {
        eprintln!("Error: {}", String::from_utf8_lossy(&status.stderr));
        return Err(Error::new(ErrorKind::Other, "Failed to execute command"));
    }
    Ok(())
}
