use std::{fs, thread};
use std::path::Path;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
use std::process::{Command, Stdio};
use std::io::{BufReader, Read, Result, Write, BufRead};

use dirs;
use ssh2::Session;
use zstd::{Encoder, Decoder};
use tar::Builder;
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
    address: String,
    port: u16,
    authenticated: bool,
    storage: Storage
}

pub struct Config {
    user: String,
    host: String,
    port: u16,
}
#[derive(Serialize, Deserialize)]
pub struct Storage {
    total_size: u16, // Max 65535 GB -> 65.5 TB
    used_size: u16
}

fn read_config() -> Result<Config> {
    let file = fs::File::open("grapsy.conf")?;
    let reader = BufReader::new(file);

    let mut user = String::new();
    let mut host = String::new();
    let mut port = String::new();

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("user=") {
            user = line[5..].to_string();
        } else if line.starts_with("host=") {
            host = line[5..].to_string();
        } else if line.starts_with("port=") {
            port = line[5..].to_string();
        }
    }

    let config = Config {
        user,
        host,
        port:port.trim().parse().unwrap(),
};

    Ok(config)
}
pub fn read_config_json() -> Result<String> {
    let file = fs::File::open("grapsy.conf")?;
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
    let mut file = fs::File::create("grapsy.conf")?;
    file.write_all(format!("user={}\nhost={}\nport={}\nlocal_path={}\nremote_path={}",
        user, host, port, local_folder, remote_folder).as_bytes())?;

    Ok("Config written succesfully.".to_string())
}
pub fn compress_file(input_path: &str) -> Result<String> {
    // If file is compressed (like mp3 or similar), skip compression
    if matches!(Path::new(input_path).extension().and_then(|ext| ext.to_str()),
    Some("mp3"|"aac"|"ogg"|"jpg"|"jpeg"|"png"|"gif"|"mp4"|"avi"|"mkv"|"zip"|"rar"|
        "7z"|"exe"|"dll"|"pdf"|"docx"|"iso")) {
        return Ok(input_path.to_string());
    }

    // Process folder to tar file
    let metadata_path = fs::metadata(input_path)?;
    let local_file;
    if metadata_path.is_dir() {
        local_file = format!("{}.tar", input_path);
        let tar_file = fs::File::create(&local_file)?;
        let mut tar_builder = Builder::new(tar_file);

        tar_builder.append_dir_all(".", input_path)?;
        tar_builder.finish()?;
    } else {
        local_file = input_path.to_string();
    }

    // Open local file and create file for zst compress
    let mut input_file = fs::File::open(&local_file)?;
    let output_path = format!("{}.zst", local_file);
    let output_file = fs::File::create(&output_path)?;

    // Remove tar file
    fs::remove_file(local_file)?;

    // Get availables logical cores
    let logical_cores = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    // Create encoder
    let mut encoder = Encoder::new(output_file, 10)?;
    encoder.multithread(logical_cores.try_into().unwrap())?;
    
    let mut buffer = [0; 4096];
    while let Ok(bytes_read) = input_file.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        encoder.write_all(&buffer[..bytes_read])?;
    }
    
    encoder.finish()?;
    Ok(output_path)
}
pub fn decompress_file(input_path: &str) -> Result<String> {
    // Check if the file is compressed or not
    if !Path::new(input_path).extension().map_or(false, |ext| ext == "zst") {
        return Ok("File downloaded and don't need a decompress".to_string());
    }

    let input_file = fs::File::open(input_path)?;
    let output_path = input_path.strip_suffix(".zst").unwrap_or(input_path);
    let mut output_file = fs::File::create(output_path)?;
    let mut decoder = Decoder::new(input_file)?;

    let mut buffer = [0; 4096];
    while let Ok(bytes_read) = decoder.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        output_file.write_all(&buffer[..bytes_read])?;
    }

    fs::remove_file(input_path)?;
    return Ok("File downloaded and decompressed".to_string());
}

pub fn send_file(input_path: &str, remote_path: &str) -> Result<String> {
    let home_dir = dirs::home_dir().expect("Error msg");
    let config = read_config()?;
    let remote_port = config.port;
    let remote_host = config.host;
    let user = config.user;

    // Get content and size of local file
    let content = fs::read(input_path)?;
    let file_size: u64 = fs::metadata(input_path)?.len();

    // Stablish ssh connection
    let tcp = TcpStream::connect(format!("{}:{}", remote_host, remote_port)).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_pubkey_file(
        &user,
        None,
        Path::new(&format!("{}/.ssh/id_rsa", home_dir.display())),
        None,
    ).unwrap();

    let input_file = Path::new(&input_path);
    let file_name = input_file.file_name();

    if let Some(file_name) = file_name {
        // Write the file
        let mut remote_file = sess.scp_send(Path::new(&format!("{}/{}", remote_path,
                file_name.to_string_lossy())), 0o644, file_size, None).unwrap();
        remote_file.write(&content)?;

        // Close the channel and wait for the whole content to be transferred
        remote_file.send_eof().unwrap();
        remote_file.wait_eof().unwrap();
        remote_file.close().unwrap();
        remote_file.wait_close().unwrap();

        // Remove Original file
        fs::remove_file(input_path)?;

        Ok("Successed: File sent to server.".to_string())
    } else {
        Ok("Err: File name not accessible".to_string())
    }
}
pub fn validate_file_type(file_path: &str) -> Result<bool> {
    let home_dir = dirs::home_dir().expect("/");
    let config = read_config()?;
    let address = config.host;
    let port = config.port;

    // Try resolve hostname if it is
    let sv_address = match format!("{}:{}", address, port).to_socket_addrs() {
        Ok(mut addrs) => {
            if let Some(sv_address) = addrs.next() {
                sv_address.to_string()
            } else {
                format!("{}:{}", address, port)
            }
        }
        Err(_) => format!("{}:{}", address, port),
    };

    // Check if server is ON
    if let Err(_) = TcpStream::connect(&sv_address) {
        return Ok(false);
    }
    
    // Check if can login with key file
    let tcp = TcpStream::connect(&sv_address).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    if !Path::new(&format!("{}/.ssh/id_rsa", home_dir.display())).exists() {
        return Ok(false)
    }
    
    sess.userauth_pubkey_file(
        &config.user,
        None,
        Path::new(&format!("{}/.ssh/id_rsa", home_dir.display())),
        None,
    ).unwrap();

    // Compare file_name parameter in server
    if sess.authenticated() {
        let mut channel = sess.channel_session().unwrap();
        channel.exec(&format!("stat -c %F {}", file_path)).unwrap();
        let mut output = String::new();
        channel.read_to_string(&mut output).unwrap();

        if output.split_whitespace().count() > 1 {
            return Ok(true)
        } else {
            return Ok(false)
        }
    } else {
        return Ok(false)
    }
}

pub fn get_content_folder(remote_folder: &str) -> Result<String> {
    let home_dir = dirs::home_dir().expect("Error msg");
    let config = read_config()?;
    let address = config.host;
    let port = config.port;
    
    // Try resolve hostname if it is
    let sv_address = match format!("{}:{}", address, port).to_socket_addrs() {
        Ok(mut addrs) => {
            if let Some(sv_address) = addrs.next() {
                sv_address.to_string()
            } else {
                format!("{}:{}", address, port)
            }
        }
        Err(_) => format!("{}:{}", address, port),
    };

    // Check if server is ON
    if let Err(_) = TcpStream::connect(&sv_address) {
        return Ok("Err: Can't connect with the server".to_string());
    }
    
    // Check if can login with key file
    let tcp = TcpStream::connect(&sv_address).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    if !Path::new(&format!("{}/.ssh/id_rsa", home_dir.display())).exists() {
        return Ok("Err: key file not found".to_string())
    }
    
    sess.userauth_pubkey_file(
        &config.user,
        None,
        Path::new(&format!("{}/.ssh/id_rsa", home_dir.display())),
        None,
    ).unwrap();

    // Get server storage info
    if sess.authenticated() {
        let mut folders: Vec<&str> = Vec::new();
        let mut channel = sess.channel_session().unwrap();
        channel.exec(&format!("ls -l {}", remote_folder)).unwrap();
        let mut output = String::new();
        channel.read_to_string(&mut output).unwrap();
        channel.wait_close().unwrap();

        for line in output.lines() {
            if line.split_whitespace().count() > 2 && !line.trim().is_empty() {
                let fields: Vec<&str> = line.split_whitespace().collect();
                let folder_name = fields[8].trim();
                folders.push(folder_name);
            }
        }
        return Ok(serde_json::to_string(&folders).unwrap())
    } else {
        return Ok("Err authenticating in the server".to_string())
    }
}

pub fn receive_file(local_path: &str, remote_path: &str) -> Result<String> {
    let home_dir = dirs::home_dir().expect("Error msg");
    let config = read_config()?;
    let remote_port = config.port;
    let remote_host = config.host;
    let user = config.user;

    // Stablish ssh connection
    let tcp = TcpStream::connect(format!("{}:{}", remote_host, remote_port)).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake()?;

    sess.userauth_pubkey_file(
        &user,
        None,
        Path::new(&format!("{}/.ssh/id_rsa", home_dir.display())),
        None,
    ).unwrap();

    // Read remote file
    let (mut remote_file, _stat) = sess.scp_recv(Path::new(remote_path)).unwrap();
    let mut contents = Vec::new();
    remote_file.read_to_end(&mut contents).unwrap();

    // Get filename of remote_path
    let file_name = Path::new(remote_path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");

    // Save local file
    let mut local_file = fs::File::create(format!("{}/{}", local_path, file_name)).unwrap();
    local_file.write_all(&contents).unwrap();

    // Close the channel and wait for the whole content to be tranferred
    remote_file.send_eof().unwrap();
    remote_file.wait_eof().unwrap();
    remote_file.close().unwrap();
    remote_file.wait_close().unwrap();

    Ok(format!("{}/{}", local_path, file_name))
}

pub fn send_key(desc: &str, user: &str, password: &str, address: &str, port: &str) -> Result<String> {
    let home_dir = dirs::home_dir().expect("Error msg");
    let config = read_config()?;

    // Rename existent key file
    if Path::new(&format!("{}/.ssh/id_rsa", home_dir.display())).exists() {
        let _ = fs::rename(format!("{}/.ssh/id_rsa",
            home_dir.display()), format!("{}/.ssh/id_rsa.old", home_dir.display()))?;
    }
    if Path::new(&format!("{}/.ssh/id_rsa.pub", home_dir.display())).exists() {
        let _ = fs::rename(format!("{}/.ssh/id_rsa.pub",
            home_dir.display()), format!("{}/.ssh/id_rsa.pub.old", home_dir.display()))?;
    }

    // Create key
    let _create_key = Command::new("ssh-keygen")
        .arg(format!("-trsa"))
        .arg(format!("-b4096"))
        .arg(format!("-C'{}'", desc))
        .arg("-mPEM")
        .arg(format!("-f{}/.ssh/id_rsa", home_dir.display()))
        .arg("-N")
        .arg("")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;


    // Try resolve hostname if it is
    let sv_address = match format!("{}:{}", address, port).to_socket_addrs() {
        Ok(mut addrs) => {
            if let Some(sv_address) = addrs.next() {
                sv_address.to_string()
            } else {
                format!("{}:{}", address, port)
            }
        }
        Err(_) => format!("{}:{}", address, port),
    };

    // Send Key to Remote server
    let tcp = TcpStream::connect(sv_address).unwrap();
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;
    sess.userauth_password(user, password.trim())?;

    let mut channel = sess.channel_session().unwrap();
    //if !sess.authenticated() {
    //    return Err("Check user and password".to_string())
    //}

    // Wait while key file isn't created
    while !Path::new(&format!("{}/.ssh/id_rsa.pub", home_dir.display())).exists() {
        thread::sleep(Duration::from_millis(500));
    }

    // Open key file and read content
    let mut local_file = fs::File::open(format!("{}/.ssh/id_rsa.pub", home_dir.display()))?;
    let mut file_content = Vec::new();
    local_file.read_to_end(&mut file_content)?;

    // Key file size
    let file_size: u64 = fs::metadata(format!("{}/.ssh/id_rsa.pub", home_dir.display()))?.len();

    // Send file using SCP
    let mut remote_file = sess.scp_send(Path::new(&format!("/home/{}/.ssh/grapsy_key",
            user.trim())), 0o644, file_size, None).unwrap();
    remote_file.write_all(&file_content).unwrap();

    // Add new key to remote authorized keys
    channel.exec(format!("printf '\n%s' \"$(cat /home/{}/.ssh/grapsy_key)\" >> /home/{}/.ssh/authorized_keys",
        config.user, config.user).as_str()).unwrap();

    // Close connection
    remote_file.send_eof().unwrap();
    remote_file.wait_eof().unwrap();
    remote_file.close().unwrap();
    remote_file.wait_close().unwrap();

    Ok("Authenticated in the server".to_string())
}

pub fn check_rsa_key() -> Result<bool> {
    let home_dir = dirs::home_dir().expect("Error msg");
    if Path::new(&format!("{}/.ssh/id_rsa", home_dir.display())).exists() {
        return Ok(true);
    } else {
        return Ok(false);
    }
}

pub fn check_server_status() -> Result<bool> {
    let config = read_config()?;
    let address = &config.host;
    let port = &config.port;

    let sv_address = match format!("{}:{}", address, port).to_socket_addrs() {
        Ok(mut addrs) => {
            if let Some(sv_address) = addrs.next() {
                sv_address
            } else {
                format!("{}:{}", address, port).parse().unwrap()
            }
        }
        Err(_) => return Ok(false),
    };

    let timeout = Duration::from_millis(500);
    let result = TcpStream::connect_timeout(&sv_address ,timeout);

    match result {
        Ok(_) => {
            println!("Conexión exitosa a {}", address);
            return Ok(true)
        }
        Err(_) => {
            return Ok(false)
        }
    };
}

pub fn server_info() -> Result<String> {
    let home_dir = dirs::home_dir().expect("Error msg");
    let auth: bool;
    let storage: Storage;
    // Read config
    let config = read_config()?;
    let address = &config.host;
    let port = &config.port;

    // Try resolve hostname if it is
    let sv_address = match format!("{}:{}", address, port).to_socket_addrs() {
        Ok(mut addrs) => {
            if let Some(sv_address) = addrs.next() {
                sv_address.to_string()
            } else {
                format!("{}:{}", address, port)
            }
        }
        Err(_) => format!("{}:{}", address, port),
    };

    // Check if server is ON
    if let Err(_) = TcpStream::connect(&sv_address) {
        return Ok(serde_json::to_string(&Server {
            status: false,
            address: config.host.to_string().parse().expect("0.0.0.0"),
            port: config.port,
            authenticated: false,
            storage: Storage { total_size: 0, used_size: 0 }
        })?);
    }
    
    // Check if can login with key file
    let tcp = TcpStream::connect(&sv_address).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    if !Path::new(&format!("{}/.ssh/id_rsa", home_dir.display())).exists() {
        return Ok("Err: key file not found".to_string())
    }
    
    match sess.userauth_pubkey_file(
        &config.user,
        None,
        Path::new(&format!("{}/.ssh/id_rsa", home_dir.display())),
        None,
    ) {
        Ok(()) => {},
        Err(_e) => return Ok("Error: can't login in the server, please check the key".to_string()),
    };

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
            if line.contains("/") && !line.trim().is_empty() {
                let fields: Vec<&str> = line.split_whitespace().collect();
                total_size = match fields[1].chars().last() {
                    Some('M') => {
                        let number: f64 = fields[1][..fields[1].len()-1].parse().unwrap_or(0.0);
                        (number / 1024.0) as u16
                    }
                    Some('G') => {
                        let number: f64 = fields[1][..fields[1].len()-1].parse().unwrap_or(0.0);
                        number as u16
                    }
                    Some('T') => {
                        let number: f64 = fields[1][..fields[1].len()-1].parse().unwrap_or(0.0);
                        (number * 1024.0) as u16
                    }
                    _ => {
                        0
                    }
                };
                used_size = match fields[2].chars().last() {
                    Some('M') => {
                        let number: f64 = fields[2][..fields[2].len()-1].parse().unwrap_or(0.0);
                        (number / 1024.0) as u16
                    }
                    Some('G') => {
                        let number: f64 = fields[2][..fields[2].len()-1].parse().unwrap_or(0.0);
                        number as u16
                    }
                    Some('T') => {
                        let number: f64 = fields[2][..fields[2].len()-1].parse().unwrap_or(0.0);
                        (number * 1024.0) as u16
                    }
                    _ => {
                        0
                    }
                };
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
