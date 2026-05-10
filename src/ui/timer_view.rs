use crate::timer::{Phase, Profile, ProfileKind, TimerState};
use eframe::egui;

/// Dibuixa un comptador de cicles amb punts horitzontals.
///
/// Nota: S'ha de saltar la renderització completament si el perfil és "No long break" (responsabilitat de qui crida).
pub fn draw_cycle_counter(ui: &mut egui::Ui, completed: u32, total: u32, accent_color: egui::Color32) {
    if total == 0 {
        return;
    }
    
    let radius = 5.0;
    let spacing = 14.0;
    let width = (total - 1) as f32 * spacing + radius * 2.0;
    let height = radius * 2.0;
    
    let (rect, _) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
    
    let painter = ui.painter();
    let center_y = rect.center().y;
    let start_x = rect.left() + radius;
    
    for i in 0..total {
        let center_x = start_x + i as f32 * spacing;
        let center = egui::pos2(center_x, center_y);
        
        if i < completed {
            painter.circle_filled(center, radius, accent_color);
        } else {
            painter.circle_stroke(center, radius, egui::Stroke::new(1.0, accent_color));
        }
    }
}

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
            
            let accent_color = match timer.phase {
                Phase::Work => egui::Color32::from_rgb(255, 100, 100),
                Phase::ShortBreak => egui::Color32::from_rgb(100, 255, 100),
                Phase::LongBreak => egui::Color32::from_rgb(100, 100, 255),
            };
            
            draw_cycle_counter(ui, timer.cycle_count, profile.cycles_before_long, accent_color);
        }
    });
}
