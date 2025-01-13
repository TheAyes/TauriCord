import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { sleep } from "shared/src/utils.ts";

await sleep(5);

const webview = new WebviewWindow("main", { url: "https://discord.com/app" });

webview.once("tauri://window-created", () => {
	console.log("Hello World!a");
});
