#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use ui::app::App;

mod asi;
mod ui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1600., 900.]),
        ..Default::default()
    };

    eframe::run_native(
        "Sol'EX Analyzer",
        options,
        Box::new(|cc| {
            // This gives us image support:
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Box::<App>::new(App::new())
        }),
    )
}
