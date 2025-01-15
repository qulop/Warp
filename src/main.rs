mod networking;
mod capture;
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

#[inline]
fn get_native_window_options(title: String) -> eframe::NativeOptions {
    return eframe::NativeOptions {
        centered: true,
        viewport: egui::ViewportBuilder {
            title: Some(title),
            fullscreen: Some(false),
            resizable: Some(true),
            ..Default::default()
        },
        ..Default::default()
    };
}


fn main() -> Result<(), eframe::Error> {
    set_log_level();
    colog::init();

    let options = get_native_window_options(String::from("Warp")); 
    let title =  options.viewport.clone().title.unwrap_or("Warp".to_string());

    return eframe::run_native(
        "Hello",
        options,
        Box::new(
            |cc| Ok(Box::new(ui::App::new(cc, title)))
    ));
}