mod frames;


pub struct App {
    active_frame: Box<dyn frames::EguiFrame>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_theme(egui::Theme::Dark);

        return Self {
            active_frame: Box::new(frames::StartScreen::default()),
        };
    }


    fn draw_current_frame(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if let Some(new_frame) = self.active_frame.show_frame(ui, ctx, frame) {
            use frames::FrameState;
            
            self.active_frame = match new_frame {
                FrameState::ViewerFrame(connection) => Box::new(frames::ViewerScreen::new(connection)),
                FrameState::SharerFrame(server) => Box::new(frames::HostScreen::new(server)),
                FrameState::StartFrame =>  Box::new(frames::StartScreen::default()),
            }
        }
    }
}


impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_current_frame(ui, ctx, frame);
        });
    }
}
