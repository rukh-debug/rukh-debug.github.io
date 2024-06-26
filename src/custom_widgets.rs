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

pub fn organize_items(ui: &mut egui::Ui) {
    if ui.add(egui::Button::new("Tidy up")).clicked() {
        ui.ctx().memory_mut(|mem| mem.reset_areas());
        ui.close_menu();
    }
}

pub fn separator_size(ui: &mut egui::Ui, large: bool) {
    if large {
        ui.add(egui::Separator::spacing(egui::Separator::default(), 20.0));
    } else {
        ui.add(egui::Separator::shrink(egui::Separator::default(), 50.0));
    }
}

pub fn resume_section_seperator(ui: &mut egui::Ui, title: &str) {
    ui.add_space(10.0);
    ui.separator();
    ui.monospace(title);
    ui.separator();
}

pub fn wrapped_label(ui: &mut egui::Ui, text: &str) {
    ui.add(egui::Label::new(text).wrap(true));
}
