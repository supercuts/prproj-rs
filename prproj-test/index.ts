import * as wasm from "prproj-rs";
import { xmlFromFile, parsePrproj, Sequence, Timeline } from 'prproj-ts';
console.groupCollapsed("xd");

interface PremiereFileKinda {
    media: wasm.PremiereMedium[],
    sequences: wasm.PremiereSequence[]
}
let whetherWasmElement: HTMLInputElement = document.querySelector("input#whether-wasm");
let whetherWasmOn = false;
const checkCheckboxWhetherWasm = (e: Event) => {
    whetherWasmOn = (e.target as HTMLInputElement).checked;
};
checkCheckboxWhetherWasm({target: whetherWasmElement} as any);
whetherWasmElement.addEventListener("change", checkCheckboxWhetherWasm);
let inputElement: HTMLInputElement = document.querySelector("input#choose-file");

function wasmReadPrproj(file: File) {
    return new Promise(function(resolve, reject) {
        let reader = new FileReader();
        reader.onload = (evt) => {
            let arrayBuffer = evt.target.result as ArrayBuffer;
            let uint8View = new Uint8Array(arrayBuffer);
            let premiere_file  = wasm.read_prproj(uint8View);
            let file = premiere_file.take() as PremiereFileKinda;
            let media0 = file.media[0];
            let duration0 = media0.duration as wasm.WasmDuration;
            console.debug(file);
            console.log('file', file);
            console.log('media0', media0);
            console.log('duration0', duration0);
            resolve();
        };
        reader.onerror = evt => {
            reject(evt);
        }
        reader.readAsArrayBuffer(file);
    })
}
async function readPrproj(file: File) {
    let xml = await xmlFromFile(file);
    const parser = new DOMParser();
    let xml_document = parser.parseFromString(xml, 'text/xml');
    const sequences = parsePrproj(xml_document);
    console.log(sequences);
}

inputElement.addEventListener("change", async (e) => {
    let file = (e.target as HTMLInputElement).files[0];
    let before = new Date();
    if (file == null) {
        console.error("No file specified");
        return;
    }
    if(whetherWasmOn) {
        await wasmReadPrproj(file);
    } else {
        await readPrproj(file);
    }
    let after = new Date();
    console.groupEnd();
    console.log(
        `${whetherWasmOn ? `wasm` : `not wasm`} took ${after.getMilliseconds() - before.getMilliseconds()}ms.`
    );
    console.groupCollapsed();
})

