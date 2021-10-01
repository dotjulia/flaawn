use std::sync::Arc;

use flaawn::flaawn_renderer::flaawn_component::FlaawnComponent;
use flaawn::flaawn_renderer::html_components::html_site::HTMLSite;
use flaawn::flaawn_server::route::RouteMethod::GET;
use flaawn::flaawn_server::{route::Route, FlaawnServer};
use flaawn::{default_renderer, li, s, ul, HTMLBoilerplate, PlainText};
use flawn_proc::FlaawnComponentMacro;
use lazy_static::lazy_static;

/*
 * Input for components could be implemented, by them sending as post request with the component id
 * and then sending it down it a seperate function every component implements
 */

#[derive(FlaawnComponentMacro, Default)]
struct TodoComp {}

impl FlaawnComponent for TodoComp {
    fn build(&self, session: &mut std::collections::HashMap<String, String>) -> String {
        // Add a new todo item
        session
            .entry(s!("todo_list"))
            .or_insert(s!("todo item,"))
            .push_str("todo item,");
        let mut list = ul!((),);
        let todo_string = session[&s!("todo_list")].clone();
        for todo_item in todo_string.split(",") {
            if todo_item.len() > 0 {
                list.child_components
                    .push(Arc::from(li!((), PlainText!(todo_item),)));
            }
        }
        list.build(session)
    }
}

lazy_static! {
    static ref TODO_SITE: HTMLSite = HTMLBoilerplate!("Test", TodoComp_m!(),);
}

default_renderer!(renderer, TODO_SITE);

#[test]
fn todo_no_unput() {
    let server = FlaawnServer::new(None, None);
    server
        .route_manager
        .lock()
        .unwrap()
        .add_route(Route::new(GET, "/", renderer));
    server.start();
}
