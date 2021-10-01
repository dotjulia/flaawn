use crate::flaawn_renderer::flaawn_component::FlaawnComponent;
use crate::flaawn_renderer::flaawn_component_with_children::FlaawnComponentWithChildren;
use std::sync::Arc;

#[macro_export]
macro_rules! HTMLBoilerplate {
    ($title:expr, $($child:expr,)*) => {
        flaawn::flaawn_renderer::html_components::html_site::HTMLSite {
            title: $title.to_string(),
            child_components: vec![
                $(
                    std::sync::Arc::from($child),
                )*
            ],
        }
    }
}

pub struct HTMLSite {
    pub title: String,
    pub child_components: Vec<Arc<dyn FlaawnComponent>>,
}

impl HTMLSite {
    pub fn new(title: String, child_components: Vec<Arc<dyn FlaawnComponent>>) -> HTMLSite {
        HTMLSite {
            title,
            child_components,
        }
    }
}

impl FlaawnComponentWithChildren for HTMLSite {
    fn children(&self) -> Vec<Arc<dyn FlaawnComponent>> {
        self.child_components.clone()
    }
}

impl FlaawnComponent for HTMLSite {
    fn build(
        &self,
        session: &mut std::collections::HashMap<String, String>,
    ) -> std::string::String {
        format!(
            r#"
            <!DOCTYPE html>
            <html>
                <head>
                    <meta charset="UTF-8">
                </head>
                <body>
                {}
                </body>
            </html>
            "#,
            self.build_children(session)
        )
    }
}
