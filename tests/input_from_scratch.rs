use flaawn::flaawn_renderer::flaawn_component::FlaawnComponent;
use flaawn::flaawn_renderer::html_components::generic_html_component::RandomHtmlId;
use flaawn::flaawn_renderer::html_components::html_site::HTMLSite;
use flaawn::flaawn_server::route::Route;
use flaawn::flaawn_server::route::RouteMethod::GET;
use flaawn::flaawn_server::FlaawnServer;
use flaawn::{default_renderer, s, HTMLBoilerplate};
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
    fn build(&self, _: &mut std::collections::HashMap<String, String>) -> String {
        format!(
            r#"
            <script>
            function on_button_press() {{
                flaawn_submit("{}")
            }}
            </script>
            <button onclick="on_button_press()">{} {}</button>"#,
            self.id.id,
            self.label.clone().unwrap(),
            self.test.unwrap()
        )
    }

    fn handle_input(
        &self,
        _: &mut std::collections::HashMap<String, String>, //session
        data: &std::collections::HashMap<String, String>,  //input data
    ) {
        print!("{:?}", data);
    }
}

lazy_static! {
    static ref TODO_SITE: HTMLSite = HTMLBoilerplate!("Test", Button_m!(s!("Click me!"), 42,),);
}

default_renderer!(renderer, TODO_SITE);

#[test]
fn input_from_scratch() {
    let server = FlaawnServer::new(None, None);
    server
        .route_manager
        .lock()
        .unwrap()
        .add_route(Route::new(GET, "/", renderer));
    server.start();
}
