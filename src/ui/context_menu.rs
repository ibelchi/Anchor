use crate::app::App;
use crate::timer::{Profile, ProfileKind};
use eframe::egui;

pub fn render(ctx: &egui::Context, app: &mut App) {
    if !app.ui_state.show_context_menu {
        return;
    }

    let mut close_menu = false;

    let frame = egui::Frame::popup(&ctx.style())
        .fill(egui::Color32::from_rgb(30, 30, 30))
        .rounding(4.0)
        .inner_margin(egui::Margin::same(6.0));

    let area_response = egui::Area::new("context_menu".into())
        .fixed_pos(app.ui_state.context_menu_pos)
        .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            frame.show(ui, |ui| {
                ui.visuals_mut().widgets.inactive.fg_stroke.color = egui::Color32::WHITE;
                ui.visuals_mut().widgets.hovered.fg_stroke.color = egui::Color32::WHITE;
                
                ui.style_mut().spacing.item_spacing = egui::vec2(8.0, 4.0);

                let run_text = if app.timer.running { "Pause" } else { "Start" };
                if ui.button(run_text).clicked() {
                    if app.timer.running {
                        app.timer.pause();
                    } else {
                        app.timer.start();
                    }
                    close_menu = true;
                }

                if ui.button("Restart").clicked() {
                    app.timer.reset(&app.active_profile);
                    close_menu = true;
                }

                if ui.button("Skip phase").clicked() {
                    app.timer.skip(&app.active_profile);
                    close_menu = true;
                }

                ui.separator();

                ui.menu_button("Profile", |ui| {
                    let mut switch_to = None;
                    
                    let classic_text = if app.active_profile.kind == ProfileKind::Classic { "● Classic" } else { "  Classic" };
                    if ui.button(classic_text).clicked() {
                        switch_to = Some(ProfileKind::Classic);
                    }

                    let no_long_text = if app.active_profile.kind == ProfileKind::NoLongBreak { "● No long break" } else { "  No long break" };
                    if ui.button(no_long_text).clicked() {
                        switch_to = Some(ProfileKind::NoLongBreak);
                    }

                    if let Some(kind) = switch_to {
                        let new_profile = match kind {
                            ProfileKind::Classic => Profile {
                                kind: ProfileKind::Classic,
                                work_secs: 25 * 60,
                                short_break_secs: 5 * 60,
                                long_break_secs: 15 * 60,
                                cycles_before_long: 4,
                            },
                            ProfileKind::NoLongBreak => Profile {
                                kind: ProfileKind::NoLongBreak,
                                work_secs: 25 * 60,
                                short_break_secs: 5 * 60,
                                long_break_secs: 0,
                                cycles_before_long: 0,
                            },
                        };
                        app.active_profile = new_profile;
                        app.timer.reset(&app.active_profile);
                        close_menu = true;
                        ui.close_menu();
                    }
                });

                ui.separator();

                if ui.button("Settings").clicked() {
                    app.ui_state.show_settings = true;
                    close_menu = true;
                }

                if ui.button("Close").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    close_menu = true;
                }
            });
        }).response;

    if close_menu {
        app.ui_state.show_context_menu = false;
    } else if ctx.input(|i| i.pointer.any_pressed()) {
        if let Some(pos) = ctx.input(|i| i.pointer.interact_pos()) {
            if !area_response.rect.contains(pos) && !app.just_opened_menu {
                app.ui_state.show_context_menu = false;
            }
        }
    }
}
