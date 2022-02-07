use eframe::{
    egui::{self, plot},
    epi,
};

struct QuadraticPlotter {
    data: Data,
    resolution: i16,
}

impl QuadraticPlotter {
    fn new() -> Self {
        Self {
            data: Data {
                a: 1.,
                b: 0.,
                c: 0.,
            },
            resolution: 101,
        }
    }
}

#[derive(Clone, Copy)]
struct Data {
    a: f64,
    b: f64,
    c: f64,
}

impl epi::App for QuadraticPlotter {
    fn name(&self) -> &str {
        "Quadratic Plotter"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(), |ui| {
                    ui.add(
                        egui::DragValue::new(&mut self.data.a)
                            .speed(0.1)
                            .clamp_range(-100..=100)
                            .prefix("a: ")
                            .suffix("x\u{00B2}"),
                    );
                    ui.label("+");
                    ui.add(
                        egui::DragValue::new(&mut self.data.b)
                            .speed(0.1)
                            .clamp_range(-100..=100)
                            .prefix("b: ")
                            .suffix("x"),
                    );
                    ui.label("+");
                    ui.add(
                        egui::DragValue::new(&mut self.data.c)
                            .speed(0.1)
                            .clamp_range(-100..=100)
                            .prefix("c: "),
                    );
                });
                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                    ui.add(
                        egui::Slider::from_get_set(5.0..=501.0, |x| {
                            if let Some(x) = x {
                                self.resolution = (x / 2.).floor() as i16 * 2 + 1;
                            }
                            self.resolution.into()
                        })
                        .fixed_decimals(0)
                        .logarithmic(true)
                        .prefix("Resolution: "),
                    );
                })
            });
            plot::Plot::new("function_plot")
                .data_aspect(1.)
                .center_x_axis(true)
                .show(ui, |plot_ui| {
                    let data = self.data;
                    plot_ui.line(plot::Line::new(plot::Values::from_explicit_callback(
                        move |x| data.a * x.powi(2) + data.b * x + data.c,
                        ..,
                        self.resolution as usize,
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
