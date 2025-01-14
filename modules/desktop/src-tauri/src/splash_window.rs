use tauri::{App, Wry};

pub fn create_splash_window(app: &App) -> tauri::webview::WebviewWindowBuilder<Wry, App> {
    let webview_window = tauri::WebviewWindowBuilder::new(app, "splash", tauri::WebviewUrl::App("/".into()))
        .background_color(tauri::webview::Color(33, 33, 33, 0))
        .decorations(false)
        .resizable(false)
        .inner_size(300f64, 350f64)
        .always_on_top(true)
        .center()
        .transparent(true);

    webview_window
}