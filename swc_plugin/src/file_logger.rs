use simplelog::*;
use std::fs::File;

pub fn init_logger() {
    let _ = WriteLogger::init(
        LevelFilter::Debug,
        Config::default(),
        File::create("plugin_debug.log").unwrap(),
    );
}