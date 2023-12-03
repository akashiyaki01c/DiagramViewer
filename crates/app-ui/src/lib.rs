mod test;

use diagram_types::{root::RootFile, diagram::Diagram};
use eframe::{egui::Ui, epaint::{Pos2, Vec2, Rect, Stroke}, emath::{RectTransform, Rangef}};


/// 1分を何ピクセルで描画するか (標準値)
pub const PIXEL_PER_TIME: f32 = 20.0;
/// 1kmを何ピクセルで描画するか (標準値)
pub const PIXEL_PER_KM: f32 = 10.0;

/// 列車線を描画する関数
/// offset: ウィンドウの左上オフセット
/// size: ウィンドウのサイズ
/// scale: 列車の描画スケール
/// diagram_offset: ダイヤグラムのオフセット(描画が何pxから始まるか)
pub fn draw_train(ui: &Ui, offset: Pos2, size: Vec2, scale: Vec2, root: &RootFile, diagram: &Diagram, diagram_offset: Vec2) {
	let width_minute = size.x / PIXEL_PER_TIME;
	let height_km = size.y / PIXEL_PER_KM;
	let transform: RectTransform = RectTransform::from_to(Rect { // 1分 = 1px, 1km = 1px
		min: Pos2 { x: (diagram_offset.x / PIXEL_PER_TIME), y: (diagram_offset.y / PIXEL_PER_KM) },
		max: Pos2 { x: (diagram_offset.x / PIXEL_PER_TIME) + width_minute, y: (diagram_offset.y / PIXEL_PER_KM) + height_km }
	}, Rect {
		min: Pos2 { x: offset.x, y: offset.y },
		max: Pos2 { x: offset.x + size.x, y: offset.y + size.y }
	});
	let painter = ui.painter();

	let positions = train_drawer::station::get_station_positions(&root.settings.diagram_display, &root, false);
	for station in &positions {
		painter.hline(Rangef { min: 0.0, max: size.x }, station.1, Stroke::default());
	};
	for train in &diagram.trains {
	 	let times = train_drawer::train::get_train_lines(train, root, false);
		'time: for i in 0..times.len()-1 {
			let times = (&times[i], &times[i+1]);
			let positions = (&positions[i], &positions[i+1]);
			if times.0.is_empty() || times.1.is_empty() {
				continue 'time;
			}
			painter.line_segment([
				transform.transform_pos(Pos2 { x: times.0.departure.unwrap().total_minute() as f32, y: positions.0.1 }),
				transform.transform_pos(Pos2 { x: times.1.departure.unwrap().total_minute() as f32, y: positions.1.1 })
			], Stroke::default())
		}
	}
}