use eframe::egui::{self, CentralPanel, SidePanel, TopBottomPanel, Visuals, Window};

use crate::{
    io::file, model::dia::diafile::DiaFile, ui::diagram_viewer::DiagramViewer,
    ui::menu_bar::Menubar,
};

use super::explorer::Explorer;

#[derive(Debug, Default)]
pub struct DiagramApp {
    pub diagram_data: DiaFile,
    pub diagram_viewer: DiagramViewer,
    pub menubar: Menubar,
    pub explorer: Explorer,
}

impl DiagramApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        cc.egui_ctx.set_visuals(Visuals::light());

        Self {
            diagram_viewer: DiagramViewer::new(),
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
                self.diagram_viewer.update(ctx, ui, &self.diagram_data);
                TopBottomPanel::top("viewer-menu").show_inside(ui, |ui| {
                    ui.menu_button("View", |ui| {
                        if ui.button("Reset Scale").clicked() {
                            self.diagram_viewer.scale_x = 1.0;
                            self.diagram_viewer.scale_y = 1.0;
                        }
                    })
                });
            })
        });
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        let _ = std::fs::write("./export.dia", file::serialize(&self.diagram_data));
        let _ = std::fs::write("./export.dia_c", file::serialize_comp(&self.diagram_data));
    }
}
