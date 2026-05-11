# mon-manager

A monitor management tool for Hyprland (Wayland).  
CLI-first, with a GTK4 GUI frontend built on top.

## Features

- List all connected monitors with their current configuration
- Set resolution, refresh rate, scale, and position
- Enable / disable monitors
- Mirror monitors
- Control brightness — laptop backlight and external monitors (DDC/CI)
- Save and load named profiles for different setups

## CLI Quick Reference

```bash
# List monitors
mon list

# Configure a monitor
mon set DP-1 --res 1920x1080 --rate 144 --scale 1.0

# Set position relative to another monitor
mon set DP-2 --right-of DP-1

# Brightness (0–100)
mon brightness DP-1 80

# Enable / disable
mon enable DP-1
mon disable DP-2

# Mirror
mon mirror DP-2 --of DP-1

# Profiles
mon profile save home
mon profile load home
mon profile list
mon profile delete home
```

## Docs

- [Architecture](docs/ARCHITECTURE.md)
- [Installation](docs/INSTALLATION.md)
- [Build Plan](docs/BUILD_PLAN.md)
- [Progress](docs/PROGRESS.md)
- [Tasks](docs/TASKS.md)

## License

MIT
