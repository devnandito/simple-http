use std::{
  io::{self, Read, Write},
  net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream}
};

use simple_http::http::request;

fn create_socket() -> SocketAddr {
    SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::LOCALHOST), 5500)
}

fn handle_client(stream: &mut TcpStream) -> io::Result<()> {
    let mut buffer = [0;1024];
    stream.read(&mut buffer)?;

    let buf_str = String::from_utf8_lossy(&buffer);
    let request = request::HttpRequest::new(&buf_str);
    let response = request?.response()?;
    
    println!("{:?}", response);
    println!("{}", &response.response_body);
    
    let body = response.response_body.clone();
    
    stream.write(&mut body.as_bytes())?;
    // let mut file = "";
    // let mut begin_html = r#"
    // <!DOCTYPE html> 
    // <html> 
    // <head> 
    //     <meta charset="utf-8"> 
    // </head> 
    // <body>"#.to_string();

    // let mut header =
    //                 format!("<h1>Currently in {}</h1>", file.to_string_lossy()).into_bytes();

    // // Build your response here

    // let mut end_html = r#"
    // </body>
    // </html>"#.to_string();

    // let valid_response = "HTTP/2 200\ncontent-type: text/html\nvary: Accept-Encoding\r\n\r\n\
    // <html>
    // <body>
    // <h1> Hello World!</h1>
    // </body>
    // </html>";
    // let stringified_buffer = String::from_utf8_lossy(&buffer);
    // print!("HTTP response?\n{}", stringified_buffer);
    // stream.write(&mut valid_response.as_bytes())?;
    stream.flush()?;
    Ok(())

}

fn serve(socket: SocketAddr) -> io::Result<()> {
    let listener = TcpListener::bind(socket)?;
    let mut counter = 0;
    for stream in listener.incoming() {
     match std::thread::spawn(|| handle_client(&mut stream?)).join(){
        Ok(_) => {
            counter += 1;
            print!("Connected stream....{}", counter);
        }
        Err(_) => continue,
     };
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let socket: SocketAddr = create_socket();
    serve(socket)?;
    Ok(())
}