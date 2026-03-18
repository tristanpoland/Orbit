# Orbit - Windows Shell Replacement

# INCOMPLETE WIP

A complete Windows shell replacement built with [WGPUI](https://github.com/Far-Beyond-Pulsar/WGPUI), designed to replace Explorer and provide a modern, customizable desktop environment.

## ⚠️ Warning

This software replaces critical Windows components. Use with caution and ensure you have a system restore point before installation.

## Architecture

Orbit consists of four main components:

- **orbit-core**: Shared library with common types and IPC protocol
- **orbit-wm**: Window manager library with hooks and window state tracking
- **orbit-service**: Windows service handling system-level operations (DWM replacement, service control)
- **orbit-ui**: Main UI executable providing the desktop environment and compositor

## Features

- 🎨 Custom desktop environment using WGPUI
- 🪟 Complete Explorer.exe replacement
- 🖼️ Custom window decorations and compositor (DWM replacement)
- 🎯 Full window management (hooks, frame removal, z-order)
- 🔧 System-level service for display management
- 📝 Registry-based shell replacement
- 🔒 Secure IPC between service and UI

## Building

```powershell
# Build all components
cargo build --release

# Build individual components
cargo build -p orbit-core --release
cargo build -p orbit-service --release
cargo build -p orbit-ui --release
```

## Installation

**Administrator privileges required**

```powershell
# Install the service
.\target\release\orbit-service.exe install

# Start the service
.\target\release\orbit-service.exe start

# Set as Windows shell (backs up Explorer)
.\target\release\orbit-service.exe set-shell
```

## Uninstallation

```powershell
# Restore Explorer as shell
.\target\release\orbit-service.exe restore-shell

# Stop and uninstall service
.\target\release\orbit-service.exe stop
.\target\release\orbit-service.exe uninstall
```

## Development

```powershell
# Run UI in development mode (without service)
cargo run -p orbit-ui

# Run tests
cargo test
```

## Requirements

- Windows 10/11
- Administrator privileges for installation
- Rust 1.70+ for building

## License

MIT License - See LICENSE file for details

## Safety

Always maintain a system restore point. To boot into safe mode if something goes wrong:
1. Restart and press F8 repeatedly
2. Select "Safe Mode"
3. Run uninstall scripts

## Contributing

Contributions welcome! Please ensure all changes are tested in a VM before submitting PRs.
