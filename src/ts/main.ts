import '../scss/styles.scss'
// import * as bs from 'bootstrap'

import { invoke } from "@tauri-apps/api/tauri";

// let greetInputEl: HTMLInputElement | null;
// let greetMsgEl: HTMLElement | null;

// async function greet() {
//   if (greetMsgEl && greetInputEl) {
//     // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//     greetMsgEl.textContent = await invoke("greet", {
//       name: greetInputEl.value,
//     });
//   }
// }

// window.addEventListener("DOMContentLoaded", () => {
//   greetInputEl = document.querySelector("#greet-input");
//   greetMsgEl = document.querySelector("#greet-msg");
//   document
//     .querySelector("#greet-button")
//     ?.addEventListener("click", () => greet());
// });

function setMaxHeight() {
    let container = document.getElementById("soundbites-list-container");
    let soundbitesList = document.getElementById("soundbites-list");
    soundbitesList!.style.maxHeight = (window.innerHeight * 80 / 100) + "px";
    console.log(soundbitesList!.style.maxHeight);
}

function updateSoundbiteInfo(event: MouseEvent) {
    //TODO: update soundbite info on right panel and update soundbite pointed to
    console.log("updateSoundbiteInfo fired");
}

function assignKeyToSoundbite(event: MouseEvent) {
    //TODO: set key combination for selected soundbite
    console.log("assignKeyToSoundbite fired");
}

function onAddSoundbite(event: MouseEvent) {
    //TODO: call tauri-command to add soundbite
    console.log("onAddSoundbite fired");
}

function onRemoveSoundbite(event: MouseEvent) {
    //TODO: call tauri-command to remove soundbite
    console.log("onRemoveSoundbite fired");
}

function onSettings(event: MouseEvent) {
    //TODO: display submenu for various settings
    console.log("onSettings fired");
}

window.addEventListener("DOMContentLoaded", () => {
    setMaxHeight();
    window.onresize = (event) => setMaxHeight();

    let volumeRange = document.querySelector<HTMLInputElement>('#volumeRange');
    volumeRange!.onchange = (event) => {
        let volumeRangeValue = document.getElementById('volumeRangeValue');
        volumeRangeValue!.textContent = volumeRange!.value + "%";
    };
    volumeRange!.onmousemove = (event) => {
        let volumeRangeValue = document.getElementById('volumeRangeValue');
        volumeRangeValue!.textContent = volumeRange!.value + "%";
    };

    let speedRange = document.querySelector<HTMLInputElement>('#speedRange');
    speedRange!.onchange = (event) => {
        let speedRangeValue = document.getElementById('speedRangeValue');
        speedRangeValue!.textContent = speedRange!.value + "%";
    };
    speedRange!.onmousemove = (event) => {
        let speedRangeValue = document.getElementById('speedRangeValue');
        speedRangeValue!.textContent = speedRange!.value + "%";
    };

    let recordKey = document.getElementById('recordKey');
    recordKey!.onclick = assignKeyToSoundbite;

    let addSoundbite = document.getElementById('add-soundbite');
    addSoundbite!.onclick = onAddSoundbite;

    let removeSoundbite = document.getElementById('remove-soundbite');
    removeSoundbite!.onclick = onRemoveSoundbite;

    let settings = document.getElementById('settings');
    settings!.onclick = onSettings;

    let soundbitesList = document.getElementById("soundbites-list");
    soundbitesList!.onclick = updateSoundbiteInfo;
});

