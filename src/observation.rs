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
