# Build Plan

## Phase 1 — CLI

Goal: a working `mon` binary that talks directly to the kernel, no external tools.

1. Project scaffold (`cargo new`, workspace layout)
2. DRM/KMS foundation — open `/dev/dri/card*`, enumerate connectors, read current state
3. `mon list` — display all connected monitors, their supported modes, and current config
4. `mon set` — apply resolution, refresh rate, position via DRM ioctls (`drmModeSetCrtc`)
5. `mon enable` / `mon disable` — connect or disconnect a CRTC
6. `mon mirror` — assign two connectors to the same CRTC and mode
7. `mon brightness` for laptop — read/write `/sys/class/backlight/{dev}/brightness`
8. `mon brightness` for external — implement DDC/CI over I2C (`/dev/i2c-*`, VCP code 0x10)
9. `mon profile save` / `load` / `list` / `delete` — TOML-based profile system

## Phase 2 — GUI

Goal: a GTK4 app that wraps the CLI in a visual interface.

1. Project scaffold (`mon-gui` crate in workspace)
2. Display monitor list — run `mon list --json`, render as cards
3. Per-monitor controls — resolution dropdown, rate dropdown, scale slider, brightness slider
4. Monitor arrangement — drag-and-drop canvas showing relative positions
5. Profile panel — save, load, delete profiles

## Phase 3 — Packaging

Goal: distribute mon-manager so others can install it easily.

1. AUR package (Arch) — `PKGBUILD`
2. Flatpak — `org.mon-manager.App` manifest
3. `.deb` package (Debian/Ubuntu)
4. GitHub Actions CI — build and publish releases

## Phase 4 — Polish

- Auto-apply profile on monitor hotplug (udev events via `netlink`)
- System tray icon with quick profile switcher
- Notification on profile load / brightness change

## Key DRM/KMS Concepts to Learn

| Concept | What it is |
|---------|-----------|
| Connector | Physical port (DP-1, HDMI-A-1, eDP-1) |
| CRTC | The scanout engine — drives one display at a time |
| Mode | A resolution + refresh rate combo (e.g. 1920x1080@144) |
| Framebuffer | The pixel buffer the CRTC reads from |
| Plane | Overlay layer composited onto the framebuffer |
| ioctl | How userspace talks to kernel drivers |
