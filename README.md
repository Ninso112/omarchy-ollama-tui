# Ollama TUI 🦙

[![CI](https://github.com/Ninso112/omarchy-ollama-tui/actions/workflows/ci.yml/badge.svg)](https://github.com/Ninso112/omarchy-ollama-tui/actions/workflows/ci.yml)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org)

A powerful terminal user interface (TUI) for managing [Ollama](https://ollama.ai/) - your local LLM server - with **automatic multi-GPU support** (NVIDIA, AMD, Intel) and real-time monitoring.

![Ollama TUI Screenshot](docs/screenshot.png)

## ✨ Features

- 🎮 **Interactive TUI**: Beautiful terminal interface built with [ratatui](https://github.com/ratatui-org/ratatui)
- 🔧 **Ollama Control**: Start/Stop the Ollama server directly from the TUI
- 📦 **Model Management**: 
  - List all available models
  - Load models into memory
  - Unload models to free VRAM
  - **Pull new models from Ollama library with interactive dialog**
  - View model details (size, modified date)
- 📊 **Real-time GPU Monitoring**:
  - **Automatic GPU detection (NVIDIA, AMD, Intel)**
  - GPU utilization percentage
  - VRAM usage (used/total)
  - GPU temperature
  - Visual progress bars
- 📝 **Status Logging**: Real-time status updates and error messages
- ⌨️ **Keyboard Navigation**: Vim-style keybindings for efficient control

## 📋 Prerequisites

- **Rust**: 1.70 or later ([install](https://rustup.rs/))
- **Ollama**: Installed on your system ([install](https://ollama.ai/download))
- **GPU** (optional): For GPU monitoring
  - **NVIDIA**: Drivers + CUDA toolkit (NVML support)
  - **AMD**: ROCm drivers (sysfs monitoring)
  - **Intel**: Intel graphics drivers (sysfs monitoring)
  - Automatic detection - works with any supported GPU!

## 🚀 Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/Ninso112/omarchy-ollama-tui.git
cd omarchy-ollama-tui

# Build and install (with all GPU support)
cargo install --path . --features nvidia

# Or build without optional features
cargo install --path . --no-default-features
```

### Using Cargo

```bash
# With NVIDIA support
cargo install ollama-tui --features nvidia

# Without NVIDIA support
cargo install ollama-tui --no-default-features
```

## 🎯 Usage

Simply run the application:

```bash
ollama-tui
```

### Keybindings

| Key | Action |
|-----|--------|
| `q` / `Esc` | Quit the application |
| `↑` / `k` | Move selection up |
| `↓` / `j` | Move selection down |
| `Enter` | Load selected model |
| `r` | Refresh model list |
| `s` | Start/Stop Ollama server |
| `u` | Unload all models from memory |
| `p` | **Pull a new model (interactive dialog)** |

## 🏗️ Architecture

```
ollama-tui/
├── src/
│   ├── main.rs           # Entry point and async runtime
│   ├── app.rs            # Application state and logic
│   ├── config.rs         # Configuration management
│   ├── ui/               # User interface components
│   │   ├── layout.rs     # Main layout composition
│   │   ├── model_list.rs # Model list widget
│   │   ├── gpu_stats.rs  # GPU statistics widget
│   │   └── status_bar.rs # Status bar and keybindings
│   ├── ollama/           # Ollama API client
│   │   ├── client.rs     # HTTP client
│   │   ├── models.rs     # Data structures
│   │   └── process.rs    # Process management
│   ├── gpu/              # GPU monitoring
│   │   ├── nvidia.rs     # NVML wrapper
│   │   └── fallback.rs   # Fallback for non-NVIDIA
│   └── events/           # Event handling
│       ├── handler.rs    # Keyboard events
│       └── tick.rs       # Tick events
└── tests/                # Integration tests
```

## ⚙️ Configuration

Ollama TUI stores its configuration in:
- **Linux**: `~/.config/ollama-tui/config.toml`
- **macOS**: `~/Library/Application Support/ollama-tui/config.toml`
- **Windows**: `%APPDATA%\ollama-tui\config.toml`

### Configuration Options

```toml
# Ollama API URL
ollama_url = "http://localhost:11434"

# Update interval in milliseconds
update_interval_ms = 1000

# Maximum number of status messages to keep
max_status_messages = 100
```

## 🔧 Development

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# With NVIDIA support
cargo build --features nvidia

# Without default features
cargo build --no-default-features
```

### Testing

```bash
# Run all tests
cargo test

# Run tests with all features
cargo test --all-features

# Run tests without default features
cargo test --no-default-features
```

### Code Quality

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Run clippy
cargo clippy -- -D warnings

# Run clippy with all features
cargo clippy --all-features -- -D warnings
```

## 🐛 Troubleshooting

### GPU Monitoring Not Working

If GPU statistics show "N/A" or zeros:

1. **Check your GPU type**:
   - **NVIDIA**: Run `nvidia-smi` to verify drivers
   - **AMD**: Run `rocm-smi` or check `/sys/class/drm/`
   - **Intel**: Check `/sys/class/drm/` for Intel GPU

2. **Ensure proper drivers are installed**:
   - **NVIDIA**: NVIDIA drivers + CUDA toolkit
   - **AMD**: ROCm drivers or AMDGPU drivers
   - **Intel**: Intel graphics drivers (usually pre-installed)

3. **Rebuild if needed**:
   ```bash
   cargo install --path . --features nvidia --force
   ```

The application automatically detects your GPU type and uses the appropriate monitoring method!

### Ollama Server Won't Start

1. **Check if Ollama is installed**:
   ```bash
   which ollama
   ```

2. **Try starting manually**:
   ```bash
   ollama serve
   ```

3. **Check port availability**: Ensure port 11434 is not in use by another process.

### Models Not Loading

1. **Verify Ollama is running**: Check the "Ollama" status in the GPU Statistics panel
2. **Check available models**: Run `ollama list` in your terminal
3. **Pull a model if none exist**: Run `ollama pull llama3.2`

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

### Areas for Contribution

- [ ] Model deletion functionality
- [ ] Chat interface integration
- [ ] macOS Metal support
- [ ] Configuration UI
- [ ] Model search and filtering
- [ ] Export logs functionality
- [ ] Streaming progress for model pulls

## 📝 License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Ollama](https://ollama.ai/) - Local LLM server
- [ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) - Cross-platform terminal manipulation
- [nvml-wrapper](https://github.com/Cldfire/nvml-wrapper) - NVIDIA Management Library wrapper

## 📚 Resources

- [Ollama Documentation](https://github.com/ollama/ollama/tree/main/docs)
- [Ollama API Reference](https://github.com/ollama/ollama/blob/main/docs/api.md)
- [Ratatui Book](https://ratatui.rs/)

## 🔗 Related Projects

- [Ollama](https://github.com/ollama/ollama) - The Ollama LLM server
- [ollama-python](https://github.com/ollama/ollama-python) - Python client for Ollama
- [ollama-js](https://github.com/ollama/ollama-js) - JavaScript client for Ollama

## 📮 Contact

- Issues: [GitHub Issues](https://github.com/Ninso112/omarchy-ollama-tui/issues)
- Discussions: [GitHub Discussions](https://github.com/Ninso112/omarchy-ollama-tui/discussions)

---

Made with ❤️ by the open-source community