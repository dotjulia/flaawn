use crate::{flaawn_renderer::flaawn_component::FlaawnComponent, no_input};

#[macro_export]
macro_rules! PlainText {
    ($text:expr) => {
        PlainText!($text, true)
    };
    ($text:expr, $sanitized: expr) => {
        flaawn::flaawn_renderer::html_components::plain_text_component::PlainTextComponent {
            text: $text.to_string(),
            sanitized: $sanitized,
        }
    };
}

pub struct PlainTextComponent {
    pub text: String,
    pub sanitized: bool,
}

impl FlaawnComponent for PlainTextComponent {
    fn build(&self, _: &mut std::collections::HashMap<String, String>) -> std::string::String {
        if self.sanitized {
            return self.text.replace("<", "&lt;").replace(">", "&gt;");
        }
        self.text.clone()
    }
    no_input!();
}
