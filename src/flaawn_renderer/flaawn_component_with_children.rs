use crate::flaawn_renderer::flaawn_component::FlaawnComponent;
use std::{collections::HashMap, sync::Arc};

#[macro_export]
macro_rules! children_no_input {
    () => {
        fn handle_input(
            &self,
            s: &mut std::collections::HashMap<String, String>, //session
            d: &serde_json::Value,                             //input data
        ) {
            self.handle_input_for_children(s, d);
        }
    };
}

pub trait FlaawnComponentWithChildren: FlaawnComponent {
    fn children(&self) -> Vec<Arc<dyn FlaawnComponent>>;
    fn handle_input_for_children(
        &self,
        session: &mut HashMap<String, String>,
        data: &serde_json::Value,
    ) {
        let children = self.children();
        for child in children {
            child.handle_input(session, data);
        }
    }
    fn build_children(&self, session: &mut HashMap<String, String>) -> String {
        let mut ret_val = String::new();
        let children = self.children();
        for child in children {
            ret_val.push_str(&child.build(session));
        }
        ret_val
    }
}
