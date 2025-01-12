let last_update = 0;

let last_delta = {};

// create WebSocket instance/connection
var ws = new WebSocket("ws://127.0.0.1:8080/ws/");

ws.onopen = function(event) {
    console.log("Connection opened");
};

// Handle WebSocket events
// ws.onmessage = function(event) {
//     console.log("Message from server", event.data);
//     document.getElementById('editor').innerHTML = event.data;
// }

ws.onclose = function(event) {
    console.log("Connection closed");
};

var toolbarOptions = [
    ['bold', 'italic', 'underline']
];

var quill = new Quill('#editor', {
    modules: {
        toolbar: toolbarOptions
    },
    theme: 'snow'
})

quill.on('text-change', (delta, oldDelta, source) =>{
    last_update = Date.now();
    last_delta = delta;

    let obj = {
        time: last_update,
        delta: delta
    };

    if(source == 'user') {
        console.log(JSON.stringify(obj));
        ws.send(JSON.stringify(obj));
    }

});

ws.onmessage = function(event) {

    try {

        console.log("Message from server: ", event.data);
        let obj = JSON.parse(event.data);

        let time = obj.time;
        let delta = obj.delta;

        if(last_update < time ){

            quill.updateContents(delta);
        } else if(last_update > time ){
            console.warn("Received outdated server update!");
        }

        last_update = Math.max(last_update, time);

    } catch (error) {
        console.error("Error handling message: ", error);
    }

};

// Create QuillJS instance

// Set handling functions for the text-change event (QuillJS)