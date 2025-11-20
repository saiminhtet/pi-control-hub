mod touch_design;
pub use touch_design::TouchDesignSystem;

use egui::{Context, Color32};
use crate::modules::{Module, ModuleType, system_monitor::SystemMonitor};

pub struct PiControlApp {
    pub current_module: ModuleType,
    pub modules: std::collections::HashMap<ModuleType, Box<dyn Module>>,
    pub design: TouchDesignSystem,
    pub system_info: SystemInfo,
    pub frame_count: u64,
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
        
        Self {
            current_module: ModuleType::Dashboard,
            modules,
            design,
            system_info: SystemInfo::default(),
            frame_count: 0,
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
    
    pub fn navigation_panel(&mut self, ui: &mut egui::Ui) {
        // Add scrollable area for navigation buttons
        // Touch-optimized: always show scroll bar and use smooth scrolling
        egui::ScrollArea::vertical()
            .id_source("nav_scroll")
            .auto_shrink([false; 2])
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
            .drag_to_scroll(true) // Enable touch drag scrolling
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
                            .min_size(egui::vec2(72.0, self.design.touch_targets.min_button_size.y))
                            .fill(button_color);

                        if ui.add(button).clicked() {
                            self.current_module = module_type;
                        }

                        ui.add_space(4.0);
                    }

                    // Add extra space at bottom for better scrolling
                    ui.add_space(8.0);
                });
            });
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
            x if x > 80.0 => self.design.colors.error,
            x if x > 60.0 => self.design.colors.warning,
            _ => self.design.colors.success,
        }
    }
    
    fn get_temperature_color(&self, temp: f32) -> Color32 {
        match temp {
            x if x > 70.0 => self.design.colors.error,
            x if x > 60.0 => self.design.colors.warning,
            _ => self.design.colors.success,
        }
    }
}

impl eframe::App for PiControlApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Update system info once per second (not every frame)
        if self.frame_count % 60 == 0 {
            self.update_system_info();
        }
        self.frame_count += 1;
        
        // TOP PANEL - Header with system status
        egui::TopBottomPanel::top("header")
            .exact_height(40.0) // Fixed height for touch
            .show(ctx, |ui| {
                self.header_panel(ui);
            });
        
        // LEFT PANEL - Navigation
        egui::SidePanel::left("navigation")
            .resizable(false)
            .min_width(80.0)
            .max_width(100.0)
            .show(ctx, |ui| {
                self.navigation_panel(ui);
            });
        
        // MAIN CONTENT AREA
        egui::CentralPanel::default().show(ctx, |ui| {
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
        });
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