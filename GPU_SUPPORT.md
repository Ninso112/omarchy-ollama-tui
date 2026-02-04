# GPU Support Documentation

## Overview

Ollama TUI features **automatic GPU detection** and monitoring for NVIDIA, AMD, and Intel graphics cards. The application intelligently detects your GPU hardware on startup and uses the most appropriate monitoring method.

## Supported GPUs

### ✅ NVIDIA GPUs (Full Support)

**Detection Method**: NVML (NVIDIA Management Library)

**Supported Cards**:
- GeForce RTX 40 Series (4090, 4080, 4070, etc.)
- GeForce RTX 30 Series (3090, 3080, 3070, 3060, etc.)
- GeForce RTX 20 Series (2080 Ti, 2080, 2070, etc.)
- GeForce GTX 16 Series (1660 Ti, 1650, etc.)
- GeForce GTX 10 Series (1080 Ti, 1070, 1060, etc.)
- Tesla/Quadro/A-Series professional cards

**Monitored Metrics**:
- ✅ GPU Utilization (%)
- ✅ VRAM Usage (used/total)
- ✅ GPU Temperature (°C)
- ✅ GPU Name/Model

**Requirements**:
```bash
# NVIDIA drivers
nvidia-smi  # Should work

# CUDA toolkit (for NVML)
# Install from: https://developer.nvidia.com/cuda-downloads
```

### ✅ AMD GPUs (Good Support)

**Detection Method**: sysfs + hwmon

**Supported Cards**:
- Radeon RX 7000 Series (7900 XTX, 7900 XT, etc.)
- Radeon RX 6000 Series (6950 XT, 6900 XT, 6800 XT, 6700 XT, etc.)
- Radeon RX 5000 Series (5700 XT, 5600 XT, etc.)
- Radeon VII
- Vega Series
- Polaris Series (RX 580, 570, etc.)

**Monitored Metrics**:
- ✅ GPU Utilization (%)
- ✅ VRAM Usage (used/total)
- ✅ GPU Temperature (°C)
- ✅ GPU Name/Model

**Requirements**:
```bash
# AMDGPU drivers (usually pre-installed on modern Linux)
ls /sys/class/drm/card*/device/vendor  # Should show 0x1002

# Optional: ROCm for enhanced features
# Install from: https://rocm.docs.amd.com/
```

**Monitoring Paths**:
- GPU Busy: `/sys/class/drm/card0/device/gpu_busy_percent`
- VRAM: `/sys/class/drm/card0/device/mem_info_vram_*`
- Temperature: `/sys/class/drm/card0/device/hwmon/hwmon*/temp1_input`

### ✅ Intel GPUs (Basic Support)

**Detection Method**: sysfs + i915 driver

**Supported Cards**:
- Intel Arc A-Series (A770, A750, A580, A380, A310)
- Intel Iris Xe Graphics (11th/12th/13th Gen)
- Intel UHD Graphics (10th Gen and newer)
- Intel HD Graphics (some older models)

**Monitored Metrics**:
- ✅ GPU Utilization (estimated from frequency)
- ✅ Memory Usage (shared RAM for integrated)
- ✅ GPU Temperature (°C)
- ✅ GPU Name/Model

**Requirements**:
```bash
# i915 driver (usually pre-installed on Linux)
ls /sys/class/drm/card*/device/vendor  # Should show 0x8086

# Intel graphics drivers
# Usually pre-installed, or install: intel-media-driver
```

**Monitoring Paths**:
- Frequency: `/sys/class/drm/card0/gt/gt0/rps_cur_freq_mhz`
- Temperature: `/sys/class/drm/card0/device/hwmon/hwmon*/temp*_input`

## How It Works

### Automatic Detection Process

1. **Startup**: Application scans for available GPUs
2. **NVIDIA Check**: Attempts to initialize NVML
3. **AMD Check**: If NVIDIA fails, checks for AMD GPU in sysfs
4. **Intel Check**: If AMD fails, checks for Intel GPU in sysfs
5. **Fallback**: If all fail, uses basic monitoring (no GPU stats)

```
┌─────────────────────────────────────┐
│      GPU Detection on Startup       │
└─────────────────────────────────────┘
                  │
                  ▼
         ┌────────────────┐
         │ Try NVIDIA GPU │
         └────────────────┘
                  │
        ┌─────────┴─────────┐
        │ Success?          │
        └─────────┬─────────┘
         Yes │    │ No
             │    │
             │    ▼
             │ ┌────────────────┐
             │ │  Try AMD GPU   │
             │ └────────────────┘
             │         │
             │    ┌────┴────┐
             │    │Success? │
             │    └────┬────┘
             │    Yes │ │ No
             │        │ │
             │        │ ▼
             │        │ ┌──────────────┐
             │        │ │Try Intel GPU │
             │        │ └──────────────┘
             │        │         │
             │        │    ┌────┴────┐
             │        │    │Success? │
             │        │    └────┬────┘
             │        │    Yes │ │ No
             │        │        │ │
             │        │        │ ▼
             │        │        │ ┌──────────┐
             │        │        │ │ Fallback │
             │        │        │ └──────────┘
             │        │        │
             ▼        ▼        ▼
        ┌──────────────────────────┐
        │   Use Detected Monitor   │
        └──────────────────────────┘
```

### Logging Output

Check the logs to see which GPU was detected:

```bash
# Run with debug logging
RUST_LOG=debug ./ollama-tui

# Example output for NVIDIA:
# [INFO] Detecting GPU hardware...
# [INFO] NVIDIA GPU detected
# [DEBUG] Got NVIDIA GPU stats

# Example output for AMD:
# [INFO] Detecting GPU hardware...
# [INFO] AMD GPU detected
# [DEBUG] Found AMD GPU at: /sys/class/drm/card0
# [DEBUG] Got AMD GPU stats

# Example output for Intel:
# [INFO] Detecting GPU hardware...
# [INFO] Intel GPU detected
# [DEBUG] Found Intel GPU at: /sys/class/drm/card0
# [DEBUG] Got Intel GPU stats
```

## Platform Support

### Linux (Full Support)

All GPU types supported via:
- NVML for NVIDIA
- sysfs for AMD and Intel
- hwmon for temperature readings

**Recommended distributions**:
- Ubuntu 22.04+ (best driver support)
- Fedora 38+ (latest drivers)
- Arch Linux (rolling release)
- Pop!_OS (NVIDIA optimized)

### macOS (Limited)

Currently no GPU monitoring on macOS. Future support planned via:
- Metal Performance Shaders (MPS)
- IOKit framework

### Windows (Limited)

Currently limited support. Future plans:
- NVML for NVIDIA (already works)
- DirectX for AMD/Intel monitoring

## Troubleshooting

### No GPU Stats Showing

**Check 1: Verify GPU is detected by system**
```bash
# For NVIDIA:
nvidia-smi

# For AMD:
ls /sys/class/drm/card*/device/vendor | xargs cat
# Should show: 0x1002

# For Intel:
ls /sys/class/drm/card*/device/vendor | xargs cat
# Should show: 0x8086

# General:
lspci | grep -i vga
```

**Check 2: Verify driver installation**
```bash
# NVIDIA:
lsmod | grep nvidia

# AMD:
lsmod | grep amdgpu

# Intel:
lsmod | grep i915
```

**Check 3: Check permissions**
```bash
# sysfs paths should be readable
ls -la /sys/class/drm/card*/device/

# Add user to video group if needed
sudo usermod -a -G video $USER
# Log out and back in
```

### NVIDIA GPU Not Detected

```bash
# 1. Install NVIDIA drivers
sudo ubuntu-drivers autoinstall

# 2. Install CUDA toolkit
wget https://developer.download.nvidia.com/compute/cuda/repos/ubuntu2204/x86_64/cuda-keyring_1.0-1_all.deb
sudo dpkg -i cuda-keyring_1.0-1_all.deb
sudo apt update
sudo apt install cuda

# 3. Reboot
sudo reboot

# 4. Verify
nvidia-smi
```

### AMD GPU Not Detected

```bash
# 1. Verify AMDGPU driver loaded
lsmod | grep amdgpu

# 2. Check sysfs availability
ls /sys/class/drm/card*/device/gpu_busy_percent

# 3. Install ROCm (optional, for better support)
sudo apt install rocm-smi

# 4. Verify
rocm-smi
```

### Intel GPU Not Detected

```bash
# 1. Verify i915 driver loaded
lsmod | grep i915

# 2. Check sysfs availability
ls /sys/class/drm/card*/gt/gt0/

# 3. Install Intel drivers (if needed)
sudo apt install intel-media-driver

# 4. For Arc GPUs, ensure recent kernel (6.0+)
uname -r
```

### Temperature Not Showing

```bash
# Check hwmon availability
ls /sys/class/drm/card*/device/hwmon/hwmon*/temp*_input

# Install lm-sensors (may help)
sudo apt install lm-sensors
sudo sensors-detect

# For NVIDIA:
nvidia-smi -q -d TEMPERATURE
```

## Performance Considerations

### Update Frequency

Default: 1000ms (1 second)

GPU stats are polled every second to balance:
- Real-time feedback
- Low CPU overhead
- Responsive UI

Configure in `~/.config/ollama-tui/config.toml`:
```toml
update_interval_ms = 1000  # Adjust as needed
```

### CPU Usage

- **NVIDIA**: ~0.1% CPU (NVML is very efficient)
- **AMD**: ~0.5% CPU (sysfs reads are fast)
- **Intel**: ~0.5% CPU (sysfs reads are fast)

### Memory Usage

GPU monitoring adds minimal memory overhead:
- ~1-2 MB for monitoring structures
- No memory leaks (Rust safety guarantees)

## Advanced Usage

### Multiple GPUs

Currently, the app monitors the **first detected GPU** only. If you have multiple GPUs:

- NVIDIA: Monitors GPU 0 (change in `nvidia.rs`)
- AMD: Monitors first card found in `/sys/class/drm/`
- Intel: Monitors first card found in `/sys/class/drm/`

Future enhancement: Multi-GPU support with selection.

### Custom GPU Monitoring

For developers who want to add custom GPU monitoring:

1. Implement the monitoring logic in a new file (e.g., `src/gpu/custom.rs`)
2. Add detection logic to `src/gpu/mod.rs`
3. Return `GpuStats` structure with your data

Example structure:
```rust
pub struct GpuStats {
    pub name: String,        // GPU model name
    pub utilization: u32,    // 0-100%
    pub memory_used: u64,    // Bytes
    pub memory_total: u64,   // Bytes
    pub temperature: u32,    // Celsius
}
```

## Contributing

Have a GPU that's not supported? We'd love to add it!

1. Check which vendor ID your GPU uses:
   ```bash
   lspci -nn | grep VGA
   ```

2. Share the output in a GitHub issue

3. We'll work on adding support!

**Most wanted**:
- Apple Silicon (M1/M2/M3) Metal support
- Windows DirectX monitoring
- Older GPU architectures

## References

- [NVIDIA NVML Documentation](https://developer.nvidia.com/nvidia-management-library-nvml)
- [AMD ROCm Documentation](https://rocm.docs.amd.com/)
- [Linux DRM Subsystem](https://www.kernel.org/doc/html/latest/gpu/index.html)
- [Intel i915 Driver](https://www.kernel.org/doc/html/latest/gpu/i915.html)

## License

This GPU monitoring functionality is part of Ollama TUI and is licensed under GPLv3.