const { invoke } = window.__TAURI__.tauri;
const {listen, emit} = window.__TAURI__.event;
// import { listen } from '@tauri-apps/api/event'

let greetInputEl;
let greetMsgEl;
let bufferMsgEl;


console.log("hello")


window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  bufferMsgEl = document.querySelector("#buffer-msg");
});

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

window.greet = greet;
// window.listener = listener;

const listener = await listen('got-udp', (e) => {
  const bits = e.payload.data;
  const bytesToBitsArray = bits.map((byte) => {
    const bitsWithOutLeadingZeros = byte.toString(2);
    const bitsWithLeadingZeros =
      "00000000".substr(bitsWithOutLeadingZeros.length) +
      bitsWithOutLeadingZeros;
      return bitsWithLeadingZeros;
  })
  const combinedBits = bytesToBitsArray.join("");
  const splitArray = combinedBits.match(/.{1,32}/g);
  const splitWithLineBreaks = splitArray.map((x, i) => `Row ${i}: ` + x+'\n');

  bufferMsgEl.textContent = splitWithLineBreaks;
})