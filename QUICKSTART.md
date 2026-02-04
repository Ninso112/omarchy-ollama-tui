# Quick Start Guide

Get up and running with Ollama TUI in minutes! Now with **automatic GPU detection** for NVIDIA, AMD, and Intel GPUs!

## Prerequisites

Before you begin, ensure you have:

1. **Rust** installed (1.70 or later)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Ollama** installed and running
   ```bash
   # Install Ollama (visit https://ollama.ai/download)
   
   # Start Ollama server
   ollama serve
   
   # Pull a model (in another terminal)
   ollama pull llama3.2
   ```

3. **(Optional) GPU** with drivers for GPU monitoring
   - **NVIDIA**: NVIDIA drivers + CUDA toolkit
   - **AMD**: ROCm or AMDGPU drivers
   - **Intel**: Intel graphics drivers (usually pre-installed)
   - The app automatically detects and uses your GPU!

## Installation

### Option 1: Build from Source (Recommended)

```bash
# Clone the repository
git clone https://github.com/Ninso112/omarchy-ollama-tui.git
cd omarchy-ollama-tui

# Build with all features (recommended)
cargo build --release --features nvidia

# Or build without optional features
cargo build --release --no-default-features

# Run the application
./target/release/ollama-tui
```

### Option 2: Install with Cargo

```bash
# With all features (recommended)
cargo install --git https://github.com/Ninso112/omarchy-ollama-tui.git --features nvidia

# Without optional features
cargo install --git https://github.com/Ninso112/omarchy-ollama-tui.git --no-default-features

# Run
ollama-tui
```

## First Run

1. **Start the application**
   ```bash
   ollama-tui
   ```

2. **Navigate the interface**
   - Use `↑`/`↓` or `j`/`k` to navigate the model list
   - Press `Enter` to load a model into memory
   - Press `r` to refresh the model list
   - Press `q` to quit

3. **Check Ollama status**
   - Look at the "GPU Statistics" panel on the right
   - The "Ollama:" line shows if the server is running
   - If stopped, press `s` to start it

## Common Tasks

### Loading a Model

1. Navigate to the model using arrow keys
2. Press `Enter` to load it
3. Watch the status log for confirmation

### Managing Memory

- Press `u` to unload all models from VRAM
- This frees up GPU memory

### Monitoring GPU

- **Automatic GPU detection** - works with NVIDIA, AMD, and Intel!
- Real-time GPU stats appear in the top-right panel
- Shows utilization, VRAM usage, and temperature
- Updates automatically every second

### Starting/Stopping Ollama

- Press `s` to toggle the Ollama server
- Watch the status log for confirmation

### Pulling New Models

1. Press `p` to open the pull dialog
2. Type the model name (e.g., `llama3.2`, `mistral:7b`)
3. Press `Enter` to start downloading
4. Wait for the download to complete (may take a few minutes)
5. Model will automatically appear in your list

## Keyboard Reference

| Key            | Action                    |
|----------------|---------------------------|
| `q` / `Esc`    | Quit application          |
| `↑` / `k`      | Move selection up         |
| `↓` / `j`      | Move selection down       |
| `Enter`        | Load selected model       |
| `r`            | Refresh model list        |
| `s`            | Start/Stop Ollama server  |
| `u`            | Unload all models         |
| `p`            | Pull new model (dialog)   |

## Troubleshooting

### "No models found"

```bash
# Pull a model first
ollama pull llama3.2
# or
ollama pull mistral
# Then press 'r' in the TUI to refresh
```

### "Ollama server is not running"

```bash
# Start Ollama manually
ollama serve
# Or press 's' in the TUI
```

### "GPU: N/A" or No GPU Stats

The app automatically detects your GPU! If stats don't show:

1. **Check your GPU type**:
   - NVIDIA: Run `nvidia-smi`
   - AMD: Run `rocm-smi` or check `/sys/class/drm/`
   - Intel: Check `/sys/class/drm/card*`

2. **Install proper drivers**:
   - NVIDIA: NVIDIA drivers + CUDA toolkit
   - AMD: ROCm or AMDGPU drivers
   - Intel: Usually pre-installed on Linux

3. **Rebuild if needed**:
   ```bash
   cargo build --release --features nvidia
   ```

### Application won't start

1. Check Rust version:
   ```bash
   rustc --version  # Should be 1.70+
   ```

2. Update dependencies:
   ```bash
   cargo update
   ```

3. Clean and rebuild:
   ```bash
   cargo clean
   cargo build --release
   ```

## Configuration

The app creates a config file at:
- **Linux**: `~/.config/ollama-tui/config.toml`
- **macOS**: `~/Library/Application Support/ollama-tui/config.toml`
- **Windows**: `%APPDATA%\ollama-tui\config.toml`

Default configuration:
```toml
ollama_url = "http://localhost:11434"
update_interval_ms = 1000
max_status_messages = 100
```

## Next Steps

- Read the full [README.md](README.md) for detailed features
- Check [CONTRIBUTING.md](CONTRIBUTING.md) to contribute
- Report issues on [GitHub Issues](https://github.com/yourusername/ollama-tui/issues)

## Tips

- **Performance**: Models load faster on subsequent runs (cached in VRAM)
- **Memory**: Monitor VRAM usage to avoid OOM errors
- **Models**: Smaller models (7B) load faster than larger ones (70B)
- **Shortcuts**: Use Vim-style `j`/`k` for faster navigation
- **Pulling**: Press `p` to pull models directly from the TUI!
- **GPU Auto-detect**: Works with NVIDIA, AMD, and Intel GPUs automatically

## Example Workflow

```bash
# 1. Start Ollama (if not running)
ollama serve

# 2. Start the TUI
ollama-tui

# 3. In the TUI:
#    - Press 'p' to pull a new model (e.g., "llama3.2")
#    - Or press 'r' if you already have models
#    - Navigate to a model
#    - Press Enter to load it
#    - Monitor GPU usage in real-time (auto-detected!)
#    - Press 'u' to unload when done
#    - Press 'q' to quit
```

## Support

Need help? Check these resources:
- [GitHub Issues](https://github.com/Ninso112/omarchy-ollama-tui/issues)
- [GitHub Discussions](https://github.com/Ninso112/omarchy-ollama-tui/discussions)
- [Ollama Documentation](https://github.com/ollama/ollama/tree/main/docs)

Happy LLM management! 🦙