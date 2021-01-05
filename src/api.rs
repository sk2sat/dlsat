use std::sync::{Arc, Mutex};

use actix_web::{get, http, middleware, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Deserialize;
use youtube_dl::{YoutubeDl, YoutubeDlOutput};

use crate::download::Target;

#[derive(Debug, Deserialize)]
pub struct Params {
    param: String,
}

pub async fn index() -> Result<HttpResponse> {
    Ok(redirect_to("/"))
}

pub async fn download(
    params: web::Form<Params>,
    data: web::Data<Arc<Mutex<crate::Data>>>,
) -> Result<HttpResponse> {
    log::info!("param: {}", &params.param);

    let target = Target::new(&params.param);
    log::info!("target: {:?}", target);

    Ok(redirect_to("/"))
}

fn redirect_to(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .header(http::header::LOCATION, location)
        .finish()
}
