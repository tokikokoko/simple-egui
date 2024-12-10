#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use std::string;

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "egui example: global font style",
        options,
        Box::new(|cc| Ok(Box::new(SimpleApp::new(cc)))),
    )
}

struct SimpleApp {
    counter: u64,
    input: String,
    dpi: f32,
}

impl SimpleApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            counter: 0,
            input: "".to_string(),
            dpi: 1.5,
        }
    }

    fn reset(&mut self) {
        self.counter = 0;
        self.input = "".to_string();
        self.dpi = 1.5;
    }
}

impl eframe::App for SimpleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(self.dpi);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Simple app");

            if ui.button("Reset").clicked() {
                self.reset();
            };

            if ui.button("Increment").clicked() {
                self.counter = self.counter + 1;
            };

            ui.text_edit_multiline(&mut self.input);
            
            ui.label(format!("Counter: {}", self.counter));
            ui.label(format!("Input: {}", self.input));

            ui.heading("Setting");
            ui.add(egui::Slider::new(&mut self.dpi, 0.5..=3.0).text("dpi"));
        });
    }
}
