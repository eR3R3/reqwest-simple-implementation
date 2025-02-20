use std::collections::HashMap;
use std::ops::Deref;
use super::error::{ClientError, Error, ErrorKind, RequestError};
use std::sync::Arc;
use std::time::Duration;
use crate::io::request::{HttpVersion, Method};
use crate::io::request::Request;



#[derive(Clone, Debug)]
struct Config {
    pub(crate) timeout: Option<Duration>,
    // pub(crate) connect_timeout: Option<Duration>,
    // pub(crate) read_timeout: Option<Duration>,
    // pub(crate) write_timeout: Option<Duration>,
    // pub(crate) pool_idle_timeout: Option<Duration>,
    // pub(crate) pool_max_idle_per_host: usize,
    pub(crate) user_agent: Option<String>,
    pub(crate) follow_redirects: bool,
    pub(crate) max_redirects: u32,
    pub(crate) http_version_pref: HttpVersion,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            timeout: Some(Duration::from_secs(30)),
            // connect_timeout: Some(Duration::from_secs(10)),
            // read_timeout: Some(Duration::from_secs(30)),
            // write_timeout: Some(Duration::from_secs(30)),
            // pool_idle_timeout: Some(Duration::from_secs(90)),
            // pool_max_idle_per_host: 32,
            user_agent: Some(format!("rust-http-client/{}", env!("CARGO_PKG_VERSION"))),
            follow_redirects: true,
            max_redirects: 10,
            http_version_pref: HttpVersion::All,
        }
    }
}

#[derive(Clone, Debug)]
struct Client {
    config: Arc<Config>,
}

impl Client {
    pub fn new() -> Self {
        Client {
            config: Arc::new(Config::default()),
        }
    }

    pub fn from(config: Config) -> Self {
        Client {
            config: Arc::new(config),
        }
    }

    pub fn get(&self,
               url: &str,
               headers: &HashMap<String, String>,
               timeout: Option<Duration>,
               version: &str) -> Request {
        let req = self.request(
            "Get",
            url,
            headers,
            timeout,
            version,
        );
        req
    }


    pub fn post(&self,
                url: &str,
                headers: &HashMap<String, String>,
                body: Option<&str>,
                timeout: Option<Duration>,
                version: &str) -> Request {
        let req = self.request(
            "Post",
            url,
            headers,
            body,
            timeout,
            version,
        );
        req
    }

    pub fn put(&self,
               url: &str,
               headers: &HashMap<String, String>,
               body: Option<&str>,
               timeout: Option<Duration>,
               version: &str) -> Request {
        let req = self.request(
            "Put",
            url,
            headers,
            body,
            timeout,
            version,
        );
        req
    }

    pub fn delete(&self,
                  url: &str,
                  headers: &HashMap<String, String>,
                  timeout: Option<Duration>,
                  version: &str) ->Request {
        let req = self.request(
            "Delete",
            url,
            headers,
            timeout,
            version,
        );
        req
    }

    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub fn request(
        method: &str,
        url: &str,
        headers: &HashMap<String, String>,
        body: Option<&str>,
        timeout: Option<Duration>,
        version: &str) -> Request {
        Request::from(
            method.into(),
            url.into(),
            headers.clone(),
            body.into(),
            timeout.unwrap_or_else(|| Duration::from_secs(30)),
            version.into(),
        )
    }

    pub fn request_builder() {
        Request::builder();
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

struct ClientBuilder {
    config: Config,
}

impl ClientBuilder {
    // pub(crate) follow_redirects: bool,
    // pub(crate) max_redirects: u32,
    // pub(crate) http_version_pref: HttpVersion,
    fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }

    fn http_version_pref<T: AsRef<str>>(&mut self, http_version: T) -> Result<&mut Self, Error> {
        match http_version.as_ref().into() {
            HttpVersion::Uninitialized => Err(Box::new(ErrorKind::Client(ClientError::InvalidHttpVersion))),
            _ => {
                self.config.http_version_pref = http_version.as_ref().into();
                Ok(self)
            },
        }
    }

    fn follow_redirects(&mut self, follow_redirects: bool) -> &mut Self {
        self.config.follow_redirects = follow_redirects;
        self
    }

    fn max_redirects(&mut self, max_redirects: u32) -> &mut Self {
        self.config.max_redirects = max_redirects;
        self
    }

    fn timeout(&mut self, time: Duration) -> Result<&mut Self, Error> {
        if time <= Duration::from_secs(10) {
            return Err(Box::new(ErrorKind::Request(RequestError::InvalidTimeout)));
        }
        self.config.timeout = Some(time);
        Ok(self)
    }

    fn user_agent(&mut self, user_agent: String) -> &mut Self {
        self.config.user_agent = Some(user_agent);
        self
    }

    fn build(&self) -> Client {
        Client::from(self.config.clone())
    }
}