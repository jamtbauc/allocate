# Allocate

A desktop budgeting app built using Tauri and Angular

## Logging

Logs are written to the default log directory for your operating system, as determined by the [Tauri log plugin](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/log).

### Log Locations

| OS      | Path                                                  |
|---------|-------------------------------------------------------|
| Linux   | `{configDir}/com.allocate.app/logs/`                  |
| macOS   | `{homeDir}/Library/Logs/com.allocate.app/`            |
| Windows | `{APPDATA}\com.allocate.app\logs\`                    |


### Log Files

| File        | Contents    |
|-------------|-------------|
| `error.log` | Errors only |
| `info.log`  | Info events |

### Log Format

```
[<timestamp>-<level>] <message>
```

**Example:**
```
[2026-06-09 14:32:01-INFO] Server started
[2026-06-09 14:32:02-ERROR] Failed to connect to database
```