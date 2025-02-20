use std::collections::HashMap;
use std::mem;
use std::net::TcpStream;
use std::time::Duration;
use url::Url;
use crate::io::error::Error;
use crate::io::request::HttpVersion::{All, Http1Only, Http2Only};
use crate::io::request::Method::{Uninitialized, DELETE, GET, POST, PUT};
use crate::io::response::Response;

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
    Uninitialized,
}

impl From<&str> for HttpVersion{
    fn from(version: &str) -> Self {
        match version {
            "http1" => Http1Only,
            "http2" => Http2Only,
            "all" => All,
            _ => HttpVersion::Uninitialized,
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

impl Method {
    fn as_str(&self) -> &str {
        match self {
            GET => "GET",
            POST => "POST",
            PUT => "PUT",
            DELETE => "DELETE",
            _ => "Uninitialized",
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Request {
    pub(crate) method: Method,
    pub(crate) url: Url,
    pub(crate) headers: HashMap<String, String>,
    pub(crate) body: Option<String>,
    pub(crate) timeout: Duration,
    pub(crate) version: HttpVersion
}


impl Request {

    pub fn new() -> Self {
        Self {
            method: Uninitialized,
            url: Url::parse("").unwrap(),
            headers: HashMap::new(),
            body: None,
            timeout: Duration::new(30, 0),
            version: HttpVersion::All
        }
    }

    pub fn from(
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

    pub(crate) fn builder() -> RequestBuilder {
        RequestBuilder::new()
    }

    pub fn get_header(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn get_body(&self) -> &Option<String> {
        &self.body
    }

    pub fn get_timeout(&self) -> &Duration {
        &self.timeout
    }

    pub fn get_version(&self) -> &HttpVersion {
        &self.version
    }

    pub fn get_url(&self) -> &Url {
        &self.url
    }

    pub async fn send(&self) -> Result<Response, Error> {
        // Get host and port from URL
        let host = self.url.host_str()
            .ok_or_else(|| Error::new("Missing host in URL"))?;
        let port = self.url.port().unwrap_or(80);

        // Create TCP connection
        let mut stream = TcpStream::connect(format!("{}:{}", host, port))
            .await
            .map_err(|e| Error::new(&format!("Connection failed: {}", e)))?;

        // Build HTTP request
        let mut request_string = format!(
            "{} {} HTTP/1.1\r\n\
             Host: {}\r\n\
             X-Timestamp: {}\r\n\
             X-User-Login: eR3R3\r\n", // Adding required headers
            self.method.as_str(),
            self.url
            host,
            Utc::now().format("%Y-%m-%d %H:%M:%S")
        );

        // Add other headers
        for (name, value) in &self.headers {
            request_string.push_str(&format!("{}: {}\r\n", name, value));
        }

        // Add Content-Length if body is present
        if let Some(body) = &self.body {
            request_string.push_str(&format!("Content-Length: {}\r\n", body.len()));
        }

        // Add empty line to indicate end of headers
        request_string.push_str("\r\n");

        // Add body if present
        if let Some(body) = &self.body {
            request_string.push_str(body);
        }

        // Send request
        stream.write_all(request_string.as_bytes())
            .await
            .map_err(|e| Error::new(&format!("Failed to send request: {}", e)))?;

        // Read response
        let mut response_data = Vec::new();
        let mut buffer = [0; 8192];

        loop {
            let n = stream.read(&mut buffer)
                .await
                .map_err(|e| Error::new(&format!("Failed to read response: {}", e)))?;

            if n == 0 {
                break;
            }

            response_data.extend_from_slice(&buffer[..n]);

            // Check if we've received the complete response
            if response_data.windows(4).any(|window| window == b"\r\n\r\n") {
                break;
            }
        }

        // Parse response
        let response_str = String::from_utf8_lossy(&response_data);
        let mut lines = response_str.lines();

        // Parse status line
        let status_line = lines.next()
            .ok_or_else(|| Error::new("Empty response"))?;
        let status = parse_status_line(status_line)?;

        Ok(Response {
            status,
            body: response_data,
            headers: parse_headers(&response_str)?,
        })
    }
}

fn parse_status_line(line: &str) -> Result<u16, Error> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 3 {
        return Err(Error::new("Invalid status line"));
    }

    parts[1].parse()
        .map_err(|_| Error::new("Invalid status code"))
}

// Helper function to parse headers
fn parse_headers(response: &str) -> Result<HashMap<String, String>, Error> {
    let mut headers = HashMap::new();
    let mut lines = response.lines();

    // Skip status line
    lines.next();

    for line in lines {
        if line.is_empty() || line == "\r" {
            break;
        }

        if let Some((name, value)) = line.split_once(':') {
            headers.insert(
                name.trim().to_string(),
                value.trim().to_string()
            );
        }
    }

    Ok(headers)
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



