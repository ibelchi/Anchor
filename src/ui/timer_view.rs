use crate::timer::{Phase, Profile, ProfileKind, TimerState};
use eframe::egui;

pub fn render(ui: &mut egui::Ui, timer: &mut TimerState, profile: &Profile, size: crate::app::WindowSize) {
    ui.vertical_centered(|ui| {
        let mins = timer.remaining_secs / 60;
        let secs = timer.remaining_secs % 60;
        let time_str = format!("{:02}:{:02}", mins, secs);

        let font_size = match size {
            crate::app::WindowSize::S => 18.0,
            crate::app::WindowSize::M => 28.0,
            crate::app::WindowSize::L => 42.0,
        };

        ui.add_space(10.0);
        ui.heading(egui::RichText::new(time_str).size(font_size).strong());

        let phase_name = match timer.phase {
            Phase::Work => "Work",
            Phase::ShortBreak => "Short Break",
            Phase::LongBreak => "Long Break",
        };
        ui.label(egui::RichText::new(phase_name).size(14.0));

        if profile.kind == ProfileKind::Classic {
            ui.add_space(5.0);
            let total = profile.cycles_before_long;
            let mut dots = String::new();
            for i in 0..total {
                if i < timer.cycle_count {
                    dots.push('●');
                } else {
                    dots.push('○');
                }
                dots.push(' ');
            }
            ui.label(egui::RichText::new(dots.trim()).size(14.0));
        }
    });
}
