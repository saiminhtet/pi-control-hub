pub mod system_monitor;
pub mod media_center;
// TODO: Implement these modules
// pub mod home_automation;
// pub mod settings;

use egui::Context;

pub trait Module {
    fn name(&self) -> &'static str;
    fn show(&mut self, ctx: &Context, ui: &mut egui::Ui);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleType {
    Dashboard,
    SystemMonitor,
    MediaCenter,
    HomeAutomation,
    Gaming,
    Security,
    Settings,
}

impl ModuleType {
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::Dashboard,
            Self::SystemMonitor,
            Self::MediaCenter,
            Self::HomeAutomation,
            Self::Gaming,
            Self::Security,
            Self::Settings,
        ].into_iter()
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Dashboard => "📊 Dashboard",
            Self::SystemMonitor => "🖥️ System Monitor",
            Self::MediaCenter => "🎵 Media Center",
            Self::HomeAutomation => "🏠 Home Auto",
            Self::Gaming => "🎮 Gaming",
            Self::Security => "🔒 Security",
            Self::Settings => "⚙️ Settings",
        }
    }
}