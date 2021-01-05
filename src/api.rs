use std::sync::{Arc, Mutex};

use actix_web::{http, web, HttpResponse, Result};
use serde::Deserialize;

use crate::download::Target;

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

    let mut target = Target::new(&params.param);

    if let Some(mut t) = target {
        //log::info!("target: {:?}", t);
        let pool = &data.lock().unwrap().tpool;
        pool.spawn_ok(kusodeka());
        pool.spawn_ok(async move {
            t.download().await;
        });
    }

    Ok(redirect_to("/"))
}

fn redirect_to(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .header(http::header::LOCATION, location)
        .finish()
}
