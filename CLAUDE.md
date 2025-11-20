# CLAUDE.md - Project Memory & Change History

This file contains important context, decisions, and change history for the PiControl Hub project. It helps Claude (and future maintainers) understand the project's evolution and key decisions.

## 📋 Project Overview

**Project Name:** PiControl Hub
**Target Platform:** Raspberry Pi 4B with 3.5" Touchscreen (480x320)
**Language:** Rust
**UI Framework:** egui/eframe v0.24
**Purpose:** All-in-one touch-optimized control center for Raspberry Pi

## 🔧 Initial Setup & Bug Fixes (Session 1)

### Problems Encountered

1. **rppal Dependency Issue**
   - **Problem:** Cargo.toml specified non-existent features for rppal v0.14
   - **Error:** `rppal does not have these features: i2c, spi, pwm, serial`
   - **Solution:** Removed invalid features. These are built-in modules, not cargo features.
   - **Fixed in:** `Cargo.toml:9`
   ```toml
   # Before
   rppal = { version = "0.14", features = ["hal", "i2c", "spi", "pwm", "serial"] }
   # After
   rppal = { version = "0.14", features = ["hal"] }
   ```

2. **egui API Changes (v0.24)**
   - **Problem:** Text color API changed from direct field access to methods
   - **Error:** `visuals.weak_text_color = ...; method, not a field`
   - **Solution:** Updated to use `override_text_color` and `warn_fg_color`
   - **Fixed in:** `src/app/touch_design.rs:77-79`
   ```rust
   // Before
   visuals.text_color = self.text_primary;
   visuals.weak_text_color = self.text_secondary;
   // After
   visuals.override_text_color = Some(self.text_primary);
   visuals.warn_fg_color = self.text_secondary;
   ```

3. **egui style_mut() API Change**
   - **Problem:** `style_mut()` now takes a closure instead of returning mutable reference
   - **Error:** `this method takes 1 argument but 0 arguments were supplied`
   - **Solution:** Wrapped style application in closure
   - **Fixed in:** `src/app/mod.rs:26-28`
   ```rust
   // Before
   design.apply_to_style(&mut cc.egui_ctx.style_mut());
   // After
   cc.egui_ctx.style_mut(|style| {
       design.apply_to_style(style);
   });
   ```

4. **sysinfo API Changes (v0.29)**
   - **Problem:** API methods renamed and trait imports required
   - **Error:** `no method named 'global_cpu_usage'`
   - **Solution:** Added trait imports and updated method calls
   - **Fixed in:** `src/modules/system_monitor.rs:1,37`
   ```rust
   // Added imports
   use sysinfo::{System, SystemExt, ComponentExt, CpuExt};

   // Before
   let cpu_usage = self.system.global_cpu_usage();
   // After
   let cpu_usage = self.system.global_cpu_info().cpu_usage();
   ```

5. **ProgressBar API Change**
   - **Problem:** `min_size()` method removed from ProgressBar in egui v0.24
   - **Error:** `no method named 'min_size' found for struct 'ProgressBar'`
   - **Solution:** Removed `min_size()` calls (progress bars size automatically)
   - **Fixed in:** `src/modules/system_monitor.rs:42,62`

6. **NativeOptions Structure Change**
   - **Problem:** Window configuration moved to ViewportBuilder
   - **Error:** `struct 'NativeOptions' has no field named 'initial_window_size'`
   - **Solution:** Migrated to ViewportBuilder API
   - **Fixed in:** `src/main.rs:11-15`
   ```rust
   // Before
   initial_window_size: Some(egui::vec2(480.0, 320.0)),
   resizable: false,
   // After
   viewport: egui::ViewportBuilder::default()
       .with_inner_size([480.0, 320.0])
       .with_resizable(false)
       .with_decorations(true),
   ```

7. **Missing Module Declarations**
   - **Problem:** Module files referenced but not created
   - **Error:** `file not found for module 'home_automation'`, etc.
   - **Solution:** Commented out unimplemented modules with TODO markers
   - **Fixed in:** `src/modules/mod.rs:3-5`, `src/main.rs:6-8`

8. **Missing Imports**
   - **Problem:** Color32 and CpuExt not imported where used
   - **Solution:** Added necessary imports
   - **Fixed in:** `src/app/mod.rs:4`, `src/modules/system_monitor.rs:1`

### Compilation Result
✅ **All errors fixed** - Project compiles successfully with only minor unused code warnings

## 🎨 Feature Additions

### 1. Scrollable Navigation Panel (Session 1)

**User Request:** Enable scrollable navigation panel for multiple module components

**Implementation:**
- Added `egui::ScrollArea::vertical()` wrapper around navigation buttons
- Configured for touch optimization:
  - `drag_to_scroll(true)` - Touch drag/swipe scrolling
  - `AlwaysVisible` scroll bar - Visual feedback
  - `auto_shrink([false; 2])` - Prevents layout issues

**Files Modified:** `src/app/mod.rs:69-104`

**Benefits:**
- Handles unlimited module count
- Touch-friendly scrolling interface
- Visual scroll indicator always visible
- Maintains 44px minimum touch targets

### 2. Comprehensive Documentation

**Created Files:**
1. **README.md** - Full project documentation including:
   - Feature overview
   - Installation instructions
   - Architecture diagram
   - Design system specs
   - Roadmap with completed/planned features
   - Hardware integration details
   - Contributing guidelines

2. **CLAUDE.md** (this file) - Project memory and change history

## 🏗️ Architecture Decisions

### Design Philosophy
1. **Touch-First Design**
   - Minimum 44x44pt touch targets for all interactive elements
   - Large, clear typography (14-18pt)
   - High-contrast dark theme optimized for small displays
   - Generous spacing between elements

2. **Modular Architecture**
   - Plugin-based module system using trait objects
   - Each module implements `Module` trait
   - Modules are independent and can be added/removed easily
   - Module state stored in HashMap for dynamic loading

3. **Performance Considerations**
   - Fixed 480x320 resolution (no dynamic scaling overhead)
   - VSync enabled for smooth rendering
   - System info updates throttled to once per second (frame_count % 60)
   - Non-resizable window for consistent performance

### File Structure
```
src/
├── main.rs                  # Entry point, window configuration
├── app/
│   ├── mod.rs              # Main application logic, UI orchestration
│   └── touch_design.rs     # Design system (colors, spacing, typography)
├── modules/
│   ├── mod.rs              # Module trait and type definitions
│   ├── system_monitor.rs   # CPU, memory, temperature monitoring
│   └── media_center.rs     # Media controls (stub)
└── hardware/
    └── gpio_manager.rs     # GPIO abstraction (stub)
```

## 🎯 Current Module Status

### ✅ Implemented
- **Dashboard** - Overview with quick actions and status widgets
- **System Monitor** - Real-time CPU, memory, temperature monitoring

### 🚧 Stub/Planned
- **Media Center** - Media playback controls
- **Home Automation** - Smart device control (commented out)
- **Gaming** - Retro game launcher
- **Security** - Camera and monitoring
- **Settings** - Configuration interface (commented out)

## 🔑 Key Technical Details

### Dependencies
```toml
eframe = "0.24"        # Application framework
egui = "0.24"          # Immediate mode GUI
rppal = "0.14"         # Raspberry Pi GPIO/hardware
sysinfo = "0.29"       # System information
tokio = "1.0"          # Async runtime
chrono = "0.4"         # Date/time handling
serde/serde_json       # Serialization
anyhow = "1.0"         # Error handling
```

### Design System Colors
```rust
Primary:    #3B82F6  // Blue-500 (active states)
Secondary:  #64748B  // Slate-500 (unused currently)
Success:    #22C55E  // Green-500 (< 60% usage)
Warning:    #EAB308  // Yellow-500 (60-80% usage)
Error:      #EF4444  // Red-500 (> 80% usage)
Background: #0F172A  // Slate-900 (main bg)
Surface:    #1E293B  // Slate-800 (panels, inactive buttons)
```

### Performance Thresholds
```rust
// CPU/Memory usage colors
Normal (Green):   < 60%
Warning (Yellow): 60-80%
Critical (Red):   > 80%

// Temperature colors
Normal (Green):   < 60°C
Warning (Yellow): 60-70°C
Critical (Red):   > 70°C
```

## 🐛 Known Issues & Warnings

### Compilation Warnings (Non-Critical)
1. Unused variable `ctx` in `system_monitor.rs:29`
   - Not an issue; reserved for future use

2. Unused method `icon_button` in `app/mod.rs:63`
   - Planned for future icon-based navigation

3. Unused fields in design system structs
   - Reserved for future features (secondary color, touch_target_size, etc.)

4. Unused trait method `name()` in Module trait
   - Reserved for module metadata display

### Planned Fixes
- Remove or prefix unused variables with underscore
- Implement icon button functionality
- Utilize all design system fields
- Add module name display in UI

## 📝 Important Notes for Future Development

### When Adding New Modules

1. **Create module file** in `src/modules/`
2. **Implement Module trait:**
   ```rust
   impl Module for YourModule {
       fn name(&self) -> &'static str { "Module Name" }
       fn show(&mut self, ctx: &Context, ui: &mut egui::Ui) { /* UI code */ }
   }
   ```
3. **Add module type** to `ModuleType` enum in `src/modules/mod.rs`
4. **Register in iterator** and `name()` match statement
5. **Initialize in app** `PiControlApp::new()`
6. **Add match arm** in `app/mod.rs` update() function

### Touch Optimization Guidelines
- **Always** use `min_size()` with at least 44x44pt for buttons
- Use `self.design.touch_targets.min_button_size` for consistency
- Add generous spacing (`ui.add_space()`) between elements
- Test scrollable areas with `drag_to_scroll(true)`
- Consider finger-friendly hit areas (bigger is better)

### Hardware Integration
- rppal provides GPIO, I2C, SPI, PWM, UART access
- No separate features needed for peripheral types
- Use `hal` feature for embedded-hal trait implementations
- Test hardware code on actual Raspberry Pi (not in WSL/VM)

## 🔄 Version History

### v0.1.0 (Current)
- Initial project setup
- Fixed all egui v0.24 compatibility issues
- Implemented touch-optimized design system
- Added System Monitor module
- Created scrollable navigation panel
- Generated comprehensive documentation

## 💡 Future Roadmap Ideas

### High Priority
1. Implement remaining core modules (Media Center, Settings)
2. Add actual hardware GPIO integration
3. Create configuration file system
4. Add module enable/disable functionality

### Medium Priority
1. Theme switcher (light/dark modes)
2. Custom widget layouts (drag & drop)
3. Data persistence and history
4. Network status monitoring
5. Battery indicator (if on UPS)

### Low Priority
1. Module marketplace/plugin system
2. Remote web interface
3. Mobile companion app
4. Voice control integration
5. Advanced automation rules

## 🤝 Contributing Notes

When contributing to this project:
1. Follow Rust best practices and idioms
2. Maintain touch-first design principles
3. Test on actual Raspberry Pi hardware when possible
4. Update this CLAUDE.md file with significant changes
5. Keep modules independent and loosely coupled
6. Document breaking API changes
7. Add user-facing changes to README.md

## 📞 Questions to Ask User

If implementing new features, consider asking:
- [ ] Should this module be always loaded or optional?
- [ ] What's the preferred data storage format?
- [ ] Are there specific GPIO pins assigned for this?
- [ ] Should this feature work offline?
- [ ] What's the update frequency needed?
- [ ] Are there any safety concerns (hardware damage)?

## 🎓 Lessons Learned

1. **Always check crate feature flags** - Many crates (like rppal) don't use features for built-in functionality
2. **egui updates can break APIs** - Always check docs when upgrading versions
3. **Trait imports matter** - Rust extension traits require explicit imports
4. **Touch UIs need different metrics** - Desktop UI guidelines don't apply to touchscreens
5. **Module system is powerful** - Trait objects enable flexible plugin architecture

---

**Last Updated:** 2025-11-20
**Rust Version:** 1.86+
**egui Version:** 0.24
**Platform:** Raspberry Pi 4B + 3.5" Touchscreen (480x320)
