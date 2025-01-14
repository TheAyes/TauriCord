pub struct Task {
	id: String,
	task: Box<dyn Fn() -> bool + Send + Sync + 'static>,
}

#[derive(Default)]
pub struct AppState {
	tasks: Vec<Task>,
}

#[derive(serde::Serialize)]
pub struct HeadResponse {
	pub status: u16,
	pub headers: std::collections::HashMap<String, String>,
}

pub trait Plugin {
	fn name(&self) -> &'static str;
	fn register(&self, app_state: &mut AppState);
}
