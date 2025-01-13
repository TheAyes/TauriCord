use tauri::Manager;

struct Task {
    id: String,
    task: Box<dyn Fn() -> bool + Send + Sync + 'static>,
}

#[derive(Default)] // Allows instance creation with `AppState::default()`
struct AppState {
    tasks: Vec<Task>,
}

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
            let app_state = AppState::default();

            // Initialize and manage state
            app.manage(app_state);

            let mut app_state = app.state::<AppState>();
            //initialize_plugins(&mut app_state);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
