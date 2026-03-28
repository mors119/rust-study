use std::env;

mod asynchronous;
mod command;
mod crawling;
mod encryption;
mod files;
mod matcher;
mod network;
mod thread;
mod web_server;

fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    log::info!("application started");

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 && args[1].as_str() != "axum" {
        log::warn!("this is warn.");
        println!("사용법: cargo run <category> <number> [extra args...]");
        return;
    }

    matcher::run(&args);
}
