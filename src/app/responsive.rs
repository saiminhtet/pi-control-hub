use egui::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScreenSize {
    Tiny,    // < 480px (Pi touchscreen)
    Small,   // 480px - 768px (Tablets)
    Medium,  // 768px - 1024px (Small laptops)
    Large,   // 1024px - 1440px (Desktop)
    XLarge,  // > 1440px (Large monitors)
}

impl ScreenSize {
    pub fn from_width(width: f32) -> Self {
        if width < 480.0 {
            ScreenSize::Tiny
        } else if width < 768.0 {
            ScreenSize::Small
        } else if width < 1024.0 {
            ScreenSize::Medium
        } else if width < 1440.0 {
            ScreenSize::Large
        } else {
            ScreenSize::XLarge
        }
    }

    pub fn is_mobile(&self) -> bool {
        matches!(self, ScreenSize::Tiny | ScreenSize::Small)
    }

    pub fn is_desktop(&self) -> bool {
        !self.is_mobile()
    }
}

#[derive(Clone)]
pub struct LayoutConfig {
    pub nav_panel_width: f32,
    pub header_height: f32,
    pub margins: f32,
    pub touch_size: f32,
    pub font_scale: f32,
    pub nav_button_width: f32,
    pub nav_button_height: f32,
    pub bottom_nav_height: f32,
}

impl LayoutConfig {
    pub fn for_screen_size(screen_size: ScreenSize) -> Self {
        match screen_size {
            ScreenSize::Tiny => Self {
                nav_panel_width: 0.0,  // No side panel
                header_height: 40.0,
                margins: 8.0,
                touch_size: 44.0,
                font_scale: 1.5,
                nav_button_width: 60.0,  // Smaller for Pi screen (480px / 7 modules ≈ 68px)
                nav_button_height: 56.0,
                bottom_nav_height: 64.0, // Adjusted to fit buttons properly
            },
            ScreenSize::Small => Self {
                nav_panel_width: 0.0,  // No side panel
                header_height: 48.0,
                margins: 10.0,
                touch_size: 40.0,
                font_scale: 1.2,
                nav_button_width: 90.0,
                nav_button_height: 60.0,
                bottom_nav_height: 70.0,
            },
            ScreenSize::Medium => Self {
                nav_panel_width: 120.0,
                header_height: 50.0,
                margins: 12.0,
                touch_size: 36.0,
                font_scale: 1.0,
                nav_button_width: 100.0,
                nav_button_height: 50.0,
                bottom_nav_height: 0.0,  // No bottom nav
            },
            ScreenSize::Large => Self {
                nav_panel_width: 150.0,
                header_height: 52.0,
                margins: 16.0,
                touch_size: 32.0,
                font_scale: 0.95,
                nav_button_width: 130.0,
                nav_button_height: 50.0,
                bottom_nav_height: 0.0,  // No bottom nav
            },
            ScreenSize::XLarge => Self {
                nav_panel_width: 180.0,
                header_height: 56.0,
                margins: 20.0,
                touch_size: 32.0,
                font_scale: 0.9,
                nav_button_width: 160.0,
                nav_button_height: 52.0,
                bottom_nav_height: 0.0,  // No bottom nav
            },
        }
    }
}

pub struct ResponsiveState {
    pub screen_size: ScreenSize,
    pub window_size: Vec2,
    pub layout: LayoutConfig,
}

impl ResponsiveState {
    pub fn new(window_size: Vec2) -> Self {
        let screen_size = ScreenSize::from_width(window_size.x);
        let layout = LayoutConfig::for_screen_size(screen_size);

        Self {
            screen_size,
            window_size,
            layout,
        }
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        let new_size = ctx.screen_rect().size();

        if (new_size - self.window_size).length() > 1.0 {
            self.window_size = new_size;
            let new_screen_size = ScreenSize::from_width(new_size.x);

            if new_screen_size != self.screen_size {
                self.screen_size = new_screen_size;
                self.layout = LayoutConfig::for_screen_size(new_screen_size);
            }
        }
    }

    pub fn is_mobile(&self) -> bool {
        self.screen_size.is_mobile()
    }

    pub fn is_desktop(&self) -> bool {
        self.screen_size.is_desktop()
    }
}
