pub mod flaawn_component;
pub mod flaawn_component_with_children;
pub mod html_components;

#[macro_export]
macro_rules! default_renderer {
    ($name: ident, $comp:ident) => {
        fn $name(
            session: std::sync::Arc<std::sync::Mutex<std::collections::HashMap<String, String>>>,
        ) -> String {
            $comp.build(&mut session.lock().unwrap())
        }
    };
}
