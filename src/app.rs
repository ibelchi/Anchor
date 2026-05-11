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
    pub just_opened_menu: bool,
    last_tick: Instant,
    pub base_opacity: f32,
    pub window_size: WindowSize,
    pub last_interaction: Option<Instant>,
    pub initialized: bool,
    pub hwnd: Option<isize>,
    pub overlay_hwnd: Option<isize>,
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
            just_opened_menu: false,
            last_tick: Instant::now(),
            base_opacity: 0.30,
            window_size: WindowSize::M,
            last_interaction: None,
            initialized: false,
            hwnd: None,
            overlay_hwnd: None,
            audio_manager: crate::audio::AudioManager::new(),
            config: crate::config::AppConfig::load(),
        }
    }

    pub fn set_opacity(&mut self, value: f32) {
        self.base_opacity = value.clamp(0.1, 1.0);
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
                self.overlay_hwnd = Some(crate::window::create_overlay_window());
            }
            self.initialized = true;
        }

        let target_opacity = if self.ui_state.interaction_mode {
            0.90
        } else {
            self.base_opacity
        };

        #[cfg(windows)]
        if let Some(hwnd) = self.hwnd {
            crate::window::set_window_transparent(hwnd, !self.ui_state.interaction_mode, target_opacity);
            
            if let Some(overlay_hwnd) = self.overlay_hwnd {
                crate::window::update_overlay_position(overlay_hwnd, hwnd);
            }
        }

        if crate::window::RCLICK_DETECTED.swap(false, std::sync::atomic::Ordering::Relaxed) {
            self.ui_state.interaction_mode = true;
            self.last_interaction = Some(Instant::now());
            self.ui_state.show_context_menu = true;
            
            if let Some(pos) = ctx.pointer_hover_pos() {
                self.ui_state.context_menu_pos = pos;
            } else {
                let rect = ctx.screen_rect();
                self.ui_state.context_menu_pos = rect.center();
            }
            self.just_opened_menu = true;
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

        self.just_opened_menu = false;

        if self.ui_state.show_context_menu {
            self.ui_state.interaction_mode = true;
            self.last_interaction = Some(Instant::now());
        } else if self.ui_state.interaction_mode {
            if let Some(last) = self.last_interaction {
                if last.elapsed().as_secs() >= 3 {
                    self.ui_state.interaction_mode = false;
                    self.ui_state.dragging = false;
                }
            }
        }

        ctx.send_viewport_cmd(egui::ViewportCommand::Transparent(true));

        let size = match self.window_size {
            WindowSize::S => SIZE_S,
            WindowSize::M => SIZE_M,
            WindowSize::L => SIZE_L,
        };
        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(size.0, size.1)));

        let frame = egui::Frame::none().fill(egui::Color32::from_black_alpha(255));
        let response = egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            crate::ui::timer_view::render(ui, &mut self.timer, &self.active_profile, self.window_size);
        }).response;

        if self.ui_state.interaction_mode {
            let interact_response = response.interact(egui::Sense::click_and_drag());

            if interact_response.drag_started() {
                self.ui_state.dragging = true;
            }
            
            if self.ui_state.dragging {
                self.last_interaction = Some(Instant::now()); // reset timeout mentre arrossega
                if ctx.input(|i| i.pointer.primary_down()) {
                    ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                } else {
                    self.ui_state.dragging = false;
                }
            }

            if interact_response.secondary_clicked() {
                self.ui_state.show_context_menu = true;
                if let Some(pos) = ctx.pointer_interact_pos() {
                    self.ui_state.context_menu_pos = pos;
                }
                self.just_opened_menu = true;
                self.last_interaction = Some(Instant::now());
            }
        }

        crate::ui::context_menu::render(ctx, self);
    }
}
