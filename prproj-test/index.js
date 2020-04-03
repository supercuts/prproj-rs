import * as wasm from "prproj-rs";
let inputElement = document.querySelector("input");
inputElement.addEventListener("change", (e) => {
    let file = e.target.files[0];
    if (file == null) {
        console.error("No file specified");
        return;
    }
    let reader = new FileReader();
    reader.onload = (evt) => {
        let arrayBuffer = evt.target.result;
        let uint8View = new Uint8Array(arrayBuffer);
        let file = wasm.read_prproj(uint8View);
        file = file.take();
        let media0 = file.media[0];
        let duration0 = media0.duration;
        console.debug(file);
        console.log('file', file);
        console.log('media0', media0);
        console.log('duration0', duration0);
    };
    reader.readAsArrayBuffer(file);

})

