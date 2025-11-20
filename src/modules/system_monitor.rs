use sysinfo::{System, SystemExt};
use egui::{Context, ProgressBar};

pub struct SystemMonitor {
    system: System,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        Self { system }
    }
}

impl super::Module for SystemMonitor {
    fn name(&self) -> &'static str {
        "System Monitor"
    }

    fn show(&mut self, ctx: &Context, ui: &mut egui::Ui) {
        self.system.refresh_all();

        // CPU Usage
        ui.heading("CPU Usage");
        let cpu_usage = self.system.global_cpu_usage();
        ui.add(ProgressBar::new(cpu_usage as f32 / 100.0).text(format!("{:.1}%", cpu_usage)));

        // Memory Usage
        ui.heading("Memory Usage");
        let used_memory = self.system.used_memory() as f32;
        let total_memory = self.system.total_memory() as f32;
        let memory_percent = used_memory / total_memory;
        ui.add(ProgressBar::new(memory_percent).text(
            format!("{:.1} MB / {:.1} MB ({:.1}%)", 
                used_memory / 1024.0, 
                total_memory / 1024.0, 
                memory_percent * 100.0)
        ));

        // Temperature (if available)
        if let Some(components) = self.system.components().first() {
            ui.heading("Temperature");
            ui.label(format!("{:.1}°C", components.temperature()));
        }
    }
}