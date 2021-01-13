use std::sync::{Arc, Mutex, MutexGuard};

use actix::{Actor, Arbiter, SyncArbiter};
use actix_web::{http, web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

use crate::download;
use crate::download::{DownloadStart, Downloader, Target, YtStatus, YtStatusProgress};

#[derive(Debug, Deserialize)]
pub struct Params {
    param: String,
}

#[derive(Serialize)]
pub struct Status {
    hoge: u32,
}

pub async fn index() -> Result<HttpResponse> {
    Ok(redirect_to("/"))
}

pub async fn status(data: web::Data<Arc<Mutex<crate::Data>>>) -> Result<HttpResponse> {
    let data = &data.lock().unwrap();
    let addr = &data.addr;

    //log::debug!("status");

    if addr.is_none() {
        log::debug!("addr is none");
        return Ok(HttpResponse::Ok().json(Status { hoge: 0 }));
    }
    let addr = addr.as_ref().unwrap();

    if addr.connected() {
        log::debug!("connected");
        let res = addr.send(download::Status {}).await;
        if let Ok(res) = res {
            log::debug!("res");
            let res: Result<YtStatus, _> = res;
            if let Ok(res) = res {
                log::info!("{:?}", res);
                match res.progress() {
                    YtStatusProgress::Finished => {}
                    _ => {}
                }
            }
        } else {
            log::info!("send failed");
        }
    } else {
        log::info!("actor is dead!");
    }

    Ok(HttpResponse::Ok().json(Status { hoge: 0 }))
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

    //let addr = Downloader::start_in_arbiter(&Arbiter::new(), |_| downloader);
    let addr = SyncArbiter::start(3, move || Downloader::new(&params.param));
    addr.do_send(DownloadStart);

    // move addr to Data
    let mut data = data.lock().unwrap();
    data.addr = Some(addr);

    Ok(redirect_to("/"))
}

fn redirect_to(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .header(http::header::LOCATION, location)
        .finish()
}
