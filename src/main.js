const { invoke } = window.__TAURI__.core;

let greetInputEl;
let greetMsgEl;
let usernameInputEl;
let usernameWordsInputEl;
let usernameMsgEl;
let usernameRomajiCheckEl;
let usernameNamesCheckEl;

async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

async function generate_username() {
    const length = Number(usernameInputEl.value);
    const word_count = Number(usernameWordsInputEl.value);

    if (!Number.isInteger(length) || length <= 0) {
        usernameMsgEl.textContent = "Please enter a valid number";
        return;
    }

    usernameMsgEl.textContent = "";

    for (let i = 0; i < word_count; i++) {
        try {
            if (usernameRomajiCheckEl.checked) {
                usernameMsgEl.textContent += await invoke("generate_romaji_username", { length });
            } else if (usernameNamesCheckEl.checked) {
                usernameMsgEl.textContent += await invoke("generate_random_names", { length });
            } else {
                usernameMsgEl.textContent += await invoke("generate_username", { length });
            }
            usernameMsgEl.textContent += " ";
        } catch (error) {
            console.error("Invoke failed:", error);
            usernameMsgEl.textContent = "Failed to generate username";
        }
    }
}

window.addEventListener("DOMContentLoaded", () => {
    usernameWordsInputEl = document.querySelector("#username-words-input");
    usernameInputEl = document.querySelector("#username-input");
    usernameMsgEl = document.querySelector("#username-msg");
    usernameRomajiCheckEl = document.querySelector("#username-romaji-check");
    usernameNamesCheckEl = document.querySelector("#username-names-check");
    document.querySelector("#username-form").addEventListener("submit", (e) => {
        e.preventDefault();
        generate_username();
    });
});
