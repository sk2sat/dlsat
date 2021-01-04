use std::sync::{Arc, Mutex};

use actix_web::{get, http, middleware, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Deserialize;
use youtube_dl::{YoutubeDl, YoutubeDlOutput};

use crate::download::Host;

#[derive(Debug, Deserialize)]
pub struct Params {
    param: String,
}

pub async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html")))
}

pub async fn download(
    params: web::Form<Params>,
    data: web::Data<Arc<Mutex<crate::Data>>>,
) -> Result<HttpResponse> {
    log::info!("param: {}", &params.param);

    let host = Host::new(&params.param);
    log::info!("host: {:?}", host);

    Ok(redirect_to("/"))
}

fn redirect_to(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .header(http::header::LOCATION, location)
        .finish()
}
