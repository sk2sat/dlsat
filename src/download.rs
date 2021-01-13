use actix::prelude::*;
use futures::executor::ThreadPool;
use futures::stream::once;
use inline_python::python;
use std::sync::{Arc, RwLock};
use std::thread;
use url::{Host::Domain, Url};
use youtube_dl::{YoutubeDl, YoutubeDlOutput};

#[derive(Debug)]
pub enum Host {
    YouTube,
    NicoVideo,
    NicoLive,
}

pub struct Target {
    pub s: String,
    host: Host,
    info: Option<YoutubeDlOutput>,
}

#[derive(Message)]
#[rtype(result = "Result<(), ()>")]
pub struct DownloadStart;

pub struct Downloader {
    pub target: Option<Target>,
    py_ctx: Arc<RwLock<Option<inline_python::Context>>>,
    data: usize,
}

#[derive(Message)]
#[rtype(result = "Result<YtStatus, ()>")]
pub struct Status;

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

impl Downloader {
    pub fn new(s: &str) -> Self {
        Self {
            target: Target::new(s),
            py_ctx: Arc::new(RwLock::new(None)),
            data: 0,
        }
    }

    pub fn download(&mut self) {
        self.data += 1;
        let mut target = self.target.as_mut().unwrap();
        target.get_info();

        if let Some(info) = &target.info {
            match info {
                YoutubeDlOutput::SingleVideo(sv) => {
                    log::info!("downloading single video: {}", sv.title);
                    let url = &target.s;
                    let py_ctx: inline_python::Context = python! {
                        status = "preparing"
                        downloaded_bytes = 0
                        fragment_index = 0
                        fragment_count = 0
                        filename = ""
                        tmpfilename = ""
                        elapsed = 0.0
                        total_bytes = 0.0
                        eta = 0
                        speed = 0.0
                    };
                    self.py_ctx = Arc::new(RwLock::new(Some(py_ctx)));

                    let py_ctx = self.py_ctx.clone();

                    let ctx: &Option<inline_python::Context> = &*py_ctx.read().unwrap();
                    if ctx.is_none() {
                        log::debug!("????????");
                    }
                    let ctx: &inline_python::Context = ctx.as_ref().unwrap();
                    let c = do_youtube_dl(url, ctx);
                }
                YoutubeDlOutput::Playlist(_pl) => {
                    log::info!("downloading playlist...");
                }
            }
        }
    }
}

impl Actor for Downloader {
    type Context = actix::SyncContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        log::info!("downloader started");

        //self.target.download();
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> actix::Running {
        actix::Running::Stop
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        log::info!("downloader stopped");
    }
}

impl Handler<DownloadStart> for Downloader {
    type Result = Result<(), ()>;

    fn handle(&mut self, msg: DownloadStart, ctx: &mut Self::Context) -> Self::Result {
        log::info!("download handler");
        self.download();
        log::info!("download finished!!");

        Ok(())
    }
}

impl Handler<Status> for Downloader {
    type Result = Result<YtStatus, ()>; // <- Message response type

    fn handle(&mut self, msg: Status, ctx: &mut Self::Context) -> Self::Result {
        log::debug!("data: {}", self.data);
        let py_ctx = self.py_ctx.clone();
        let ctx: &Option<inline_python::Context> = &*py_ctx.read().unwrap();
        if ctx.is_none() {
            log::debug!("py context is none!");
            return Err(());
        }
        let ctx = ctx.as_ref().unwrap();
        //let ctx = &py_ctx.as_ref().as_ref().unwrap();
        Ok(ctx.into())
    }
}

impl Target {
    pub fn new(s: &str) -> Option<Self> {
        let host = Host::new(s)?;
        return Some(Self {
            s: s.to_string(),
            host,
            info: None,
        });
    }

    pub fn get_info(&mut self) {
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

    pub fn download(&mut self) {
        //self.get_info();
        //if let Some(info) = &self.info {
        //    match info {
        //        YoutubeDlOutput::SingleVideo(sv) => {
        //            log::info!("downloading single video: {}", sv.title);
        //            let url = &self.s;
        //            let py_ctx: inline_python::Context = python! {
        //                status = "preparing"
        //                downloaded_bytes = 0
        //                fragment_index = 0
        //                fragment_count = 0
        //                filename = ""
        //                tmpfilename = ""
        //                elapsed = 0.0
        //                total_bytes = 0.0
        //                eta = 0
        //                speed = 0.0
        //            };
        //            let py_ctx = Arc::new(Some(py_ctx));
        //            self.py_ctx = py_ctx;

        //            let py_ctx = self.py_ctx.clone();
        //            let pool = ThreadPool::new().unwrap();

        //            //pool.spawn_ok(async move {
        //            //    loop {
        //            //        //ctx.run(python!({ print(status) }));
        //            //        let status: YtStatus = ctx.clone().into();
        //            //        //let status = s.get_status();
        //            //        //log::info!(
        //            //        //    "{:?}, {:?}/{:?} tmp: {:?}, out={}",
        //            //        //    status.progress(),
        //            //        //    status.fragment_index,
        //            //        //    status.fragment_count,
        //            //        //    status.tmpfilename,
        //            //        //    status.filename
        //            //        //);
        //            //        std::thread::sleep(std::time::Duration::from_millis(50));
        //            //    }
        //            //});

        //            let ctx: &Option<inline_python::Context> = &*py_ctx;
        //            let ctx: &inline_python::Context = ctx.as_ref().unwrap();
        //            let c = do_youtube_dl(url, ctx);
        //        }
        //        YoutubeDlOutput::Playlist(_pl) => {
        //            log::info!("downloading playlist...");
        //        }
        //    }
        //}
    }

    //pub fn get_status(&self) -> Option<YtStatus> {
    //    let py_ctx = self.py_ctx.clone();
    //    let ctx = &*py_ctx;
    //    if ctx.is_none() {
    //        log::info!("py context is none!");
    //        return None;
    //    }
    //    let ctx = ctx.as_ref().unwrap();
    //    //let ctx = &py_ctx.as_ref().as_ref().unwrap();
    //    Some(ctx.into())
    //}
}

// https://github.com/ytdl-org/youtube-dl/blob/9c1e164e0cd77331ea4f0b474b32fd06f84bad71/youtube_dl/YoutubeDL.py#L234
//use pyo3::prelude::*;
//#[pyclass(dict)]
#[derive(Debug)]
pub struct YtStatus {
    status: String,
    pub downloaded_bytes: Option<usize>,
    pub fragment_index: Option<usize>,
    pub fragment_count: Option<usize>,
    pub filename: String,
    pub tmpfilename: Option<String>,
    pub elapsed: f64,
    pub total_bytes: Option<f64>,
    pub eta: Option<usize>,
    pub speed: Option<f64>,
}

#[derive(Debug)]
pub enum YtStatusProgress {
    Preparing,
    Downloading(f64),
    Finished,
    Error,
}

fn do_youtube_dl(url: &str, ctx: &inline_python::Context) {
    //let ctx = Arc::new(ctx);

    ctx.run(python! {
        import youtube_dl
        import time

        def phook(d):
            global status
            global downloaded_bytes
            global fragment_index
            global fragment_count
            global filename
            global tmpfilename
            global elapsed
            global total_bytes
            global eta
            global speed
            status = d["status"]
            filename = d["filename"]
            elapsed = d["elapsed"]
            if status == "downloading":
                downloaded_bytes = d["downloaded_bytes"]
                fragment_index = d["fragment_index"]
                fragment_count = d["fragment_count"]
                tmpfilename = d["tmpfilename"]
                total_bytes = d["total_bytes_estimate"]
                eta = d["eta"]
                speed = d["speed"]
                if speed is None:
                    speed = 0.0
            elif status == "finished":
                total_bytes = d["total_bytes"]
            #print(d)

        ret = youtube_dl.YoutubeDL(params={
            "quiet": True,
            "progress_hooks": [lambda d: phook(d)],
        }).download(['url])
    });
}

impl YtStatus {
    pub fn progress(&self) -> YtStatusProgress {
        match &*self.status {
            "preparing" => YtStatusProgress::Preparing,
            "downloading" => YtStatusProgress::Downloading(
                self.downloaded_bytes.unwrap() as f64 / self.total_bytes.unwrap(),
            ),
            "finished" => YtStatusProgress::Finished,
            _ => YtStatusProgress::Error,
        }
    }
}

impl From<&inline_python::Context> for YtStatus {
    fn from(ctx: &inline_python::Context) -> Self {
        let status = ctx.get("status");
        let filename = ctx.get("filename");
        let elapsed = ctx.get("elapsed");
        let (
            downloaded_bytes,
            fragment_index,
            fragment_count,
            tmpfilename,
            total_bytes,
            eta,
            speed,
        ) = if status == "downloading" {
            (
                Some(ctx.get::<usize>("downloaded_bytes")),
                Some(ctx.get::<usize>("fragment_index")),
                Some(ctx.get::<usize>("fragment_count")),
                Some(ctx.get::<String>("tmpfilename")),
                Some(ctx.get::<f64>("total_bytes")),
                Some(ctx.get::<usize>("eta")),
                Some(ctx.get::<f64>("speed")),
            )
        } else if status == "finished" {
            (
                None,
                None,
                None,
                None,
                Some(ctx.get::<usize>("total_bytes") as f64),
                None,
                None,
            )
        } else {
            (None, None, None, None, None, None, None)
        };
        Self {
            status,
            downloaded_bytes,
            fragment_index,
            fragment_count,
            filename,
            tmpfilename,
            elapsed,
            total_bytes,
            eta,
            speed,
        }
    }
}
