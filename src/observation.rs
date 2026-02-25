use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WakeObservation {
    pub schema_version: u32,
    pub ts: f64,
    pub machine: MachineInfo,
    pub user: UserInfo,
    pub datetime: DateTimeInfo,
    pub filesystem: FilesystemInfo,
    pub installed_apps: Vec<InstalledApp>,
    pub network_identity: NetworkIdentity,
    pub listening_ports: Vec<ListeningPort>,
    pub resources: ResourceInfo,
    pub recent_activity: RecentActivity,
    pub other_sessions: Vec<SessionInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineInfo {
    pub hostname: String,
    pub os: String,
    pub os_version: String,
    pub kernel: String,
    pub arch: String,
    pub is_vm: bool,
    pub is_container: bool,
    pub hypervisor: Option<String>,
    pub chassis: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub username: String,
    pub full_name: String,
    pub home_dir: String,
    pub shell: String,
    pub uid: u32,
    pub groups: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateTimeInfo {
    pub ts: f64,
    pub iso: String,
    pub timezone: String,
    pub utc_offset_seconds: i32,
    pub uptime_seconds: u64,
    pub login_ts: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilesystemInfo {
    pub home_tree: Vec<HomeTreeEntry>,
    pub recent_files: Vec<RecentFileInfo>,
    pub mounts: Vec<MountInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomeTreeEntry {
    pub path: String,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_count: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentFileInfo {
    pub path: String,
    pub modified_ago_s: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountInfo {
    pub path: String,
    pub fs_type: String,
    pub total_gb: f64,
    pub free_gb: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledApp {
    pub name: String,
    pub id: String,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkIdentity {
    pub local_ips: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ip: Option<String>,
    pub vpn_active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpn_interface: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_gateway: Option<String>,
    pub dns_servers: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname_fqdn: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeningPort {
    pub port: u16,
    pub proto: String,
    pub pid: u32,
    pub app: String,
    pub addr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceInfo {
    pub cpu_cores: u32,
    pub cpu_model: String,
    pub ram_total_gb: f64,
    pub ram_free_gb: f64,
    pub gpus: Vec<GpuInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    pub name: String,
    pub vram_gb: Option<f64>,
    pub driver: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentActivity {
    pub shell_history: Vec<String>,
    pub running_since_boot: Vec<RunningProcessInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunningProcessInfo {
    pub pid: u32,
    pub app: String,
    pub started_ago_s: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub username: String,
    pub tty: String,
    pub from: String,
    pub login_ts: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    pub schema_version: u32,
    pub ts: f64,
    pub monotonic_ms: u64,
    pub idle_ms: u64,
    pub focus: Option<WindowInfo>,
    pub windows: Vec<WindowInfo>,
    pub cursor: Point,
    pub displays: Vec<DisplayInfo>,
    pub terminal_ctx: Option<TerminalCtx>,
    pub net_connections: Vec<ConnInfo>,
    pub fs_events: Vec<FSEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowInfo {
    pub id: String,
    pub title: String,
    pub app: String,
    pub pid: u32,
    pub bounds: Bounds,
    pub workspace: i32,
    pub is_minimized: bool,
    pub is_fullscreen: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayInfo {
    pub id: i32,
    pub bounds: Bounds,
    pub is_primary: bool,
    pub scale_factor: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalCtx {
    pub cwd: String,
    pub shell: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnInfo {
    pub proto: String,
    pub local_port: u16,
    pub remote_addr: String,
    pub remote_port: u16,
    pub pid: u32,
    pub app: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FSEvent {
    pub path: String,
    pub kind: String,
    pub ts: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bounds {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl WakeObservation {
    pub fn compact(mut self) -> Self {
        self.user.groups = compact_groups(std::mem::take(&mut self.user.groups));
        self.filesystem.home_tree =
            compact_home_tree(std::mem::take(&mut self.filesystem.home_tree));
        self.filesystem.recent_files =
            compact_recent_files(std::mem::take(&mut self.filesystem.recent_files));
        self.filesystem.mounts = compact_mounts(std::mem::take(&mut self.filesystem.mounts));
        self.network_identity.local_ips =
            compact_local_ips(std::mem::take(&mut self.network_identity.local_ips));
        self.listening_ports = compact_listening_ports(std::mem::take(&mut self.listening_ports));
        self.recent_activity.shell_history =
            compact_shell_history(std::mem::take(&mut self.recent_activity.shell_history));
        self.recent_activity.running_since_boot.clear();
        self.other_sessions.retain(|session| {
            let from = session.from.trim().to_ascii_lowercase();
            !(from.is_empty() || from == "local" || from == "-")
        });
        self.other_sessions.truncate(3);
        self
    }
}

fn compact_groups(groups: Vec<String>) -> Vec<String> {
    let mut filtered: Vec<String> = groups
        .into_iter()
        .filter(|group| {
            !group.starts_with('_')
                && !group.starts_with("com.apple.")
                && !matches!(
                    group.as_str(),
                    "everyone" | "localaccounts" | "_appserverusr" | "_appserveradm"
                )
        })
        .collect();

    filtered.sort();
    filtered.dedup();

    if filtered.iter().any(|group| group == "admin") {
        return vec!["admin".to_string()];
    }

    filtered.truncate(2);
    filtered
}

fn compact_home_tree(entries: Vec<HomeTreeEntry>) -> Vec<HomeTreeEntry> {
    let mut compacted: Vec<HomeTreeEntry> = entries
        .into_iter()
        .filter(|entry| !entry.path.starts_with("~/."))
        .map(|mut entry| {
            if entry.entry_count.is_none() {
                entry.entry_count = entry.children.as_ref().map(|children| children.len());
            }
            entry.children = None;
            entry
        })
        .collect();

    compacted.sort_by_key(|entry| home_tree_priority(&entry.path));
    compacted.truncate(6);
    compacted
}

fn compact_recent_files(files: Vec<RecentFileInfo>) -> Vec<RecentFileInfo> {
    let original = files.clone();
    let mut compacted: Vec<RecentFileInfo> = files
        .into_iter()
        .filter(|file| !is_noise_path(&file.path))
        .collect();

    if compacted.is_empty() {
        compacted = original;
    }

    compacted.truncate(5);
    compacted
}

fn compact_mounts(mounts: Vec<MountInfo>) -> Vec<MountInfo> {
    let mut compacted: Vec<MountInfo> = mounts
        .into_iter()
        .filter(|mount| {
            mount.path == "/" || mount.path == "/home" || mount.path.starts_with("/Volumes/")
        })
        .collect();

    compacted.sort_by(|left, right| left.path.cmp(&right.path));
    compacted.dedup_by(|left, right| left.path == right.path);
    compacted.truncate(3);
    compacted
}

fn compact_local_ips(ips: Vec<String>) -> Vec<String> {
    let mut filtered: Vec<String> = ips.into_iter().filter(|ip| ip.contains('.')).collect();
    filtered.sort();
    filtered.dedup();
    filtered.truncate(2);
    filtered
}

fn compact_listening_ports(ports: Vec<ListeningPort>) -> Vec<ListeningPort> {
    let keep_ports = [11434_u16, 8080, 6379, 5432, 5173, 3030, 3000, 5000];
    let noise_apps = ["controlce", "rapportd", "ardagent", "identitys"];

    let mut filtered: Vec<ListeningPort> = ports
        .into_iter()
        .filter(|port| {
            let app = port.app.to_ascii_lowercase();
            !noise_apps.iter().any(|noise| app.starts_with(noise))
                && (keep_ports.contains(&port.port) || port.port <= 32768)
        })
        .collect();

    filtered.sort_by(|left, right| left.port.cmp(&right.port).then(left.app.cmp(&right.app)));
    filtered.dedup_by(|left, right| left.port == right.port && left.app == right.app);
    filtered.truncate(12);
    filtered
}

fn compact_shell_history(history: Vec<String>) -> Vec<String> {
    let mut filtered: Vec<String> = history
        .into_iter()
        .filter_map(|line| normalize_shell_history_line(&line))
        .collect();

    filtered.dedup();
    let start = filtered.len().saturating_sub(5);
    filtered.drain(0..start);
    filtered
}

fn is_noise_path(path: &str) -> bool {
    let noise_fragments = [
        "/.cursor/",
        "/.yarn/",
        "/.docker/",
        "/Music Library.musiclibrary/",
        "/.cache/",
        "/.config/",
    ];

    noise_fragments
        .iter()
        .any(|fragment| path.contains(fragment))
}

fn home_tree_priority(path: &str) -> u8 {
    match path {
        "~/Code" | "~/code" | "~/Projects" | "~/projects" | "~/Work" | "~/work" => 0,
        "~/Desktop" | "~/Downloads" | "~/Documents" => 1,
        _ => 2,
    }
}

fn normalize_shell_history_line(line: &str) -> Option<String> {
    let mut value = line.trim().to_string();
    if value.is_empty() {
        return None;
    }

    if let Some((prefix, _)) = value.split_once(";                 EC=") {
        value = prefix.trim().to_string();
    }

    if value.starts_with("PS1=") || value.starts_with("PS2=") {
        return None;
    }

    if value.contains("___BEGIN___COMMAND_DONE_MARKER___") {
        return None;
    }

    if value.is_empty() { None } else { Some(value) }
}
