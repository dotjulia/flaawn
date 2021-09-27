extern crate proc_macro;
use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::ItemStruct;

#[proc_macro_derive(FlaawnComponentMacro, attributes(html_attributes))]
pub fn flaawn_component_macro(input: TokenStream) -> TokenStream {
    let s2 = parse_macro_input!(input as ItemStruct);
    let mut contains_atributtes = false;
    let mut attributes_name = String::new();
    let macro_name_ident = s2.ident;
    let mut final_macro = format!(
        r#"#[macro_export] {}macro_rules! {}_m {{("#,
        "\n",
        macro_name_ident.to_string()
    );
    for field in &s2.fields {
        if field.ident.as_ref().unwrap().to_string() == "attributes" {
            //println!("{:?}", field.attrs[0].path.get_ident().unwrap().to_string());
            contains_atributtes = true;
            continue;
        }
        final_macro.push_str(&format!(
            "${}:expr, ",
            field.ident.as_ref().unwrap().to_string()
        ));
    }
    if contains_atributtes {
        final_macro.push_str("( $($on:ident = $ov:expr,)* )");
    }
    final_macro.push_str(&format!(
        r#") => {{
      {} {{"#,
        macro_name_ident.to_string(),
    ));
    for field in s2.fields {
        if field.ident.as_ref().unwrap().to_string() == "attributes" {
            continue;
        }
        final_macro.push_str(&format!(
            "{}: Some(${}), ",
            field.ident.as_ref().unwrap().to_string(),
            field.ident.as_ref().unwrap().to_string()
        ));
    }
    if contains_atributtes {
        final_macro.push_str(
            r#"attributes: flaawn::flaawn_renderer::html_components::generic_html_component::GenericHTMLComponentOptions {
        $($on: Some($ov),)*
        ..Default::default()
    },"#,
        );
    }
    final_macro.push_str(
        r#"}
    };
}"#,
    );
    final_macro.parse().unwrap()
}
