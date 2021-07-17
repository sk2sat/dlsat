use actix_web::{http, HttpResponse, Result};

pub async fn index() -> Result<HttpResponse> {
    Ok(redirect_to("/"))
}

fn redirect_to(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .header(http::header::LOCATION, location)
        .finish()
}
