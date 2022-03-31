#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console on Windows

use eframe::egui::plot::{Line, Plot, Values};
use eframe::egui::{CentralPanel, Context, DragValue, Layout, Slider};
use eframe::epi::{App, Frame};

struct QuadraticPlotter {
    data: Data,
    quality: i16,
}

impl QuadraticPlotter {
    fn new() -> Self {
        Self {
            data: Data {
                a: 1.,
                b: 0.,
                c: 0.,
            },
            quality: 101,
        }
    }
}

#[derive(Clone, Copy)]
struct Data {
    a: f64,
    b: f64,
    c: f64,
}

impl App for QuadraticPlotter {
    fn name(&self) -> &str {
        "Quadratic Plotter"
    }

    fn update(&mut self, ctx: &Context, _: &Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(Layout::left_to_right(), |ui| {
                    ui.add(
                        DragValue::new(&mut self.data.a)
                            .speed(0.1)
                            .clamp_range::<f64>(-100.0..=100.0)
                            .prefix("a: ")
                            .suffix("x\u{00B2}"),
                    );
                    ui.label("+");
                    ui.add(
                        DragValue::new(&mut self.data.b)
                            .speed(0.1)
                            .clamp_range::<f64>(-100.0..=100.0)
                            .prefix("b: ")
                            .suffix("x"),
                    );
                    ui.label("+");
                    ui.add(
                        DragValue::new(&mut self.data.c)
                            .speed(0.1)
                            .clamp_range::<f64>(-100.0..=100.0)
                            .prefix("c: "),
                    );
                });
                ui.with_layout(Layout::right_to_left(), |ui| {
                    ui.add(
                        Slider::from_get_set(5.0..=501.0, |x| {
                            if let Some(x) = x {
                                self.quality = (x / 2.).floor() as i16 * 2 + 1;
                            }
                            self.quality.into()
                        })
                        .fixed_decimals(0)
                        .logarithmic(true)
                        .prefix("Quality: "),
                    );
                })
            });
            Plot::new("function_plot")
                .data_aspect(1.)
                .center_x_axis(true)
                .show(ui, |plot_ui| {
                    let data = self.data;
                    plot_ui.line(Line::new(Values::from_explicit_callback(
                        move |x| data.a * x.powi(2) + data.b * x + data.c,
                        ..,
                        self.quality as usize,
                    )))
                });
        });
    }
}

fn main() {
    eframe::run_native(
        Box::new(QuadraticPlotter::new()),
        eframe::NativeOptions::default(),
    );
}
