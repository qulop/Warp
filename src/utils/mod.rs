pub mod result_single;
mod details;


#[macro_export]
macro_rules! ternary {
    ($expression: expr, $f: ty, $s: ty) => {
        if $expression { $f } else { $s };
    }
}

#[macro_export]
#[allow(unused)]
macro_rules! unknown_platform {
    () => {
        panic!("Unknown platform is using");
    };
}


pub fn get_screen_size() -> (u32, u32) {
    #[cfg(target_os = "windows")] {
        return details::win32::get_screen_size()
    }

    #[cfg(target_os = "linux")] {
        details::unix::get_screen_size() 
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux")))] {
        unknown_platform!();
    }
}

