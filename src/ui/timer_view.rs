use crate::timer::{Phase, Profile, ProfileKind, TimerState};
use eframe::egui;

pub struct UiScale {
    pub font_size_timer: f32,
    pub font_size_label: f32,
    pub show_phase_label: bool,
    pub show_dots: bool,
    pub element_spacing: f32,
    pub dot_radius: f32,
    pub dot_spacing: f32,
    pub btn_size: f32,
    pub grip_size: f32,
    pub top_bar_height: f32,
    pub bottom_bar_height: f32,
    pub center_height: f32,
    pub padding: f32,
}

impl UiScale {
    pub fn from_size(size: &crate::app::WindowSize) -> Self {
        match size {
            crate::app::WindowSize::S => Self {
                font_size_timer: 14.0,
                font_size_label: 0.0,
                show_phase_label: false,
                show_dots: false,
                element_spacing: 0.0,
                dot_radius: 0.0,
                dot_spacing: 0.0,
                btn_size: 10.0,
                grip_size: 12.0,
                top_bar_height: 14.0,
                bottom_bar_height: 16.0,
                center_height: 30.0,
                padding: 2.0,
            },
            crate::app::WindowSize::M => Self {
                font_size_timer: 24.0,
                font_size_label: 11.0,
                show_phase_label: true,
                show_dots: true,
                element_spacing: 1.0,
                dot_radius: 2.5,
                dot_spacing: 8.0,
                btn_size: 14.0,
                grip_size: 18.0,
                top_bar_height: 18.0,
                bottom_bar_height: 20.0,
                center_height: 52.0,
                padding: 4.0,
            },
            crate::app::WindowSize::L => Self {
                font_size_timer: 36.0,
                font_size_label: 15.0,
                show_phase_label: true,
                show_dots: true,
                element_spacing: 4.0,
                dot_radius: 3.5,
                dot_spacing: 10.0,
                btn_size: 18.0,
                grip_size: 24.0,
                top_bar_height: 22.0,
                bottom_bar_height: 24.0,
                center_height: 84.0,
                padding: 6.0,
            },
        }
    }
}

pub fn draw_cycle_counter(ui: &mut egui::Ui, completed: u32, total: u32, accent_color: egui::Color32, radius: f32, spacing: f32) {
    if total == 0 { return; }

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

pub fn render(ui: &mut egui::Ui, timer: &mut TimerState, profile: &Profile, size: &crate::app::WindowSize) {
    let scale = UiScale::from_size(size);

    let icon_color = egui::Color32::from_white_alpha(180);

    let available_rect = ui.available_rect_before_wrap();

    let top_rect = egui::Rect::from_min_max(
        available_rect.min,
        egui::pos2(available_rect.max.x, available_rect.min.y + scale.top_bar_height),
    );
    let bottom_rect = egui::Rect::from_min_max(
        egui::pos2(available_rect.min.x, available_rect.max.y - scale.bottom_bar_height),
        available_rect.max,
    );
    let center_rect = egui::Rect::from_min_max(
        egui::pos2(available_rect.min.x, top_rect.max.y),
        egui::pos2(available_rect.max.x, bottom_rect.min.y),
    );

    // 1. Top Bar: Close button
    ui.allocate_ui_at_rect(top_rect, |ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_space(scale.padding);
            if ui.add(
                egui::Button::new(egui::RichText::new("×").size(scale.btn_size).color(icon_color))
                    .frame(false)
            ).clicked() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    });

    // 2. Center Zone: Timer + Phase + Cycles
    ui.allocate_ui_at_rect(center_rect, |ui| {
        ui.set_clip_rect(center_rect);
        ui.spacing_mut().item_spacing.y = scale.element_spacing;
        
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
            let mut content_height = scale.font_size_timer;
            if scale.show_phase_label {
                content_height += scale.element_spacing + scale.font_size_label;
            }
            if scale.show_dots && profile.kind == ProfileKind::Classic {
                content_height += scale.element_spacing + (scale.dot_radius * 2.0); // diameter
            }
            
            let space_to_add = (scale.center_height - content_height) / 2.0;
            if space_to_add > 0.0 {
                ui.add_space(space_to_add);
            }

            ui.heading(egui::RichText::new(time_str).size(scale.font_size_timer).strong());
            
            if scale.show_phase_label {
                ui.label(egui::RichText::new(phase_name).size(scale.font_size_label));
            }

            if scale.show_dots && profile.kind == ProfileKind::Classic {
                let accent_color = match timer.phase {
                    Phase::Work => egui::Color32::from_rgb(255, 100, 100),
                    Phase::ShortBreak => egui::Color32::from_rgb(100, 255, 100),
                    Phase::LongBreak => egui::Color32::from_rgb(100, 100, 255),
                };
                let display_completed = match timer.phase {
                    Phase::Work => timer.cycle_count + 1,
                    Phase::ShortBreak | Phase::LongBreak => timer.cycle_count,
                };
                draw_cycle_counter(ui, display_completed, profile.cycles_before_long, accent_color, scale.dot_radius, scale.dot_spacing);
            }
        });
    });

    // 3. Bottom Bar: Controls
    ui.allocate_ui_at_rect(bottom_rect, |ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.add_space(scale.padding);

            // Nav buttons
            if ui.add(
                egui::Button::new(egui::RichText::new("⏮").size(scale.btn_size).color(icon_color))
                    .frame(false)
            ).clicked() {
                timer.prev_phase(profile);
            }
            if ui.add(
                egui::Button::new(egui::RichText::new("⏭").size(scale.btn_size).color(icon_color))
                    .frame(false)
            ).clicked() {
                timer.advance_phase(profile);
            }

            // Flexible space
            let remaining = ui.available_width() - scale.grip_size - scale.btn_size - 8.0;
            ui.add_space(remaining.max(0.0));

            // Play/pause
            let play_icon = if timer.running { "⏸" } else { "⏵" };
            if ui.add(
                egui::Button::new(egui::RichText::new(play_icon).size(scale.btn_size).color(icon_color))
                    .frame(false)
            ).clicked() {
                timer.running = !timer.running;
            }

            ui.add_space(scale.padding);

            // Grip
            let (rect, response) = ui.allocate_exact_size(
                egui::vec2(scale.grip_size, scale.grip_size),
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
            let half = scale.grip_size * 0.25;
            let sy = rect.top() + scale.grip_size * 0.25;
            let sp = scale.grip_size * 0.25;
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
