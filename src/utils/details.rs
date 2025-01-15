

#[cfg(target_os = "windows")]
pub mod win32 {
    pub fn get_screen_size() -> (u32, u32) {
        todo!();
    }
}


#[cfg(target_os = "linux")]
pub mod unix {
    pub fn get_screen_size() -> (u32, u32) {
        todo!();
    }
}