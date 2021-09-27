use crate::flaawn_renderer::flaawn_component::FlaawnComponent;
use crate::flaawn_renderer::html_components::generic_html_component::GenericHTMLComponentOptions;
use flawn_proc::FlaawnComponentMacro;

macro_rules! script {
    ($script:expr, ( $($on:ident = $ov:expr,)*  )) => {
        ScriptComponent {
            script: Some($script),
            GenericHTMLComponentOptions {
                $($on: Some($ov),)*
                ..Default::default()
            },
        }
    };
}

#[derive(FlaawnComponentMacro)]
pub struct ScriptComponent {
    script: Option<String>,
    attributes: GenericHTMLComponentOptions,
}

impl FlaawnComponent for ScriptComponent {
    fn build(&self) -> std::string::String {
        format!(
            "<script {}>{}</script>",
            self.attributes.build_attributes(),
            self.script.as_ref().unwrap_or(&"".to_string())
        )
    }
}
