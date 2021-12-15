use flaawn::flaawn_renderer::flaawn_component::FlaawnComponent;
use flaawn::flaawn_renderer::html_components::generic_html_component::RandomHtmlId;
use flaawn::flaawn_renderer::html_components::html_site::HTMLSite;
use flaawn::flaawn_server::route::Route;
use flaawn::flaawn_server::route::RouteMethod::{GET, POST};
use flaawn::flaawn_server::FlaawnServer;
use flaawn::{default_input_handler, default_renderer, s, HTMLBoilerplate};
use flawn_proc::FlaawnComponentMacro;
use lazy_static::lazy_static;

#[derive(FlaawnComponentMacro, Default)]
struct Button {
    label: Option<String>,
    test: Option<u32>,
    #[random_id]
    id: RandomHtmlId,
}

impl FlaawnComponent for Button {
    fn build(&self, session: &mut std::collections::HashMap<String, String>) -> String {
        format!(
            r#"
            <script>
            function on_button_press() {{
                flaawn_submit("{}")
                //location.reload();
            }}
            </script>
            <button onclick="on_button_press()">{} {}</button>
            <p>{}</p>"#,
            self.id.id,
            self.label.clone().unwrap(),
            self.test.unwrap(),
            session.get("counter").unwrap_or(&String::from("0")),
        )
    }

    fn handle_input(
        &self,
        session: &mut std::collections::HashMap<String, String>, //session
        data: &serde_json::Value,                          //input data
    ) {
        let counter = session.get_mut("counter");
        match counter {
            Some(a) => {
                *a = ((*a).parse::<i32>().unwrap_or(0) + 1).to_string();
            },
            None => {
                session.insert("counter".to_string(), "1".to_string());
            },
        }
    }
}

lazy_static! {
    static ref TODO_SITE: HTMLSite = HTMLBoilerplate!("Test", Button_m!(s!("Click me!"), 42,),);
}

default_renderer!(renderer, TODO_SITE);
default_input_handler!(input_handler, TODO_SITE);

#[test]
fn input_from_scratch() {
    let server = FlaawnServer::new(None, None);
    server.route_manager.lock().unwrap().add_route(Route::new(
        GET,
        "/",
        renderer,
        input_handler,
        POST,
    ));
    server.start();
}
