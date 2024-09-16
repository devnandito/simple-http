use std::collections::btree_set::Difference;
use std::fmt::format;
use std::fmt::Display;
use std::fs;
use std::io;
use super::request::Version;
use super::request::HttpRequest;

#[derive(Debug)]
pub struct HttpResponse {
    version: Version,
    status: ResponseStatus,
    content_length: usize,
    accept_ranges: AcceptRanges,
    pub response_body: String,
    pub current_path: String
}

impl HttpResponse {
    pub fn new(request: &HttpRequest) -> io::Result<HttpResponse> {
        let version: Version = Version::V2_0;
        let mut status: ResponseStatus = ResponseStatus::NotFound;
        let mut content_length: usize = 0;
        let mut accept_ranges: AcceptRanges = AcceptRanges::None;
        let current_path:String = request.resource.path.clone();
        let mut response_body = String::new();
        let server_root_path = std::env::current_dir()?;
        let resource = request.resource.path.clone();
        let new_path = server_root_path.join(resource);
        let diretory = fs::read_dir(server_root_path.clone()).unwrap();
        if new_path.exists() {
            if new_path.is_file() {
                let content = std::fs::read_to_string(&new_path)?;
                content_length = content.len();
                status = ResponseStatus::Ok;
                accept_ranges = AcceptRanges::Bytes;
                let content = format!("{} {}\n{}\ncontent-length: {}\r\n\r\n{}", version, status, accept_ranges, content_length, content);
                response_body.push_str(&content)
            } else {
                let rootcwd_len = server_root_path.canonicalize()?.components().count();
                let resource = server_root_path.join(&request.resource.path);
                let resource_len = resource.canonicalize()?.components().count();
                if rootcwd_len <= resource_len {
                    println!("Hola ===== {:?}", resource);
                } else {
                    println!("Chau ======={:?}", new_path.clone());
                }
                let mut begin_html = r#"
                <!DOCTYPE html> 
                <html> 
                <head> 
                    <meta charset="utf-8"> 
                </head> 
                <body>"#.to_string();

                let mut header =format!("<h1>Currently in {}</h1>", server_root_path.to_string_lossy());
                // Build your response here
                let mut dir: Vec<String> = Vec::new();
                for item in diretory {
                    let value = item.unwrap().path().display().to_string();
                    let s = String::from(value);
                    let last = s.split("/").last().unwrap();
                    let val = format!("<a href='{}'>{}</a>", last, s);
                    dir.push(val);
                    // p = format!("<p>{:?}</p>", item.unwrap().path().display().to_string());
                };

                let mut result = String::new();
                for p in dir {
                    result = format!("<p>{}</p><p>{}</p>",result, p);
                }
                let mut end_html = r#"
                </body>
                </html>"#.to_string();
                let body = format!("{} {:?} {:?} {}", begin_html, header, result, end_html);
            
                let content_length = body.len();
                // let four_o_four = "
                // <html>
                // <body>
                // <h1>404 NOT FOUND</h1>
                // </body>
                // </html>";
                // let content_length = four_o_four.len();
                let content = format!("{} {}\n{}\ncontent-length: {}\r\n\r\n{}", version, status, accept_ranges, content_length, body);
                response_body.push_str(&content);
            }
        }
        Ok(
            HttpResponse{version, status, content_length, accept_ranges, response_body, current_path}
        )
    }
}

#[derive(Debug)]
enum ResponseStatus {
    Ok = 200,
    NotFound = 404,
}

impl Display for ResponseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            ResponseStatus::Ok => "200 OK",
            ResponseStatus::NotFound => "404 NOT FOUND"
        };
        write!(f, "{}", msg)
    }
}

#[derive(Debug)]
enum AcceptRanges {
    Bytes,
    None
}

impl Display for AcceptRanges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            AcceptRanges::Bytes => "accept-ranges: bytes",
            AcceptRanges::None => "accept-ranges:None",
        };
        write!(f, "{}", msg)
    }
}