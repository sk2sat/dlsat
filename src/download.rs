use url::{Host::Domain, Url};

#[derive(Debug)]
pub enum Target {
    YouTube,
    NicoVideo,
    NicoLive(String),
}

impl Target {
    pub fn new(s: &str) -> Option<Self> {
        let url = Url::parse(s);
        match Url::parse(s) {
            Ok(url) => {
                if let Some(host) = url.host() {
                    match host {
                        Domain("youtube.com") => Some(Target::YouTube),
                        Domain("youtu.be") => Some(Target::YouTube),
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
