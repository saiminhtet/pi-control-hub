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

### 1. Responsive Multi-Screen Support (Session 2)

**User Request:** Implement responsive UI for multiple screen sizes (Pi touchscreen, Windows, macOS)

**Implementation:**

**New File: `src/app/responsive.rs`**
- `ScreenSize` enum with 5 breakpoints (Tiny to XLarge)
- `LayoutConfig` struct with screen-specific layout parameters
- `ResponsiveState` managing current screen size and layout
- Automatic breakpoint detection from window width
- Layout configuration caching per screen size

**Modified: `src/app/mod.rs`**
- Added `ResponsiveState` to app struct
- Implemented dual layout system:
  - `mobile_layout()` - Bottom navigation for Tiny/Small screens
  - `desktop_layout()` - Side navigation for Medium+ screens
- `bottom_navigation()` - Horizontal scrollable icon navigation
- `side_navigation()` - Vertical scrollable text navigation
- `apply_responsive_styles()` - Dynamic styling based on breakpoint
- Module icon mapping for compact mobile view

**Modified: `src/main.rs`**
- Changed window from fixed to resizable
- Added minimum window size (320x240)
- Removed fixed size constraint

**Modified: `src/app/touch_design.rs`**
- Added `apply_responsive_fonts()` method
- `to_responsive_text_styles()` for font scaling
- Font scaling based on screen size (1.5x tiny → 0.9x xlarge)

**Files Modified:**
- `src/app/responsive.rs` (new)
- `src/app/mod.rs:1-330`
- `src/main.rs:11-20`
- `src/app/touch_design.rs:34-37, 141-156`

**Benefits:**
- Seamless experience across all screen sizes
- Optimal layout for each device category
- Touch-optimized on small screens
- Desktop-optimized on larger displays
- Automatic adaptation on window resize
- Font scaling for readability at all sizes

**Layout Specifications:**

| Screen Size | Width Range | Nav Type | Font Scale | Touch Size | Header Height |
|------------|-------------|----------|------------|------------|---------------|
| Tiny       | < 480px     | Bottom   | 1.5x       | 44px       | 40px          |
| Small      | 480-768px   | Bottom   | 1.2x       | 40px       | 48px          |
| Medium     | 768-1024px  | Side     | 1.0x       | 36px       | 50px          |
| Large      | 1024-1440px | Side     | 0.95x      | 32px       | 52px          |
| XLarge     | > 1440px    | Side     | 0.9x       | 32px       | 56px          |

### 2. Scrollable Navigation Panel (Session 1)

**User Request:** Enable scrollable navigation panel for multiple module components

**Implementation:**
- Added `egui::ScrollArea::vertical()` wrapper around navigation buttons
- Configured for touch optimization:
  - `drag_to_scroll(true)` - Touch drag/swipe scrolling
  - `AlwaysVisible` scroll bar - Visual feedback
  - `auto_shrink([false; 2])` - Prevents layout issues

**Files Modified:** `src/app/mod.rs:69-104` (now superseded by responsive layouts)

**Benefits:**
- Handles unlimited module count
- Touch-friendly scrolling interface
- Visual scroll indicator always visible
- Maintains 44px minimum touch targets

### 3. Comprehensive Documentation

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
   - Responsive layouts with minimal overhead
   - VSync enabled for smooth rendering
   - System info updates throttled to once per second (frame_count % 60)
   - Resizable window with efficient layout recalculation

4. **Responsive Design Strategy**
   - **Breakpoints:** Tiny (<480px), Small (480-768px), Medium (768-1024px), Large (1024-1440px), XLarge (>1440px)
   - **Mobile Layout** (Tiny/Small): Bottom navigation with icons, maximized content area
   - **Desktop Layout** (Medium+): Side navigation with text labels, traditional desktop UX
   - **Adaptive Sizing:** Font scale, margins, touch targets, and button sizes adjust per breakpoint
   - **Real-time Adaptation:** Layout updates automatically on window resize

### File Structure
```
src/
├── main.rs                  # Entry point, window configuration (resizable)
├── app/
│   ├── mod.rs              # Main application logic, responsive UI orchestration
│   ├── touch_design.rs     # Design system (colors, spacing, typography)
│   └── responsive.rs       # Responsive breakpoints and layout configs
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
// UI Colors
Primary:    #3B82F6  // Blue-500 (active states)
Secondary:  #64748B  // Slate-500 (unused currently)
Background: #0F172A  // Slate-900 (main bg)
Surface:    #1E293B  // Slate-800 (panels, inactive buttons)

// Status Colors (Progress Bars) - Darker shades for better text contrast
Success:    #16A34A  // Green-600 (< 60% usage)
Warning:    #CA8A04  // Yellow-600 (60-80% usage) - improved contrast
Error:      #DC2626  // Red-600 (> 80% usage)
```

### Performance Thresholds
```rust
// CPU/Memory usage colors
Normal (Green-600):   < 60%  - #16A34A
Warning (Yellow-600): 60-80% - #CA8A04 (darker for better contrast)
Critical (Red-600):   > 80%  - #DC2626

// Temperature colors
Normal (Green-600):   < 60°C - #16A34A
Warning (Yellow-600): 60-70°C - #CA8A04
Critical (Red-600):   > 70°C - #DC2626
```

**Note:** Status colors use 600-level shades (darker) instead of 500-level to ensure proper contrast with white text on progress bars and labels.

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

### v0.2.1 (Current)
- **Contrast Improvements**
  - Fixed poor contrast on yellow progress bars (memory usage)
  - Changed status colors from 500-level to 600-level shades
  - Yellow warning color: #EAB308 → #CA8A04 (darker, better contrast)
  - Green success color: #22C55E → #16A34A (darker, better contrast)
  - Red error color: #EF4444 → #DC2626 (darker, better contrast)
  - Improved readability of white text on all status indicators

### v0.2.0
- **Responsive Design System Implementation**
  - Added multi-screen support (Tiny/Small/Medium/Large/XLarge breakpoints)
  - Mobile-first layout with bottom navigation for screens < 768px
  - Desktop layout with side navigation for screens >= 768px
  - Responsive font scaling based on screen size
  - Dynamic layout adaptation on window resize
  - Resizable window with minimum size constraints
- **Enhanced Navigation**
  - Bottom navigation bar for mobile/Pi screens (icon-based)
  - Side navigation panel for desktop screens (text-based)
  - Scrollable navigation areas for both layouts
- **Adaptive Styling**
  - Responsive margins and spacing
  - Font scaling (1.5x for Tiny, down to 0.9x for XLarge)
  - Touch target sizes adapt to screen size

### v0.1.0
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

## 🎯 Responsive Design Implementation Notes

### Breakpoint Selection Rationale
- **Tiny (< 480px):** Raspberry Pi 3.5" touchscreen (480x320) and similar embedded displays
- **Small (480-768px):** Tablets and larger embedded displays
- **Medium (768-1024px):** Small laptops and desktop windows
- **Large (1024-1440px):** Standard desktop displays
- **XLarge (> 1440px):** Large monitors and ultra-wide displays

### Layout Switching Logic
The app automatically detects screen size on every frame update and switches between:
1. **Mobile Mode** (Tiny/Small): Icon-based bottom navigation, no side panel, maximized content
2. **Desktop Mode** (Medium+): Text-based side navigation, traditional desktop layout

### Testing Responsive Layouts
To test different layouts:
1. **Pi Screen:** Run normally on Raspberry Pi (480x320 → Tiny)
2. **Tablet Mode:** Resize window to 600px wide (Small)
3. **Desktop Mode:** Resize window to 1024px+ (Medium/Large)
4. **Large Monitor:** Maximize on 1440px+ display (XLarge)

### Future Enhancements
- [ ] Add debug overlay showing current breakpoint
- [ ] Implement window maximize to overlay taskbar (platform-specific)
- [ ] Add user preference for forcing mobile/desktop layout
- [ ] Implement responsive grid layouts for dashboard widgets
- [ ] Add orientation detection for tablets (portrait/landscape)

---

**Last Updated:** 2025-11-21
**Rust Version:** 1.86+
**egui Version:** 0.24
**Platform:** Multi-platform (Raspberry Pi, Windows, macOS)
**Primary Target:** Raspberry Pi 4B + 3.5" Touchscreen (480x320)
