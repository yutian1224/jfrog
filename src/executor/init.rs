use crate::modules::{args::ARGS, logger::init as logger_init};
use knox_libs::logger::Logger;
use lazy_static::lazy_static;
use log::{Log, info};
use parking_lot::RwLock;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use tokio::{
    signal::unix::{SignalKind, signal},
    sync::Notify,
    time::Duration,
};

lazy_static! {
    pub static ref INIT_NOTIFY: Arc<Notify> = Arc::new(Notify::new());
    pub static ref INIT_DONE: AtomicBool = AtomicBool::new(false);
    pub static ref LOGGER: RwLock<Option<&'static Logger>> = RwLock::new(None);
}

#[tokio::main]
pub async fn exec() {
    let logger = logger_init(&ARGS.loglevel, &ARGS.logoutput).await;
    *LOGGER.write() = Some(logger);

    let mut sig_int = signal(SignalKind::interrupt()).unwrap();
    let mut sig_term = signal(SignalKind::terminate()).unwrap();

    tokio::select! {
        _ = async {
            INIT_DONE.store(true, Ordering::Release);
            INIT_NOTIFY.clone().notify_waiters();
            tokio::signal::ctrl_c().await.unwrap();
        } => {},
        _ = sig_int.recv() => {
            info!("logger shutdown");
            exit(0, Some(1));
        },
        _ = sig_term.recv() => {
            info!("logger shutdown");
            exit(0, Some(1));
        },
    }
}

pub fn exit(code: i32, sleep: Option<u64>) {
    LOGGER.read().unwrap().flush();
    if let Some(_s) = sleep {
        std::thread::sleep(Duration::from_secs(_s));
    }
    std::process::exit(code);
}
