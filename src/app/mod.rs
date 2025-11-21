mod touch_design;
mod responsive;

pub use touch_design::TouchDesignSystem;
pub use responsive::ResponsiveState;

use egui::{Context, Color32};
use crate::modules::{Module, ModuleType, system_monitor::SystemMonitor};

pub struct PiControlApp {
    pub current_module: ModuleType,
    pub modules: std::collections::HashMap<ModuleType, Box<dyn Module>>,
    pub design: TouchDesignSystem,
    pub system_info: SystemInfo,
    pub frame_count: u64,
    pub responsive: ResponsiveState,
}

#[derive(Default)]
pub struct SystemInfo {
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub temperature: f32,
}

impl PiControlApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Apply touch-optimized design system
        let design = TouchDesignSystem::new();
        cc.egui_ctx.style_mut(|style| {
            design.apply_to_style(style);
        });

        // Initialize modules
        let mut modules = std::collections::HashMap::new();
        modules.insert(ModuleType::SystemMonitor, Box::new(SystemMonitor::new()) as Box<dyn Module>);

        // Initialize responsive state with default window size
        let initial_size = cc.egui_ctx.screen_rect().size();
        let responsive = ResponsiveState::new(initial_size);

        Self {
            current_module: ModuleType::Dashboard,
            modules,
            design,
            system_info: SystemInfo::default(),
            frame_count: 0,
            responsive,
        }
    }
    
    fn update_system_info(&mut self) {
        // This would integrate with actual system monitoring
        // For now, use mock data
        use std::time::{SystemTime, UNIX_EPOCH};
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        self.system_info.cpu_usage = 25.0 + (time % 50) as f32; // Oscillate for demo
        self.system_info.memory_usage = 65.0;
        self.system_info.temperature = 45.0;
    }

    fn apply_responsive_styles(&mut self, ctx: &Context) {
        let layout = &self.responsive.layout;

        ctx.style_mut(|style| {
            // Adjust button padding based on screen size
            style.spacing.button_padding = egui::vec2(
                layout.margins * 1.5,
                layout.margins,
            );

            // Adjust item spacing
            style.spacing.item_spacing = egui::vec2(layout.margins, layout.margins);

            // Adjust window margins
            style.spacing.window_margin = egui::Margin::same(layout.margins);

            // Apply responsive font scaling
            self.design.apply_responsive_fonts(style, layout.font_scale);
        });
    }
    
    // TOUCH-OPTIMIZED COMPONENTS
    pub fn touch_button(&self, ui: &mut egui::Ui, text: impl Into<egui::WidgetText>) -> egui::Response {
        ui.add(
            egui::Button::new(text)
                .min_size(self.design.touch_targets.min_button_size)
                .fill(self.design.colors.primary)
        )
    }
    
    pub fn icon_button(&self, ui: &mut egui::Ui, icon: &str) -> egui::Response {
        ui.add(
            egui::Button::new(icon)
                .min_size(self.design.touch_targets.min_icon_size)
                .frame(false)
        )
    }
    
    pub fn header_panel(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // App title with icon
            ui.heading("🐻 Pi Control Hub");
            
            // System status with touch-friendly spacing
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // CPU indicator
                ui.colored_label(
                    self.get_usage_color(self.system_info.cpu_usage),
                    format!("CPU: {:.0}%", self.system_info.cpu_usage)
                );
                
                ui.label("•");
                
                // Temperature indicator
                ui.colored_label(
                    self.get_temperature_color(self.system_info.temperature),
                    format!("{:.0}°C", self.system_info.temperature)
                );
                
                ui.label("•");
                
                // Time
                ui.label(format!("{}", chrono::Local::now().format("%H:%M")));
            });
        });
    }
    
    fn get_usage_color(&self, usage: f32) -> Color32 {
        match usage {
            // Use darker shades for better contrast with white text on progress bars
            x if x > 80.0 => Color32::from_rgb(220, 38, 38),   // Red-600
            x if x > 60.0 => Color32::from_rgb(202, 138, 4),   // Yellow-600 (darker for contrast)
            _ => Color32::from_rgb(22, 163, 74),               // Green-600
        }
    }

    fn get_temperature_color(&self, temp: f32) -> Color32 {
        match temp {
            x if x > 70.0 => Color32::from_rgb(220, 38, 38),   // Red-600
            x if x > 60.0 => Color32::from_rgb(202, 138, 4),   // Yellow-600
            _ => Color32::from_rgb(22, 163, 74),               // Green-600
        }
    }

    // RESPONSIVE LAYOUTS

    fn mobile_layout(&mut self, ctx: &Context) {
        let header_height = self.responsive.layout.header_height;
        let bottom_nav_height = self.responsive.layout.bottom_nav_height;

        // TOP PANEL - Header with system status
        egui::TopBottomPanel::top("header")
            .exact_height(header_height)
            .show(ctx, |ui| {
                self.header_panel(ui);
            });

        // BOTTOM PANEL - Navigation (Mobile style)
        egui::TopBottomPanel::bottom("bottom_nav")
            .exact_height(bottom_nav_height)
            .show(ctx, |ui| {
                self.bottom_navigation(ui);
            });

        // MAIN CONTENT AREA
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_current_module(ctx, ui);
        });
    }

    fn desktop_layout(&mut self, ctx: &Context) {
        let header_height = self.responsive.layout.header_height;
        let nav_panel_width = self.responsive.layout.nav_panel_width;

        // TOP PANEL - Header with system status
        egui::TopBottomPanel::top("header")
            .exact_height(header_height)
            .show(ctx, |ui| {
                self.header_panel(ui);
            });

        // LEFT PANEL - Side Navigation (Desktop style)
        egui::SidePanel::left("navigation")
            .resizable(false)
            .exact_width(nav_panel_width)
            .show(ctx, |ui| {
                self.side_navigation(ui);
            });

        // MAIN CONTENT AREA
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_current_module(ctx, ui);
        });
    }

    fn render_current_module(&mut self, ctx: &Context, ui: &mut egui::Ui) {
        match self.current_module {
            ModuleType::Dashboard => {
                self.dashboard_view(ui);
            }
            ModuleType::SystemMonitor => {
                if let Some(module) = self.modules.get_mut(&ModuleType::SystemMonitor) {
                    module.show(ctx, ui);
                }
            }
            _ => {
                self.placeholder_view(ui, self.current_module.name());
            }
        }
    }

    fn bottom_navigation(&mut self, ui: &mut egui::Ui) {
        let nav_button_width = self.responsive.layout.nav_button_width;
        let nav_button_height = self.responsive.layout.nav_button_height;
        let current_module = self.current_module;
        let primary_color = self.design.colors.primary;
        let surface_color = self.design.colors.surface;

        // Create scrollable horizontal navigation for mobile
        egui::ScrollArea::horizontal()
            .id_source("bottom_nav_scroll")
            .auto_shrink([false; 2])
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded)
            .drag_to_scroll(true)  // Enable touch scrolling
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 2.0; // Tighter spacing for mobile

                    for module_type in ModuleType::iter() {
                        let is_active = current_module == module_type;
                        let button_color = if is_active {
                            primary_color
                        } else {
                            surface_color
                        };

                        let button = egui::Button::new(self.get_module_icon(module_type))
                            .min_size(egui::vec2(nav_button_width, nav_button_height))
                            .fill(button_color);

                        if ui.add(button).clicked() {
                            self.current_module = module_type;
                        }
                    }
                });
            });
    }

    fn side_navigation(&mut self, ui: &mut egui::Ui) {
        let layout = &self.responsive.layout;

        // Add scrollable area for navigation buttons
        egui::ScrollArea::vertical()
            .id_source("nav_scroll")
            .auto_shrink([false; 2])
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
            .drag_to_scroll(true)
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(8.0);

                    for module_type in ModuleType::iter() {
                        let is_active = self.current_module == module_type;
                        let button_color = if is_active {
                            self.design.colors.primary
                        } else {
                            self.design.colors.surface
                        };

                        let button = egui::Button::new(module_type.name())
                            .min_size(egui::vec2(
                                layout.nav_button_width - 20.0,
                                layout.nav_button_height,
                            ))
                            .fill(button_color);

                        if ui.add(button).clicked() {
                            self.current_module = module_type;
                        }

                        ui.add_space(4.0);
                    }

                    ui.add_space(8.0);
                });
            });
    }

    fn get_module_icon(&self, module_type: ModuleType) -> &'static str {
        match module_type {
            ModuleType::Dashboard => "🏠",
            ModuleType::SystemMonitor => "📊",
            ModuleType::MediaCenter => "🎵",
            ModuleType::HomeAutomation => "🏡",
            ModuleType::Gaming => "🎮",
            ModuleType::Security => "🔒",
            ModuleType::Settings => "⚙️",
        }
    }
}

impl eframe::App for PiControlApp {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        // Update responsive state
        self.responsive.update(ctx);

        // Update system info once per second (not every frame)
        if self.frame_count % 60 == 0 {
            self.update_system_info();
        }
        self.frame_count += 1;

        // Apply responsive styling
        self.apply_responsive_styles(ctx);

        // Use appropriate layout based on screen size
        if self.responsive.is_mobile() {
            self.mobile_layout(ctx);
        } else {
            self.desktop_layout(ctx);
        }
    }
}

// VIEW COMPONENTS
impl PiControlApp {
    fn dashboard_view(&mut self, ui: &mut egui::Ui) {
        ui.heading("Dashboard");
        ui.add_space(12.0);
        
        // Use columns for widget layout
        ui.columns(2, |columns| {
            // Column 1 - System Status
            self.system_status_widget(&mut columns[0]);
            
            // Column 2 - Quick Actions
            self.quick_actions_widget(&mut columns[1]);
        });
    }
    
    fn system_status_widget(&mut self, ui: &mut egui::Ui) {
        egui::Frame::group(ui.style())
            .inner_margin(egui::Margin::same(12.0))
            .show(ui, |ui| {
                ui.heading("System Status");
                ui.add_space(8.0);
                
                // CPU Usage with colored progress bar
                ui.label("CPU Usage");
                ui.add(
                    egui::ProgressBar::new(self.system_info.cpu_usage / 100.0)
                        .text(format!("{:.1}%", self.system_info.cpu_usage))
                        .fill(self.get_usage_color(self.system_info.cpu_usage))
                );
                
                ui.add_space(8.0);
                
                // Memory Usage
                ui.label("Memory Usage");
                ui.add(
                    egui::ProgressBar::new(self.system_info.memory_usage / 100.0)
                        .text(format!("{:.1}%", self.system_info.memory_usage))
                        .fill(self.get_usage_color(self.system_info.memory_usage))
                );
            });
    }
    
    fn quick_actions_widget(&mut self, ui: &mut egui::Ui) {
        egui::Frame::group(ui.style())
            .inner_margin(egui::Margin::same(12.0))
            .show(ui, |ui| {
                ui.heading("Quick Actions");
                ui.add_space(8.0);
                
                // Large touch-friendly buttons
                if self.touch_button(ui, "🔄 Refresh").clicked() {
                    self.update_system_info();
                }
                
                ui.add_space(4.0);
                
                if self.touch_button(ui, "🔒 Lock").clicked() {
                    // Implement lock functionality
                }
                
                ui.add_space(4.0);
                
                if self.touch_button(ui, "⚙️ Settings").clicked() {
                    self.current_module = ModuleType::Settings;
                }
            });
    }
    
    fn placeholder_view(&mut self, ui: &mut egui::Ui, module_name: &str) {
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);
            ui.heading(format!("{}", module_name));
            ui.add_space(20.0);
            ui.label("🚧 Module under construction");
            ui.add_space(20.0);
            
            // Use touch button for consistency
            if self.touch_button(ui, "← Back to Dashboard").clicked() {
                self.current_module = ModuleType::Dashboard;
            }
        });
    }
}