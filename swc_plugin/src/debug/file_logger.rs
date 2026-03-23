use simplelog::*;
use std::fs::File;
use std::sync::OnceLock;
use time::macros::format_description;

static LOGGER: OnceLock<()> = OnceLock::new();

pub fn init_logger() {
    LOGGER.get_or_init(|| {
        let config = ConfigBuilder::new()
            .set_time_format_custom(format_description!("[hour]:[minute]:[second]"))
            .set_thread_level(LevelFilter::Off)
            .set_target_level(LevelFilter::Off)
            .set_level_padding(LevelPadding::Right)
            .build();

        WriteLogger::init(
            LevelFilter::Debug,
            config,
            File::create("plugin_debugging.log").unwrap(),
        )
        .unwrap();
    });
}
