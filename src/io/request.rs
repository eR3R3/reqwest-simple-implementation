use std::collections::HashMap;
use std::mem;
use std::time::Duration;
use url::Url;
use crate::io::error::Error;
use crate::io::request::HttpVersion::{All, Http1Only, Http2Only};
use crate::io::request::Method::{Uninitialized, DELETE, GET, POST, PUT};

#[derive(Clone, Debug)]
pub(crate) enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    Uninitialized,
}


#[derive(Clone, Debug)]
pub(crate) enum HttpVersion {
    Http1Only,
    Http2Only,
    All,
}

impl From<&str> for HttpVersion{
    fn from(version: &str) -> Self {
        match version {
            "Http1" => Http1Only,
            "Http2" => Http2Only,
            _ => All,
        }
    }
}

impl From<&str> for Method {
    fn from(method:&str) -> Self {
        match method {
            "Get" => GET,
            "Post" => POST,
            "Put" => PUT,
            "Delete" => DELETE,
            _ => Uninitialized,
        }
    }
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

struct RequestBuilder {
    request: Request,
}


impl RequestBuilder {
    fn new() -> Self {
        RequestBuilder {
            request: Request::new(),
        }
    }

    fn url<T: AsRef<str>>(&mut self, url: T) -> Result<&mut Self, Error> {
        let req = &mut self.request;
        req.url = Url::parse(url.as_ref())?;
        Ok(self)
    }

    fn body<T: Into<String>>(&mut self, body: T) -> Result<&mut Self, Error> {
        let req = &mut self.request;
        req.body = Some(body.into());
        Ok(self)
    }

    fn headers<K: Into<String>, V: Into<String>>(&mut self, headers: HashMap<K, V>) -> Result<&mut Self, Error> {
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
        let req = &mut self.request;
        for (k, v) in headers.iter() {
            req.headers.insert(k.into(), v.into());
        }
        Ok(self)
    }

    fn method<T: AsRef<str> + Into<String>>(&mut self, method:T) -> Result<&mut Self, Error> {
        let req = &mut self.request;
        req.method = method.into().into();
        Ok(self)
    }

    fn version<T: AsRef<str>>(&mut self, version: T) -> Result<&mut Self, Error> {
        let req = &mut self.request;
        req.version = version.into().into();
        Ok(self)
    }

    fn timeout(&mut self, timeout: Duration) -> Result<&mut Self, Error> {
        self.request.timeout = timeout;
        Ok(self)
    }

    fn build(&mut self) -> Result<Request, Error> {
        Ok(Request::from(
            self.request.method.clone(),
            self.request.url.clone(),
            self.request.headers.clone(),
            self.request.body.clone(),
            self.request.timeout,
            self.request.version.clone(),
        ))
    }
}



