use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum RouteMethod {
    POST,
    GET,
    PUT,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl fmt::Display for RouteMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
type RendererFn = fn(
    session: std::sync::Arc<std::sync::Mutex<std::collections::HashMap<String, String>>>,
) -> String;
type InputFn = fn(
    session: std::sync::Arc<std::sync::Mutex<std::collections::HashMap<String, String>>>,
    input: std::sync::Arc<serde_json::Value>,
);

#[derive(Copy, Clone)]
pub struct Route {
    pub route: &'static str,
    pub method: RouteMethod,
    pub renderer: RendererFn,
    pub input: InputFn,
    pub input_method: RouteMethod,
}

impl Route {
    /**
     * route needs to start with a /
     * there must not be a / at the end
     * example: /test, /test/test, /
     * bad example: test/, test, /test/test/
     */
    pub fn new(
        method: RouteMethod,
        route: &'static str,
        renderer: RendererFn,
        input: InputFn,
        input_method: RouteMethod,
    ) -> Route {
        assert!(!(route.ends_with("/") && route.len() > 1));
        assert!(route.starts_with("/"));
        Route {
            route,
            method,
            renderer,
            input,
            input_method,
        }
    }
}
