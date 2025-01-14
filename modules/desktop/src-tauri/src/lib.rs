use models::AppState;

use tauri::Manager;

mod models;
mod splash;
mod store;

pub trait Plugin {
    fn name(&self) -> &'static str;
    fn register(&self, app_state: &mut AppState);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .setup(|app| {
            let webview_window = tauri::WebviewWindowBuilder::new(
                app,
                "splash",
                tauri::WebviewUrl::App("splash.html".into()),
            )
            .background_color(tauri::webview::Color(33, 33, 33, 255))
            .decorations(false)
            .resizable(false)
            .inner_size(300f64, 350f64)
            .always_on_top(true)
            .center()
            .build()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
