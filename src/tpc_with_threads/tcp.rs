use std::io::{prelude::*, BufWriter, BufReader};
use std::net::TcpStream;
use std::time::Duration;
use std::{net::TcpListener, thread};
use std::{str, fs};



fn process_connection(mut stream: TcpStream){
    println!("un nuevo cliente conectado desde {:?}", stream);
    let mut buf_reader = BufReader::new(&stream);
    //thread::sleep(Duration::from_secs(10));
    let mut final_line = String::new();
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("index.html").unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    println!("termina cliente {:?}", stream)

}

pub fn start_server(){
    let listener = TcpListener::bind("127.0.0.1:9123").unwrap();
    println!("listening started, ready to accept");
    for stream in listener.incoming() {
        thread::spawn(|| {
            let mut stream = stream.unwrap();
            process_connection(stream);
        });
    }
}