use crate::flaawn_renderer::flaawn_component::FlaawnComponent;
use std::{collections::HashMap, sync::Arc};

pub trait FlaawnComponentWithChildren: FlaawnComponent {
    fn children(&self) -> Vec<Arc<dyn FlaawnComponent>>;
    fn build_children(&self, session: &mut HashMap<String, String>) -> String {
        let mut ret_val = String::new();
        let children = self.children();
        for child in children {
            ret_val.push_str(&child.build(session));
        }
        ret_val
    }
}
