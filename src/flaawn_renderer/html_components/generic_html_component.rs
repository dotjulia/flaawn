use crate::flaawn_renderer::flaawn_component::FlaawnComponent;
use crate::flaawn_renderer::flaawn_component_with_children::FlaawnComponentWithChildren;
use std::sync::Arc;

#[macro_export]
macro_rules! GenericHTMLTag {
    ($tag:expr, $($child:expr,)*) => {
        GenericHTMLComponent {
            tag: $tag.to_string(),
            child_components: vec![
                $(
                    Arc::from($child),
                )*
            ],
        }
    }
}

pub struct GenericHTMLComponent {
    pub tag: String,
    pub child_components: Vec<Arc<dyn FlaawnComponent>>,
}

impl GenericHTMLComponent {
    pub fn new(
        tag: String,
        child_components: Vec<Arc<dyn FlaawnComponent>>,
    ) -> GenericHTMLComponent {
        GenericHTMLComponent {
            tag,
            child_components: child_components,
        }
    }
}

impl FlaawnComponentWithChildren for GenericHTMLComponent {
    fn children(&self) -> Vec<Arc<dyn FlaawnComponent>> {
        self.child_components.clone()
    }
}

impl FlaawnComponent for GenericHTMLComponent {
    fn build(&self) -> std::string::String {
        format!("<{}>{}</{}>", self.tag, self.build_children(), self.tag)
    }
}
