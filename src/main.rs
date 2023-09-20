use eframe::egui::{self, TopBottomPanel};
use eframe::epaint::Color32;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "RustyDiffusion",
        native_options,
        Box::new(|cc| Box::new(RustyDiffusion::new(cc))),
    );
}

#[derive(Default)]
struct RustyDiffusion {
    errmsg: String,
}

impl RustyDiffusion {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for RustyDiffusion {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.);
                ui.colored_label(Color32::RED, self.errmsg.clone());
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome to RustyDiffusion");
            ui.separator();
            if ui.button("Click me bitch").clicked() {
                match writefile() {
                    Ok(v) => (),
                    Err(e) => self.errmsg = e.to_string().clone(),
                }
            }
            if ui.button("CLICK FOR ERROR").clicked() {
                self.errmsg = "FUCK FUCK FUCK".to_owned();
            }
        });
    }
}

fn writefile() -> std::io::Result<()> {
    let mut file = File::create("foo.txt")?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}
