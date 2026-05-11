# Architecture

## Overview

mon-manager talks directly to the Linux kernel — no external tools, no middleware.

```
mon-gui  (GTK4 frontend)
    │
    └── executes
          │
        mon  (CLI — source of truth)
              │
    ┌─────────┴──────────────┐
    │                        │
 DRM/KMS                  I2C bus
 /dev/dri/card*           /dev/i2c-*
    │                        │
 resolution, rate,       DDC/CI (external brightness)
 scale, layout           /sys/class/backlight (laptop)
```

No `hyprctl`, no `ddcutil`, no `brightnessctl`. Everything goes through kernel interfaces directly.

## Linux Display Stack

```
Hardware (GPU + monitors)
    │
 DRM/KMS  ← we talk here
 (kernel)
    │
 Wayland compositor (Hyprland)
    │
 Applications
```

DRM (Direct Rendering Manager) and KMS (Kernel Mode Setting) are the kernel subsystem responsible for managing display hardware. By talking to it directly via `/dev/dri/card*` we bypass the compositor entirely for configuration.

## Components

### mon (CLI)

Written in Rust. Responsible for:
- Parsing subcommands and flags
- Opening and querying DRM devices (`/dev/dri/card*`) via ioctls
- Reading connector state, supported modes, current config
- Applying mode changes (resolution, refresh rate, position)
- Controlling brightness via I2C/DDC-CI (`/dev/i2c-*`) for external monitors
- Controlling brightness via sysfs (`/sys/class/backlight/`) for laptop screens
- Reading and writing profile configs (TOML in `~/.config/mon-manager/profiles/`)

### mon-gui (GTK4 GUI)

Written in Rust using `gtk-rs`. Responsible for:
- Displaying current monitor state (by running `mon list --json`)
- Letting the user interact with controls (sliders, dropdowns, drag-to-arrange)
- Translating interactions into `mon` CLI calls
- Showing output / errors back to the user

## Data Flow

```
User runs: mon set DP-1 --res 1920x1080 --rate 144
    │
    ├── open /dev/dri/card* (scan all cards)
    ├── enumerate connectors → find DP-1
    ├── find matching mode (1920x1080 @ 144hz) in connector's mode list
    ├── call DRM ioctl: drmModeSetCrtc
    └── report success or error
```

```
User runs: mon brightness DP-1 80
    │
    ├── detect monitor type:
    │     laptop screen → /sys/class/backlight/{dev}/brightness
    │     external      → /dev/i2c-* (DDC/CI VCP code 0x10)
    └── write value directly
```

## Key Kernel Interfaces

| Interface | Path | Used for |
|-----------|------|----------|
| DRM/KMS | `/dev/dri/card*` | Monitor detection, resolution, refresh rate, layout |
| sysfs backlight | `/sys/class/backlight/` | Laptop screen brightness |
| I2C / DDC-CI | `/dev/i2c-*` | External monitor brightness and contrast |

## Profile Storage

Profiles are stored as TOML files:

```
~/.config/mon-manager/
└── profiles/
    ├── home.toml
    └── mobile.toml
```

Each profile captures the full DRM state of all active monitors at save time — connector names, modes, positions, and brightness levels.

## No External Dependencies

mon-manager has zero runtime dependencies on other tools.  
Everything it does, it does itself.
