use crate::modules::variable::PKG_VERSION;
use clap::{Arg, Command};
use faststr::FastStr;
use lazy_static::lazy_static;
use log::LevelFilter;
use std::{fs, net::SocketAddr, path::Path};

lazy_static! {
    pub static ref ARGS: ArgsResult = parse();
}

pub struct ArgsResult {
    pub listen: String,
    pub loglevel: Option<LevelFilter>,
    pub logoutput: FastStr,
}

impl ArgsResult {
    fn new() -> Self {
        Self {
            listen: String::new(),
            loglevel: None,
            logoutput: FastStr::from_static_str("console"),
        }
    }
}

fn validate_addr(addr: &str) -> Result<String, String> {
    let _: SocketAddr = addr.parse().map_err(|e| format!("{e}"))?;
    Ok(addr.to_string())
}

fn validate_log_path(path: &str) -> Result<String, String> {
    if path == "console" {
        return Ok(path.to_string());
    }
    let p = Path::new(path);
    if !p.exists() {
        match fs::File::create(path) {
            Ok(_) => return Ok(path.to_string()),
            Err(e) => return Err(format!("create log file failed: {e}")),
        }
    }
    match fs::OpenOptions::new().append(true).open(path) {
        Ok(_) => Ok(path.to_string()),
        Err(e) => Err(format!("log file unwriteable: {e}")),
    }
}

pub fn parse() -> ArgsResult {
    let matches = Command::new("jfrog_supervisor")
        .version(PKG_VERSION)
        .about("JFROG supervisor")
        .arg(
            Arg::new("listen")
                .short('l')
                .long("listen")
                .value_name("LISTEN_ADDR")
                .value_parser(validate_addr)
                .help("addr for api")
                .num_args(1)
                .default_value("0.0.0.0:8080"),
        )
        .arg(
            Arg::new("loglevel")
                .long("loglevel")
                .value_name("LOG_LEVEL")
                .value_parser(["default", "trace", "debug", "info", "warn", "error"])
                .help("log level")
                .num_args(1)
                .default_value("default"),
        )
        .arg(
            Arg::new("logoutput")
                .long("logoutput")
                .value_name("LOG_OUTPUT")
                .value_parser(validate_log_path)
                .help("log output")
                .num_args(1)
                .default_value("console")
                .required(false),
        )
        .get_matches();
    let mut result = ArgsResult::new();
    let listen = matches
        .get_one::<String>("listen")
        .unwrap_or(&String::new())
        .to_string();

    let log_level = match matches.get_one::<String>("loglevel").unwrap().as_str() {
        "trace" => Some(LevelFilter::Trace),
        "debug" => Some(LevelFilter::Debug),
        "info" => Some(LevelFilter::Info),
        "warn" => Some(LevelFilter::Warn),
        "error" => Some(LevelFilter::Error),
        _ => None,
    };
    let log_output = matches.get_one::<String>("logoutput").unwrap().to_string();
    result.listen = listen;
    result.loglevel = log_level;
    result.logoutput = log_output.into();
    result
}
