const { invoke } = window.__TAURI__.tauri;

//INFO: example on how to call Rust functions from frontend
// let greetInputEl;
// let greetMsgEl;

// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//   greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
// }

// window.addEventListener("DOMContentLoaded", () => {
//   greetInputEl = document.querySelector("#greet-input");
//   greetMsgEl = document.querySelector("#greet-msg");
//   document
//     .querySelector("#greet-button")
//     .addEventListener("click", () => greet());
// });

async function add_soundbite() {
    try {
        //TODO: populate command args from UI
        await invoke("add_soundbite", { soundbiteName: "", path: "" });
    } catch (commandException) {
        console.log(commandException)
    } finally {
        //TODO: get list of soundbites
    }
}

async function set_volume() {
    try {
        await invoke("set_soundbite_volume", { soundbiteName: "", volume: 0.0 });
    } catch (commandExpection) {
        console.log(commandExpection)
    }
}

async function set_speed() {
    try {
        await invoke("set_soundbite_speed", { soundbiteName: "", speed: 0.0 });
    } catch (commandExpection) {
        console.log(commandExpection)
    }
}
