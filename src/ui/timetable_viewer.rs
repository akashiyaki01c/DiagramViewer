use eframe::egui::{Context, Ui};
use egui::{Event, Key, ScrollArea};
use egui_extras::{Column, TableBuilder};

use crate::model::dia::diafile::DiaFile;

#[derive(Default)]
pub struct TimetableViewer {
    pub display_line_index: usize,
    pub display_diagram_index: usize,
    pub display_direction_is_down: bool,
}

impl TimetableViewer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, ctx: &Context, ui: &mut Ui, diagram_data: &DiaFile) {
        ui.push_id("timetable-vew", |_ui| {});

        ctx.input(|i| {
            i.events.iter().for_each(|v| match *v {
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
                    Key::U => {
                        self.display_direction_is_down = false;
                    }
                    Key::D => {
                        self.display_direction_is_down = true;
                    }
                    _ => {}
                },
                _ => {}
            });
        });

        let target_trains = if self.display_direction_is_down {
            &diagram_data.railway.diagrams[self.display_diagram_index].down_trains
        } else {
            &diagram_data.railway.diagrams[self.display_diagram_index].up_trains
        };
        let station_count = diagram_data.railway.lines[self.display_line_index]
            .stations
            .len();
        let train_count = target_trains.len();
        ScrollArea::both().show(ui, |ui| {
            TableBuilder::new(ui)
                .striped(true)
                // .resizable(true)
                .column(Column::auto())
                .columns(
                    Column::auto().at_least(20.0 * 2.0).at_most(20.0 * 2.0),
                    train_count,
                )
                .header(20.0, |mut ui| {
                    ui.col(|ui| {
                        ui.label("");
                    });
                    for i in 0..train_count {
                        ui.col(|ui| {
                            ui.vertical_centered(|ui| {
                                let train_type = &diagram_data.railway.train_types
                                    [target_trains[i].train_type_index];
                                ui.label(String::from(&train_type.name));
                            });
                        });
                    }
                })
                .body(|mut ui| {
                    let mut func = |i: usize| {
                        ui.row(20.0, |mut ui| {
                            ui.col(|ui| {
                                ui.vertical_centered(|ui| {
                                    ui.label(
                                        &diagram_data.railway.lines[self.display_line_index]
                                            .stations[i]
                                            .name,
                                    );
                                });
                            });
                            for train in target_trains {
                                ui.col(|ui| {
                                    ui.vertical_centered(|ui| {
                                        if let Some(time) =
                                            train.station_times.iter().find(|train| {
                                                train.line_index == self.display_line_index
                                                    && train.station_index == i
                                            })
                                        {
                                            if let Some(time) = time.departure {
                                                ui.label(time.to_string());
                                            }
                                        } else {
                                            ui.label("･･･");
                                        }
                                    });
                                });
                            }
                        });
                    };

                    if self.display_direction_is_down {
                        for i in 0..station_count {
                            func(i);
                        }
                    } else {
                        for i in (0..station_count).rev() {
                            func(i);
                        }
                    }
                });
        });
    }
}
