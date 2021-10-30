use crate::flaawn_renderer::flaawn_component::FlaawnComponent;
use crate::flaawn_renderer::html_components::generic_html_component::GenericHTMLComponentOptions;
use crate::no_input;
use flawn_proc::FlaawnComponentMacro;

#[derive(FlaawnComponentMacro)]
pub struct CSSComponent {
    pub css: Option<String>,
    #[html_attributes]
    pub html_attrs: GenericHTMLComponentOptions,
}

impl FlaawnComponent for CSSComponent {
    fn build(&self, _: &mut std::collections::HashMap<String, String>) -> String {
        format!(
            "<style {}>{}</style>",
            self.html_attrs.build_attributes(),
            self.css.as_ref().unwrap_or(&"".to_string()),
        )
    }
    no_input!();
}
