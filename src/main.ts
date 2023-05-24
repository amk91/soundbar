// import { invoke } from "@tauri-apps/api/tauri";

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

import { invoke } from "@tauri-apps/api/tauri";
import { listen } from '@tauri-apps/api/event'

async () => {
  await listen('command_result', (event) => console.log(event))
    .then(() => console.log("listened"))
    .catch((error) => console.log(error));
  // await listen('command_result', (event) => {
  //   console.log(event);
  // })
}

async function add_soundbite() {
  try {
    //TODO: populate command args from UI
    console.log("add_soundbite call")
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

document.addEventListener("DOMContentLoaded", () => {
  console.log("init");
  document
    .querySelector("#add_soundbite_button")
    ?.addEventListener("click", () => add_soundbite());
})
