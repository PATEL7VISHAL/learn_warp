// use std::fmt::Display;
use warp::log::Info;

#[derive(Debug)]
pub struct RequestHeaderInfo<'a> {
    host: &'a str,
    agent: &'a str,
    // accept: String,
}

impl<'a> RequestHeaderInfo<'a> {
    pub fn new(host: &'a str, agent: &'a str) -> Self {
        Self {
            host,
            agent,
            // accept,
        }
    }
}

pub trait ToString {
    fn to_string(&self) -> String;
}
impl<'a> ToString for Info<'a> {
    fn to_string(&self) -> String {
        let header_info = RequestHeaderInfo::new(
            self.host().unwrap_or("None"),
            self.user_agent().unwrap_or("None"),
        );

        format!(
            "{} {} {} {} micros  from {} with {:?}",
            self.method(),
            self.path(),
            self.status(),
            self.elapsed().as_micros(),
            self.remote_addr().unwrap(),
            // self.request_headers()
            header_info,
        )
    }
}
