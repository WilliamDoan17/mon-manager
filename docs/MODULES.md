# Modules

## Source Layout

```
mon/src/
  main.rs              — entry point, arg parsing, dispatches to commands
  commands/
    mod.rs             — declares command modules
    list.rs            — mon list
  drm/
    mod.rs             — Card type, opens /dev/dri/card*, DRM device handle
  types/
    mod.rs             — declares type modules
    monitor.rs         — Monitor, Mode structs
```

## Responsibilities

### `main.rs`
Parses subcommands and flags, calls the matching command function.

### `commands/`
One file per subcommand. Each command receives what it needs (a Card, args) and returns a result. Commands call into `drm/`, `i2c/`, or `profiles/` — never the other way around.

### `drm/`
Owns everything related to talking to the kernel DRM subsystem. Provides the `Card` type (an open `/dev/dri/card*` device) and functions for querying connector state, modes, CRTCs, and applying changes.

### `i2c/` _(planned)_
Owns DDC/CI communication over `/dev/i2c-*` for external monitor brightness.

### `profiles/` _(planned)_
Owns reading and writing TOML profile files in `~/.config/mon-manager/profiles/`.

### `types/`
Shared data structures used across the whole crate. No logic — just types.

## Dependency Rules

```
main.rs
  └── commands/
        └── drm/
        └── i2c/
        └── profiles/
              └── types/
```

`types/` has no dependencies. `drm/`, `i2c/`, `profiles/` have no knowledge of each other. Commands orchestrate them.
