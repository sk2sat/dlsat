use futures::executor::ThreadPool;
use inline_python::python;
use std::sync::{Arc, Mutex};
use url::{Host::Domain, Url};
use youtube_dl::{YoutubeDl, YoutubeDlOutput};

#[derive(Debug)]
pub enum Host {
    YouTube,
    NicoVideo,
    NicoLive,
}

pub struct Target {
    s: String,
    host: Host,
    info: Option<YoutubeDlOutput>,
    progress: f32,
}

impl Host {
    pub fn new(s: &str) -> Option<Self> {
        let url = Url::parse(s).unwrap(); //TODO: lm

        if let Some(host) = url.host() {
            match host {
                Domain("www.youtube.com") => Some(Host::YouTube),
                Domain("youtube.com") => Some(Host::YouTube),
                Domain("youtu.be") => Some(Host::YouTube),
                _ => {
                    log::info!("unknown host: {}", host);
                    None
                }
            }
        } else {
            log::info!("no host: {}", url);
            None
        }
    }
}

impl Target {
    pub fn new(s: &str) -> Option<Self> {
        let host = Host::new(s)?;
        return Some(Self {
            s: s.to_string(),
            host,
            info: None,
            progress: 0.0,
        });
    }

    pub async fn get_info(&mut self) {
        match self.host {
            Host::YouTube => {
                let output = YoutubeDl::new(&self.s).socket_timeout("15").run();
                if output.is_err() {
                    return;
                }
                let o = output.unwrap();
                self.info = Some(o);
            }
            _ => {}
        }
    }

    pub async fn download(&mut self) {
        self.get_info().await;
        if let Some(info) = &self.info {
            match info {
                YoutubeDlOutput::SingleVideo(sv) => {
                    log::info!("downloading single video: {}", sv.title);
                    let url = &self.s;
                    do_youtube_dl(url);
                }
                YoutubeDlOutput::Playlist(pl) => {
                    log::info!("downloading playlist...");
                }
            }
        }
    }
}

use inline_python::pyo3::{
    prelude::*, pyclass_slots::PyClassDictSlot, types::IntoPyDict, types::PyDict, wrap_pyfunction,
    FromPyObject, Py, PyClass, PyObject, PyResult, Python,
};

//use pyo3::prelude::*;
#[pyclass(dict)]
//#[derive(Clone)]
struct YtStatus {
    status: String,
    downloaded_bytes: usize,
    fragment_index: usize,
    fragment_count: usize,
    filename: String,
    tmpfilename: String,
    elapsed: f64,
    total_bytes_estimate: f64,
    eta: usize,
    speed: f64,
    _eta_str: String,
    _percent_str: String,
    _speed_str: String,
    _total_bytes_estimate_str: String,
}

//use pyo3::prelude::*;
//use pyo3::wrap_pyfunction;
#[pyfunction]
fn phook_rs(py: pyo3::Python, d: &PyDict) {
    println!("hook");
    //let d: &YtStatus = d.into();
}

fn do_youtube_dl(url: &str) {
    //let mut percent = String::new();
    let ctx: inline_python::Context = python! {
        status = {}
        downloaded = 0
    };
    let ctx = Arc::new(ctx);
    //let percent0 = Arc::new(percent);

    {
        // progress monitor thread
        let pool = ThreadPool::new().unwrap();
        let ctx = ctx.clone();
        //let p = percent0.clone();
        pool.spawn_ok(async move {
            loop {
                //log::info!("{}", p);
                //log::info!("{}", ctx.get::<i32>("downloaded"));
                //log::info!("{}", ctx.get::<String>("percent"));
                //let gil = Python::acquire_gil();
                //let py = gil.python();
                //let status: PyObject = ctx.get::<PyObject>("status").extract(py).unwrap();
                //let status: PyDict = status.extract(py).unwrap();
                //let s = ctx.get::<YtStatus>("status").into();
                //log::info!("{}", s);
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        });
    }

    ctx.add_wrapped(wrap_pyfunction!(phook_rs));

    ctx.run(python! {
        import youtube_dl
        import time

        def phook(d):
            global status
            global downloaded
            status = d
            downloaded = d["downloaded_bytes"]
            //'percent = d["_percent_str"]
            #print(time.time())

        ret = youtube_dl.YoutubeDL(params={
            "quiet": True,
            "progress_hooks": [lambda d: phook_rs(d)],
        }).download(['url])
    });
}
