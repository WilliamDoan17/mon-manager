use std::fs;
use std::fs::DirEntry;
use crate::drm::Card;
use drm::control::Device;
use drm::control::connector;
use crate::types::monitor::{Monitor, Mode};

fn open_card_file(card_file: DirEntry) -> Option<Card> {
    let name = card_file.file_name();
    let name_str = name.to_str().unwrap();
    if !name_str.starts_with("card") {
        return None;
    }
    let path = card_file.path();
    let path_str = path.to_str().unwrap();
    Some(Card::open(path_str))
}

fn get_card_monitors(card: Card) -> Vec<Monitor> {
    let mut monitors: Vec<Monitor> = Vec::new();
    let card_resources = card.resource_handles().unwrap();
    let conn_handles = card_resources.connectors().to_vec();
    for conn_handle in conn_handles {
        let conn = card.get_connector(conn_handle, false).unwrap();
        let interface_str = match conn.interface() {
            connector::Interface::DisplayPort => "DP",
            connector::Interface::HDMIA => "HDMI-A",
            connector::Interface::HDMIB => "HDMI-B",
            connector::Interface::LVDS => "LVDS",
            connector::Interface::EmbeddedDisplayPort => "eDP",
            connector::Interface::VGA => "VGA",
            _ => "Unknown",
        };
        let name = format!("{}-{}", interface_str, conn.interface_id());
        let connected = conn.state() == connector::State::Connected;
        let modes: Vec<Mode> = conn.modes().iter().map(|m| Mode {
            width: m.size().0 as u32,
            height: m.size().1 as u32,
            refresh: m.vrefresh() as f32,
        }).collect();

        let (enabled, active_mode, position) = conn.current_encoder()
            .and_then(|enc_handle| card.get_encoder(enc_handle).ok())
            .and_then(|enc| enc.crtc())
            .and_then(|crtc_handle| card.get_crtc(crtc_handle).ok())
            .map(|crtc| {
                let active = crtc.mode().map(|m| Mode {
                    width: m.size().0 as u32,
                    height: m.size().1 as u32,
                    refresh: m.vrefresh() as f32,
                });
                (active.is_some(), active, crtc.position())
            })
            .unwrap_or((false, None, (0, 0)));

        monitors.push(Monitor {
            name,
            alias: None,
            connected,
            enabled,
            modes,
            active_mode,
            position,
            scale: 1.0,
            brightness: 0,
        });
    }
    monitors
}

pub fn list_monitors() {
    let cards = fs::read_dir("/dev/dri/").unwrap();
    let mut monitors: Vec<Monitor> = Vec::new();
    for card_rs in cards {
        let card_file = card_rs.unwrap();
        if let Some(card) = open_card_file(card_file) {
            let mut card_monitors = get_card_monitors(card);
            monitors.append(&mut card_monitors);
        }
    }
    for monitor in &monitors {
        println!("{}", monitor.name);
        println!("- connected: {}", monitor.connected);
        println!("- enabled: {}", monitor.enabled);
        println!("- position: ({}, {})", monitor.position.0, monitor.position.1);
        println!("- scale: {}", monitor.scale);
        println!("- brightness: {}", monitor.brightness);
        if let Some(mode) = &monitor.active_mode {
            println!("- active_mode: {}x{}@{:.0}Hz", mode.width, mode.height, mode.refresh);
        }
        println!("- modes:");
        for mode in &monitor.modes {
            println!("  - {}x{}@{:.0}Hz", mode.width, mode.height, mode.refresh);
        }
    }
}
