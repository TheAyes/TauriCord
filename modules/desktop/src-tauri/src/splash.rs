/*#[tauri::command]
pub async fn create_splashscreen(app: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        let webview_window =
            tauri::WebviewWindowBuilder::new(&app, "splash", tauri::WebviewUrl::App("/".into()))
                .always_on_top(true)
                .resizable(false)
                .inner_size(800.0, 600.0)
                .build()
                .unwrap();
    })
}*/
