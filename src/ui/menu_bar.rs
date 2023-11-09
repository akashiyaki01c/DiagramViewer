use eframe::egui::{self, Ui};

#[derive(Debug, Default)]
pub struct Menubar {}

impl Menubar {
    pub fn update(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame, ui: &mut Ui) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                ui.label("開く (&0)");
            });
        });
    }
}
