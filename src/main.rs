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

    let default_profile = Profile {
        kind: ProfileKind::Classic,
        work_secs: 25 * 60,
        short_break_secs: 5 * 60,
        long_break_secs: 15 * 60,
        cycles_before_long: 4,
    };

    eframe::run_native(
        "anchor",
        options,
        Box::new(|_cc| Box::new(App::new(default_profile)) as Box<dyn eframe::App>),
    )
}
