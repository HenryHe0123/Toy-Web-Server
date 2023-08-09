use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, thread};
use webserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    let request_limit = 10;

    for stream in listener.incoming().take(request_limit) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let home = b"GET / HTTP/1.1\r\n";
    let hello = b"GET /hello HTTP/1.1\r\n";
    let sjtu = b"GET /sjtu HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let favicon = b"GET /favicon.ico HTTP/1.1\r\n";

    if buffer.starts_with(favicon) {
        //favicon.ico is the icon of the website (binary file), which is requested by the browser automatically.
        let contents = fs::read(format!("webserver/html/favicon.ico")).unwrap();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: image/x-icon\r\nContent-Length: {}\r\n\r\n",
            contents.len()
        );
        let mut response = response.as_bytes().to_vec();
        response.extend_from_slice(&contents);
        stream.write_all(response.as_slice()).unwrap();
        stream.flush().unwrap();
        return;
    }

    let (status_line, filename, wait) = if buffer.starts_with(home) {
        ("HTTP/1.1 200 OK", "hello.html", 0)
    } else if buffer.starts_with(sjtu) {
        ("HTTP/1.1 200 OK", "sjtu.html", 0)
    } else if buffer.starts_with(hello) {
        ("HTTP/1.1 200 OK", "hello.html", 0)
    } else if buffer.starts_with(sleep) {
        ("HTTP/1.1 200 OK", "hello.html", 5) //simulate a slow request
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html", 0)
    };

    let contents = fs::read_to_string(format!("webserver/html/{filename}")).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    if wait > 0 {
        thread::sleep(Duration::from_secs(wait));
    }

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
