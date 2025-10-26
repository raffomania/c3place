use eframe::web::web_location;
use egui::{Align, CentralPanel, Layout, MenuBar, TopBottomPanel, widgets};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    base_url: String,
}

impl Default for App {
    fn default() -> Self {
        let base_url = web_location().origin;
        Self { base_url }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Default::default()
    }
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per
    /// second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`,
        // `Window` or `Area`. For inspiration and more examples, go to https://emilk.github.io/egui

        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            MenuBar::new().ui(ui, |ui| {
                ui.heading("c3nav");

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    widgets::global_theme_preference_switch(ui);
                });
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            egui::Area::new(egui::Id::new("my_area"))
                .constrain(false)
                .show(ctx, |ui| {
                    ui.label("uwu");
                    ui.image(format!("{}/img/owl.jpg", self.base_url));
                });

            ui.with_layout(Layout::bottom_up(Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
