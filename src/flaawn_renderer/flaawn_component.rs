#[macro_export]
macro_rules! options_struct {
    ($name:ident, {$($n:ident: $t:ty = $dv:expr,)+ }) => {
        #[derive(Default)]
        pub struct $name {
            $(pub $n: Option<$t>,)*
            pub style: Option<CSSStyleOptions>,
        }
        impl $name {
            pub fn build_attributes(&self) -> String {
                let mut ret_val = String::new();
                $(
                    if !self.$n.is_none() {
                        ret_val.push_str(&format!(
                            " {}=\"{}\"",
                            stringify!($n).replace("__", "").replace("_", "-"),
                            self.$n.clone().unwrap()
                        ));
                    }
                )*
                if !self.style.is_none() {
                    ret_val.push_str(&format!("style=\"{}\"", self.style.as_ref().unwrap().build_css()))
                }
                ret_val
            }
        }
    };
}

#[macro_export]
macro_rules! render_attribute {
    ($self:ident.$name:ident, $retVal:ident) => {
        if !$self.$name.is_none() {
            $retVal.push_str(&format!(
                " {}=\"{}\"",
                stringify!($name).replace("__", "").replace("_", "-"),
                $self.$name.clone().unwrap()
            ));
        }
    };
}

#[macro_export]
macro_rules! s {
    ($str:expr) => {
        $str.to_string()
    };
}

pub trait FlaawnComponent: Send + Sync {
    fn build(&self, session: &mut std::collections::HashMap<String, String>) -> String;
    fn handle_input(
        &self,
        _: &mut std::collections::HashMap<String, String>, //session
        _: &std::collections::HashMap<String, String>,     //input data
    ) {
    }
}
