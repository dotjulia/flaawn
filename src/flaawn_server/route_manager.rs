use httparse::Request;

use crate::flaawn_server::route::Route;

pub struct RouteManager {
    routes: Vec<Route>,
}

impl RouteManager {
    pub fn new() -> RouteManager {
        RouteManager {
            routes: Vec::<Route>::new(),
        }
    }
    pub fn add_route(&mut self, route: Route) {
        self.routes.push(route);
    }
    // fn construct_request_line(r: &Route) -> String {
    //     format!("{} {} HTTP/1.1", r.method, r.route).to_string()
    // }
    pub fn match_route(&self, request: &Request) -> Result<Route, u8> {
        for r in &self.routes {
            if request.method.unwrap_or("") == r.method.to_string()
                && request.path.unwrap_or("") == r.route
            {
                return Ok(*r);
            }
        }
        return Err(0);
    }
}
