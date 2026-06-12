use crate::flashcards::http::flashcards::create_flashcard::create_flashcard_api;
use crate::shared::http::app_state::AppState;
use axum::routing::post;
use axum::Router;

pub fn routes() -> Router<AppState> {
    Router::new().route("/flashcard", post(create_flashcard_api))
}
