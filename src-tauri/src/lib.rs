use serde::Serialize;
use std::sync::{Arc, Mutex};
use sysinfo::{System, Networks};
use tauri::State;

// System data structures
#[derive(Serialize, Clone)]
pub struct CpuInfo {
    name: String,
    cores: usize,
    usage: f32,
    #[serde(rename = "coreUsage")]
    core_usage: Vec<f32>,
    frequency: f64,
    temperature: Option<f32>,
}

#[derive(Serialize, Clone)]
pub struct MemoryInfo {
    total: u64,
    used: u64,
    available: u64,
    #[serde(rename = "usagePercent")]
    usage_percent: f32,
    swap_total: u64,
    swap_used: u64,
}

#[derive(Serialize, Clone)]
pub struct NetworkInterface {
    name: String,
    #[serde(rename = "isUp")]
    is_up: bool,
    download_speed: u64,
    upload_speed: u64,
    #[serde(rename = "totalDownloaded")]
    total_downloaded: u64,
    #[serde(rename = "totalUploaded")]
    total_uploaded: u64,
}

#[derive(Serialize, Clone)]
pub struct NetworkInfo {
    interfaces: Vec<NetworkInterface>,
    #[serde(rename = "totalDownload")]
    total_download: u64,
    #[serde(rename = "totalUpload")]
    total_upload: u64,
    #[serde(rename = "downloadSpeed")]
    download_speed: u64,
    #[serde(rename = "uploadSpeed")]
    upload_speed: u64,
}

#[derive(Serialize, Clone)]
pub struct SystemStats {
    timestamp: u64,
    cpu: CpuInfo,
    memory: MemoryInfo,
    network: NetworkInfo,
}

// Application state
pub struct AppState {
    system: Arc<Mutex<System>>,
    networks: Arc<Mutex<Networks>>,
    last_network_stats: Arc<Mutex<std::collections::HashMap<String, (u64, u64)>>>,
}

impl Default for AppState {
    fn default() -> Self {
        let mut system = System::new_all();
        system.refresh_all();

        let networks = Networks::new_with_refreshed_list();

        let mut last_network_stats = std::collections::HashMap::new();
        for (interface_name, data) in &networks {
            last_network_stats.insert(
                interface_name.clone(),
                (data.total_received(), data.total_transmitted())
            );
        }

        Self {
            system: Arc::new(Mutex::new(system)),
            networks: Arc::new(Mutex::new(networks)),
            last_network_stats: Arc::new(Mutex::new(last_network_stats)),
        }
    }
}

#[tauri::command]
async fn get_system_stats(state: State<'_, AppState>) -> Result<SystemStats, String> {
    let mut system = state.system.lock().map_err(|e| e.to_string())?;
    let mut networks = state.networks.lock().map_err(|e| e.to_string())?;
    let mut last_stats = state.last_network_stats.lock().map_err(|e| e.to_string())?;

    // Refresh system information
    system.refresh_all();
    networks.refresh();

    // CPU Information
    let cpu_usage = system.global_cpu_usage();
    let cpu_count = system.cpus().len();
    let core_usage: Vec<f32> = system.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();

    let cpu_info = CpuInfo {
        name: "CPU".to_string(), // sysinfo doesn't provide CPU name easily
        cores: cpu_count,
        usage: cpu_usage,
        core_usage,
        frequency: system.cpus().first().map(|cpu| cpu.frequency() as f64 / 1000.0).unwrap_or(0.0), // Convert MHz to GHz
        temperature: None, // Temperature monitoring not implemented yet
    };

    // Memory Information
    let total_memory = system.total_memory();
    let used_memory = system.used_memory();
    // 在macOS上，available_memory可能返回0，所以需要计算
    let available_memory = total_memory.saturating_sub(used_memory);

    let memory_info = MemoryInfo {
        total: total_memory,
        used: used_memory,
        available: available_memory,
        usage_percent: if total_memory > 0 { (used_memory as f32 / total_memory as f32) * 100.0 } else { 0.0 },
        swap_total: system.total_swap(),
        swap_used: system.used_swap(),
    };

    // Network Information
    let mut interfaces = Vec::new();
    let mut total_download = 0u64;
    let mut total_upload = 0u64;

    for (interface_name, data) in networks.iter() {
        let current_received = data.total_received();
        let current_transmitted = data.total_transmitted();

        let (last_received, last_transmitted) = last_stats
            .get(interface_name)
            .copied()
            .unwrap_or((0, 0));

        let download_speed = current_received.saturating_sub(last_received);
        let upload_speed = current_transmitted.saturating_sub(last_transmitted);

        // Update last stats for next calculation
        last_stats.insert(interface_name.clone(), (current_received, current_transmitted));

        interfaces.push(NetworkInterface {
            name: interface_name.clone(),
            is_up: current_received > 0 || current_transmitted > 0,
            download_speed,
            upload_speed,
            total_downloaded: current_received,
            total_uploaded: current_transmitted,
        });

        total_download += download_speed;
        total_upload += upload_speed;
    }

    // Debug output for network
    println!("Total download speed: {} bytes/s", total_download);
    println!("Total upload speed: {} bytes/s", total_upload);
    println!("Number of interfaces: {}", interfaces.len());

    let network_info = NetworkInfo {
        interfaces,
        total_download,
        total_upload,
        download_speed: total_download,
        upload_speed: total_upload,
    };

    // Return system stats
    Ok(SystemStats {
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        cpu: cpu_info,
        memory: memory_info,
        network: network_info,
    })
}

// Keep the greet command for now
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![get_system_stats, greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
