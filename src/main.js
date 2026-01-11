const { invoke } = window.__TAURI__.core;

let greetInputEl;
let greetMsgEl;
let usernameInputEl;
let usernameMsgEl;
let usernameCheckEl;

async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

async function generate_username() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    const length = Number(usernameInputEl.value);

    if (!Number.isInteger(length) || length <= 0) {
        usernameMsgEl.textContent = "Please enter a valid number";
        return;
    }

    if (usernameCheckEl.checked) {
        usernameMsgEl.textContent = await invoke("generate_romaji_username", { length });
    } else {
        usernameMsgEl.textContent = await invoke("generate_username", { length });
    }
}

window.addEventListener("DOMContentLoaded", () => {
    usernameInputEl = document.querySelector("#username-input");
    usernameMsgEl = document.querySelector("#username-msg");
    usernameCheckEl = document.querySelector("#username-romaji-check");
    document.querySelector("#username-form").addEventListener("submit", (e) => {
        e.preventDefault();
        generate_username();
    });
});
