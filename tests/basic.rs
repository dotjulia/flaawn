#[macro_use]
extern crate lazy_static;

use flaawn::flaawn_renderer::flaawn_component::FlaawnComponent;
use flaawn::flaawn_renderer::html_components::generic_html_component::GenericHTMLComponent;
use flaawn::flaawn_renderer::html_components::html_site::HTMLSite;
use flaawn::flaawn_renderer::html_components::plain_text_component::PlainTextComponent;
use flaawn::flaawn_server::route::Route;
use flaawn::flaawn_server::route::RouteMethod::GET;
use flaawn::flaawn_server::FlaawnServer;
use flaawn::GenericHTMLTag;
use flaawn::HTMLBoilerplate;
use flaawn::PlainText;
use std::sync::Arc;

lazy_static! {
    static ref main_comp: HTMLSite = HTMLBoilerplate!(
        "Test",
        GenericHTMLTag!(
            "ul",
            GenericHTMLTag!("li", PlainText!("Test1"),),
            GenericHTMLTag!("li", PlainText!("Test2"),),
            GenericHTMLTag!("li", PlainText!("Test3"),),
            GenericHTMLTag!("li", PlainText!("Test4"),),
        ),
        GenericHTMLTag!(
            "ul",
            GenericHTMLTag!("li", PlainText!("Test1"),),
            GenericHTMLTag!("li", PlainText!("Test2"),),
            GenericHTMLTag!("li", PlainText!("Test3"),),
            GenericHTMLTag!("li", PlainText!("Test4"),),
        ),
        GenericHTMLTag!(
            "ul",
            GenericHTMLTag!("li", PlainText!("Test1"),),
            GenericHTMLTag!("li", PlainText!("Test2"),),
            GenericHTMLTag!("li", PlainText!("Test3"),),
            GenericHTMLTag!("li", PlainText!("Test4"),),
        ),
        GenericHTMLTag!(
            "ul",
            GenericHTMLTag!("li", PlainText!("Test1"),),
            GenericHTMLTag!("li", PlainText!("Test2"),),
            GenericHTMLTag!("li", PlainText!("Test3"),),
            GenericHTMLTag!("li", PlainText!("Test4"),),
        ),
        GenericHTMLTag!(
            "ul",
            GenericHTMLTag!("li", PlainText!("Test1"),),
            GenericHTMLTag!("li", PlainText!("Test2"),),
            GenericHTMLTag!("li", PlainText!("Test3"),),
            GenericHTMLTag!("li", PlainText!("Test4"),),
        ),
        GenericHTMLTag!(
            "ul",
            GenericHTMLTag!("li", PlainText!("Test1"),),
            GenericHTMLTag!("li", PlainText!("Test2"),),
            GenericHTMLTag!("li", PlainText!("Test3"),),
            GenericHTMLTag!("li", PlainText!("Test4"),),
        ),
        GenericHTMLTag!(
            "ul",
            GenericHTMLTag!("li", PlainText!("Test1"),),
            GenericHTMLTag!("li", PlainText!("Test2"),),
            GenericHTMLTag!("li", PlainText!("Test3"),),
            GenericHTMLTag!("li", PlainText!("Test4"),),
        ),
    );
}

fn main_renderer() -> String {
    main_comp.build()
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
