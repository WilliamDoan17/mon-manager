# Types

## Monitor

### Mode

| Field | Type | Notes | 
| --- | --- | --- |
| width | u32 | pixels |
| height | u32 | pixels |
| refresh | f32 | Hz |

### Monitor

| Field | Type | Notes |
| --- | --- | --- |
| name | String | connector name e.g. "DP-1" |
| alias | Option\<String\> | user-defined label |
| connected | bool | cable plugged in |
| enabled | bool | actively driven |
| modes | Vec\<Mode\> | supported modes |
| active_mode | Option\<Mode\> | current mode, None if disabled |
| position | (u32, u32) | x, y offset in layout |
| scale | f32 | HiDPI scale factor |
| brightness | u8 | 0–100 |


