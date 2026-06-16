use crate::flashcards::http::folder::create_folder::create_folder_api;
use crate::flashcards::http::folder::delete_folder::delete_folder_api;
use crate::flashcards::http::folder::get_folder_content::get_folder_contents_api;
use crate::flashcards::http::folder::get_folder_review_queue::get_folder_review_queue_api;
use crate::flashcards::http::folder::get_folder_stats::get_folder_stats_api;
use crate::flashcards::http::folder::get_root_folder_content::get_root_folder_contents_api;
use crate::shared::http::app_state::AppState;
use axum::routing::{delete, get, post};
use axum::Router;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/flashcard/folder", post(create_folder_api))
        .route(
            "/flashcard/folder/root/contents",
            get(get_root_folder_contents_api),
        )
        .route(
            "/flashcard/folder/{folder_id}/contents",
            get(get_folder_contents_api),
        )
        .route(
            "/flashcard/folder/{folder_id}/stats",
            get(get_folder_stats_api),
        )
        .route(
            "/flashcard/folder/{folder_id}/review/queue",
            get(get_folder_review_queue_api),
        )
        .route("/flashcard/folder/{folder_id}", delete(delete_folder_api))
}
