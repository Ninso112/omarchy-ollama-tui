# ollama-tui

Terminal UI for [Ollama](https://ollama.ai/). Manage models, start/stop the server, and monitor GPU usage -- all from your terminal.

> Work in progress. Usable but rough around the edges.

![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)

## What it does

- Start and stop the Ollama server
- List, load, unload, and pull models
- Live GPU stats (utilization, VRAM, temperature) for NVIDIA, AMD, and Intel
- Vim-style navigation

## Requirements

- [Rust](https://rustup.rs/) >= 1.70
- [Ollama](https://ollama.ai/download)
- GPU drivers (optional) -- NVIDIA (NVML/CUDA), AMD (ROCm/AMDGPU), or Intel

## Install

```bash
git clone https://github.com/Ninso112/omarchy-ollama-tui.git
cd omarchy-ollama-tui
cargo install --path .
```

Build without NVIDIA support:

```bash
cargo install --path . --no-default-features
```

## Usage

```bash
ollama-tui
```

### Keys

| Key | Action |
|---|---|
| `q` / `Esc` | Quit |
| `j` / `k` / arrows | Navigate |
| `Enter` | Load model |
| `r` | Refresh list |
| `s` | Start/stop server |
| `u` | Unload all models |
| `p` | Pull model |

## Configuration

Config lives at `~/.config/ollama-tui/config.toml` (Linux), `~/Library/Application Support/ollama-tui/config.toml` (macOS), or `%APPDATA%\ollama-tui\config.toml` (Windows).

```toml
ollama_url = "http://localhost:11434"
update_interval_ms = 1000
max_status_messages = 100
```

## Building / Testing

```bash
cargo build                          # debug
cargo build --release                # release
cargo test --all-features            # tests
cargo clippy --all-features -- -D warnings
```

## Troubleshooting

**GPU stats show N/A?** Check that your drivers are installed (`nvidia-smi`, `rocm-smi`, or `/sys/class/drm/`). Rebuild with `--features nvidia` if needed.

**Ollama won't start?** Make sure `ollama` is in your PATH and port 11434 is free. Try `ollama serve` manually to see errors.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Some things that are still missing: model deletion, chat interface, Metal support, streaming pull progress.

## License

[GPL-3.0](LICENSE)
