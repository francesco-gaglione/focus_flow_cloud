use crate::flashcards::http::flashcards::create_flashcard::create_flashcard_api;
use crate::flashcards::http::flashcards::delete_flashcard::delete_flashcard_api;
use crate::flashcards::http::flashcards::get_activity_heatmap::get_activity_heatmap_api;
use crate::flashcards::http::flashcards::get_due_flashcards::get_due_flashcards_api;
use crate::flashcards::http::flashcards::get_flashcard::get_flashcard_api;
use crate::flashcards::http::flashcards::get_flashcard_stats::get_flashcard_stats_api;
use crate::flashcards::http::flashcards::review_flashcard::review_flashcard_api;
use crate::flashcards::http::flashcards::update_flashcard::update_flashcard_api;
use crate::shared::http::app_state::AppState;
use axum::routing::{delete, get, post, put};
use axum::Router;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/flashcard", post(create_flashcard_api))
        .route("/flashcard/due", get(get_due_flashcards_api))
        .route("/flashcard/stats", get(get_flashcard_stats_api))
        .route("/flashcard/stats/activity", get(get_activity_heatmap_api))
        .route("/flashcard/{id}", get(get_flashcard_api))
        .route("/flashcard/{id}", put(update_flashcard_api))
        .route("/flashcard/{id}", delete(delete_flashcard_api))
        .route("/flashcard/{id}/review", post(review_flashcard_api))
}
