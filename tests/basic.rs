#[macro_use]
extern crate lazy_static;

use flaawn::div;
use flaawn::flaawn_renderer::flaawn_component::FlaawnComponent;
use flaawn::flaawn_renderer::html_components::html_site::HTMLSite;
use flaawn::flaawn_renderer::html_components::script_component::ScriptComponent;
use flaawn::flaawn_server::route::Route;
use flaawn::flaawn_server::route::RouteMethod::GET;
use flaawn::flaawn_server::FlaawnServer;
use flaawn::img;
use flaawn::p;
use flaawn::s;
use flaawn::CSSStyle;
use flaawn::GenericHTMLTag;
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
        div!((), PlainText!("Test"),),
        ScriptComponent_m!(s!("alert(1)"), ()),
    );
}

fn main_renderer() -> String {
    MAIN_COMP.build()
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
