use diagram_types::root::RootFile;
use eframe::{egui::{Context, CentralPanel}, Frame};

use crate::diagram_viewer::DiagramViewer;


#[derive(Default)]
pub struct DiagramApp {
	pub root_file: RootFile,
	pub diagram_viewer: DiagramViewer,
}

impl DiagramApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            diagram_viewer: DiagramViewer::new(),
            ..Self::default()
        }
    }
}

impl eframe::App for DiagramApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            
            self.diagram_viewer.update(ctx, ui, &self.root_file);
        });
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        let _ = std::fs::write("./export.dia", &self.root_file.serialize());
        let _ = std::fs::write("./export.dia_c", &self.root_file.serialize_comp());
    }
}
