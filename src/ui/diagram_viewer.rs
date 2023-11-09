use eframe::{egui::{self, Event, Ui}, epaint::{Stroke, Color32, Pos2, FontId, Rect, Rounding}, emath::Align2};

use super::{super::model::dia::diafile::DiaFile, train_drawer::draw_train};

#[derive(Debug, Default)]
pub struct DiagramViewer {
    pub display_line_index: usize,
    pub display_diagram_index: usize,
    pub offset_x: f32,
    pub offset_y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub window_size_x: f32,
    pub window_size_y: f32,
}

impl DiagramViewer {
    /// 1分を何ピクセルで描画するかの初期値
    pub const TIME_PER_PIXEL: f32 = 10.0;
    /// 1km何ピクセルで描画するかの初期値
    pub const KM_PER_PIXEL: f32 = 40.0;

    pub const OFFSET_X: f32 = 100.0;
    pub const OFFSET_Y: f32 = 20.0;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, ctx: &egui::Context, ui: &mut Ui, diagram_data: &DiaFile) {
        let now_size = ui.available_size();
        let start_pos = ui.clip_rect().min;

        ctx.input(|i| {
            i.events.iter().for_each(|v| match *v {
                Event::MouseWheel { unit: _, delta, modifiers: _ } => {
                    /* if modifiers.shift {
                        self.offset_y = (self.offset_y + delta.y).clamp(-((1440 * DiagramViewer::TIME_PER_PIXEL) as f32), 0.0);
                    } else {
                        self.offset_x = (self.offset_x + delta.x).clamp(-((1440 * DiagramViewer::TIME_PER_PIXEL) as f32), 0.0);
                    } */
                    self.offset_x = (self.offset_x + delta.x * DiagramViewer::TIME_PER_PIXEL).clamp(-1440.0 * DiagramViewer::TIME_PER_PIXEL, 0.0);
                    self.offset_y = (self.offset_y + delta.y * DiagramViewer::TIME_PER_PIXEL).clamp(-1440.0 * DiagramViewer::TIME_PER_PIXEL, 0.0);
                },
                Event::Zoom(zoom) => {
                    self.scale_x = zoom;
                    self.scale_y = zoom;
                },
                _ => {},
            });
            
        });
        
        ui.allocate_space(now_size);
        let painter = ui.painter();

        let station_y = super::train_drawer::get_station_y(diagram_data, self.display_line_index);

        painter.rect(ui.clip_rect(), Rounding::ZERO, ui.style().visuals.window_fill, Stroke::NONE);

        for i in (0..=24*60).step_by(2) {
            let line = eframe::epaint::Shape::dashed_line(&[
                Pos2 {x: (i as f32) * DiagramViewer::TIME_PER_PIXEL + DiagramViewer::OFFSET_X + self.offset_x + start_pos.x, y: DiagramViewer::OFFSET_Y + self.offset_y + start_pos.y, },
                Pos2 {x: (i as f32) * DiagramViewer::TIME_PER_PIXEL + DiagramViewer::OFFSET_X + self.offset_x + start_pos.x, y: *station_y.last().unwrap() + DiagramViewer::OFFSET_Y + self.offset_y + start_pos.y, },
            ], Stroke {width:0.3, color:Color32::GRAY}, 10.0, 5.0);
            painter.extend(line);
        }
        for i in (0..=24*60).step_by(10) {
            painter.vline(
                (i as f32) * DiagramViewer::TIME_PER_PIXEL + DiagramViewer::OFFSET_X + self.offset_x + start_pos.x, 
                std::ops::RangeInclusive::new(
                    DiagramViewer::OFFSET_Y + self.offset_y + start_pos.y, 
                    *station_y.last().unwrap() + DiagramViewer::OFFSET_Y + self.offset_y + start_pos.y), 
                Stroke {width:1.0, color:Color32::GRAY},);
        }
        for i in (0..=24*60).step_by(30) {
            painter.vline(
                (i as f32) * DiagramViewer::TIME_PER_PIXEL + DiagramViewer::OFFSET_X + self.offset_x + start_pos.x, 
                std::ops::RangeInclusive::new(
                    DiagramViewer::OFFSET_Y + self.offset_y + start_pos.y, 
                    *station_y.last().unwrap() + DiagramViewer::OFFSET_Y + self.offset_y + start_pos.y), 
                Stroke {width:3.0, color:Color32::GRAY},);
        }
        {
            let mut y = self.offset_y;
            for stn in &diagram_data.railway.lines[self.display_line_index].stations {
                let stroke_width: f32 = if stn.is_main { 3.0 } else { 1.0 };
                painter.hline(
                    std::ops::RangeInclusive::new(
                        DiagramViewer::OFFSET_X + start_pos.x, 
                        now_size.x + start_pos.x), 
                    y + DiagramViewer::OFFSET_Y + start_pos.y, 
                    Stroke {width:stroke_width, color:Color32::GRAY},);
                y += stn.next_station_distance * DiagramViewer::KM_PER_PIXEL;
            }
        }
        
        // draw train
        for updw in [&diagram_data.railway.diagrams[self.display_diagram_index].down_trains, &diagram_data.railway.diagrams[self.display_diagram_index].up_trains ] {
            for train in updw {
                painter.extend(draw_train(self, ui, diagram_data, train));
            }
        }

        // header background
        {
            let color = ui.style().visuals.window_fill;
            painter.rect(Rect::from_points(&[
                Pos2 {x: 0.0 + start_pos.x , y: 0.0 + start_pos.y},
                Pos2 {x: now_size.x + start_pos.x, y: DiagramViewer::OFFSET_Y + start_pos.y},
            ]), 0.0, color, Stroke::new(1.0, Color32::BLACK));
            painter.rect(Rect::from_points(&[
                Pos2 {x: 0.0  + start_pos.x, y: 0.0 + start_pos.y},
                Pos2 {x: DiagramViewer::OFFSET_X + start_pos.x, y: now_size.y + start_pos.y},
            ]), 0.0, color, Stroke::new(1.0, Color32::BLACK));
        }

        // Y header
        for i in (0..24*60).step_by(60) {
            painter.text(Pos2 { x: (i as f32) * DiagramViewer::TIME_PER_PIXEL + DiagramViewer::OFFSET_X + self.offset_x + start_pos.x, y: 0.0 + start_pos.y }, Align2::CENTER_TOP, i / 60, FontId::default(), Color32::BLACK);
        }
        
        // X header            
        let mut y = self.offset_y;
        for stn in &diagram_data.railway.lines[self.display_line_index].stations {
            let stroke_width: f32 = if stn.is_main { 3.0 } else { 1.0 };
            painter.hline(std::ops::RangeInclusive::new(0.0 + start_pos.x, DiagramViewer::OFFSET_X + start_pos.x), y + DiagramViewer::OFFSET_Y + start_pos.y, Stroke {width:stroke_width, color:Color32::GRAY},);
            painter.text(Pos2 {x: 0.0 + 1.0 + start_pos.x, y: y + 1.0 + DiagramViewer::OFFSET_Y + start_pos.y}, Align2::LEFT_TOP, &stn.name, FontId::default(), Color32::BLACK);
            y += stn.next_station_distance * DiagramViewer::KM_PER_PIXEL;
        }
        
    }
}