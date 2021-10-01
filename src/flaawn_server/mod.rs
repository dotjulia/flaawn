use lazy_static::lazy_static;

use crate::flaawn_server::route_manager::RouteManager;
use httparse::Header;
use std::collections::HashMap;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;
use threadpool::ThreadPool;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use self::cookie::Cookie;

mod cookie;
pub mod route;
pub mod route_manager;

pub struct FlaawnServer {
    pub port: i16,
    pub addr: &'static str,
    pub route_manager: Arc<Mutex<RouteManager>>,
}

fn find_cookie_header(headers: [Header; 64], cookie_name: String) -> Result<Cookie, ()> {
    for header in headers {
        if header.name == "Cookie" {
            let cookie = Cookie::parse(String::from_utf8_lossy(header.value).to_string());
            if cookie.name == cookie_name {
                return Ok(cookie);
            }
        }
    }
    return Err(());
}

fn generate_session_id() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect()
}

lazy_static! {
    static ref SESSIONS: Mutex<HashMap<String, Arc<Mutex<HashMap<String, String>>>>> =
        Mutex::from(HashMap::new());
}

fn get_session_map(session_id: &String) -> Arc<Mutex<HashMap<String, String>>> {
    if SESSIONS.lock().unwrap().contains_key(session_id) {
        return SESSIONS.lock().unwrap()[session_id].clone();
    } else {
        let new_session = Arc::from(Mutex::from(HashMap::new()));
        SESSIONS
            .lock()
            .unwrap()
            .insert(session_id.clone(), new_session);
        return get_session_map(session_id);
    }
}

fn handle_connection(mut stream: TcpStream, route_manager: Arc<Mutex<RouteManager>>) {
    let mut request_buffer = [0; 1024];
    match stream.read(&mut request_buffer) {
        Ok(_) => {
            let mut headers = [httparse::EMPTY_HEADER; 64];
            let mut req = httparse::Request::new(&mut headers);
            let _ = req.parse(&request_buffer);
            match route_manager.lock().unwrap().match_route(&req) {
                Ok(route) => {
                    let session_id = match find_cookie_header(headers, "session_id".to_string()) {
                        Ok(cookie) => cookie.value,
                        Err(_) => generate_session_id(),
                    };
                    let session = get_session_map(&session_id);
                    let renderer = route.renderer;
                    let now = Instant::now();
                    let content = renderer(session);
                    let dur = now.elapsed();
                    println!(
                        "Rendering took: {}μs or {}ms",
                        dur.as_micros(),
                        dur.as_millis()
                    );
                    println!("{}", String::from_utf8(request_buffer.to_vec()).unwrap());
                    let status_line = "HTTP/1.1 200 OK";
                    let response = format!(
                    "{}\r\nContent-Type: text/html\r\nSet-Cookie: session_id={}\r\nContent-Length: {}\r\n\r\n{}",
                    status_line,
                    session_id,
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
            };
        }
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
