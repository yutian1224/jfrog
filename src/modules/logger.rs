use faststr::FastStr;
use knox_libs::logger::{
    Logger,
    config::Config,
    filter::ModuleFilter,
    formats::{LoggerFormat, TimeType},
    init as logger_init,
};
use log::LevelFilter;

pub async fn init(level: &Option<LevelFilter>, output: &str) -> &'static Logger {
    let filter_comm = level.unwrap_or(LevelFilter::Info);
    let filter_mod = level.unwrap_or(LevelFilter::Info);
    let mut filters = ModuleFilter::new();
    filters
        .modules
        .insert(FastStr::from_static_str("actix_web"), filter_mod);
    filters
        .modules
        .insert(FastStr::from_static_str("actix_server"), filter_mod);

    let format = LoggerFormat::new()
        .set_display_module_level(filter_comm)
        .set_time_type(TimeType::Utc);

    let config = Config::new().format(format).add_filter(filters);
    match output {
        "console" => logger_init(config.level(filter_comm).console().chan_len(Some(10000)))
            .await
            .unwrap(),
        _ => logger_init(config.level(filter_comm).file(output).chan_len(Some(10000)))
            .await
            .unwrap(),
    }
}
