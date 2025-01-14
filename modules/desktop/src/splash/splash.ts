import { invoke } from "@tauri-apps/api/core";
import { sleep } from "shared/src/utils.ts";
import { createMainWindow } from "../main.ts";

console.log("hello");

const setup = async () => {
	const loadingHint = document.getElementById("loading-hint");
	if (!loadingHint) {
		console.error("Loading hint not found.");
		return;
	}

	try {
		const pollDiscord = async () => {
			return await invoke<{ status: number; headers: Record<string, string> }>("make_head_request", {
				url: "https://discord.com/app"
			});
		};

		while ((await pollDiscord()).status !== 200) {
			sleep(5);
		}

		createMainWindow();
	} catch (error) {
		if (error instanceof TypeError) {
			loadingHint.innerText = error.message;
		}
	}
};

setup();
