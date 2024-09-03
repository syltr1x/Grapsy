use std::{fs, thread};
use std::path::Path;
use std::net::{Ipv4Addr, TcpStream};
use std::time::Duration;
use std::fs::File;
use std::process::{Command, Stdio};
use std::io::{BufReader, Read, Result, Write, BufRead};

use dirs;
use ssh2::Session;
use zstd::{Encoder, Decoder};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ConfigJson {
    user: String,
    host: String,
    port: u16,
    local_path: String,
    remote_path: String
}
#[derive(Serialize, Deserialize)]
pub struct Server {
    status: bool,
    address: Ipv4Addr,
    port: u16,
    authenticated: bool,
    storage: Storage
}

pub struct Config {
    user: String,
    host: String,
    port: u16,
    //local_path: String,
    //remote_path: String
}
#[derive(Serialize, Deserialize)]
pub struct Storage {
    total_size: u16, // Max 65535 GB -> 65.5 TB
    used_size: u16
}

fn read_config() -> Result<Config> {
    let file = File::open("cian.conf")?;
    let reader = BufReader::new(file);

    let mut user = String::new();
    let mut host = String::new();
    let mut port = String::new();
    //let mut local_path = String::new();
    //let mut remote_path = String::new();

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("user=") {
            user = line[5..].to_string();
        } else if line.starts_with("host=") {
            host = line[5..].to_string();
        } else if line.starts_with("port=") {
            port = line[5..].to_string();
        }// else if line.starts_with("local_path=") {
        //    local_path = line[11..].to_string();
        //} else if line.starts_with("remote_path=") {
        //    remote_path = line[12..].to_string();
        //}
    }

    let config = Config {
        user,
        host,
        port:port.trim().parse().unwrap(),
        //local_path,
        //remote_path
};

    Ok(config)
}
pub fn read_config_json() -> Result<String> {
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

    let config_json = serde_json::to_string(&ConfigJson {
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
    let config = read_config()?;
    let remote_port = config.port;
    let remote_host = config.host;
    let user = config.user;

    let content = fs::read(input_path.trim())?;

    // Stablish ssh connection
    let tcp = TcpStream::connect(format!("{}:{}", remote_host, remote_port)).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_agent(user.trim()).unwrap();

    // Write the file
    let mut remote_file = sess.scp_send(Path::new(&format!("{}/",remote_path.trim())),0o644, 10, None).unwrap();
    remote_file.write(&content)?;

    // Close the channel and wait for the whole content to be transferred
    remote_file.send_eof().unwrap();
    remote_file.wait_eof().unwrap();
    remote_file.close().unwrap();
    remote_file.wait_close().unwrap();

    Ok(())
}

pub fn receive_file(local_path: &str, remote_path: &str) -> Result<()> {
    let config = read_config()?;
    let remote_port = config.port;
    let remote_host = config.host;
    let user = config.user;

    // Stablish ssh connection
    let tcp = TcpStream::connect(format!("{}:{}", remote_host, remote_port)).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_agent(&user).unwrap();

    // Read remote file
    let (mut remote_file, stat) = sess.scp_recv(Path::new(remote_path)).unwrap();
    println!("remote file size: {}", stat.size());
    let mut contents = Vec::new();
    remote_file.read_to_end(&mut contents).unwrap();

    // Save local file
    let mut local_file = File::create(local_path).unwrap();
    local_file.write_all(&contents).unwrap();

    // Close the channel and wait for the whole content to be tranferred
    remote_file.send_eof().unwrap();
    remote_file.wait_eof().unwrap();
    remote_file.close().unwrap();
    remote_file.wait_close().unwrap();

    Ok(())
}

pub fn send_key(desc: &str, user: &str, password: &str, address: &str, port: &str) -> Result<()> {
    let home_dir = dirs::home_dir().expect("Error msg");
    // Create key
     let _create_key = Command::new("ssh-keygen")
        .arg(format!("-trsa"))
        .arg(format!("-b4096"))
        .arg(format!("-C'{}'", desc))
        .arg(format!("-f{}/.ssh/{}_server_rsa", home_dir.display(), user.trim()))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    // Send Key to Remote server
    let tcp = TcpStream::connect(format!("{}:{}", address.trim(), port.trim()))?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;

    sess.userauth_password(user.trim(), password.trim())?;

    //if !sess.authenticated() {
    //    println!("Err: Authentication failed :(");
    //}

    while !Path::new(&format!("{}/.ssh/{}_server_rsa.pub", home_dir.display(), user.trim())).exists() {
        thread::sleep(Duration::from_millis(500));
    }

    let mut local_file = File::open(format!("{}/.ssh/{}_server_rsa.pub", home_dir.display(), user.trim()))?;
    let mut file_content = Vec::new();
    local_file.read_to_end(&mut file_content)?;

    let mut remote_file = sess.scp_send(Path::new(&format!("/home/{}/.ssh/authorized_keys",
            user.trim())), 0o644, 10, None).unwrap();

    remote_file.write_all(&file_content).unwrap();

    remote_file.send_eof().unwrap();
    remote_file.wait_eof().unwrap();
    remote_file.close().unwrap();
    remote_file.wait_close().unwrap();

    Ok(())
}
pub fn server_info() -> Result<String> {
    let home_dir = dirs::home_dir().expect("Error msg");
    let auth: bool;
    let storage: Storage;
    // Read config
    let config = read_config()?;

    // Check if server is ON
    let ping_command = Command::new("ping")
        .arg("-c1")
        .arg(format!("{}",config.host.to_string()))
        .arg("-w3") // Ping wait 3 seconds
        .status();
    
    // If server doesn't response
    if !ping_command.is_ok() {
        let server = serde_json::to_string(&Server {
            status: false,
            address: config.host.to_string().parse().expect("0.0.0.0"),
            port: config.port,
            authenticated: false,
            storage: Storage { total_size: 0, used_size: 0 }
        })?;
        return Ok(server)
    }

    // Check if can login with key file
    let tcp = TcpStream::connect(format!("{}:{}", config.host, config.port)).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    if !Path::new(&format!("{}/.ssh/{}_server_rsa.pub", home_dir.display(), config.user)).exists() {
        return Ok("Err: key file not found".to_string())
    }

    sess.userauth_pubkey_file(
        &config.user,
        None,
        Path::new(&format!("{}/.ssh/{}_server_rsa.pub", home_dir.display(), config.user)),
        None,
    ).unwrap();

    auth = sess.authenticated();

    // Get server storage info
    if auth {
        let mut total_size: u16 = 0;
        let mut used_size: u16 = 0;
        let mut channel = sess.channel_session().unwrap();
        channel.exec("df -h .").unwrap();
        let mut output = String::new();
        channel.read_to_string(&mut output).unwrap();
        channel.wait_close().unwrap();

        for line in output.lines() {
            if !line.contains("Filesystem") && !line.trim().is_empty() {
                let fields: Vec<&str> = line.split_whitespace().collect();
                total_size = fields[1][..fields[1].len() - 1].parse().unwrap();
                used_size = fields[2][..fields[2].len() - 1].parse().unwrap();
            }
        }
        storage = Storage { total_size, used_size }
    } else {
        storage = Storage { total_size: 0, used_size: 0 }
    }

    // Return server info
    let server = serde_json::to_string(&Server {
        status: true,
        address: config.host.to_string().parse().expect("0.0.0.0"),
        port: config.port,
        authenticated: auth,
        storage
    })?;

    Ok(server)
}
