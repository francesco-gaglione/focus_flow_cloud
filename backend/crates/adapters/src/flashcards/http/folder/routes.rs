use crate::flashcards::http::folder::get_folder_content::get_folder_contents_api;
use crate::flashcards::http::folder::get_root_folder_content::get_root_folder_contents_api;
use crate::shared::http::app_state::AppState;
use axum::routing::get;
use axum::Router;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/flashcard/folder/root/contents",
            get(get_root_folder_contents_api),
        )
        .route(
            "/flashcard/folder/{folder_id}/contents",
            get(get_folder_contents_api),
        )
}
