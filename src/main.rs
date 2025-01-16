mod networking;
mod client;
mod server;
mod utils;
mod ui;


#[inline(always)]
fn set_log_level() {
    if cfg!(debug_assertions) {
        std::env::set_var("RUST_LOG", "debug");
    } else {
        std::env::set_var("RUST_LOG", "info");
    }
}

#[inline(always)]
fn get_native_window_options(title: String) -> eframe::NativeOptions {
    return eframe::NativeOptions {
        centered: true,
        viewport: egui::ViewportBuilder {
            title: Some(title),
            fullscreen: Some(false),
            resizable: Some(true),
            ..Default::default()
        },
        shader_version: Some(eframe::egui_glow::ShaderVersion::Gl140),
        ..Default::default()
    };
}


#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    set_log_level();
    colog::init();

    const APP_TITLE: &str = "Warp";
    let options = get_native_window_options(String::from(APP_TITLE)); 

    return eframe::run_native(
        APP_TITLE,
        options,
        Box::new(
            |cc| Ok(Box::new(ui::App::new(cc)))
    ));
}