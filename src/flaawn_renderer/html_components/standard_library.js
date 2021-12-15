function flaawn_submit(element_id, reload=false) {
    var xhr = new XMLHttpRequest();
    xhr.open('POST', window.location.href, true);
    xhr.setRequestHeader('Content-Type', 'application/json');
    xhr.onload = () => location.reload();
    xhr.send(JSON.stringify({
        id: element_id,
    }));
}