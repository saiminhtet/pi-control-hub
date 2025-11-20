use egui::Context;
use crate::modules::{Module, ModuleType};

pub struct PiControlApp {
    current_module: ModuleType,
    modules: std::collections::HashMap<ModuleType, Box<dyn Module>>,
    memory_usage: f32,
}

impl PiControlApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            current_module: ModuleType::Dashboard,
            modules: std::collections::HashMap::new(),
            memory_usage: 0.0,
        }
    }
}

impl eframe::App for PiControlApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("🐻 Pi Control Hub");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("RAM: {:.1}%", self.memory_usage));
                });
            });
        });

        egui::SidePanel::left("navigation")
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Modules");
                    ui.separator();
                    
                    for module_type in ModuleType::iter() {
                        if ui.button(module_type.name()).clicked() {
                            self.current_module = module_type;
                        }
                    }
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_module {
                ModuleType::Dashboard => {
                    ui.heading("Dashboard");
                    ui.label("Welcome to your Pi Control Hub!");
                }
                ModuleType::SystemMonitor => {
                    ui.heading("System Monitor");
                    // System monitor content will go here
                }
                _ => {
                    ui.heading(format!("{} - Coming Soon", self.current_module.name()));
                }
            }
        });
    }
}