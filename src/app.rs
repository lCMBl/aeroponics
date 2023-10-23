use egui::Visuals;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    // label: String,

    // #[serde(skip)] // This how you opt-out of serialization of a field
    // value: f32,
    mister_on_time: f32,
    mister_off_time: f32,
    mist_status: bool,
    timer_status: bool,
    show_timer_confirm: bool,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            mister_on_time: 3.0,
            mister_off_time: 180.0,
            mist_status: false,
            timer_status: false,
            show_timer_confirm: false,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        cc.egui_ctx.set_visuals(Visuals {
            dark_mode: true,
            ..Default::default()
        });

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            _frame.close();
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::Window::new("Misting Control").show(ctx, |ui| {
            ui.label("Controls the solenoid to the misters on a set schedule.");
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("On time:");
                ui.add_enabled(!self.timer_status, egui::Slider::new(&mut self.mister_on_time, 1.0..=10.0).text("seconds"));
            });
            ui.horizontal(|ui| {
                ui.label("Off time:");
                ui.add_enabled(!self.timer_status, egui::Slider::new(&mut self.mister_off_time, 60.0..=300.0).text("seconds"));
            });
            ui.separator();

            let mist_status_text = if self.mist_status { "On" } else { "Off" }; 
            ui.heading(format!("Mister Status: {}", mist_status_text));
            ui.separator();

            let next_cycle_status = if self.mist_status { "Off" } else { "On" }; 
            ui.label(format!("Time till next {} cycle: {:02}:{:02}",
                next_cycle_status,
                3,
                0
            ));
            ui.separator();

            ui.horizontal(|ui| {
                ui.heading("Timer Status:");
                if self.timer_status {
                    ui.label(egui::RichText::new("On").heading().color(egui::Color32::from_rgb(87, 165, 171)));
                } else {
                    ui.label(egui::RichText::new("Off").heading().color(egui::Color32::from_rgb(255, 100, 100)));
                }
                // red: 255, 100, 100
                // green: 87, 165, 171
            });
            ui.horizontal(|ui| {
                if ui.button("Turn On").clicked() {
                    self.timer_status = true;
                }

                if ui.button("Turn Off").clicked() {
                    self.show_timer_confirm = true;
                }
            });
         });

         if self.show_timer_confirm {
             egui::Window::new("Timer Change Confirmation").show(ctx, |ui| {
                ui.label("Are you sure you wish to turn off the misting timer? Plants will dry out and die rapidly without active misting.");
                ui.separator();
                ui.with_layout(
                    egui::Layout::left_to_right(egui::Align::TOP),
                    // .with_cross_align(egui::Align::Center),
                |ui| {
                    if ui.button("Turn Off").clicked() {
                        self.timer_status = false;
                        self.show_timer_confirm = false;
                    }
    
                    if ui.button("Cancel").clicked() {
                        self.show_timer_confirm = false;
                    }
                });
             });
         }

    }
}
