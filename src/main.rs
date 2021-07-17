#![warn(clippy::pedantic)]

use std::env;
use std::sync::{Arc, Mutex};

use env_logger;

use actix_web::{middleware, web, App, HttpServer};

mod api;
mod config;

pub struct Data {}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use clap::Arg;
    let matches = clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .default_value("./config.toml")
                .help("set config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("debug")
                .long("debug")
                .short("d")
                .help("print debug information verbosely"),
        )
        .arg(Arg::with_name("URL").help("URL").required(false).index(1))
        .get_matches();

    let debug_level = if matches.is_present("debug") {
        "debug"
    } else {
        "info"
    };
    std::env::set_var(
        "RUST_LOG",
        format!("{}={}", env!("CARGO_PKG_NAME"), debug_level),
    );
    env_logger::init();
    log::debug!("debug mode");

    // cmdline
    if let Some(url) = matches.value_of("URL") {
        log::info!("URL: {}", url);

        return Ok(());
    }

    log::info!("starting...");

    let cfg = config::load("config.toml").unwrap();
    log::info!("config loaded");
    log::info!("workers: {}", cfg.workers);
    log::info!("bind: {}", cfg.bind);

    let data = Data {};
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
    cfg
        //    .service(
        //        web::scope("/api")
        //            .route("/status", web::get().to(api::status))
        //            .route("/download", web::post().to(api::download)),
        //    )
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
