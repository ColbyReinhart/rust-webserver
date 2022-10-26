// Rust webserver from scratch (and rust std libraries)
// By Colby Reinhart
// 10-26-2022

use std::
{
	fs,
	io::{prelude::*, BufReader},
	net::{TcpListener, TcpStream}
};

use threadpool::ThreadPool;

// Some global constants
static NUM_THREADS: usize = 10;

fn main()
{
	// Create a listener to listen for incoming TCP connections
	let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

	// Create a thread pool to handle requests
	let pool = ThreadPool::new(NUM_THREADS);
	
	// Handle each incoming connection
	for stream in listener.incoming()
	{
		let stream = stream.unwrap();

		pool.execute(|| {
			handle_connection(stream);
		});
	}
}

fn handle_connection(mut stream: TcpStream)
{
	// Create a buffer reader to read the TCP stream
	let buf_reader = BufReader::new(&mut stream);

	// Process the incoming HTTP request
	let http_request: Vec<_> = buf_reader
		.lines()
		.map(|result| result.unwrap())
		.take_while(|line| !line.is_empty())
		.collect();

	// Output the processed request
	println!("Request: {:#?}", http_request);

	// Prepare the response headers and body
	let status_line = "HTTP/1.1 200 OK";
	let contents = fs::read_to_string("static/hello.html").unwrap();
	let length = contents.len();

	// Format and write the response
	let response =
		format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
	stream.write_all(response.as_bytes()).unwrap();
}
