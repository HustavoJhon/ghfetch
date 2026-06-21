use sysinfo::System;

#[derive(Debug, Clone, Default)]
pub struct SystemInfo {
    pub os: String,
    pub kernel: String,
    pub hostname: String,
    pub cpu: String,
    pub gpu: String,
    pub ram_used: u64,
    pub ram_total: u64,
    pub disk_used: u64,
    pub disk_total: u64,
    pub shell: String,
    pub terminal: String,
    pub resolution: String,
    pub uptime: String,
    pub arch: String,
}

fn get_gpu() -> String {
    if cfg!(target_os = "linux") {
        if let Ok(out) = std::process::Command::new("lspci")
            .args(["-mm"])
            .output()
        {
            let s = String::from_utf8_lossy(&out.stdout);
            for line in s.lines() {
                if line.contains("VGA") || line.contains("3D") || line.contains("Display") {
                    let parts: Vec<&str> = line.split('"').collect();
                    if parts.len() >= 4 {
                        let gpu = parts[3].trim().to_string();
                        if !gpu.is_empty() {
                            return gpu.trim_start_matches("Intel Corporation ").to_string();
                        }
                    }
                }
            }
        }
    } else if cfg!(target_os = "macos") {
        if let Ok(out) = std::process::Command::new("system_profiler")
            .args(["SPDisplaysDataType"])
            .output()
        {
            let s = String::from_utf8_lossy(&out.stdout);
            for line in s.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("Chipset Model:") {
                    return trimmed
                        .strip_prefix("Chipset Model:")
                        .unwrap_or("")
                        .trim()
                        .to_string();
                }
            }
        }
    } else if cfg!(target_os = "windows") {
        if let Ok(out) = std::process::Command::new("wmic")
            .args(["path", "win32_videocontroller", "get", "name"])
            .output()
        {
            let s = String::from_utf8_lossy(&out.stdout);
            for line in s.lines().skip(1) {
                let t = line.trim();
                if !t.is_empty() && !t.eq_ignore_ascii_case("name") {
                    return t.to_string();
                }
            }
        }
    }
    "Unknown".to_string()
}

fn get_resolution() -> String {
    if cfg!(target_os = "linux") {
        if let Ok(out) = std::process::Command::new("xrandr").output() {
            let s = String::from_utf8_lossy(&out.stdout);
            for line in s.lines() {
                if line.contains(" connected") && !line.contains("disconnected") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    for (i, p) in parts.iter().enumerate() {
                        if p.contains('x') && p.contains('+') {
                            let res = p.split('+').next().unwrap_or(p);
                            let prev = if i > 0 { parts[i - 1] } else { "" };
                            if prev.ends_with('x') || prev.parse::<i32>().is_ok() {
                                continue;
                            }
                            return res.to_string();
                        }
                    }
                }
            }
        }
    } else if cfg!(target_os = "macos") {
        if let Ok(out) = std::process::Command::new("system_profiler")
            .args(["SPDisplaysDataType"])
            .output()
        {
            let s = String::from_utf8_lossy(&out.stdout);
            for line in s.lines() {
                let t = line.trim();
                if t.starts_with("Resolution:") {
                    return t.strip_prefix("Resolution:").unwrap_or("").trim().to_string();
                }
            }
        }
    } else if cfg!(target_os = "windows") {
        if let Ok(out) = std::process::Command::new("wmic")
            .args([
                "path",
                "Win32_VideoController",
                "get",
                "CurrentHorizontalResolution,CurrentVerticalResolution",
            ])
            .output()
        {
            let s = String::from_utf8_lossy(&out.stdout);
            for line in s.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let (Ok(w), Ok(h)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                        return format!("{}x{}", w, h);
                    }
                }
            }
        }
    }
    "Unknown".to_string()
}

fn format_uptime(secs: u64) -> String {
    let days = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let mins = (secs % 3600) / 60;
    if days > 0 {
        format!("{}d {}h {}m", days, hours, mins)
    } else if hours > 0 {
        format!("{}h {}m", hours, mins)
    } else {
        format!("{}m", mins)
    }
}

fn format_bytes(bytes: u64) -> String {
    let units = ["B", "KiB", "MiB", "GiB", "TiB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;
    while size >= 1024.0 && unit_idx < units.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }
    format!("{:.1} {}", size, units[unit_idx])
}

pub fn gather() -> SystemInfo {
    let mut sys = System::new_all();
    sys.refresh_all();

    let os = System::name().unwrap_or_else(|| "Unknown".to_string());
    let kernel = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
    let hostname = System::host_name().unwrap_or_else(|| "Unknown".to_string());
    let arch = std::env::consts::ARCH.to_string();

    let cpu = sys
        .cpus()
        .first()
        .map(|c| c.brand().trim().to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let ram_used = sys.used_memory();
    let ram_total = sys.total_memory();
    let uptime = format_uptime(System::uptime());

    let gpu = get_gpu();
    let resolution = get_resolution();

    let shell = std::env::var("SHELL")
        .unwrap_or_default()
        .split('/')
        .next_back()
        .unwrap_or("unknown")
        .to_string();

    let terminal = std::env::var("TERM").unwrap_or_else(|_| "unknown".to_string());

    let cwd = std::env::current_dir().unwrap_or_default();
    let disks = sysinfo::Disks::new_with_refreshed_list();
    let mut disk_used = 0u64;
    let mut disk_total = 0u64;
    for d in disks.list() {
        if cwd.starts_with(d.mount_point()) {
            disk_used = d.total_space() - d.available_space();
            disk_total = d.total_space();
            break;
        }
    }

    SystemInfo {
        os,
        kernel,
        hostname,
        cpu,
        gpu,
        ram_used,
        ram_total,
        disk_used,
        disk_total,
        shell,
        terminal,
        resolution,
        uptime,
        arch,
    }
}

impl SystemInfo {
    pub fn ram_used_str(&self) -> String {
        format_bytes(self.ram_used)
    }

    pub fn ram_total_str(&self) -> String {
        format_bytes(self.ram_total)
    }

    pub fn disk_used_str(&self) -> String {
        format_bytes(self.disk_used)
    }

    pub fn disk_total_str(&self) -> String {
        format_bytes(self.disk_total)
    }
}
