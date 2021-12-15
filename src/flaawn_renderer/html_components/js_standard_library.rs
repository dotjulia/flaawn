use std::fs;

/**
 * flaawn_submit(component_id): sends data to the backend
 */
pub fn js_std_lib() -> String {
    let contents = fs::read_to_string("src/flaawn_renderer/html_components/standard_library.js")
        .expect("Couldn't load standard js library");
    // r#"function flaawn_submit(e){var n=new XMLHttpRequest;n.open("POST",window.location.href,!0),n.setRequestHeader("Content-Type","application/json"),n.send(JSON.stringify({id:e}))}"#
    return contents;
}
