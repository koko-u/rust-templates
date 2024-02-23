use yewdux::Store;

#[derive(Debug, Clone, PartialEq, Store)]
pub struct AppStore {
    pub count: i32,
}

impl Default for AppStore {
    fn default() -> Self {
        Self { count: 0 }
    }
}