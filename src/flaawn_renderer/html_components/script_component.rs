use crate::flaawn_renderer::flaawn_component::FlaawnComponent;
use crate::flaawn_renderer::html_components::generic_html_component::GenericHTMLComponentOptions;
use flawn_proc::FlaawnComponentMacro;

#[derive(FlaawnComponentMacro, Default)]
pub struct ScriptComponent {
    pub script: Option<String>,
    #[html_attributes]
    pub html_args: GenericHTMLComponentOptions,
}

impl FlaawnComponent for ScriptComponent {
    fn build(&self) -> std::string::String {
        format!(
            "<script {}>{}</script>",
            self.html_args.build_attributes(),
            self.script.as_ref().unwrap_or(&"".to_string())
        )
    }
}
