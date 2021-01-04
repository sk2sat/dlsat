use std::env;
use std::sync::{Arc, Mutex};

use futures::executor::{block_on, ThreadPool};

use env_logger;

use serde::{Deserialize, Serialize};

use actix_web::http::StatusCode;
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder, Result};

mod api;
mod config;
mod download;

pub struct Data {
    tpool: ThreadPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    log::info!("starting...");

    let cfg = config::load("config.toml").unwrap();
    log::info!("config loaded");
    log::info!("workers: {}", cfg.workers);
    log::info!("bind: {}", cfg.bind);

    let data = Data {
        tpool: ThreadPool::new()?,
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
        web::scope("")
            .service(web::resource("/").route(web::get().to(api::index)))
            .service(web::resource("/download").route(web::post().to(api::download)))
            .default_service(web::route().to(api::index)),
    );
    log::info!("app config done");
}
