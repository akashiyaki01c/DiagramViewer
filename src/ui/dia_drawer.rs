use eframe::egui::Ui;
use eframe::epaint::Shape;

use crate::egui::Color32;
use crate::egui::Pos2;
use crate::egui::Stroke;
use crate::model::dia::diafile::DiaFile;
use crate::model::dia::diagram::Time;
use crate::model::dia::diagram::Train;
use crate::model::dia::traintype::LineType;
use crate::ui::diagram_viewer::DiagramViewer;

pub fn get_station_y(diagram_data: &DiaFile, line_index: usize) -> Vec<f32> {
    let mut result = vec![];
    let mut y = 0.0;
    for stn in &diagram_data.railway.lines[line_index].stations {
        result.push(y);
        y += stn.next_station_distance * DiagramViewer::KM_PER_PIXEL;
    }
    result
}

pub fn draw_train(
    viewer: &DiagramViewer,
    ui: &Ui,
    diagram_data: &DiaFile,
    train: &Train,
) -> Vec<Shape> {
    let y = viewer.offset_y;
    let x = viewer.offset_x;
    let x_scale = DiagramViewer::TIME_PER_PIXEL * viewer.scale_x;
    let mut shapes: Vec<Shape> = vec![];
    let start_pos = ui.clip_rect().min;
    let station_y = get_station_y(diagram_data, viewer.display_line_index)
        .iter()
        .map(|v| v * viewer.scale_y)
        .collect::<Vec<f32>>();
    let train_type = traintype_to_stroke(diagram_data, train);
    let mut positions: Vec<Pos2> = vec![];
    for i in 0..train.station_times.len() - 1 {
        let before_sta = &train.station_times[i];
        let next_sta = &train.station_times[i + 1];

        let mut time = if before_sta.departure.is_some() && next_sta.arrive.is_some() {
            (
                before_sta.station_index,
                before_sta.departure,
                next_sta.station_index,
                next_sta.arrive,
            )
        } else if before_sta.departure.is_some() && next_sta.departure.is_some() {
            (
                before_sta.station_index,
                before_sta.departure,
                next_sta.station_index,
                next_sta.departure,
            )
        } else {
            (0, None, 0, None)
        };
        if time.1.unwrap().get_total_minute() < viewer.day_start_minute.get_total_minute() {
            time.1 = Some(Time {
                hour: time.1.unwrap().hour + 24,
                ..time.1.unwrap()
            })
        }
        if time.3.unwrap().get_total_minute() < viewer.day_start_minute.get_total_minute() {
            time.3 = Some(Time {
                hour: time.3.unwrap().hour + 24,
                ..time.3.unwrap()
            })
        }

        if time.1.is_some() {
            positions.push(Pos2 {
                x: (time.1.unwrap().get_total_minute() as f32) * x_scale
                    + x
                    + DiagramViewer::OFFSET_X
                    + start_pos.x,
                y: station_y[before_sta.station_index] + y + DiagramViewer::OFFSET_Y + start_pos.y,
            });
        }
        if time.3.is_some() {
            positions.push(Pos2 {
                x: (time.3.unwrap().get_total_minute() as f32) * x_scale
                    + x
                    + DiagramViewer::OFFSET_X
                    + start_pos.x,
                y: station_y[next_sta.station_index] + y + DiagramViewer::OFFSET_Y + start_pos.y,
            });
        }
        /* if before_sta.arrive.is_some() && before_sta.departure.is_some() {
            shapes.extend(Shape::dashed_line(
                &[
                    Pos2 {
                        x: (before_sta.arrive.unwrap().get_total_minute() as f32) * x_scale
                            + x
                            + DiagramViewer::OFFSET_X
                            + start_pos.x,
                        y: station_y[before_sta.station_index]
                            + y
                            + DiagramViewer::OFFSET_Y
                            + start_pos.y,
                    },
                    Pos2 {
                        x: (before_sta.departure.unwrap().get_total_minute() as f32) * x_scale
                            + x
                            + DiagramViewer::OFFSET_X
                            + start_pos.x,
                        y: station_y[before_sta.station_index]
                            + y
                            + DiagramViewer::OFFSET_Y
                            + start_pos.y,
                    },
                ],
                train_type.0,
                train_type.1,
                train_type.2,
            ));
        }
        shapes.extend(Shape::dashed_line(
            &[
                Pos2 {
                    x: (time.1.unwrap().get_total_minute() as f32) * x_scale
                        + x
                        + DiagramViewer::OFFSET_X
                        + start_pos.x,
                    y: station_y[time.0] + y + DiagramViewer::OFFSET_Y + start_pos.y,
                },
                Pos2 {
                    x: (time.3.unwrap().get_total_minute() as f32) * x_scale
                        + x
                        + DiagramViewer::OFFSET_X
                        + start_pos.x,
                    y: station_y[time.2] + y + DiagramViewer::OFFSET_Y + start_pos.y,
                },
            ],
            train_type.0,
            train_type.1,
            train_type.2,
        )); */
    }
    shapes.extend(Shape::dashed_line(
        &positions,
        train_type.0,
        train_type.1,
        train_type.2,
    ));
    shapes
}

fn arr_to_color32(arr: &[u8; 3]) -> Color32 {
    Color32::from_rgb(arr[0], arr[1], arr[2])
}

fn traintype_to_stroke(diagram_data: &DiaFile, train: &Train) -> (Stroke, f32, f32) {
    let train_type = &diagram_data.railway.train_types[train.train_type_index];
    const DEFAULT_WIDTH: f32 = 1.0;

    match train_type.line_type {
        LineType::Solid => (
            Stroke {
                width: DEFAULT_WIDTH,
                color: arr_to_color32(&train_type.line_color),
            },
            100.0,
            0.0,
        ),
        LineType::Bold(width) => (
            Stroke {
                width,
                color: arr_to_color32(&train_type.line_color),
            },
            100.0,
            0.0,
        ),
        LineType::Dashed(width, dash_length, gap_length) => (
            Stroke {
                width,
                color: arr_to_color32(&train_type.line_color),
            },
            dash_length,
            gap_length,
        ),
    }
}

pub fn draw_time_line(viewer: &DiagramViewer, ui: &Ui, diagram_data: &DiaFile) -> Vec<Shape> {
    let y = viewer.offset_y;
    let x = viewer.offset_x;
    let x_scale = DiagramViewer::TIME_PER_PIXEL * viewer.scale_x;
    let mut shapes: Vec<Shape> = vec![];
    let start_pos = ui.clip_rect().min;
    let station_y = get_station_y(diagram_data, viewer.display_line_index)
        .iter()
        .map(|v| v * viewer.scale_y)
        .collect::<Vec<f32>>();

    let total_min = 24 * 60 + viewer.day_start_minute.get_total_minute();
    let start_min = viewer.day_start_minute.get_total_minute();
    for i in (start_min..=total_min).step_by(2) {
        let line = eframe::epaint::Shape::dashed_line(
            &[
                Pos2 {
                    x: (i as f32) * x_scale + DiagramViewer::OFFSET_X + x + start_pos.x,
                    y: DiagramViewer::OFFSET_Y + y + start_pos.y,
                },
                Pos2 {
                    x: (i as f32) * x_scale + DiagramViewer::OFFSET_X + x + start_pos.x,
                    y: *station_y.last().unwrap() + DiagramViewer::OFFSET_Y + y + start_pos.y,
                },
            ],
            Stroke {
                width: 0.3,
                color: Color32::GRAY,
            },
            10.0,
            5.0,
        );
        shapes.extend(line);
    }
    for i in (start_min..=total_min).step_by(10) {
        shapes.push(Shape::vline(
            (i as f32) * x_scale + DiagramViewer::OFFSET_X + x + start_pos.x,
            std::ops::RangeInclusive::new(
                DiagramViewer::OFFSET_Y + y + start_pos.y,
                *station_y.last().unwrap() + DiagramViewer::OFFSET_Y + y + start_pos.y,
            ),
            Stroke {
                width: 0.6,
                color: Color32::GRAY,
            },
        ));
    }
    for i in (start_min..=total_min).step_by(30) {
        shapes.push(Shape::vline(
            (i as f32) * x_scale + DiagramViewer::OFFSET_X + x + start_pos.x,
            std::ops::RangeInclusive::new(
                DiagramViewer::OFFSET_Y + y + start_pos.y,
                *station_y.last().unwrap() + DiagramViewer::OFFSET_Y + y + start_pos.y,
            ),
            Stroke {
                width: 2.0,
                color: Color32::GRAY,
            },
        ));
    }

    shapes
}

pub fn get_station_line(viewer: &DiagramViewer, ui: &Ui, diagram_data: &DiaFile) -> Vec<Shape> {
    let now_size = ui.available_size();
    let mut shapes: Vec<Shape> = vec![];
    let start_pos = ui.clip_rect().min;
    let station_y = get_station_y(diagram_data, viewer.display_line_index)
        .iter()
        .map(|v| v * viewer.scale_y)
        .collect::<Vec<f32>>();

    #[allow(clippy::needless_range_loop)]
    for i in 0..station_y.len() {
        let stn = &diagram_data.railway.lines[viewer.display_line_index].stations[i];
        let y = station_y[i] + viewer.offset_y;

        let stroke_width: f32 = if stn.is_main { 2.0 } else { 0.6 };
        shapes.push(Shape::hline(
            std::ops::RangeInclusive::new(
                DiagramViewer::OFFSET_X + start_pos.x,
                now_size.x + start_pos.x,
            ),
            y + DiagramViewer::OFFSET_Y + start_pos.y,
            Stroke {
                width: stroke_width,
                color: Color32::GRAY,
            },
        ));
    }

    shapes
}
