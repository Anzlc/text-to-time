import { generate_response } from "./pkg/wasm.js";

let input = document.querySelector("#natural-input");
let output = document.querySelector("#formated-output");
let info_modal = document.querySelector(".modal");

const letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

let interval = null;

let info_opened = false;

input.addEventListener("keyup", (e) => {
	output.dataset.value = generate_response(e.target.value.toLowerCase());
	let iteration = 0;

	clearInterval(interval);

	interval = setInterval(() => {
		output.innerText = output.innerText
			.split("")
			.map((letter, index) => {
				if (index < iteration) {
					return output.dataset.value[index];
				}

				return letters[Math.floor(Math.random() * 26)];
			})
			.join("");

		if (iteration >= output.dataset.value.length) {
			clearInterval(interval);
		}

		iteration += 1 / 3;
	}, 30);
});

window.toggleInfo = function toggleInfo() {
	console.log("adsasd");
	info_modal.classList.toggle("hidden");
};
