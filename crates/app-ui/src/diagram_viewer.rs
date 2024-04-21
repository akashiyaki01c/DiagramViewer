use diagram_types::{root::RootFile, diagram::Diagram};
use eframe::{egui::{Ui, self, Window, Sense}, epaint::{Pos2, Vec2, Rect, Stroke, vec2, pos2, Color32}, emath::{RectTransform, Rangef}};

#[derive(Default)]
pub struct DiagramViewer {
	pub display_line_index: usize,
	/// 表示するダイヤグラムのインデックス
    pub display_diagram_index: usize,
	/// ウィンドウのオフセット
	pub offset: Pos2,
	/// ダイヤグラムのスケール
	pub scale: Vec2,
	/// ウィンドウのサイズ
	pub window_size: Vec2,
	/// ダイヤグラム上のオフセット
	pub diagram_offset: Vec2,
}

impl DiagramViewer {

	pub fn new() -> Self {
		Self {
			scale: Vec2 { x: 0.1, y: 0.33 },
			..Default::default()
		}
	}

	pub fn update(&mut self, ctx: &egui::Context, ui: &mut Ui, root_file: &RootFile) {
		self.window_size = ui.available_size();
		self.offset = ui.clip_rect().min;

		Window::new("test").show(ctx, |ui| {
			
			draw_train(ui, self.offset, self.window_size, self.scale, root_file, &root_file.diagrams[self.display_diagram_index], vec2(0.0, 0.0))
		});
	}
}

/// 1分を何ピクセルで描画するか (標準値)
pub const PIXEL_PER_TIME: f32 = 20.0;
/// 1kmを何ピクセルで描画するか (標準値)
pub const PIXEL_PER_KM: f32 = 10.0;

/// 列車線を描画する関数
/// offset: ウィンドウの左上オフセット
/// size: ウィンドウのサイズ
/// scale: 列車の描画スケール
/// diagram_offset: ダイヤグラムのオフセット(描画が何pxから始まるか)
pub fn draw_train(ui: &mut Ui, offset: Pos2, size: Vec2, scale: Vec2, root: &RootFile, diagram: &Diagram, diagram_offset: Vec2) {
	let width_minute = size.x / PIXEL_PER_TIME;
	let height_km = size.y / PIXEL_PER_KM;
	/* let transform: RectTransform = RectTransform::from_to(Rect { // 1分 = 1px, 1km = 1px
		min: Pos2 { x: (diagram_offset.x / PIXEL_PER_TIME), y: (diagram_offset.y / PIXEL_PER_KM) },
		max: Pos2 { x: (diagram_offset.x / PIXEL_PER_TIME) + width_minute, y: (diagram_offset.y / PIXEL_PER_KM) + height_km }
	}, Rect {
		min: Pos2 { x: offset.x, y: offset.y },
		max: Pos2 { x: offset.x + size.x, y: offset.y + size.y }
	}); */
	ui.allocate_space(size);
	let painter = ui.painter();

	let positions = train_drawer::station::get_station_positions(&root.settings.diagram_display, &root, false);
	for station in &positions {
		painter.hline(Rangef { min: 0.0, max: size.x }, station.1 * PIXEL_PER_KM * scale.y, Stroke::new(1.0, Color32::BLACK));
	};
	for time in 0..24*60 {
		painter.vline(time as f32*PIXEL_PER_TIME*scale.x, Rangef { min: 0.0, max: size.y }, Stroke::new(1.0, Color32::BLACK));
	}
	for train in &diagram.trains {
	 	let times = train_drawer::train::get_train_lines(train, root, false);
		'time: for i in 0..times.len()-1 {
			let times = (&times[i], &times[i+1]);
			let positions = (&positions[i], &positions[i+1]);
			if times.0.is_empty() || times.1.is_empty() {
				continue 'time;
			}
			painter.line_segment([
				plot(positions.0.1, times.0.departure.unwrap().total_minute() as f32, diagram_offset.to_pos2(), scale),
				plot(positions.1.1, times.1.arrive.unwrap_or(times.1.departure.unwrap()).total_minute() as f32, diagram_offset.to_pos2(), scale),
			], Stroke::new(1.0, Color32::BLACK))
		}
	}
}

/// 時間と駅の距離から位置を計算する関数
fn plot_time_and_station(distance_km: f32, time_minute: f32) -> Pos2 {
	Pos2 {
		x: time_minute * PIXEL_PER_TIME,
		y: distance_km * PIXEL_PER_KM
	}
}
fn plot_with_offset_scale(pos: Pos2, offset: Pos2, scale: Vec2) -> Pos2 {
	((pos + offset.to_vec2()).to_vec2() * scale).to_pos2()
}
fn plot(distance_km: f32, time_minute: f32, offset: Pos2, scale: Vec2) -> Pos2 {
	plot_with_offset_scale(plot_time_and_station(distance_km, time_minute), offset, scale)
}