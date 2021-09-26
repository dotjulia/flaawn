use crate::flaawn_server::route_manager::RouteManager;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;
use threadpool::ThreadPool;

pub mod route;
pub mod route_manager;

pub struct FlaawnServer {
    pub port: i16,
    pub addr: &'static str,
    pub route_manager: Arc<Mutex<RouteManager>>,
}

fn handle_connection(mut stream: TcpStream, route_manager: Arc<Mutex<RouteManager>>) {
    let mut request_buffer = [0; 1024];
    match stream.read(&mut request_buffer) {
        Ok(_) => match route_manager.lock().unwrap().match_route(&request_buffer) {
            Ok(route) => {
                let renderer = route.renderer;
                let now = Instant::now();
                let content = renderer();
                let dur = now.elapsed();
                println!(
                    "Rendering took: {}μs or {}ms",
                    dur.as_micros(),
                    dur.as_millis()
                );
                let status_line = "HTTP/1.1 200 OK";
                let response = format!(
                    "{}\r\nContent-Length: {}\r\n\r\n{}",
                    status_line,
                    content.len(),
                    content
                );
                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
            Err(_) => {
                println!(
                    "Invalid route: {}",
                    std::str::from_utf8(&request_buffer)
                        .unwrap_or("Invalid request format")
                        .split("\n")
                        .nth(0)
                        .unwrap_or("??")
                );
            }
        },
        Err(e) => {
            println!("Couldn't handle connection: {}", e);
        }
    };
}

impl FlaawnServer {
    pub fn new(port: Option<i16>, addr: Option<&'static str>) -> FlaawnServer {
        FlaawnServer {
            port: port.unwrap_or(8080),
            addr: addr.unwrap_or("0.0.0.0"),
            route_manager: Arc::new(Mutex::new(RouteManager::new())),
        }
    }

    pub fn start(self) {
        let listener = TcpListener::bind(format!("{}:{}", self.addr, self.port)).unwrap();
        let pool = ThreadPool::new(8);
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let rm = self.route_manager.clone();
            pool.execute(|| {
                handle_connection(stream, rm);
            });
        }
    }
}
