use egui::{Color32, FontId, Margin, Style, Visuals};

#[derive(Clone)]
pub struct TouchDesignSystem {
    pub colors: AppColors,
    pub spacing: AppSpacing,
    pub typography: AppTypography,
    pub touch_targets: TouchTargets,
}

impl TouchDesignSystem {
    pub fn new() -> Self {
        Self {
            colors: AppColors::default(),
            spacing: AppSpacing::default(),
            typography: AppTypography::default(),
            touch_targets: TouchTargets::default(),
        }
    }

    pub fn apply_to_style(&self, style: &mut Style) {
        // Apply colors
        style.visuals = self.colors.to_visuals();
        
        // Apply spacing
        style.spacing.item_spacing = self.spacing.item_spacing;
        style.spacing.window_margin = self.spacing.window_margin;
        style.spacing.button_padding = self.spacing.button_padding;
        
        // Apply text styles
        style.text_styles = self.typography.to_text_styles();
    }
}

#[derive(Clone)]
pub struct AppColors {
    pub primary: Color32,
    pub secondary: Color32,
    pub success: Color32,
    pub warning: Color32,
    pub error: Color32,
    pub background: Color32,
    pub surface: Color32,
    pub text_primary: Color32,
    pub text_secondary: Color32,
}

impl Default for AppColors {
    fn default() -> Self {
        Self {
            // Dark theme optimized for touch screen visibility
            primary: Color32::from_rgb(59, 130, 246),     // Blue-500
            secondary: Color32::from_rgb(100, 116, 139),  // Slate-500
            success: Color32::from_rgb(34, 197, 94),      // Green-500
            warning: Color32::from_rgb(234, 179, 8),      // Yellow-500
            error: Color32::from_rgb(239, 68, 68),        // Red-500
            background: Color32::from_rgb(15, 23, 42),    // Slate-900
            surface: Color32::from_rgb(30, 41, 59),       // Slate-800
            text_primary: Color32::from_rgb(248, 250, 252), // Slate-50
            text_secondary: Color32::from_rgb(148, 163, 184), // Slate-400
        }
    }
}

impl AppColors {
    pub fn to_visuals(&self) -> Visuals {
        let mut visuals = Visuals::dark();

        visuals.widgets.inactive.bg_fill = self.surface;
        visuals.widgets.active.bg_fill = self.primary;
        visuals.widgets.hovered.bg_fill = self.primary.gamma_multiply(0.8);

        visuals.extreme_bg_color = self.background;
        visuals.panel_fill = self.background;
        visuals.window_fill = self.background;

        // In egui 0.24, text colors are set differently
        visuals.override_text_color = Some(self.text_primary);
        visuals.warn_fg_color = self.text_secondary;

        visuals
    }
}

#[derive(Clone)]
pub struct AppSpacing {
    pub item_spacing: egui::Vec2,
    pub window_margin: Margin,
    pub button_padding: egui::Vec2,
    pub touch_target_size: f32,
}

impl Default for AppSpacing {
    fn default() -> Self {
        Self {
            item_spacing: egui::Vec2::new(8.0, 8.0),
            window_margin: Margin::same(12.0),
            button_padding: egui::Vec2::new(16.0, 12.0),
            touch_target_size: 44.0, // Minimum touch target size
        }
    }
}

#[derive(Clone)]
pub struct AppTypography {
    pub heading: FontId,
    pub body: FontId,
    pub small: FontId,
    pub button: FontId,
}

impl Default for AppTypography {
    fn default() -> Self {
        Self {
            heading: FontId::proportional(18.0),  // Larger for touch
            body: FontId::proportional(14.0),
            small: FontId::proportional(12.0),
            button: FontId::proportional(14.0),   // Clear button text
        }
    }
}

impl AppTypography {
    pub fn to_text_styles(&self) -> std::collections::BTreeMap<egui::TextStyle, FontId> {
        use egui::TextStyle::*;
        
        [
            (Heading, self.heading.clone()),
            (Body, self.body.clone()),
            (Monospace, self.body.clone()),
            (Button, self.button.clone()),
            (Small, self.small.clone()),
        ].into()
    }
}

#[derive(Clone, Default)]
pub struct TouchTargets {
    pub min_button_size: egui::Vec2,
    pub min_icon_size: egui::Vec2,
}

impl TouchTargets {
    pub fn new() -> Self {
        Self {
            min_button_size: egui::Vec2::new(88.0, 44.0),  // Comfortable touch size
            min_icon_size: egui::Vec2::new(44.0, 44.0),    // Icon buttons
        }
    }
}