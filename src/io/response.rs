use std::collections::HashMap;

#[derive(Debug)]
pub struct Response {
    pub status: u16,
    pub body: Vec<u8>,
    pub headers: HashMap<String, String>,
}