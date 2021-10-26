extern crate proc_macro;
use proc_macro::TokenStream;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use syn::parse_macro_input;
use syn::ItemStruct;

#[proc_macro_derive(FlaawnComponentMacro, attributes(html_attributes, random_id))]
pub fn flaawn_component_macro(input: TokenStream) -> TokenStream {
    let s2 = parse_macro_input!(input as ItemStruct);
    let mut contains_atributtes = false;
    let mut attributes_name = String::new();

    let mut contains_random_id = false;
    let mut random_id_field_name = String::new();
    let mut random_id_field_content: Option<String> = None;

    let macro_name_ident = s2.ident;
    let mut final_macro = format!(
        r#"#[macro_export] {}macro_rules! {}_m {{("#,
        "\n",
        macro_name_ident.to_string()
    );
    for field in &s2.fields {
        if field.attrs.len() > 0
            && field.attrs[0].path.get_ident().unwrap().to_string() == "html_attributes"
        {
            contains_atributtes = true;
            attributes_name = field.ident.as_ref().unwrap().to_string();
            continue;
        }
        if field.attrs.len() > 0
            && field.attrs[0].path.get_ident().unwrap().to_string() == "random_id"
        {
            contains_random_id = true;
            random_id_field_name = field.ident.as_ref().unwrap().to_string();
            random_id_field_content = Some(
                thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(30)
                    .map(char::from)
                    .collect(),
            );
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
        if field.attrs.len() > 0
            && field.attrs[0].path.get_ident().unwrap().to_string() == "html_attributes"
        {
            continue;
        }
        if field.attrs.len() > 0
            && field.attrs[0].path.get_ident().unwrap().to_string() == "random_id"
        {
            continue;
        }
        final_macro.push_str(&format!(
            "{}: Some(${}), ",
            field.ident.as_ref().unwrap().to_string(),
            field.ident.as_ref().unwrap().to_string()
        ));
    }
    if contains_atributtes {
        final_macro.push_str(&format!(
            r#"{}: flaawn::flaawn_renderer::html_components::generic_html_component::GenericHTMLComponentOptions {{
        $($on: Some($ov),)*
        ..Default::default()
    }},"#,
    attributes_name));
    }
    if contains_random_id {
        final_macro.push_str(&format!(
            r#"{}: flaawn::flaawn_renderer::html_components::generic_html_component::RandomHtmlId {{
                id: String::from("{}"),
            }},"#,
            random_id_field_name,
            random_id_field_content.unwrap(),
        ))
    }
    final_macro.push_str(
        r#"}
    };
}"#,
    );
    //println!("{}", final_macro);
    final_macro.parse().unwrap()
}
