mod executor;
mod modules;

use crate::modules::args::ARGS;
use jemallocator::Jemalloc;
use log::error;
use std::process::exit;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn main() {
    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(2)
        .thread_name(|i| format!("jfrog-{i}"))
        .build()
        .unwrap();

    thread_pool.install(|| {
        rayon::scope(|s: &rayon::Scope| {
            s.spawn(|_| executor::init::exec());
            s.spawn(|_| {
                if let Err(e) =
                    executor::api::init(ARGS.listen.clone(), std::cmp::max(num_cpus::get() / 2, 2))
                {
                    error!("init api in {} failed: {}", ARGS.listen, e);
                    exit(1);
                }
            });
        });
    });
}
