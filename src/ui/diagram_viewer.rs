use eframe::{
    egui::{self, Event, Ui},
    emath::Align2,
    epaint::{Color32, FontId, Pos2, Rect, Stroke},
};

use super::{
    super::model::dia::diafile::DiaFile,
    dia_drawer::{draw_time_line, draw_train, get_station_line, get_station_y},
};

#[derive(Debug)]
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

impl Default for DiagramViewer {
    fn default() -> Self {
        Self {
            display_line_index: 0,
            display_diagram_index: 0,
            offset_x: 0.0,
            offset_y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            window_size_x: 0.0,
            window_size_y: 0.0,
        }
    }
}

impl DiagramViewer {
    /// 1分を何ピクセルで描画するかの初期値
    pub const TIME_PER_PIXEL: f32 = 10.0;
    /// 1km何ピクセルで描画するかの初期値
    pub const KM_PER_PIXEL: f32 = 20.0;

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
                Event::MouseWheel {
                    unit: _,
                    delta,
                    modifiers,
                } => {
                    let left_time = -self.offset_x / DiagramViewer::TIME_PER_PIXEL / self.scale_x;
                    let display_time_range = (now_size.x - DiagramViewer::OFFSET_X)
                        / DiagramViewer::TIME_PER_PIXEL
                        / self.scale_x;
                    let before_center_time = left_time + display_time_range / 2.0;
                    println!("{:?}", before_center_time);

                    if modifiers.ctrl {
                        let get_coeff = |v: f32| -> f32 {
                            let coeff: f32 = 0.1;
                            if v == 0.0 {
                                1.00
                            } else if v.is_sign_positive() {
                                1.0 + coeff
                            } else {
                                1.0 - coeff
                            }
                        };
                        self.scale_x *= get_coeff(delta.x);
                        self.scale_y *= get_coeff(delta.y);
                        let after_x =
                            before_center_time * DiagramViewer::TIME_PER_PIXEL * self.scale_x
                                - (now_size.x - DiagramViewer::OFFSET_X) / 2.0;
                        self.offset_x = -after_x;
                    } else {
                        let tmp: f32 = 0.0;
                        let y: f32 = *get_station_y(diagram_data, self.display_line_index)
                            .last()
                            .unwrap_or(&tmp);
                        self.offset_x = (self.offset_x + delta.x * DiagramViewer::TIME_PER_PIXEL)
                            .clamp(
                                (-1440.0 * DiagramViewer::TIME_PER_PIXEL * self.scale_x
                                    + now_size.x
                                    - DiagramViewer::OFFSET_X)
                                    .min(0.0),
                                0.0,
                            );
                        self.offset_y = (self.offset_y + delta.y * DiagramViewer::TIME_PER_PIXEL)
                            .clamp(
                                (-y * self.scale_y + now_size.y - DiagramViewer::OFFSET_Y).min(0.0),
                                0.0,
                            );
                    }
                }
                _ => {}
            });
        });

        ui.allocate_space(now_size);
        let painter = ui.painter();

        painter.extend(draw_time_line(self, ui, diagram_data));
        painter.extend(get_station_line(self, ui, diagram_data));

        // draw train
        for updw in [
            &diagram_data.railway.diagrams[self.display_diagram_index].down_trains,
            &diagram_data.railway.diagrams[self.display_diagram_index].up_trains,
        ] {
            for train in updw {
                painter.extend(draw_train(self, ui, diagram_data, train));
            }
        }

        // header background
        {
            let color = ui.style().visuals.window_fill;
            painter.rect(
                Rect::from_points(&[
                    Pos2 {
                        x: 0.0 + start_pos.x,
                        y: 0.0 + start_pos.y,
                    },
                    Pos2 {
                        x: now_size.x + start_pos.x,
                        y: DiagramViewer::OFFSET_Y + start_pos.y,
                    },
                ]),
                0.0,
                color,
                Stroke::NONE,
            );
            painter.rect(
                Rect::from_points(&[
                    Pos2 {
                        x: 0.0 + start_pos.x,
                        y: 0.0 + start_pos.y,
                    },
                    Pos2 {
                        x: DiagramViewer::OFFSET_X + start_pos.x,
                        y: now_size.y + start_pos.y,
                    },
                ]),
                0.0,
                color,
                Stroke::NONE,
            );
        }

        // Y header
        {
            for i in (0..24 * 60).step_by(60) {
                painter.text(
                    Pos2 {
                        x: (i as f32) * DiagramViewer::TIME_PER_PIXEL * self.scale_x
                            + DiagramViewer::OFFSET_X
                            + self.offset_x
                            + start_pos.x,
                        y: start_pos.y,
                    },
                    Align2::CENTER_TOP,
                    i / 60,
                    FontId::default(),
                    Color32::BLACK,
                );
            }
        }

        // X header
        {
            let mut y = self.offset_y;
            for stn in &diagram_data.railway.lines[self.display_line_index].stations {
                let stroke_width: f32 = if stn.is_main { 2.0 } else { 0.6 };
                let x = start_pos.x + Self::OFFSET_X / 2.0;

                painter.hline(
                    std::ops::RangeInclusive::new(
                        0.0 + start_pos.x,
                        DiagramViewer::OFFSET_X + start_pos.x,
                    ),
                    y + DiagramViewer::OFFSET_Y + start_pos.y,
                    Stroke {
                        width: stroke_width,
                        color: Color32::GRAY,
                    },
                );
                painter.text(
                    Pos2 {
                        x,
                        y: y + 1.0 + DiagramViewer::OFFSET_Y + start_pos.y - 2.0,
                    },
                    Align2::CENTER_TOP,
                    &stn.name,
                    FontId::default(),
                    Color32::BLACK,
                );
                y += stn.next_station_distance * DiagramViewer::KM_PER_PIXEL * self.scale_y;
            }
        }

        painter.rect(
            Rect::from_points(&[
                Pos2 {
                    x: start_pos.x,
                    y: start_pos.y,
                },
                Pos2 {
                    x: DiagramViewer::OFFSET_X + start_pos.x,
                    y: DiagramViewer::OFFSET_Y + start_pos.y,
                },
            ]),
            0.0,
            ui.style().visuals.window_fill,
            Stroke::NONE,
        );
        painter.hline(
            std::ops::RangeInclusive::new(start_pos.x, start_pos.x + now_size.x),
            DiagramViewer::OFFSET_Y,
            Stroke::new(10.0, Color32::BLACK),
        );
        painter.vline(
            DiagramViewer::OFFSET_X,
            std::ops::RangeInclusive::new(start_pos.y, start_pos.y + now_size.y),
            Stroke::new(10.0, Color32::BLACK),
        );
    }
}
