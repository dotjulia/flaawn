macro_rules! css_options_struct_creator {
    ($name:ident, [$($n:ident),+ ]) => {
        #[derive(Default)]
        pub struct $name {
            $(pub $n: Option<String>,)*
        }
        impl $name {
            pub fn build_css(&self) -> String {
                let mut ret_val = String::new();
                $(
                    if !self.$n.is_none() {
                        ret_val.push_str(&format!(
                            "{}: {};",
                            stringify!($n).replace("_", "-"),
                            self.$n.clone().unwrap()
                        ));
                    }
                )*
                ret_val
            }
        }
    };
}

/**
 * Create instance of CSSStyleOptions
 */
#[macro_export]
macro_rules! CSSStyle {
    {$($n:ident: $v:expr;)+ } => {
        flaawn::flaawn_renderer::html_components::css_styleable::CSSStyleOptions {
            $($n: Some($v.to_string()),)*
            ..Default::default()
        }
    };
}

css_options_struct_creator!(CSSStyleOptions, [color, font_size]);

// pub struct CSSStyleOptions {
//     color: Option<String>,
// }

// impl Default for CSSStyleOptions {
//     fn default() -> Self {
//         CSSStyleOptions { color: None }
//     }
// }
