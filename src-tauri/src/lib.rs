use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde::{ Deserialize, Serialize };
use tauri::Emitter;
use std::net::UdpSocket;
use std::sync::{ Arc, Mutex, RwLock };
use serde_yaml;
use once_cell::sync::Lazy;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Config {
    bport: String,
    qport: String,
}
impl Config {
    fn new() -> Config {
        Config {
            qport: "9999".to_string(),
            bport: "8888".to_string(),
        }
    }
}

static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| Mutex::new(Config::new()));
static SOCKET: Lazy<RwLock<Option<Arc<UdpSocket>>>> = Lazy::new(|| RwLock::new(None));

async fn init_config() -> Result<Config, String> {
    let config = load_config("config.yaml").await.map_err(|e| e.to_string())?;
    Ok(Config {
        bport: config.bport,
        qport: config.qport,
    })
}

fn get_socket(config: &Config) -> Result<Arc<UdpSocket>, String> {
    let mut socket_lock = SOCKET.write().unwrap();

    // 如果 socket 还没有被初始化，则进行初始化
    if socket_lock.is_none() {
        let socket = UdpSocket::bind(format!("0.0.0.0:{}", config.qport)).map_err(|e|
            e.to_string()
        )?;
        socket.set_broadcast(true).expect("Could not set broadcast option");
        *socket_lock = Some(Arc::new(socket));
    }

    // 返回对 UdpSocket 的 Arc
    Ok(socket_lock.as_ref().unwrap().clone())
}

#[tauri::command]
async fn listen_udp(window: tauri::Window) -> Result<(), String> {
    println!("listen_udp start");
    init_config();
    println!("----------------- Start ------------------");
    let config = CONFIG.lock().unwrap().clone();
    println!("Loaded config: {:?}", config);
    println!("{}", format!("UDP Server listening on port {}", config.qport));
    let mut buf = [0; 1024 * 100];
    let socket = get_socket(&config)?;
    while let Ok((size, addr)) = socket.recv_from(&mut buf) {
        if size > 1024 * 100 {
            println!("Error: Received packet is too large ({} bytes). Discarding.", size);
            let config = CONFIG.lock().unwrap().clone();
            let broadcast_addr = format!("255.255.255.255:{}", config.bport);
            socket
                .send_to("Error: Received packet is too large".as_bytes(), &broadcast_addr)
                .map_err(|e| e.to_string())?;
            continue;
        }

        let received_message = String::from_utf8_lossy(&buf[..size]);
        println!("Received from utf-8 {}: {}", addr, received_message);
        window.emit("res-updated", &received_message).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn broadcast(message: String) -> Result<(), String> {
    println!("broadcast {}", message);
    let config = CONFIG.lock().unwrap().clone();
    let broadcast_addr = format!("255.255.255.255:{}", config.bport);
    let socket = get_socket(&config)?;
    socket.send_to(message.as_bytes(), &broadcast_addr).map_err(|e| e.to_string())?;
    println!("Send: {}, to {}", message, broadcast_addr);
    Ok(())
}

async fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let path = Path::new(path);
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| e.to_string())?;
    let config: Config = serde_yaml::from_str(&contents)?;
    Ok(config)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder
        ::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![listen_udp, broadcast])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
