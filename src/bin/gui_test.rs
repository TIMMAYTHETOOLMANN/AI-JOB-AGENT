use eframe::egui;

struct MinimalApp;

impl eframe::App for MinimalApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("JARVIS 2.0 - Minimal GUI Test");
            ui.label("If you see this window, eframe/egui is working.");
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "JARVIS 2.0 - Minimal GUI Test",
        native_options,
        Box::new(|_cc| Box::new(MinimalApp)),
    );
}

