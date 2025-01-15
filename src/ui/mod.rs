#[derive(Default)]
#[allow(unused)]
pub struct App {
    title: String,
    passcode_buffer: String,
    room_id_buffer: String
}

impl App {
    pub fn new(cc: &eframe::CreationContext, title: String) -> Self {
        cc.egui_ctx.set_theme(egui::Theme::Dark);

        return Self {
            title,
            passcode_buffer: String::new(),
            room_id_buffer: String::new()
        };
    }

    pub fn base_frame(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Enter room passcode:");

            ui.add(
                egui::TextEdit::singleline(
                    &mut self.passcode_buffer)
                        .hint_text("passcode"));
        });

        ui.horizontal(|ui| {
            ui.label("Enter room ID:");
            ui.add(
                egui::TextEdit::singleline(
                    &mut self.room_id_buffer)
                        .hint_text("room ID")
            );
        });

        if ui.button("Connect").clicked() {
            println!("Connect button clicled");
        }

        ui.separator();

        if ui.button("Share your screen").clicked() {
            println!("Share button clicled");
        }
    }
}


impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.base_frame(ui);
        });
    }
}
