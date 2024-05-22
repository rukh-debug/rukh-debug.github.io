use crate::custom_widgets::organize_items;
use std::fmt;

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct State {
    resume_page: crate::pages::ResumePage,
    blog_page: crate::pages::BlogPage,
    playground_page: crate::pages::Playground,

    selected_anchor: Anchor,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
enum Anchor {
    Blog,
    Resume,
    Playground, // Clear,
}

#[derive(Clone, Copy, Debug)]
#[must_use]
enum Command {
    Nothing,
}

// ██████╗  ██╗       █████╗  ██╗   ██╗  ██████╗  ██████╗   ██████╗  ██╗   ██╗ ███╗   ██╗ ██████╗
// ██╔══██╗ ██║      ██╔══██╗ ╚██╗ ██╔╝ ██╔════╝  ██╔══██╗ ██╔═══██╗ ██║   ██║ ████╗  ██║ ██╔══██╗
// ██████╔╝ ██║      ███████║  ╚████╔╝  ██║  ███╗ ██████╔╝ ██║   ██║ ██║   ██║ ██╔██╗ ██║ ██║  ██║
// ██╔═══╝  ██║      ██╔══██║   ╚██╔╝   ██║   ██║ ██╔══██╗ ██║   ██║ ██║   ██║ ██║╚██╗██║ ██║  ██║
// ██║      ███████╗ ██║  ██║    ██║    ╚██████╔╝ ██║  ██║ ╚██████╔╝ ╚██████╔╝ ██║ ╚████║ ██████╔╝
// ╚═╝      ╚══════╝ ╚═╝  ╚═╝    ╚═╝     ╚═════╝  ╚═╝  ╚═╝  ╚═════╝   ╚═════╝  ╚═╝  ╚═══╝ ╚═════╝

impl Anchor {
    #[cfg(target_arch = "wasm32")]
    fn all() -> Vec<Self> {
        vec![Self::Blog, Self::Resume]
    }
}

impl fmt::Display for Anchor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<Anchor> for egui::WidgetText {
    fn from(value: Anchor) -> Self {
        Self::RichText(egui::RichText::new(value.to_string()))
    }
}

impl Default for Anchor {
    fn default() -> Self {
        Self::Blog
    }
}

pub struct WrapPages {
    state: State,
}

impl WrapPages {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        #[allow(unused_mut)]
        let mut slf = Self {
            state: State::default(),
        };

        #[cfg(feature = "persistence")]
        if let Some(storage) = cc.storage {
            if let Some(state) = eframe::get_value(storage, eframe::APP_KEY) {
                slf.state = state;
            }
        }
        slf
    }

    fn apps_iter_mut(&mut self) -> impl Iterator<Item = (&str, Anchor, &mut dyn eframe::App)> {
        let vec = vec![
            (
                "Blog",
                Anchor::Blog,
                &mut self.state.blog_page as &mut dyn eframe::App,
            ),
            (
                "Resume",
                Anchor::Resume,
                &mut self.state.resume_page as &mut dyn eframe::App,
            ),
            (
                "Playground",
                Anchor::Playground,
                &mut self.state.playground_page as &mut dyn eframe::App,
            ),
        ];

        vec.into_iter()
    }

    fn show_selected_app(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let selected_anchor = self.state.selected_anchor;
        for (_name, anchor, app) in self.apps_iter_mut() {
            if anchor == selected_anchor || ctx.memory(|mem| mem.everything_is_visible()) {
                app.update(ctx, frame);
            }
        }
    }

    fn bar_contents(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame, _cmd: &mut Command) {
        egui::widgets::global_dark_light_mode_switch(ui);

        ui.separator();

        let mut selected_anchor = self.state.selected_anchor;
        for (name, anchor, _app) in self.apps_iter_mut() {
            if ui
                .selectable_label(selected_anchor == anchor, name)
                .clicked()
            {
                selected_anchor = anchor;
                if frame.is_web() {
                    ui.ctx()
                        .open_url(egui::OpenUrl::same_tab(format!("#{anchor}")));
                }
            }
        }

        ui.separator();

        organize_items(ui);
        self.state.selected_anchor = selected_anchor;
    }
}

impl eframe::App for WrapPages {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.state.blog_page);
        eframe::set_value(storage, eframe::APP_KEY, &self.state.resume_page);
        eframe::set_value(storage, eframe::APP_KEY, &self.state.playground_page);
    }

    fn clear_color(&self, visuals: &egui::Visuals) -> [f32; 4] {
        // Giving the area behind the floating windows a different color, because it looks better:
        let color = egui::lerp(
            egui::Rgba::from(visuals.panel_fill)..=egui::Rgba::from(visuals.extreme_bg_color),
            0.5,
        );
        let color = egui::Color32::from(color);
        color.to_normalized_gamma_f32()
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        #[cfg(target_arch = "wasm32")]
        if let Some(anchor) = frame.info().web_info.location.hash.strip_prefix('#') {

            let parts: Vec<&str> = anchor.split("/").collect();

            let got_anchor = parts.get(0).unwrap_or(&"").to_string();

            let anchor = Anchor::all().into_iter().find(|x| x.to_string() == got_anchor);
            if let Some(v) = anchor {
                self.state.selected_anchor = v;
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::F11)) {
            let fullscreen = ctx.input(|i| i.viewport().fullscreen.unwrap_or(false));
            ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(!fullscreen));
        }

        let mut cmd = Command::Nothing;
        egui::TopBottomPanel::top("wrap_app_top_bar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.visuals_mut().button_frame = false;
                self.bar_contents(ui, frame, &mut cmd);
            });
        });

        self.show_selected_app(ctx, frame);
    }
}

