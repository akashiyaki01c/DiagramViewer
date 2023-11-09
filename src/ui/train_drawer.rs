use eframe::egui::Ui;
use eframe::epaint::Shape;

use crate::egui::Color32;
use crate::egui::Stroke;
use crate::model::dia::diafile::DiaFile;
use crate::ui::diagram_viewer::DiagramViewer;
use crate::egui::Pos2;
use crate::model::dia::diagram::Time;
use crate::model::dia::diagram::Train;

pub fn get_station_y(diagram_data: &DiaFile, line_index: usize) -> Vec<f32> {
    let mut result = vec![];
    let mut y = 0.0;
    for stn in &diagram_data.railway.lines[line_index].stations {
        result.push(y);
        y += stn.next_station_distance * DiagramViewer::KM_PER_PIXEL;
    }
    result
}

pub fn draw_train(viewer: &DiagramViewer, ui: &Ui, diagram_data: &DiaFile, train: &Train) -> Vec<Shape> {
	let y = viewer.offset_y;
	let x = viewer.offset_x;
	let mut shapes: Vec<Shape> = vec![];
	let start_pos = ui.clip_rect().min;
	let station_y = get_station_y(diagram_data, viewer.display_line_index);

	for i in 0..train.station_times.len()-1 {
		let before_sta = &train.station_times[i];
		let next_sta = &train.station_times[i+1];

		let mut time
			= if before_sta.departure.is_some() && next_sta.arrive.is_some() { (before_sta.station_index, before_sta.departure, next_sta.station_index, next_sta.arrive) }
			else if before_sta.departure.is_some() && next_sta.departure.is_some() { (before_sta.station_index, before_sta.departure, next_sta.station_index, next_sta.departure) }
			else { (0, None, 0, None) };
		if time.1.unwrap().hour > time.3.unwrap().hour {
			time.3 = Some(Time {hour: time.3.unwrap().hour + 24,  ..time.3.unwrap() })
		}

		if before_sta.arrive.is_some() && before_sta.departure.is_some() {
			shapes.push(Shape::line_segment([
				Pos2 {
					x: (before_sta.arrive.unwrap().get_total_minute() as f32) * DiagramViewer::TIME_PER_PIXEL + x + DiagramViewer::OFFSET_X + start_pos.x, 
					y: station_y[before_sta.station_index] + y + DiagramViewer::OFFSET_Y + start_pos.y}, 
				Pos2 {
					x: (before_sta.departure.unwrap().get_total_minute() as f32) * DiagramViewer::TIME_PER_PIXEL + x + DiagramViewer::OFFSET_X + start_pos.x, 
					y: station_y[before_sta.station_index] + y + DiagramViewer::OFFSET_Y + start_pos.y}], 
				Stroke {width:2.0, color:Color32::BLACK},));
		}
		shapes.push(Shape::line_segment([
			Pos2 {
				x: (time.1.unwrap().get_total_minute() as f32) * DiagramViewer::TIME_PER_PIXEL + x + DiagramViewer::OFFSET_X + start_pos.x, 
				y: station_y[time.0] + y + DiagramViewer::OFFSET_Y + start_pos.y}, 
			Pos2 {
				x: (time.3.unwrap().get_total_minute() as f32) * DiagramViewer::TIME_PER_PIXEL + x + DiagramViewer::OFFSET_X + start_pos.x, 
				y: station_y[time.2] + y + DiagramViewer::OFFSET_Y + start_pos.y}], 
			Stroke {width:2.0, color:Color32::BLACK},));
	}
	shapes
}