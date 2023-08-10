use crate::app::components::mongo::Mongo;

pub struct AppState {
    pub app_name: String,
    pub mongo: Mongo,
}