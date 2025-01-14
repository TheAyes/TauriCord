use crate::models::HeadResponse;
use crate::splash_window::create_splash_window;
use tauri::Manager;
use tauri_plugin_http::reqwest;

mod models;
mod settings;
mod splash_window;

// Define a handler for the `HEAD` request
#[tauri::command]
async fn make_head_request(url: &str) -> Result<HeadResponse, String> {
    let res = reqwest::Client::new()
        .head(url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

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
            let webview_window = create_splash_window(app).build()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
