use std::sync::{Arc, Mutex};

use eframe::Frame;
use tokio::runtime::Runtime;

use crate::client::ClientConnection;
use crate::server::Server;


#[derive(Debug)]
pub enum FrameState {
    StartFrame,
    SharerFrame(Server),
    ViewerFrame(ClientConnection)
}

pub trait EguiFrame {
    fn show_frame(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, frame: &mut eframe::Frame) -> Option<FrameState>;
}



#[derive(Default)]
pub struct StartScreen {
    host_data_buffer: String,
    passcode_buffer: String,
    error_message: String,
    is_loading: bool,
    share_error_message: Arc<Mutex<String>>, 
}

impl StartScreen {
    async fn try_connect(&self) -> Option<FrameState> {
        if self.host_data_buffer.len() == 0 || self.passcode_buffer.len() == 0 {
            return None;
        }

        let connection =  ClientConnection::new(&self.host_data_buffer).await?;
        connection.try_connect(&self.passcode_buffer).await?;

        return Some(FrameState::ViewerFrame(connection));
    }

    fn handle_connect_button() {

    } 
}

impl EguiFrame for StartScreen {
    fn show_frame(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, frame: &mut eframe::Frame) -> Option<FrameState> {
        ui.horizontal(|ui| {
            ui.label("Enter connection data:");
            ui.add(
                egui::TextEdit::singleline(&mut self.host_data_buffer)
                    .hint_text("IP:Port"),
            );
        });

        ui.horizontal(|ui| {
            ui.label("Enter room passcode:");
            ui.add(
                egui::TextEdit::singleline(&mut self.passcode_buffer)
                    .hint_text("passcode"),
            );
        });

        ui.horizontal(|ui| {
            if ui.button("Connect").clicked() {
                self.is_loading = true;
                self.error_message.clear();
                let host_data = self.host_data_buffer.clone();
                let passcode = self.passcode_buffer.clone();
                let ctx = ctx.clone();

                // Используем tokio::spawn для асинхронного выполнения
                tokio::spawn(async move {
                    let connection = self.try_connect().await;
                    if let Some(_) = connection {
                        // Успешное подключение
                    } else {
                        self.error_message = "Failed to connect".to_string();
                    }
                    self.is_loading = false;
                    ctx.request_repaint();
                });
            }

            // Отображение ошибки, если она есть
            if !self.error_message.is_empty() {
                ui.label(self.error_message.as_str());
            }
        });

        if self.is_loading {
            ui.add(egui::Spinner::new());
        }

        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Start sharing").clicked() {
                let ctx = ctx.clone();
                let share_error_message = self.share_error_message.clone();

                // Выполнение Server::new().await синхронно
                let runtime = Runtime::new().unwrap();
                runtime.block_on(async {
                    if Server::new().await.is_none() {
                        let mut error_message = share_error_message.lock().unwrap();
                        *error_message = "Failed to share your screen: failed to create a server".to_string();
                    }
                    ctx.request_repaint();
                });
            }

            // Отображение ошибки, если она есть
            let error_message = self.share_error_message.lock().unwrap();
            if !error_message.is_empty() {
                ui.label(error_message.as_str());
            }
        });

        None
    }
}



pub struct ViewerScreen {
    connection: ClientConnection,
}

impl ViewerScreen {
    pub fn new(connection: ClientConnection) -> Self {
        return Self {
            connection
        };
    }
}

impl EguiFrame for ViewerScreen {
    fn show_frame(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, frame: &mut eframe::Frame) -> Option<FrameState> {
        ui.label("ViewerScreen");

        if ui.button("Disconect").clicked() {
            return Some(FrameState::StartFrame);
        }

        return None;
    }
}



pub struct HostScreen {
    server: Server
}

impl HostScreen {
    pub fn new(server: Server) -> Self {
        return Self {
            server
        }
    }
}

impl EguiFrame for HostScreen {
    fn show_frame(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, frame: &mut eframe::Frame) -> Option<FrameState> {
        ui.label("HostScreen");
        
        if ui.button("Go back").clicked() {
            return Some(FrameState::StartFrame);
        }

        return None;
    }
}