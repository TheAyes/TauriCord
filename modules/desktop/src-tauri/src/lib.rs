use models::AppState;

use tauri::Manager;
use tauri_plugin_http::reqwest;

mod models;

pub trait Plugin {
    fn name(&self) -> &'static str;
    fn register(&self, app_state: &mut AppState);
}

#[derive(serde::Serialize)]
struct HeadResponse {
    status: u16,
    headers: std::collections::HashMap<String, String>,
}

// Define a handler for the `HEAD` request
#[tauri::command]
async fn make_head_request(url: &str) -> Result<HeadResponse, String> {
    let res = reqwest::Client::new()
        .head(url) // Make a HEAD request
        .send()
        .await
        .map_err(|e| e.to_string())?; // Handle errors by converting to a string.

    // Extract the status code and headers
    let status = res.status().as_u16();
    let headers = res
        .headers()
        .iter()
        .map(|(key, value)| {
            (
                key.to_string(),
                value.to_str().unwrap_or_default().to_string(),
            )
        })
        .collect();

    // Return a serializable struct containing the relevant response data
    Ok(HeadResponse { status, headers })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![make_head_request])
        .setup(|app| {
            let webview_window =
                tauri::WebviewWindowBuilder::new(app, "splash", tauri::WebviewUrl::App("/".into()))
                    .background_color(tauri::webview::Color(33, 33, 33, 0))
                    .decorations(false)
                    .resizable(false)
                    .inner_size(300f64, 350f64)
                    .always_on_top(true)
                    .center()
                    .transparent(true)
                    .build()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
