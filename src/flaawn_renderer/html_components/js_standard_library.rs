/**
 * flaawn_submit(component_id): sends data to the backend
 */
pub fn js_std_lib() -> &'static str {
    r#"function flaawn_submit(e){var n=new XMLHttpRequest;n.open("POST",window.location.href,!0),n.setRequestHeader("Content-Type","application/json"),n.send(JSON.stringify({id:e}))}"#
}
