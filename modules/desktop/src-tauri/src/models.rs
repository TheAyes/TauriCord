pub struct Task {
    id: String,
    task: Box<dyn Fn() -> bool + Send + Sync + 'static>,
}

#[derive(Default)]
pub struct AppState {
    tasks: Vec<Task>,
}
