pub struct Mode {
    width: u32,
    height: u32,
    refresh: f32
}

pub struct Monitor {
    name: String,
    alias: Option<String>,
    connected: bool,
    enabled: bool,
    modes: Vec<Mode>,
    active_mode: Option<Mode>,
    position: (u32, u32),
    scale: f32,
    brightness: u8,
}
