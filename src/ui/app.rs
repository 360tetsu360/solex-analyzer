use std::sync::Arc;

use eframe::egui;
use egui_plot::{Line, PlotPoints};

use crate::asi::asi_api::ASICameraInfo;

#[derive(Clone, Copy)]
pub enum ConnectionStatus {
    Connected,
    Connecting,
    Disconnecting,
    Unconnected,
}

#[derive(Clone)]
pub struct ASIStatus {
    pub connected_cams: Vec<ASICameraInfo>,
    pub connection_status: ConnectionStatus,
}

#[derive(Clone)]
pub struct SolEXStatus {
    pub connection_status: ConnectionStatus,
}

pub struct App {
    asi_status: Arc<ASIStatus>,
    solex_status: Arc<SolEXStatus>,
    fake_bool: bool,
}

impl App {
    pub fn new() -> Self {
        let asi_status = Arc::new(ASIStatus {
            connected_cams: vec![],
            connection_status: ConnectionStatus::Unconnected,
        });

        let solex_status = Arc::new(SolEXStatus {
            connection_status: ConnectionStatus::Unconnected,
        });

        Self {
            asi_status,
            solex_status,
            fake_bool: false,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("Top").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    use egui::special_emojis::GITHUB;
                    ui.hyperlink_to(
                        format!("{GITHUB} source code on GitHub"),
                        "https://github.com/360tetsu360",
                    );
                });
            });
        });

        egui::SidePanel::left("Left")
            .resizable(true)
            .default_width(300.)
            .max_width(1000.)
            .min_width(250.)
            .show(ctx, |ui| {
                egui::CollapsingHeader::new(
                    egui::RichText::new("Camera Settings").font(egui::FontId::proportional(20.0)),
                )
                .default_open(true)
                .show(ui, |ui| {
                    ui.heading("Camera 📷");
                    ui.add_space(5.);

                    ui.horizontal_wrapped(|ui| {
                        ui.label("Camera");

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                            let is_selectable = matches!(
                                self.asi_status.connection_status,
                                ConnectionStatus::Unconnected
                            );

                            let mut a = 0;
                            ui.add_enabled_ui(is_selectable, |ui| {
                                egui::ComboBox::from_id_source("combo_cam").show_ui(ui, |ui| {
                                    for cam in self.asi_status.connected_cams.iter() {
                                        ui.selectable_value(&mut a, cam.camera_id, &cam.name);
                                    }
                                    ui.style_mut().wrap = Some(false);
                                    ui.set_min_width(60.0);
                                });
                            });

                            let connected = match self.asi_status.connection_status {
                                ConnectionStatus::Connected => "Disconnect ⏹",
                                ConnectionStatus::Connecting => "Connecting",
                                ConnectionStatus::Disconnecting => "Disconnecting",
                                ConnectionStatus::Unconnected => "Connect ▶",
                            };

                            let is_enabled = matches!(
                                self.asi_status.connection_status,
                                ConnectionStatus::Connected | ConnectionStatus::Unconnected
                            );

                            ui.add_enabled_ui(is_enabled, |ui| {
                                if ui.button(connected).clicked() {
                                    self.fake_bool = !self.fake_bool;
                                }
                            });

                            if !is_enabled {
                                ui.add(egui::Spinner::new());
                            }
                        })
                    });

                    ui.separator();
                    ui.heading("Image");
                    ui.add_space(5.);

                    ui.horizontal_wrapped(|ui| {
                        ui.label("Binning");
                        ui.add_enabled_ui(self.fake_bool, |ui| {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                egui::ComboBox::from_id_source("combo_bin").show_ui(ui, |ui| {
                                    ui.style_mut().wrap = Some(false);
                                    ui.set_min_width(60.0);
                                });
                            })
                        });
                    });

                    ui.separator();
                    ui.heading("Control");
                    ui.add_space(5.);
                    ui.horizontal_wrapped(|ui| {
                        ui.label("Exposure");
                        ui.add_enabled_ui(self.fake_bool, |ui| {
                            ui.vertical(|ui| {
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::TOP),
                                    |ui| {
                                        egui::ComboBox::from_id_source("combo_exp").show_ui(
                                            ui,
                                            |ui| {
                                                ui.style_mut().wrap = Some(false);
                                                ui.set_min_width(60.0);
                                            },
                                        );
                                    },
                                );

                                ui.add_space(5.);

                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::TOP),
                                    |ui| {
                                        let mut scalar = 0;
                                        ui.add(egui::Slider::new(&mut scalar, 0..=360));
                                    },
                                );
                            });
                        });
                    });

                    ui.add_space(5.);

                    ui.horizontal_wrapped(|ui| {
                        ui.label("Gain");
                        ui.add_enabled_ui(self.fake_bool, |ui| {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                let mut scalar = 0;
                                ui.add(egui::Slider::new(&mut scalar, 0..=500));
                            });
                        });
                    });
                });

                ui.add_space(5.);
                ui.separator();
                ui.add_space(5.);

                egui::CollapsingHeader::new(
                    egui::RichText::new("Sol'EX Settings").font(egui::FontId::proportional(20.0)),
                )
                .default_open(true)
                .show(ui, |ui| {
                    ui.heading("Sol'EX");
                    ui.add_space(5.);
                    ui.horizontal_wrapped(|ui| {
                        ui.label("Sol'EX");

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                            egui::ComboBox::from_id_source("combo_com").show_ui(ui, |ui| {
                                ui.style_mut().wrap = Some(false);
                                ui.set_min_width(60.0);
                            });

                            let connected = match self.solex_status.connection_status {
                                ConnectionStatus::Connected => "Disconnect ⏹",
                                ConnectionStatus::Connecting => "Connecting",
                                ConnectionStatus::Disconnecting => "Disconnecting",
                                ConnectionStatus::Unconnected => "Connect ▶",
                            };

                            let is_enabled = matches!(
                                self.solex_status.connection_status,
                                ConnectionStatus::Connected | ConnectionStatus::Unconnected
                            );

                            ui.add_enabled_ui(is_enabled, |ui| {
                                if ui.button(connected).clicked() {
                                    self.fake_bool = !self.fake_bool;
                                }
                            });

                            if !is_enabled {
                                ui.add(egui::Spinner::new());
                            }
                        })
                    });

                    ui.separator();
                    ui.heading("Ne-Ar Lamp 💡");
                    ui.add_space(5.);

                    ui.horizontal_wrapped(|ui| {
                        ui.label("ON/OFF");
                        ui.add_enabled_ui(self.fake_bool, |ui| {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                let connected = if !self.fake_bool { "ON" } else { "OFF" };

                                if ui.button(connected).clicked() {
                                    self.fake_bool = !self.fake_bool;
                                }
                            })
                        })
                    });

                    ui.separator();
                    ui.heading("Wavelength Selector ⚙");
                    ui.add_space(5.);

                    ui.horizontal_wrapped(|ui| {
                        ui.label("Sensor angle");
                        ui.add_enabled_ui(self.fake_bool, |ui| {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                ui.label("30°");
                            })
                        })
                    });

                    ui.add_space(5.);

                    ui.horizontal_wrapped(|ui| {
                        ui.label("Calculated wavelength");
                        ui.add_enabled_ui(self.fake_bool, |ui| {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                ui.label("589nm");
                            })
                        })
                    });

                    ui.add_space(5.);

                    ui.horizontal_wrapped(|ui| {
                        ui.label("Rotation speed");
                        ui.add_enabled_ui(self.fake_bool, |ui| {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                let mut scalar = 0;
                                ui.add(egui::Slider::new(&mut scalar, 1..=8));
                            })
                        })
                    });

                    ui.add_space(5.);

                    ui.horizontal_wrapped(|ui| {
                        ui.label("Rotate");
                        ui.add_enabled_ui(self.fake_bool, |ui| {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                if ui.button("⏵").clicked() {}
                                if ui.button("⏴").clicked() {}
                            })
                        })
                    });

                    ui.add_space(5.);

                    ui.horizontal_wrapped(|ui| {
                        ui.label("Rotate to");
                        ui.add_enabled_ui(self.fake_bool, |ui| {
                            ui.vertical(|ui| {
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::TOP),
                                    |ui| {
                                        let mut scalar = 0;
                                        ui.add(
                                            egui::Slider::new(&mut scalar, 0..=360).suffix("nm"),
                                        );
                                    },
                                );
                                ui.add_space(5.);
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::TOP),
                                    |ui| {
                                        if ui.button("Rotate").clicked() {}
                                    },
                                );
                            });
                        });
                    });
                });

                ui.add_space(5.);
                ui.separator();
                ui.add_space(5.);

                ui.add_enabled_ui(self.fake_bool, |ui| {
                    egui::CollapsingHeader::new(
                        egui::RichText::new("Calibration").font(egui::FontId::proportional(20.0)),
                    )
                    .default_open(false)
                    .show(ui, |ui| {
                        ui.add_space(5.);
                        ui.horizontal_wrapped(|ui| {
                            ui.label("Reference spectrum");

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                egui::ComboBox::from_id_source("combo_spe").show_ui(ui, |ui| {
                                    ui.style_mut().wrap = Some(false);
                                    ui.set_min_width(60.0);
                                });
                            })
                        });
                        ui.add_space(5.);

                        ui.horizontal_wrapped(|ui| {
                            ui.label("Start");

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                if ui.button("Start").clicked() {}
                            })
                        });
                    })
                });

                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
            });

        egui::CentralPanel::default().show(ctx, |_ui| {});

        egui::TopBottomPanel::bottom("bottom")
            .resizable(true)
            .default_height(300.)
            .max_height(900.)
            .min_height(100.)
            .show(ctx, |ui| {
                ui.add_space(5.);
                ui.heading("Plot");

                let line_points: PlotPoints = (0..=100)
                    .map(|i| {
                        use std::f64::consts::TAU;
                        let x = egui::remap(i as f64, 0.0..=100_f64, -TAU..=TAU);
                        [x, (x.sin().cos() * 10. * x.powi(2).tan()).sqrt()]
                    })
                    .collect();
                let line = Line::new(line_points);
                egui_plot::Plot::new("example_plot")
                    .height(290.)
                    .show_axes(false)
                    .data_aspect(1.0)
                    .show(ui, |plot_ui| plot_ui.line(line));

                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
            });
    }
}
