use std::sync::{Arc, Mutex, MutexGuard};

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

async fn do_download(mut data: crate::Data) {
    //let mut data = data.lock().unwrap();
    let mut target = &mut data.target.get_mut(0).unwrap();
    target.download();
}

pub async fn download(
    params: web::Form<Params>,
    data: web::Data<Arc<Mutex<crate::Data>>>,
) -> Result<HttpResponse> {
    log::info!("param: {}", &params.param);

    let target = Target::new(&params.param).unwrap();

    log::info!("target: {:?}", target.s);
    //let target = Arc::new(RwLock::new(target));
    //let t = target.clone();

    let data0 = data.clone();
    let mut data0_lock = data0.lock().unwrap();
    data0_lock.target.push_back(target);

    let pool = &data0_lock.tpool;

    pool.spawn_ok(kusodeka());
    //pool.spawn_ok(do_download(data.clone()));

    pool.spawn_ok(async move {
        //let data: Arc<Mutex<crate::Data>> = data.into_inner();
        do_download(data.lock().unwrap().into());
    });

    //pool.spawn_ok(data1_lock.target.get_mut(0).unwrap().download());
    //pool.spawn_ok(async move {
    //    //let d = t.write().unwrap().download().await;
    //    let _ = target.download().await;
    //});

    //pool.spawn_ok(f);

    //pool.spawn_ok(async move {
    //    loop {
    //        let t = t.read().unwrap();
    //        let status = t.get_status();
    //        log::info!(
    //            "{:?}, {:?}/{:?} tmp: {:?}, out={}",
    //            status.progress(),
    //            status.fragment_index,
    //            status.fragment_count,
    //            status.tmpfilename,
    //            status.filename
    //        );
    //        std::thread::sleep(std::time::Duration::from_millis(50));
    //    }
    //});

    Ok(redirect_to("/"))
}

fn redirect_to(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .header(http::header::LOCATION, location)
        .finish()
}
