# Changelog - New Features

## Version 0.2.0 - Multi-GPU Support & Model Pulling

### 🎉 Major New Features

#### 1. Interactive Model Pulling (Press 'p')
- **Pull models directly from the TUI** - no need for command line!
- Interactive dialog with:
  - Model name input field
  - Live examples (llama3.2, mistral:7b, etc.)
  - Error handling and validation
  - Status updates in log
- Automatic model list refresh after successful pull
- Long timeout support (30 minutes) for large models

#### 2. Automatic Multi-GPU Detection
- **Works with NVIDIA, AMD, and Intel GPUs automatically!**
- Intelligent detection on startup:
  1. Try NVIDIA GPU (NVML) - best support
  2. Try AMD GPU (sysfs/hwmon) - good support
  3. Try Intel GPU (sysfs/i915) - basic support
  4. Fallback gracefully if no GPU found

### 🔧 GPU Support Details

#### NVIDIA GPUs (Full Support)
- Uses NVML (NVIDIA Management Library)
- Monitors: Utilization, VRAM, Temperature
- Supports: RTX 40/30/20 series, GTX 16/10 series, Tesla, Quadro

#### AMD GPUs (NEW! Good Support)
- Uses sysfs and hwmon
- Monitors: GPU busy %, VRAM usage, Temperature
- Supports: RX 7000/6000/5000 series, Vega, Polaris
- Paths: /sys/class/drm/cardX/device/

#### Intel GPUs (NEW! Basic Support)
- Uses sysfs and i915 driver
- Monitors: Frequency-based utilization, Memory, Temperature
- Supports: Arc A-series, Iris Xe, UHD Graphics
- Paths: /sys/class/drm/cardX/gt/

### 📝 Updated Documentation

New files:
- `GPU_SUPPORT.md` - Comprehensive GPU documentation
- Pull dialog examples in QUICKSTART.md
- Multi-GPU info in README.md

### 🔄 Repository Updates

- All GitHub URLs updated to: `github.com/Ninso112/omarchy-ollama-tui`
- Repository name reflects new features

### 🎨 UI Improvements

- New pull dialog with centered popup
- Model examples and help text
- Color-coded input states
- Loading indicator during pull
- Error messages in dialog

### ⌨️ New Keybindings

- `p` - Open pull model dialog (was scaffolded, now fully implemented!)

### 🐛 Bug Fixes

- Fixed duplicate AMD GPU device IDs
- Improved error handling for GPU detection
- Better timeout handling for long operations

### 📊 Technical Details

**New Files Added:**
- `src/ui/pull_dialog.rs` (215 lines)
- `src/gpu/amd.rs` (230 lines)
- `src/gpu/intel.rs` (280 lines)
- `GPU_SUPPORT.md` (400 lines)

**Modified Files:**
- `src/app.rs` - Pull dialog state and handlers
- `src/gpu/mod.rs` - Auto-detection logic
- `src/ollama/client.rs` - Improved pull with timeout
- All documentation files

**Total New Code:** ~1,100+ lines
**Build Status:** ✅ All tests passing, clippy clean

### 🚀 How to Use

1. **Pull a model:**
   ```bash
   ./ollama-tui
   # Press 'p', type "llama3.2", press Enter
   ```

2. **Automatic GPU detection:**
   ```bash
   # Just run - it auto-detects your GPU!
   ./ollama-tui
   
   # Check logs to see which GPU was detected
   RUST_LOG=debug ./ollama-tui
   ```

### 🎯 Future Plans

- Streaming progress for model pulls
- Multi-GPU support (select which GPU to monitor)
- macOS Metal GPU support
- Windows DirectX GPU monitoring

---

**Full Changelog**: All changes committed and ready for v0.2.0 release!
