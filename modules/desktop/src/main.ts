import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { getCurrentWindow } from "@tauri-apps/api/window";

export const createMainWindow = () => {
	const mainWindow = new WebviewWindow("main", { url: "https://discord.com/app" });

	mainWindow.once("tauri://window-created", () => {
		const splashWindow = getCurrentWindow();

		splashWindow.close();
	});

	mainWindow.once("tauri://error", (error) => {
		console.error(error.payload);
	});
};
