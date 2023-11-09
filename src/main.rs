mod ui;
mod model;
mod io;

use ui::app::DiagramApp;
use eframe::egui;
use model::testdata::testdata;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "Diagram Viewer",
        options,
        Box::new(|cc| {
            let mut app = Box::new(DiagramApp::new(cc));
            app.diagram_data = testdata();
            app
        }),
    );
}