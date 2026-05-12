use crate::timer::{Profile, TimerState};
use crate::ui::UiState;
use eframe::egui;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowSize {
    S,
    M,
    L,
}

pub const SIZE_S: (f32, f32) = (120.0, 60.0);
pub const SIZE_M: (f32, f32) = (180.0, 90.0);
pub const SIZE_L: (f32, f32) = (260.0, 130.0);

pub struct App {
    pub timer: TimerState,
    pub active_profile: Profile,
    pub ui_state: UiState,
    last_tick: Instant,
    pub base_opacity: f32,
    pub window_size: WindowSize,
    pub initialized: bool,
    pub hwnd: Option<isize>,
    pub audio_manager: crate::audio::AudioManager,
    pub config: crate::config::AppConfig,
}

impl App {
    pub fn new(active_profile: Profile) -> Self {
        let mut timer = TimerState::new();
        timer.reset(&active_profile);
        Self {
            timer,
            active_profile,
            ui_state: UiState::default(),
            last_tick: Instant::now(),
            base_opacity: 0.30,
            window_size: WindowSize::M,
            initialized: false,
            hwnd: None,
            audio_manager: crate::audio::AudioManager::new(),
            config: crate::config::AppConfig::load(),
        }
    }

    fn handle_phase_transition(&mut self, transition: crate::timer::PhaseTransition) {
        self.ui_state.flash.start();

        if self.config.global.sound_enabled {
            let sound_event = match transition {
                crate::timer::PhaseTransition::WorkEnded => crate::audio::SoundEvent::WorkEnd,
                crate::timer::PhaseTransition::ShortBreakEnded => crate::audio::SoundEvent::ShortBreakEnd,
                crate::timer::PhaseTransition::LongBreakEnded => crate::audio::SoundEvent::LongBreakEnd,
            };
            self.audio_manager.play(sound_event, self.config.global.volume);
        }

        self.timer.advance_phase(&self.active_profile);
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.initialized {
            if let Some(hwnd) = crate::window::setup() {
                self.hwnd = Some(hwnd);
            }
            self.initialized = true;
        }

        // Sync config values to app state for real-time preview FIRST
        self.base_opacity = self.config.global.opacity;
        self.window_size = match self.config.global.window_size {
            crate::config::WindowSize::S => WindowSize::S,
            crate::config::WindowSize::M => WindowSize::M,
            crate::config::WindowSize::L => WindowSize::L,
        };

        #[cfg(windows)]
        if let Some(hwnd) = self.hwnd {
            crate::window::set_window_transparent(hwnd, self.base_opacity);
        }

        let now = Instant::now();
        let elapsed = now.duration_since(self.last_tick).as_secs();
        if elapsed >= 1 {
            if self.timer.running {
                for _ in 0..elapsed {
                    if let Some(transition) = self.timer.update(1, &self.active_profile) {
                        self.handle_phase_transition(transition);
                    }
                }
            }
            self.last_tick += std::time::Duration::from_secs(elapsed);
        }

        ctx.request_repaint();

        ctx.send_viewport_cmd(egui::ViewportCommand::Transparent(true));

        let size = match self.window_size {
            WindowSize::S => SIZE_S,
            WindowSize::M => SIZE_M,
            WindowSize::L => SIZE_L,
        };
        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(size.0, size.1)));

        let frame = egui::Frame::none().fill(egui::Color32::from_black_alpha(255));
        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            crate::ui::timer_view::render(ui, &mut self.timer, &self.active_profile, self.window_size);
        });

        // Save window position in memory
        if let Some(outer_rect) = ctx.input(|i| i.viewport().outer_rect) {
            let pos = outer_rect.min;
            self.config.global.window_x = Some(pos.x);
            self.config.global.window_y = Some(pos.y);
        }
    }

    fn on_exit(&mut self) {
        self.config.save();
    }
}
