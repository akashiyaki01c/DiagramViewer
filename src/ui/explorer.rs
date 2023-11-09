use eframe::egui::{self, Ui};

#[derive(Debug, Default)]
pub struct Explorer {
	
}

impl Explorer {
	
	pub fn update(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame, ui: &mut Ui) {
		
		ui.label("Exlorer");
	}
}