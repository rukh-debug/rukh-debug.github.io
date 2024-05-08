#[derive(serde::Deserialize, serde::Serialize)]
pub struct Playground;

impl Default for Playground {
    fn default() -> Self {
        Self
    }
}

impl eframe::App for Playground {
    fn update(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(_ctx, |ui| {
            ui.heading("Playground");
            ui.label("This is a playground page.");
        });
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}
}