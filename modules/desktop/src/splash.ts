import { invoke } from "@tauri-apps/api/core";

const setup = async () => {
	const loadingHint = document.getElementById("loading-hint");
	if (!loadingHint) {
		console.error("Loading hint not found.");
		return;
	}

	try {
		console.log("invoking head request");
		const response = await invoke("make_head_request", {
			url: "https://discord.com/app"
		});
		console.log(response);
	} catch (error) {
		if (error instanceof TypeError) {
			loadingHint.innerText = error.message;
		}
	}
};

setup();
