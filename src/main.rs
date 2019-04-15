extern crate clap;
use std::io::prelude::{ Read, Write };
use std::net::{ TcpListener, TcpStream };
use clap::{App, ArgMatches, Arg};
use std::ops::Add;
use std::fs;



fn index (mut stream: TcpStream) {
    let mut buffer = [0; 512];

    match stream.read(&mut buffer) {
        Err(e) => println!("can't read stream : {}",e),
        _ => (),
    }
    let template = fs::read_to_string("index.html").unwrap_or(String::from("Could not read template")); 
    
    println!("Request : {} ", String::from_utf8_lossy(&buffer[..]));

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}",template);
    println!("{}", response);
    match stream.write(response.as_bytes()) {
        Ok(n) => println!("#{} bytes written", n),
        Err(e) => println!("could not write request : {}", e)
    } 
    
    match stream.flush() {
        Ok(_) => return,
        Err(e) => println!("Error on flushing stream : {}", e)
    }
}

fn get_port<'a>() -> u32 {
    let matches = App::new("web-server-rs")
        .author("AmirrezaAsk")
        .about("Simple HTTP web server in rust")
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .help("port to listen on")
                .takes_value(false),
        ).get_matches();
    let mut port: u32 = 0;
    if matches.occurrences_of("port") != 0 {
        port = matches.value_of("port").unwrap().parse().unwrap();
    } else {
        port = 8080;
    }
    port
    
}


fn main () {
    let port_number = get_port();
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port_number)).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        index(stream);
    }
}