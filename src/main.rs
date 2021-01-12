#![warn(clippy::pedantic)]

use std::collections::VecDeque;
use std::env;
use std::sync::{Arc, Mutex};

use futures::executor::ThreadPool;

use env_logger;

use serde::Serialize;

use actix_web::{middleware, web, App, HttpServer};

mod api;
mod config;
mod download;

pub struct Data {
    tpool: ThreadPool,
    status: Status,
}

#[derive(Serialize)]
pub struct Status {
    hoge: u32,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "dlsat=info");
    env_logger::init();
    log::info!("starting...");

    let cfg = config::load("config.toml").unwrap();
    log::info!("config loaded");
    log::info!("workers: {}", cfg.workers);
    log::info!("bind: {}", cfg.bind);

    let data = Data {
        tpool: ThreadPool::new()?,
        status: Status { hoge: 0 },
    };
    let data = Arc::new(Mutex::new(data));

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(data.clone())
            .configure(app_config)
    })
    .workers(cfg.workers)
    .bind(cfg.bind)?
    .run()
    .await
}

fn app_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/status", web::get().to(api::status))
            .route("/download", web::post().to(api::download)),
    )
    .service(
        web::scope("")
            .service(
                actix_files::Files::new("/", "ui/build")
                    .index_file("index.html")
                    .show_files_listing()
                    .use_last_modified(true),
            )
            .default_service(web::route().to(api::index)),
    );
    log::info!("app config done");
}
