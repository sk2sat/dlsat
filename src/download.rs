use actix::prelude::*;
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

pub struct Downloader {
    pub target: Option<Target>,
    data: usize,
}

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
}

#[derive(Debug)]
pub enum YtStatusProgress {
    Preparing,
    Downloading(f64),
    Finished,
    Error,
}

fn do_youtube_dl(url: &str) {
    //let ctx = Arc::new(ctx);
}
