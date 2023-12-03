use std::{collections::HashMap, sync::Arc};

use eframe::{
    egui::{self, Event, Key, TextureOptions, Ui},
    emath::Align2,
    epaint::{Color32, ColorImage, FontId, Pos2, Rect, Shape, Stroke, TextureId, TextureManager},
};

use crate::model::dia::diagram::Time;

use super::{
    super::model::dia::diafile::DiaFile,
    dia_drawer::{draw_time_line, draw_train, get_station_line, get_station_y},
};

pub struct DiagramViewer {
    pub display_line_index: usize,
    pub display_diagram_index: usize,
    pub offset_x: f32,
    pub offset_y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub window_size_x: f32,
    pub window_size_y: f32,
    pub texture_manager: TextureManager,
    pub day_start_minute: Time,
    pub texture_id: HashMap<String, TextureId>,
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
            texture_manager: TextureManager::default(),
            day_start_minute: Time {
                hour: 4,
                minute: 0,
                second: 0,
            },
            texture_id: HashMap::new(),
        }
    }
}

impl DiagramViewer {
    /// 1分を何ピクセルで描画するかの初期値
    pub const TIME_PER_PIXEL: f32 = 10.0;
    /// 1km何ピクセルで描画するかの初期値
    pub const KM_PER_PIXEL: f32 = 20.0;

    pub const OFFSET_X: f32 = 100.0;
    pub const OFFSET_Y: f32 = 50.0;

    pub fn new() -> Self {
        let decode_png = |raw_bytes: &[u8]| -> Vec<u8> {
            let decoder = png::Decoder::new(raw_bytes);
            let mut reader = decoder.read_info().unwrap();
            let mut buf = vec![0; reader.output_buffer_size()];
            reader.next_frame(&mut buf).unwrap();
            buf
        };
        let get_color_image = |raw_bytes: &[u8]| -> ColorImage {
            println!("{:?}", raw_bytes.len());
            ColorImage::from_rgba_premultiplied([128, 128], &decode_png(raw_bytes))
        };
        let texture_alloc =
            |texture_manager: &mut TextureManager, name: &str, raw_bytes: &[u8]| -> TextureId {
                texture_manager.alloc(
                    String::from(name),
                    eframe::epaint::ImageData::Color(Arc::new(get_color_image(raw_bytes))),
                    TextureOptions::NEAREST,
                )
            };
        let mut result = Self::default();
        result.texture_id.insert(
            String::from("shukko"),
            texture_alloc(
                &mut result.texture_manager,
                "shukko",
                include_bytes!("../../assets/shukko.png"),
            ),
        );
        result.texture_id.insert(
            String::from("nyuko"),
            texture_alloc(
                &mut result.texture_manager,
                "nyuko",
                include_bytes!("../../assets/nyuko.png"),
            ),
        );
        result
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
                            .max(
                                (-1440.0 - self.day_start_minute.get_total_minute() as f32)
                                    * DiagramViewer::TIME_PER_PIXEL
                                    * self.scale_x
                                    + now_size.x
                                    - DiagramViewer::OFFSET_X,
                            )
                            .min(
                                -(self.day_start_minute.get_total_minute() as f32)
                                    * DiagramViewer::TIME_PER_PIXEL
                                    * self.scale_x,
                            );
                        self.offset_y = (self.offset_y + delta.y * DiagramViewer::TIME_PER_PIXEL)
                            .clamp(
                                (-y * self.scale_y + now_size.y - DiagramViewer::OFFSET_Y).min(0.0),
                                0.0,
                            );
                    }
                }
                Event::Key {
                    key,
                    pressed: true,
                    repeat: _,
                    modifiers: _,
                } => match key {
                    Key::ArrowUp => {
                        self.display_diagram_index = (self.display_diagram_index + 1)
                            .clamp(0, diagram_data.railway.diagrams.len() - 1);
                    }
                    Key::ArrowDown => {
                        self.display_diagram_index = (self.display_diagram_index - 1)
                            .clamp(0, diagram_data.railway.diagrams.len() - 1);
                    }
                    _ => {}
                },
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

        let mut shapes = vec![];
        // header background
        {
            let color = ui.style().visuals.window_fill;
            shapes.push(Shape::rect_filled(
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
            ));
            shapes.push(Shape::rect_filled(
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
            ));
        }
        painter.extend(shapes);
        shapes = vec![];

        // Y header
        {
            for i in (0..=24 * 60 + self.day_start_minute.get_total_minute()).step_by(60) {
                painter.text(
                    Pos2 {
                        x: (i as f32) * DiagramViewer::TIME_PER_PIXEL * self.scale_x
                            + DiagramViewer::OFFSET_X
                            + self.offset_x
                            + start_pos.x,
                        y: start_pos.y + DiagramViewer::OFFSET_Y / 2.0,
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

                shapes.push(Shape::hline(
                    std::ops::RangeInclusive::new(
                        0.0 + start_pos.x,
                        DiagramViewer::OFFSET_X + start_pos.x,
                    ),
                    y + DiagramViewer::OFFSET_Y + start_pos.y,
                    Stroke {
                        width: stroke_width,
                        color: Color32::GRAY,
                    },
                ));
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
        {
            shapes.push(Shape::hline(
                std::ops::RangeInclusive::new(start_pos.x, start_pos.x + now_size.x),
                DiagramViewer::OFFSET_Y,
                Stroke::new(2.0, Color32::BLACK),
            ));
            shapes.push(Shape::vline(
                DiagramViewer::OFFSET_X,
                std::ops::RangeInclusive::new(start_pos.y, start_pos.y + now_size.y),
                Stroke::new(2.0, Color32::BLACK),
            ));
        }

        painter.extend(shapes);
    }
}
