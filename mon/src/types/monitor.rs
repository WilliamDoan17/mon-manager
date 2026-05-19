pub struct Mode {
    pub width: u32,
    pub height: u32,
    pub refresh: f32,
}

pub struct Monitor {
    pub name: String,
    pub alias: Option<String>,
    pub connected: bool,
    pub enabled: bool,
    pub modes: Vec<Mode>,
    pub active_mode: Option<Mode>,
    pub position: (u32, u32),
    pub scale: f32,
    pub brightness: u8,
}
