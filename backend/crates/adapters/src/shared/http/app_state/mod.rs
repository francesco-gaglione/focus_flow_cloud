pub mod flashcard_state;
pub mod shared_state;
pub mod tasks_state;
pub mod user_state;

use crate::config::AppConfig;
use crate::shared::http::app_state::flashcard_state::FlashcardState;
use axum::extract::ws::Message;
use domain::user::services::token_service::TokenService;
use shared_state::SharedState;
use std::collections::HashMap;
use std::sync::Arc;
use tasks_state::TasksState;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::RwLock;
use user_state::UserState;

pub type Clients = Arc<RwLock<HashMap<usize, UnboundedSender<Message>>>>;

#[derive(Clone)]
pub struct AppState {
    pub ws_clients: Clients,
    pub config: AppConfig,
    pub token_service: Arc<dyn TokenService>,
    pub version: String,

    pub shared: SharedState,
    pub tasks: TasksState,
    pub user: UserState,
    pub flashcards: FlashcardState,
}
