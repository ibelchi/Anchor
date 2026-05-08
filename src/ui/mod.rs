use eframe::egui;

pub mod context_menu;
pub mod settings;
pub mod timer_view;

pub struct FlashState {
    pub active: bool,
    pub remaining: f32,
    pub duration: f32,
}

impl FlashState {
    pub fn new(duration: f32) -> Self {
        Self {
            active: false,
            remaining: 0.0,
            duration,
        }
    }

    pub fn start(&mut self) {
        self.active = true;
        self.remaining = self.duration;
    }

    pub fn update(&mut self, dt: f32, base_opacity: f32) -> f32 {
        if !self.active {
            return base_opacity;
        }

        self.remaining -= dt;
        if self.remaining <= 0.0 {
            self.active = false;
            self.remaining = 0.0;
            return base_opacity;
        }

        let progress = self.remaining / self.duration;
        base_opacity + (0.90 - base_opacity) * progress
    }
}

pub struct UiState {
    pub show_context_menu: bool,
    pub context_menu_pos: egui::Pos2,
    pub show_settings: bool,
    pub dragging: bool,
    pub drag_start_offset: egui::Vec2,
    pub interaction_mode: bool,
    pub flash: FlashState,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            show_context_menu: false,
            context_menu_pos: egui::Pos2::ZERO,
            show_settings: false,
            dragging: false,
            drag_start_offset: egui::Vec2::ZERO,
            interaction_mode: false,
            flash: FlashState::new(1.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flash_starts_at_high_opacity() {
        let mut flash = FlashState::new(1.0);
        flash.start();
        let opacity = flash.update(0.0, 0.3);
        assert!(opacity > 0.8, "opacity was {}", opacity);
    }

    #[test]
    fn flash_returns_base_opacity_when_done() {
        let mut flash = FlashState::new(1.0);
        flash.start();
        let opacity = flash.update(2.0, 0.3); // dt > duration
        assert_eq!(opacity, 0.3);
        assert!(!flash.active);
    }
}
