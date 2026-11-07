use std::env; // standerd environment library

// checking form that library and return number accordingly and i32 => integer 32 bit
pub fn check_os() -> i32 {
    match env::consts::OS {
        "linux" => 1,
        "windows" => 2,
        "macos" => 3,
        _ => 0,
    }
}
