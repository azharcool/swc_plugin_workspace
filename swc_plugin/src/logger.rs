use std::fs::OpenOptions;
use std::io::Write;

pub fn log(message: &str) {
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("plugin.log") {
        let _ = writeln!(file, "{}", message);
    }
}