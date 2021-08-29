use log::{error, info};
use std::io::prelude::{Read, Write};
use std::path::Path;
use std::io::Error;
use std::net::{TcpListener, TcpStream};
use std::fs;

struct Request<'a> {
	method 	: &'a str,
	uri		: &'a Path,
	http_version : &'a str,
}

impl <'a> std::fmt::Display for Request<'a> {
	fn fmt(&self, f : &mut  std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{} {} {}\r \n",
			self.method,
			self.uri.display(),
			self.http_version
		)
	}
}

fn main()  {
	simple_logger::init().unwrap();
	info!("Starting Server ... ");

	let ip =  "192.168.137.3:8954";
	let listener = TcpListener::bind(ip).expect("Unable to Create Listerner");
	info!("Server started on : {} {}", "http://", ip);

	for stream in listener.incoming() {
		match stream {
			Ok(stream) => match handle_connection(stream) {
				Ok(_) => (),
				Err(e) => error!("Error handling connection: {}", e),

			},
			Err(e) => error!("Connection failed: {}", e),
		}
	}
}

fn handle_connection(mut stream : TcpStream ) -> Result<(), Error> {
	// buffer 
	let mut buffer = [0; 512];

	stream.read(&mut buffer).unwrap();

	let request = String::from_utf8_lossy(&buffer[..]);
	let request_line = request.lines().next().unwrap();

	match parse_request_line(&request_line) {
		Ok(request) => {
			info!("Request {}", &request);

			let contents = fs::read_to_string("/home/christopher/sources/rust/rust-playground/resources/index.html").unwrap();
			let response = format!("{}{}", "HTTP/1.1 200 OK\r\n\r\n", contents);

			info!("Response : {}", &response);
			stream.write(response.as_bytes()).unwrap();
			stream.flush().unwrap();	
		}
		Err(_) => error!("Badly formatted request: {}", &request_line),
	}

	Ok(())
}

fn parse_request_line(request : & str) -> Result<Request, Box<dyn std::error::Error>> {
	let mut parts = request.split_whitespace();

	let method = parts.next().ok_or("Method not specified")?;
	// we only accept get requests
	if method != "GET" {
		Err("Unsupported method")?;
	}

	let uri = Path::new(parts.next().ok_or("Uri not specified")?);
	let nrom_uri =  uri.to_str().expect("Invalid unicode !");

	const ROOT: &str = "/home/christopher/sources/rust/rust-playground/resources";

	if !Path::new(&format!("{}{}", ROOT, nrom_uri)).exists() {
		Err("Requested resource does not exits")?;
	}

	let http_version = parts.next().ok_or("HTTP version not specified")?;
	if http_version != "HTTP/1.1" {
		Err("Unsupported HTTP version, use HTTP/1.1")?;
	}

	Ok(Request {
		method,
		uri,
		http_version,
	}
	)
}