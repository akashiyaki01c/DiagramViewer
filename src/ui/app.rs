use eframe::egui::{self, CentralPanel, Frame, SidePanel, TopBottomPanel, Visuals, Window};

use crate::{
    io::file, model::dia::diafile::DiaFile, ui::diagram_viewer::DiagramViewer,
    ui::menu_bar::Menubar,
};

use super::{explorer::Explorer, timetable_viewer::TimetableViewer};

#[derive(Default)]
pub struct DiagramApp {
    pub diagram_data: DiaFile,
    pub diagram_viewer: DiagramViewer,
    pub timetable_viewer: TimetableViewer,
    pub menubar: Menubar,
    pub explorer: Explorer,
}

impl DiagramApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        cc.egui_ctx.set_visuals(Visuals::light());

        Self {
            diagram_viewer: DiagramViewer::new(),
            timetable_viewer: TimetableViewer::new(),
            ..Self::default()
        }
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../../fonts/NotoSansJP-VariableFont_wght.ttf"
        )),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());
    ctx.set_fonts(fonts);
}

impl eframe::App for DiagramApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        TopBottomPanel::top("menu_bar").show(ctx, |ui| self.menubar.update(ctx, frame, ui));
        SidePanel::left("left-panel").show(ctx, |ui| {
            self.explorer.update(ctx, frame, ui);
        });
        CentralPanel::default().show(ctx, |_ui| {
            Window::new("Diagram Viewer").show(ctx, |ui| {
                Frame::canvas(ui.style()).show(ui, |ui| {
                    self.diagram_viewer.update(ctx, ui, &self.diagram_data);
                });
            });
            Window::new("Timetable Viewer").show(ctx, |ui| {
                self.timetable_viewer.update(ctx, ui, &self.diagram_data);
            });
        });
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        let _ = std::fs::write("./export.dia", file::serialize(&self.diagram_data));
        let _ = std::fs::write("./export.dia_c", file::serialize_comp(&self.diagram_data));
    }
}
