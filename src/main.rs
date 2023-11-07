mod app;

mod diagram_viewer;
mod model;
mod io;

use diagram_viewer::DiagramViewer;
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
        Box::new(|_cc| {
            let mut viewer = Box::new(DiagramViewer::new(_cc));
            viewer.diagram_data = testdata();
            viewer
        }),
    );
}