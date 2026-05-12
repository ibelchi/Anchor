use crate::timer::{Phase, Profile, ProfileKind, TimerState};
use eframe::egui;

pub fn draw_cycle_counter(ui: &mut egui::Ui, completed: u32, total: u32, accent_color: egui::Color32, scale: f32) {
    if total == 0 { return; }

    let radius = 3.0 * scale;
    let spacing = 10.0 * scale;
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
    let (font_size, phase_font_size, scale) = match size {
        crate::app::WindowSize::S => (14.0, 9.0, 0.6),
        crate::app::WindowSize::M => (28.0, 14.0, 1.0),
        crate::app::WindowSize::L => (42.0, 18.0, 1.4),
    };

    let (top_bar_height, bottom_bar_height, center_height) = match size {
        crate::app::WindowSize::S => (14.0, 16.0, 30.0),
        crate::app::WindowSize::M => (18.0, 20.0, 52.0),
        crate::app::WindowSize::L => (22.0, 24.0, 84.0),
    };

    let btn_size = (14.0_f32 * scale).max(10.0_f32);
    let grip_size = (20.0_f32 * scale).max(12.0_f32);
    let icon_color = egui::Color32::from_white_alpha(180);

    let available_rect = ui.available_rect_before_wrap();

    let top_rect = egui::Rect::from_min_max(
        available_rect.min,
        egui::pos2(available_rect.max.x, available_rect.min.y + top_bar_height),
    );
    let bottom_rect = egui::Rect::from_min_max(
        egui::pos2(available_rect.min.x, available_rect.max.y - bottom_bar_height),
        available_rect.max,
    );
    let center_rect = egui::Rect::from_min_max(
        egui::pos2(available_rect.min.x, top_rect.max.y),
        egui::pos2(available_rect.max.x, bottom_rect.min.y),
    );

    // 1. Top Bar: Close button
    ui.allocate_ui_at_rect(top_rect, |ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_space(2.0);
            if ui.add(
                egui::Button::new(egui::RichText::new("✕").size(btn_size).color(icon_color))
                    .frame(false)
            ).clicked() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    });

    // 2. Center Zone: Timer + Phase + Cycles
    ui.allocate_ui_at_rect(center_rect, |ui| {
        ui.vertical_centered(|ui| {
            let mins = timer.remaining_secs / 60;
            let secs = timer.remaining_secs % 60;
            let time_str = format!("{:02}:{:02}", mins, secs);

            let phase_name = match timer.phase {
                Phase::Work => "Work",
                Phase::ShortBreak => "Short Break",
                Phase::LongBreak => "Long Break",
            };

            // Vertical centering calculation
            let mut content_height = font_size + phase_font_size;
            if profile.kind == ProfileKind::Classic {
                content_height += 6.0 * scale; // radius is 3.0 * scale, diameter is 6.0
            }
            
            let space_to_add = (center_height - content_height) / 2.0;
            if space_to_add > 0.0 {
                ui.add_space(space_to_add);
            }

            ui.heading(egui::RichText::new(time_str).size(font_size).strong());
            ui.label(egui::RichText::new(phase_name).size(phase_font_size));

            if profile.kind == ProfileKind::Classic {
                let accent_color = match timer.phase {
                    Phase::Work => egui::Color32::from_rgb(255, 100, 100),
                    Phase::ShortBreak => egui::Color32::from_rgb(100, 255, 100),
                    Phase::LongBreak => egui::Color32::from_rgb(100, 100, 255),
                };
                let display_completed = match timer.phase {
                    Phase::Work => timer.cycle_count + 1,
                    Phase::ShortBreak | Phase::LongBreak => timer.cycle_count,
                };
                draw_cycle_counter(ui, display_completed, profile.cycles_before_long, accent_color, scale);
            }
        });
    });

    // 3. Bottom Bar: Controls
    ui.allocate_ui_at_rect(bottom_rect, |ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.add_space(2.0);

            // Nav buttons
            if ui.add(
                egui::Button::new(egui::RichText::new("⏮").size(btn_size).color(icon_color))
                    .frame(false)
            ).clicked() {
                timer.prev_phase(profile);
            }
            if ui.add(
                egui::Button::new(egui::RichText::new("⏭").size(btn_size).color(icon_color))
                    .frame(false)
            ).clicked() {
                timer.advance_phase(profile);
            }

            // Flexible space
            let remaining = ui.available_width() - grip_size - btn_size - 8.0;
            ui.add_space(remaining.max(0.0));

            // Play/pause
            let play_icon = if timer.running { "⏸" } else { "⏵" };
            if ui.add(
                egui::Button::new(egui::RichText::new(play_icon).size(btn_size).color(icon_color))
                    .frame(false)
            ).clicked() {
                timer.running = !timer.running;
            }

            ui.add_space(2.0);

            // Grip
            let (rect, response) = ui.allocate_exact_size(
                egui::vec2(grip_size, grip_size),
                egui::Sense::drag(),
            );

            if response.drag_started() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
            }
            if response.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
            }

            let painter = ui.painter();
            let line_color = egui::Color32::from_white_alpha(80);
            let cx = rect.center().x;
            let half = grip_size * 0.25;
            let sy = rect.top() + grip_size * 0.25;
            let sp = grip_size * 0.25;
            for i in 0..3 {
                let y = sy + i as f32 * sp;
                painter.line_segment(
                    [egui::pos2(cx - half, y), egui::pos2(cx + half, y)],
                    egui::Stroke::new(1.0, line_color),
                );
            }
        });
    });
}
