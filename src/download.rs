use url::{Host::Domain, Url};

#[derive(Debug)]
pub enum Host {
    YouTube,
    NicoVideo,
    NicoLive(String),
}

impl Host {
    pub fn new(s: &str) -> Option<Self> {
        let url = Url::parse(s);
        match Url::parse(s) {
            Ok(url) => {
                if let Some(host) = url.host() {
                    match host {
                        Domain("youtube.com") => Some(Host::YouTube),
                        Domain("youtu.be") => Some(Host::YouTube),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            Err(e) => None,
        }
    }
}
