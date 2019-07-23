extern crate webserver;
mod redis_controller;
extern crate elapsed;
use elapsed::measure_time;

use webserver::ThreadPool;
use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::fs::File;
use std::thread;
use std::time::Duration;

fn main() {

	let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
	let pool = ThreadPool::new(4);

	for stream in listener.incoming() {

		let stream = stream.unwrap();

		pool.execute(|| {
			handle_connection(stream);
		});

	}

}

fn handle_connection(mut stream: TcpStream) {

	let mut DIRNAME = "web/".to_string();

	let mut buffer = [0; 512];

	stream.read(&mut buffer).unwrap();

	//define routes
	let get = b"GET / HTTP/1.1\r\n";
	let short = b"GET /short HTTP/1.1\r\n";

	//control routes
	let (status_line, filedata, is_file) = if buffer.starts_with(get) {
		("HTTP/1.1 200 OK\r\n\r\n", format!("{}hello.html", DIRNAME), true)
	}
	else if buffer.starts_with(short) {

		let (elapsed, sum) = measure_time(|| {
			redis_controller::test_run();
		});
		println!("elapsed = {}", elapsed);

		let response = "42".to_string();


		("HTTP/1.1 200 OK\r\n\r\n", format!("{}", response).to_string(), false)
	}
	else {
		("HTTP/1.1 404 NOT FOUND\r\n\r\n", format!("{}404.html", DIRNAME), true)
	};

	//render response
	let mut contents = String::new();

	if( is_file == true ){

		let mut file = File::open(filedata).unwrap();
		file.read_to_string(&mut contents).unwrap();

	}
	else{
		contents = filedata;
	}

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
