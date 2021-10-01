#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use flaawn::div;
use flaawn::flaawn_renderer::flaawn_component::FlaawnComponent;
use flaawn::flaawn_renderer::html_components::css_component::CSSComponent;
use flaawn::flaawn_renderer::html_components::html_site::HTMLSite;
use flaawn::flaawn_renderer::html_components::script_component::ScriptComponent;
use flaawn::flaawn_server::route::Route;
use flaawn::flaawn_server::route::RouteMethod::GET;
use flaawn::flaawn_server::FlaawnServer;
use flaawn::img;
use flaawn::p;
use flaawn::s;
use flaawn::CSSComponent_m;
use flaawn::CSSStyle;
use flaawn::HTMLBoilerplate;
use flaawn::PlainText;
use flaawn::ScriptComponent_m;

lazy_static! {
    static ref MAIN_COMP: HTMLSite = HTMLBoilerplate!(
        "Test",
        img!((src = s!("https://picsum.photos/500"),),),
        p!(
            (style = CSSStyle! {
                color: "#ff0000";
            },),
            PlainText!("<script>alert(1)</script>"),
        ),
        div!((class = s!("test"),), PlainText!("Test"),),
        ScriptComponent_m!(s!("alert(1)"), ()),
        CSSComponent_m!(s!(".test { background-color: #ff0000; }"), ()),
    );
}

fn main_renderer(session: Arc<Mutex<HashMap<String, String>>>) -> String {
    session
        .lock()
        .unwrap()
        .entry(s!("test"))
        .or_insert(s!("1"))
        .push_str("1");
    println!("{}", session.lock().unwrap()["test"]);
    MAIN_COMP.build(&mut session.lock().unwrap())
}

#[test]
fn basic() {
    let server = FlaawnServer::new(None, None);
    server
        .route_manager
        .lock()
        .unwrap()
        .add_route(Route::new(GET, "/", main_renderer));
    server.start();
}
