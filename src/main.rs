mod app;
mod audio;
mod config;
mod timer;
mod ui;
mod window;

use app::App;
use eframe::egui;
use timer::{Profile, ProfileKind};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([180.0, 90.0])
            .with_always_on_top()
            .with_decorations(false)
            .with_transparent(true)
            .with_title("anchor"),
        ..Default::default()
    };

    let config = config::AppConfig::load();
    let profile_name = &config.global.active_profile_name;
    let profile_cfg = config.profiles.get(profile_name)
        .or_else(|| config.profiles.get("classic"))
        .cloned()
        .unwrap_or_default();

    let active_profile = Profile {
        kind: if profile_name == "classic" {
            ProfileKind::Classic
        } else {
            ProfileKind::NoLongBreak
        },
        work_secs: profile_cfg.work_duration_secs as u64,
        short_break_secs: profile_cfg.short_break_secs as u64,
        long_break_secs: profile_cfg.long_break_secs.unwrap_or(0) as u64,
        cycles_before_long: profile_cfg.cycles_before_long_break.unwrap_or(4),
    };

    eframe::run_native(
        "anchor",
        options,
        Box::new(|_cc| Box::new(App::new(active_profile)) as Box<dyn eframe::App>),
    )
}
