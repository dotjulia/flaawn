#[macro_export]
macro_rules! options_struct {
    ($name:ident, {$($n:ident: $t:ty = $dv:expr,)+ }) => {
        pub struct $name {
            $(pub $n: Option<$t>,)*
        }
        impl Default for $name {
            fn default() -> Self {
                $name {
                    $($n: $dv,)*
                }
            }
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

pub trait FlaawnComponent: Send + Sync {
    fn build(&self) -> String;
}
