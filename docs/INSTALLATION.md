# Installation

> Packaging is in progress. See [Build Plan](build_plan.md) for the packaging roadmap.

## Build from Source

### 1. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Install system dependencies

**Arch / Hyprland (recommended):**
```bash
sudo pacman -S gtk4 ddcutil brightnessctl
```

**Debian / Ubuntu:**
```bash
sudo apt install libgtk-4-dev ddcutil brightnessctl
```

### 3. Clone and build

```bash
git clone https://github.com/yourusername/mon-manager
cd mon-manager
cargo build --release
```

### 4. Install binaries

```bash
sudo cp target/release/mon /usr/local/bin/
sudo cp target/release/mon-gui /usr/local/bin/
```

## Optional: ddcutil setup

`ddcutil` requires access to the i2c bus. Add your user to the `i2c` group:

```bash
sudo usermod -aG i2c $USER
```

Then log out and back in. Verify with:

```bash
ddcutil detect
```

## Verify installation

```bash
mon list
```
