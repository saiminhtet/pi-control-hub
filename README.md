# 🎯 PiControl Hub

**Ultimate All-in-One Control Center for Raspberry Pi 4B + 3.5" Touchscreen**

A modular, touch-optimized control hub built with Rust and egui, designed specifically for Raspberry Pi with a 3.5" touchscreen. PiControl Hub provides a unified interface for system monitoring, media control, home automation, and more.

![Rust](https://img.shields.io/badge/rust-1.86%2B-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Platform](https://img.shields.io/badge/platform-Raspberry%20Pi-red.svg)

## ✨ Features

### 🎨 Touch-Optimized Design
- **Custom Design System**: Purpose-built UI components optimized for 3.5" touchscreen
- **Dark Theme**: High-contrast color scheme optimized for screen visibility
- **Touch-Friendly Controls**: All buttons and interactive elements meet 44px minimum touch target size
- **Responsive Layout**: Fixed 480x320 resolution perfectly suited for compact displays

### 🔧 Implemented Modules

#### 📊 Dashboard
- System overview with quick-access widgets
- Real-time status indicators (CPU, memory, temperature)
- Quick action buttons for common tasks
- Modular widget layout

#### 🖥️ System Monitor
- Real-time CPU usage monitoring
- Memory usage tracking with detailed statistics
- Temperature monitoring (when sensors available)
- Touch-friendly progress bars and visualizations
- Color-coded performance indicators:
  - 🟢 Green: Normal (< 60%)
  - 🟡 Yellow: Warning (60-80%)
  - 🔴 Red: Critical (> 80%)

#### 🎵 Media Center
- Media playback controls
- Touch-optimized interface for audio/video management
- Integration-ready for various media sources

### 🏗️ Architecture

```
PiControl Hub
├── src/
│   ├── main.rs              # Application entry point
│   ├── app/
│   │   ├── mod.rs           # Main app logic & UI orchestration
│   │   └── touch_design.rs  # Touch-optimized design system
│   ├── modules/
│   │   ├── mod.rs           # Module trait & type definitions
│   │   ├── system_monitor.rs  # System monitoring module
│   │   └── media_center.rs    # Media control module
│   └── hardware/
│       └── gpio_manager.rs  # GPIO hardware abstraction (planned)
└── Cargo.toml
```

## 🚀 Getting Started

### Prerequisites

- **Raspberry Pi 4B** (or compatible model)
- **3.5" Touchscreen Display** (480x320 resolution)
- **Rust 1.86 or higher**
- **Raspbian/Raspberry Pi OS** (or compatible Linux distribution)

### Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/yourusername/pi-control-hub.git
   cd pi-control-hub
   ```

2. **Install Rust (if not already installed):**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

3. **Build the project:**
   ```bash
   cargo build --release
   ```

4. **Run the application:**
   ```bash
   cargo run --release
   ```

### Running on Raspberry Pi

For optimal performance on Raspberry Pi:

```bash
# Build with optimizations
cargo build --release

# Run the binary
./target/release/pi-control-hub
```

To run automatically on boot, create a systemd service or add to autostart.

## 🛠️ Technical Stack

### Core Technologies
- **Rust** - Systems programming language for performance and safety
- **egui** (v0.24) - Immediate mode GUI framework
- **eframe** (v0.24) - Application framework for egui
- **sysinfo** (v0.29) - System information gathering
- **rppal** (v0.14) - Raspberry Pi hardware abstraction (GPIO, I2C, SPI, PWM)

### Additional Dependencies
- **tokio** - Asynchronous runtime for concurrent operations
- **serde** & **serde_json** - Serialization/deserialization
- **chrono** - Date and time handling
- **anyhow** - Error handling

## 📐 Design System

### Color Palette (Dark Theme)
```rust
Primary:    #3B82F6  // Blue-500
Secondary:  #64748B  // Slate-500
Success:    #22C55E  // Green-500
Warning:    #EAB308  // Yellow-500
Error:      #EF4444  // Red-500
Background: #0F172A  // Slate-900
Surface:    #1E293B  // Slate-800
```

### Typography
- **Heading**: 18pt (larger for touch screens)
- **Body**: 14pt
- **Small**: 12pt
- **Button**: 14pt (clear and readable)

### Spacing
- **Item Spacing**: 8x8pt
- **Window Margin**: 12pt
- **Button Padding**: 16x12pt
- **Minimum Touch Target**: 44x44pt

## 🎯 Roadmap

### ✅ Completed
- [x] Touch-optimized design system
- [x] System monitor with CPU, memory, and temperature
- [x] Dashboard with quick actions
- [x] Navigation panel with module switching
- [x] Real-time system status indicators

### 🚧 In Progress
- [ ] Media center controls
- [ ] Hardware GPIO integration

### 📅 Planned Modules

#### 🏠 Home Automation
- Smart device control (lights, plugs, thermostat)
- Scene presets (Morning, Night, Away)
- Energy monitoring
- Automation rules engine

#### 🌡️ Weather Station
- Local weather forecasts
- Environmental sensor integration
- Weather alerts
- Historical data visualization

#### 🎮 Gaming & Entertainment
- Retro game launcher
- Touch-based mini-games
- Gamepad configuration
- Save state manager

#### 🔒 Security & Camera
- Multi-camera live view
- Motion detection alerts
- Recording scheduler
- Timeline viewer

#### 🌱 Smart Garden Manager
- Plant care database
- Automated watering schedules
- Soil monitoring
- Growth tracking

#### 🖼️ Digital Frame Plus
- Photo slideshow
- Social media integration
- Calendar overlay
- Remote upload

#### ⚙️ Settings & Configuration
- Theme customization
- Module management
- System configuration
- Backup & restore

## 🔌 Hardware Integration

### Supported Peripherals
- **GPIO Devices** - Direct control via rppal
- **I2C Sensors** - Temperature, humidity, pressure
- **SPI Devices** - High-speed sensor communication
- **PWM Controllers** - LED dimming, servo control
- **USB Cameras** - Video capture and streaming (planned)
- **External Storage** - Media and data storage

### GPIO Pin Configuration
```rust
// Example GPIO setup (from hardware/gpio_manager.rs)
// Pin assignments for common peripherals
// Documentation coming soon
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

### Development Guidelines
- Follow Rust best practices and idioms
- Maintain touch-friendly UI principles (44px minimum targets)
- Write clear, documented code
- Test on actual Raspberry Pi hardware when possible
- Keep modules independent and reusable

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🐛 Known Issues

- Warnings for unused variables and dead code (planned features)
- Some modules are stubs awaiting implementation (home automation, settings)
- Hardware abstraction layer is under development

## 🙏 Acknowledgments

- **egui** - Excellent immediate mode GUI framework
- **rppal** - Comprehensive Raspberry Pi peripheral access
- **sysinfo** - Cross-platform system information
- **Rust Community** - For the amazing ecosystem

## 📧 Contact

For questions, suggestions, or bug reports, please open an issue on GitHub.

## 🖼️ Screenshots

*Screenshots coming soon - application is in active development*

---

**Built with ❤️ in Rust for Raspberry Pi**
