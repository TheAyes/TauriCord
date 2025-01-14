import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { getCurrentWindow } from "@tauri-apps/api/window";

export const createMainWindow = () => {
	const mainWindow = new WebviewWindow("main", {
		url: "https://discord.com/app",
		focus: true
	});

	mainWindow.once("tauri://webview-created", () => {
		const splashWindow = getCurrentWindow();

		splashWindow.close();
	});

	mainWindow.listen("tauri://error", (error) => {
		console.error(error.payload);
	});
};
