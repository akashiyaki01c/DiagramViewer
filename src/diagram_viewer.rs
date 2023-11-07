use eframe::{egui::{self, CentralPanel, Event, Visuals}, epaint::{Stroke, Color32, Pos2, FontId, Rect}, emath::Align2};
use crate::{io::file, model::diagram::Time};

use super::model::diafile::DiaFile;

#[derive(Debug, Default)]
pub struct DiagramViewer {
    pub diagram_data: DiaFile,
    pub display_line_index: usize,
    pub display_diagram_index: usize,
    pub offset_x: f32,
    pub offset_y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../IBMPlexSansJP-Regular.otf"
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

fn get_station_y(diagram_data: &DiaFile, line_index: usize) -> Vec<f32> {
    let mut result = vec![];
    let mut y = 0.0;
    for stn in &diagram_data.railway.lines[line_index].stations {
        result.push(y);
        y += stn.next_station_distance * DiagramViewer::KM_PER_PIXEL as f32;
    }
    result
}

impl DiagramViewer {
    /// 1分を何ピクセルで描画するかの初期値
    const TIME_PER_PIXEL: usize = 10;
    /// 1km何ピクセルで描画するかの初期値
    const KM_PER_PIXEL: usize = 20;

    const OFFSET_X: f32 = 100.0;
    const OFFSET_Y: f32 = 20.0;

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        cc.egui_ctx.set_visuals(Visuals::light());
        Self::default()
    }
}

impl eframe::App for DiagramViewer {

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.input(|i| {
            i.events.iter().for_each(|v| match *v {
                Event::MouseWheel { unit: _, delta, modifiers: _ } => {
                    /* if modifiers.shift {
                        self.offset_y = (self.offset_y + delta.y).clamp(-((1440 * DiagramViewer::TIME_PER_PIXEL) as f32), 0.0);
                    } else {
                        self.offset_x = (self.offset_x + delta.x).clamp(-((1440 * DiagramViewer::TIME_PER_PIXEL) as f32), 0.0);
                    } */
                    self.offset_x = (self.offset_x + delta.x).clamp(-((1440 * DiagramViewer::TIME_PER_PIXEL) as f32), 0.0);
                    self.offset_y = (self.offset_y + delta.y).clamp(-((1440 * DiagramViewer::TIME_PER_PIXEL) as f32), 0.0);
                },
                Event::Zoom(zoom) => {
                    self.scale_x = zoom;
                    self.scale_y = zoom;
                },
                _ => {},
            });
            
        });

        CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();

            let station_y = get_station_y(&self.diagram_data, self.display_line_index);

            for i in (0..=24*60).step_by(2) {
                painter.vline((i * DiagramViewer::TIME_PER_PIXEL) as f32 + DiagramViewer::OFFSET_X + self.offset_x, std::ops::RangeInclusive::new(DiagramViewer::OFFSET_Y + self.offset_y, *station_y.last().unwrap() + DiagramViewer::OFFSET_Y + self.offset_y), Stroke {width:0.3, color:Color32::GRAY},);
            }
            for i in (0..=24*60).step_by(10) {
                painter.vline((i * DiagramViewer::TIME_PER_PIXEL) as f32 + DiagramViewer::OFFSET_X + self.offset_x, std::ops::RangeInclusive::new(DiagramViewer::OFFSET_Y + self.offset_y, *station_y.last().unwrap() + DiagramViewer::OFFSET_Y + self.offset_y), Stroke {width:1.0, color:Color32::GRAY},);
            }
            for i in (0..=24*60).step_by(30) {
                painter.vline((i * DiagramViewer::TIME_PER_PIXEL) as f32 + DiagramViewer::OFFSET_X + self.offset_x, std::ops::RangeInclusive::new(DiagramViewer::OFFSET_Y + self.offset_y, *station_y.last().unwrap() + DiagramViewer::OFFSET_Y + self.offset_y), Stroke {width:3.0, color:Color32::GRAY},);
            }
            {
                let mut y = self.offset_y;
                for stn in &self.diagram_data.railway.lines[self.display_line_index].stations {
                    let stroke_width: f32 = if stn.is_main { 3.0 } else { 1.0 };
                    painter.hline(std::ops::RangeInclusive::new(DiagramViewer::OFFSET_X, frame.info().window_info.size.x), y + DiagramViewer::OFFSET_Y, Stroke {width:stroke_width, color:Color32::GRAY},);
                    y += stn.next_station_distance * DiagramViewer::KM_PER_PIXEL as f32;
                }
            }
            
            // draw train
            for updw in [&self.diagram_data.railway.diagrams[self.display_diagram_index].down_trains, &self.diagram_data.railway.diagrams[self.display_diagram_index].up_trains ] {
                for train in updw {
                    let y = self.offset_y;
                    let x = self.offset_x;
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
                            painter.line_segment([
                                Pos2 {x: (before_sta.arrive.unwrap().get_total_minute() * DiagramViewer::TIME_PER_PIXEL) as f32 + x + DiagramViewer::OFFSET_X, y: station_y[before_sta.station_index] + y + DiagramViewer::OFFSET_Y}, 
                                Pos2 {x: (before_sta.departure.unwrap().get_total_minute() * DiagramViewer::TIME_PER_PIXEL) as f32 + x + DiagramViewer::OFFSET_X, y: station_y[before_sta.station_index] + y + DiagramViewer::OFFSET_Y}], 
                                Stroke {width:2.0, color:Color32::BLACK},);
                        }
                        painter.line_segment([
                            Pos2 {x: (time.1.unwrap().get_total_minute() * DiagramViewer::TIME_PER_PIXEL) as f32 + x + DiagramViewer::OFFSET_X, y: station_y[time.0] + y + DiagramViewer::OFFSET_Y}, 
                            Pos2 {x: (time.3.unwrap().get_total_minute() * DiagramViewer::TIME_PER_PIXEL) as f32 + x + DiagramViewer::OFFSET_X, y: station_y[time.2] + y + DiagramViewer::OFFSET_Y}], 
                            Stroke {width:2.0, color:Color32::BLACK},);
                    }
                }
            }
            

            // header background
            {
                let color = ui.style().visuals.window_fill;
                painter.rect(Rect::from_points(&[
                    Pos2 {x: 0.0 , y: 0.0},
                    Pos2 {x: frame.info().window_info.size.x, y: DiagramViewer::OFFSET_Y},
                ]), 0.0, color, Stroke::new(1.0, Color32::BLACK));
                painter.rect(Rect::from_points(&[
                    Pos2 {x: 0.0 , y: 0.0},
                    Pos2 {x: DiagramViewer::OFFSET_X, y: frame.info().window_info.size.y},
                ]), 0.0, color, Stroke::new(1.0, Color32::BLACK));
            }

            // Y header
            for i in (0..24*60).step_by(60) {
                painter.text(Pos2 { x: (i * DiagramViewer::TIME_PER_PIXEL) as f32 + DiagramViewer::OFFSET_X + self.offset_x, y: 0.0 }, Align2::CENTER_TOP, i / 60, FontId::default(), Color32::BLACK);
            }
            
            // X header            
            let mut y = self.offset_y;
            for stn in &self.diagram_data.railway.lines[self.display_line_index].stations {
                let stroke_width: f32 = if stn.is_main { 3.0 } else { 1.0 };
                painter.hline(std::ops::RangeInclusive::new(0.0, DiagramViewer::OFFSET_X), y + DiagramViewer::OFFSET_Y, Stroke {width:stroke_width, color:Color32::GRAY},);
                painter.text(Pos2 {x: 0.0 + 1.0, y: y + 1.0 + DiagramViewer::OFFSET_Y}, Align2::LEFT_TOP, &stn.name, FontId::default(), Color32::BLACK);
                y += stn.next_station_distance * DiagramViewer::KM_PER_PIXEL as f32;
            }
        });
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        let _ = std::fs::write("./export.dia", file::serialize(&self.diagram_data));
    }
}