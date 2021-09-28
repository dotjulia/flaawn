use crate::flaawn_renderer::flaawn_component::FlaawnComponent;
use crate::flaawn_renderer::html_components::generic_html_component::GenericHTMLComponentOptions;
use flawn_proc::FlaawnComponentMacro;

#[derive(FlaawnComponentMacro)]
pub struct CSSComponent {
    pub css: Option<String>,
    #[html_attributes]
    pub html_attrs: GenericHTMLComponentOptions,
}

impl FlaawnComponent for CSSComponent {
    fn build(&self) -> String {
        format!(
            "<style {}>{}</style>",
            self.html_attrs.build_attributes(),
            self.css.as_ref().unwrap_or(&"".to_string()),
        )
    }
}
