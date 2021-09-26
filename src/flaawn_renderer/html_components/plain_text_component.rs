use crate::flaawn_renderer::flaawn_component::FlaawnComponent;

#[macro_export]
macro_rules! PlainText {
    ($text:expr) => {
        PlainTextComponent {
            text: $text.to_string(),
        }
    };
}

pub struct PlainTextComponent {
    pub text: String,
}

impl FlaawnComponent for PlainTextComponent {
    fn build(&self) -> std::string::String {
        self.text.clone()
    }
}
