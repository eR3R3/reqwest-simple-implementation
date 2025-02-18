use std::collections::HashMap;
use std::time;
use std::time::Duration;
use url::Url;
use crate::io::request::Method::Uninitialized;

#[derive(Clone, Debug)]
pub(crate) enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    Uninitialized,
}

#[derive(Clone, Debug)]
pub(crate) enum Error {
    Builder,
    Timeout,

}

#[derive(Clone, Debug)]
pub(crate) enum HttpVersion {
    Http1Only,
    Http2Only,
    All,
}

#[derive(Clone, Debug)]
struct Request {
    method: Method,
    url: Url,
    headers: HashMap<String, String>,
    body: Option<String>,
    timeout: Duration,
    version: HttpVersion
}

struct RequestBuilder {
    request: Result<Request, Error>,
}

impl Request {
    fn new() -> Self {
        Self {
            method: Uninitialized,
            url: Url::parse("").unwrap(),
            headers: HashMap::new(),
            body: None,
            timeout: Duration::new(30, 0),
            version: HttpVersion::All
        }
    }

    fn from(
        method: Method,
        url: Url,
        headers: HashMap<String, String>,
        body: Option<String>,
        timeout: Duration,
        version: HttpVersion) -> Self {
        Self {
            method,
            url,
            headers,
            body,
            timeout,
            version,
        }
    }

    fn builder() -> RequestBuilder {
        RequestBuilder::new()
    }

    fn get_header(&self) -> &HashMap<String, String> {
        &self.headers
    }

    fn get_body(&self) -> &Option<String> {
        &self.body
    }

    fn get_timeout(&self) -> &Duration {
        &self.timeout
    }

    fn get_version(&self) -> &HttpVersion {
        &self.version
    }

    fn get_url(&self) -> &Url {
        &self.url
    }
}

impl RequestBuilder {
    // url: Url,
    // headers: HashMap<String, String>,
    // body: Option<String>,
    // timeout: Duration,
    // version: HttpVersion
    fn new() -> Self {
        RequestBuilder {
            request: Ok(Request::new()),
        }
    }

    fn url<T: Into<String> + Into<&str>>(&mut self, url: T) -> &mut Self {

    }

    fn headers<K: Into<String>, V: Into<String>>(&mut self, headers: HashMap<K, V>) -> &mut Self {
        // match &self.request {
        //     Err(err) => {
        //         self.request = Err(err.clone());
        //         &mut self
        //     },
        //     Ok(ref mut req) => {
        //         req.headers = headers;
        //         &mut self
        //     }
        // }
        if let Ok(req) = &mut self.request {
            req.headers = headers;
        } else {

        }
        self
    }
}



