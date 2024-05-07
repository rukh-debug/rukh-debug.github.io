pub fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.label("Powered by ");
            ui.hyperlink_to("egui", "https://github.com/emilk/egui");
            ui.label(", ");
            ui.hyperlink_to(
                "eframe",
                "https://github.com/emilk/egui/tree/master/crates/eframe",
            );
            ui.label(", ");
            ui.hyperlink_to("cloudflare", "https://cloudflare.com");
            ui.label(" and ");
            ui.hyperlink_to("github pages", "https://pages.github.com/");

            ui.label(".");
        });
        egui::warn_if_debug_build(ui);
    });
}

pub fn footer(ui: &mut egui::Ui) {
    ui.with_layout(egui::Layout::right_to_left(egui::Align::BOTTOM), |ui| {
        ui.horizontal(|ui| {
            ui.label("© Ruben Kharel, 2024");
        });
    });
}

