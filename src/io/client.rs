use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use url::Url;
use crate::io::request::{HttpVersion, Method};

impl Method {
    fn as_str(&self) -> &str {
        match self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::DELETE => "DELETE",
            _ => "Uninitialized",
        }
    }
}

#[derive(Clone, Debug)]
struct Config {
    pub(crate) timeout: Option<Duration>,
    pub(crate) connect_timeout: Option<Duration>,
    pub(crate) read_timeout: Option<Duration>,
    pub(crate) write_timeout: Option<Duration>,
    pub(crate) pool_idle_timeout: Option<Duration>,
    pub(crate) pool_max_idle_per_host: usize,
    pub(crate) user_agent: Option<String>,
    pub(crate) follow_redirects: bool,
    pub(crate) max_redirects: u32,
    pub(crate) http_version_pref: HttpVersion,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            timeout: Some(Duration::from_secs(30)),
            connect_timeout: Some(Duration::from_secs(10)),
            read_timeout: Some(Duration::from_secs(30)),
            write_timeout: Some(Duration::from_secs(30)),
            pool_idle_timeout: Some(Duration::from_secs(90)),
            pool_max_idle_per_host: 32,
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
    fn new(config: Config) -> Self {
        Client {
            config: Arc::new(config),
        }
    }

    fn get() {

    }

    fn post() {

    }

    fn put() {

    }

    fn delete() {

    }

    fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    fn request<>()  {

    }

    fn config(&self) -> &Config {
        &self.config
    }
}

struct ClientBuilder {
    config: Config,
    err: Option<Box<dyn Error + Send + Sync>>,
}

impl ClientBuilder {
    fn new() -> Self {
        Self {
            config: Config::default(),
            err: None,
        }
    }

    fn build(&self) -> Client {
        Client::new(self.config.clone())
    }
}