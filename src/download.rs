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
                }
                YoutubeDlOutput::Playlist(pl) => {
                    log::info!("downloading playlist...");
                }
            }
        }
    }
}
