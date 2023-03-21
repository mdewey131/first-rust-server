use std::{
    fs,
    net::{TcpStream, TcpListener},
    io::{prelude::*, BufReader}
};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream)
    }
}

/// The function to read data from the TCP stream and print it, so that we can see the data that's coming to us from the browser
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    if request_line == "GET / HTTP/1.1" {
        let status_line= "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len();

        // To ensure a valid HTTP response, the Content-Length header is added here, which is set to the size of the response body (i.e., the size of hello.html)
        let response = 
            format!("{status_line}\r\nContent-Length:{length}\r\n\r\n{contents}"
        );
        // This returns something! That means that we can actually see a web page here
        stream.write_all(response.as_bytes()).unwrap();

    } else {
        unimplemented!()
    }
    

}