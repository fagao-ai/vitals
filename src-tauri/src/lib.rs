use serde::Serialize;
use std::sync::{Arc, Mutex};
use std::process::Command;
use sysinfo::{Networks, System};
use tauri::{Manager, State, WebviewWindow};
use local_ip_address::list_afinet_netifas;

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
    #[serde(rename = "displayName")]
    display_name: String,
    #[serde(rename = "ipAddress")]
    ip_address: Option<String>,
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

// 判断是否是物理网络接口（过滤掉虚拟网卡）
fn is_physical_interface(interface_name: &str) -> bool {
    let name_lower = interface_name.to_lowercase();

    // macOS 物理网络接口
    if name_lower.starts_with("en") && name_lower.len() >= 3 {
        // 检查是否是数字结尾的接口（en0, en1, etc.）
        let chars: Vec<char> = name_lower.chars().collect();
        if chars.len() >= 3 && chars[2].is_ascii_digit() {
            // 排除特定的虚拟接口
            if name_lower.contains("bridge") || name_lower.contains("virtual") || name_lower.contains("utun") {
                return false;
            }
            return true;
        }
    }

    // Linux 物理以太网接口
    if name_lower.starts_with("eth") && name_lower.len() >= 4 {
        let chars: Vec<char> = name_lower.chars().collect();
        if chars[3].is_ascii_digit() {
            return true;
        }
    }

    // Linux 物理 Wi-Fi 接口
    if name_lower.starts_with("wlan") && name_lower.len() >= 5 {
        let chars: Vec<char> = name_lower.chars().collect();
        if chars[4].is_ascii_digit() {
            return true;
        }
    }

    // Linux 新的命名规范（预测性接口名称）
    if (name_lower.starts_with("enp") || name_lower.starts_with("ens") ||
        name_lower.starts_with("eno") || name_lower.starts_with("enx")) &&
       name_lower.len() >= 4 {
        // 确保后面跟着数字
        let chars: Vec<char> = name_lower.chars().collect();
        if chars[3].is_ascii_digit() {
            return true;
        }
    }

    if name_lower.starts_with("wlp") && name_lower.len() >= 4 {
        let chars: Vec<char> = name_lower.chars().collect();
        if chars[3].is_ascii_digit() {
            return true;
        }
    }

    // 明确的接口名称
    if name_lower == "wi-fi" || name_lower == "wifi" || name_lower == "ethernet" {
        return true;
    }

    // 默认情况下，不识别的接口都过滤掉
    false
}

// 获取 Wi-Fi SSID（跨平台）
fn get_wifi_ssid() -> Option<String> {

    // 使用系统命令获取 Wi-Fi SSID
    #[cfg(target_os = "macos")]
    {
        // macOS 使用 networksetup 命令
        if let Ok(output) = Command::new("networksetup")
            .arg("-getairportnetwork")
            .arg("en0")
            .output()
        {
            if let Ok(output_str) = String::from_utf8(output.stdout) {
                if let Some(line) = output_str.lines().next() {
                    if let Some(ssid) = line.split("Current Wi-Fi Network: ").nth(1) {
                        return Some(ssid.trim().to_string());
                    }
                }
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        // Windows 使用 netsh 命令
        if let Ok(output) = Command::new("netsh")
            .args(&["wlan", "show", "interfaces"])
            .output()
        {
            if let Ok(output_str) = String::from_utf8(output.stdout) {
                for line in output_str.lines() {
                    if line.trim().starts_with("SSID") && line.contains(":") {
                        if let Some(ssid) = line.split(":").nth(1) {
                            let ssid = ssid.trim();
                            if !ssid.is_empty() {
                                return Some(ssid.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        // Linux 使用 nmcli 或 iwgetid
        if let Ok(output) = Command::new("nmcli")
            .args(&["-t", "-f", "active", "ssid", "dev", "wifi"])
            .output()
        {
            if let Ok(output_str) = String::from_utf8(output.stdout) {
                let ssid = output_str.trim();
                if !ssid.is_empty() {
                    return Some(ssid.to_string());
                }
            }
        }

        // 备选方案：使用 iwgetid
        if let Ok(output) = Command::new("iwgetid")
            .arg("-r")
            .output()
        {
            if let Ok(output_str) = String::from_utf8(output.stdout) {
                let ssid = output_str.trim();
                if !ssid.is_empty() && ssid != "any" {
                    return Some(ssid.to_string());
                }
            }
        }
    }

    None
}

// 获取网络接口的显示名称
fn get_display_name(interface_name: &str) -> String {
    let name_lower = interface_name.to_lowercase();

    // macOS 上 en0 通常是 Wi-Fi，en1 等通常是有线网
    if name_lower == "en0" {
        // 尝试获取 Wi-Fi SSID
        if let Some(ssid) = get_wifi_ssid() {
            ssid
        } else {
            "Wi-Fi".to_string()
        }
    } else if name_lower.contains("wi-fi") || name_lower.contains("wifi") || name_lower.contains("wlan") {
        "Wi-Fi".to_string()
    } else if name_lower.starts_with("en1") || name_lower.starts_with("en2") ||
              name_lower.starts_with("en3") || name_lower.starts_with("en4") ||
              name_lower.starts_with("en5") || name_lower.contains("eth") {
        "以太网".to_string()
    } else if name_lower.contains("thunderbolt") {
        "Thunderbolt".to_string()
    } else {
        interface_name.to_string()
    }
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
                (data.total_received(), data.total_transmitted()),
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
        frequency: system
            .cpus()
            .first()
            .map(|cpu| cpu.frequency() as f64 / 1000.0)
            .unwrap_or(0.0), // Convert MHz to GHz
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
        usage_percent: if total_memory > 0 {
            (used_memory as f32 / total_memory as f32) * 100.0
        } else {
            0.0
        },
        swap_total: system.total_swap(),
        swap_used: system.used_swap(),
    };

    // Network Information
    let mut interfaces = Vec::new();
    let mut total_download = 0u64;
    let mut total_upload = 0u64;

    // 获取所有网络接口的 IP 地址
    let network_interfaces = list_afinet_netifas().unwrap_or_else(|_| Vec::new());
    let mut ip_map: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    for (name, ip) in network_interfaces {
        // 只记录 IPv4 地址，并且不是本地环回地址
        if ip.is_ipv4() && !ip.is_loopback() {
            let ip_str = ip.to_string();
            // 过滤掉本地链路地址
            if !ip_str.starts_with("169.254.") && !ip_str.starts_with("fe80") {
                ip_map.insert(name.clone(), ip_str);
            }
        }
    }

    for (interface_name, data) in networks.iter() {
        // 过滤虚拟网卡 - 只保留物理网络接口
        if !is_physical_interface(interface_name) {
            continue;
        }

        let current_received = data.total_received();
        let current_transmitted = data.total_transmitted();

        let (last_received, last_transmitted) =
            last_stats.get(interface_name).copied().unwrap_or((0, 0));

        let download_speed = current_received.saturating_sub(last_received);
        let upload_speed = current_transmitted.saturating_sub(last_transmitted);

        // Update last stats for next calculation
        last_stats.insert(
            interface_name.clone(),
            (current_received, current_transmitted),
        );

        // 获取 IP 地址
        let ip_address = ip_map.get(interface_name).cloned();

        interfaces.push(NetworkInterface {
            name: interface_name.clone(),
            display_name: get_display_name(interface_name),
            ip_address,
            is_up: current_received > 0 || current_transmitted > 0,
            download_speed,
            upload_speed,
            total_downloaded: current_received,
            total_uploaded: current_transmitted,
        });

        total_download += download_speed;
        total_upload += upload_speed;
    }

    // Debug output for network (commented out)
    // println!("Total download speed: {} bytes/s", total_download);
    // println!("Total upload speed: {} bytes/s", total_upload);
    // println!("Number of interfaces: {}", interfaces.len());
    // for iface in &interfaces {
    //     println!("Interface: {} -> {}", iface.name, iface.display_name);
    // }

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

#[tauri::command]
async fn set_pin_on_top(window: WebviewWindow, pinned: bool) -> Result<(), String> {
    window
        .set_always_on_top(pinned)
        .map_err(|e| e.to_string())?;

    // 根据置顶状态设置拖动区域
    // 当不置顶时，可以拖动窗口；置顶时，禁用拖动
    #[cfg(target_os = "macos")]
    {
        let js_code = if pinned {
            // 置顶时禁用拖动
            "document.documentElement.style.webkitAppRegion = 'no-drag';"
        } else {
            // 不置顶时启用拖动
            "document.documentElement.style.webkitAppRegion = 'drag';"
        };
        window.eval(js_code).map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
async fn close_app() -> Result<(), String> {
    std::process::exit(0);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            get_system_stats,
            greet,
            set_pin_on_top,
            close_app
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
