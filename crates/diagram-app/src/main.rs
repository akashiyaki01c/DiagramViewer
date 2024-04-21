use app_ui::app::DiagramApp;

fn main() {
    let options = eframe::NativeOptions {
        // initial_window_size: Some(eframe::egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "Diagram Viewer",
        options,
        Box::new(|cc: &eframe::CreationContext<'_>| {
            let x = DiagramApp::new(cc);
            let mut app = Box::new(x);
            app.root_file = diagram_testdata::testdata_kt();
            app
        }),
    );
}
