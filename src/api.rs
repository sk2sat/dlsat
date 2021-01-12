use std::sync::{Arc, Mutex, MutexGuard};

use actix::{Actor, Arbiter};
use actix_web::{http, web, HttpResponse, Result};
use serde::Deserialize;

use crate::download::{Downloader, Status, YtStatus, YtStatusProgress};

#[derive(Debug, Deserialize)]
pub struct Params {
    param: String,
}

pub async fn index() -> Result<HttpResponse> {
    Ok(redirect_to("/"))
}

pub async fn status(data: web::Data<Arc<Mutex<crate::Data>>>) -> Result<HttpResponse> {
    let status = &data.lock().unwrap().status;
    Ok(HttpResponse::Ok().json(status))
}

async fn kusodeka() {
    log::info!("kusodeka start");
    std::thread::sleep(std::time::Duration::from_secs(10));
    log::info!("kusodeka end");
}

pub async fn download(
    params: web::Form<Params>,
    data: web::Data<Arc<Mutex<crate::Data>>>,
) -> Result<HttpResponse> {
    log::info!("param: {}", &params.param);

    let downloder = Downloader::new(&params.param).unwrap();

    let addr = Downloader::start_in_arbiter(&Arbiter::new(), |_| downloder);

    let mut data0_lock = data.lock().unwrap();

    let pool = &data0_lock.tpool;

    pool.spawn_ok(async move {
        loop {
            let res = addr.send(Status {}).await.unwrap();
            if let Ok(res) = res {
                log::info!("{:?}", res);
                match res.progress() {
                    YtStatusProgress::Finished => break,
                    _ => continue,
                }
            }
        }
    });

    Ok(redirect_to("/"))
}

fn redirect_to(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .header(http::header::LOCATION, location)
        .finish()
}
