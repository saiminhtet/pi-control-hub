use sysinfo::{System, SystemExt, ComponentExt, CpuExt};
use egui::{Context, ProgressBar, Color32};

pub struct SystemMonitor {
    system: System,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        Self { system }
    }
    
    fn get_usage_color(&self, usage: f32) -> Color32 {
        match usage {
            // Use darker, more saturated colors for better contrast with white text
            x if x > 80.0 => Color32::from_rgb(220, 38, 38),   // Red-600 - good contrast
            x if x > 60.0 => Color32::from_rgb(202, 138, 4),   // Yellow-600 - darker yellow
            _ => Color32::from_rgb(22, 163, 74),               // Green-600 - good contrast
        }
    }

    fn get_temp_color(&self, temp: f32) -> Color32 {
        match temp {
            t if t > 70.0 => Color32::from_rgb(220, 38, 38),   // Red-600
            t if t > 60.0 => Color32::from_rgb(202, 138, 4),   // Yellow-600
            _ => Color32::from_rgb(22, 163, 74),               // Green-600
        }
    }
}

impl super::Module for SystemMonitor {
    fn name(&self) -> &'static str {
        "System Monitor"
    }

    fn show(&mut self, ctx: &Context, ui: &mut egui::Ui) {
        self.system.refresh_all();
        
        ui.heading("Detailed System Monitor");
        ui.add_space(16.0);
        
        // CPU Usage Section
        ui.heading("CPU Usage");
        let cpu_usage = self.system.global_cpu_info().cpu_usage();
        ui.add(
            ProgressBar::new(cpu_usage / 100.0)
                .text(format!("Overall: {:.1}%", cpu_usage))
                .fill(self.get_usage_color(cpu_usage))
        );
        
        ui.add_space(12.0);
        
        // Memory Usage Section
        ui.heading("Memory Usage");
        let used_memory = self.system.used_memory() as f32;
        let total_memory = self.system.total_memory() as f32;
        let memory_percent = used_memory / total_memory;

        ui.add(
            ProgressBar::new(memory_percent)
                .text(format!(
                    "{:.2} GB / {:.2} GB ({:.1}%)",
                    used_memory / 1073741824.0,  // Convert bytes to GB (1024^3)
                    total_memory / 1073741824.0,
                    memory_percent * 100.0
                ))
                .fill(self.get_usage_color(memory_percent * 100.0))
        );
        
        ui.add_space(12.0);
        
        // Temperature (if available)
        if let Some(component) = self.system.components().first() {
            ui.heading("Temperature");
            let temp = component.temperature();
            ui.colored_label(self.get_temp_color(temp), format!("{:.1}°C", temp));
        }
    }
}