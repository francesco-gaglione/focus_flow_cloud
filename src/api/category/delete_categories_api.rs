use crate::api::api_error::ApiError;
use crate::config::app_state::AppState;
use crate::config::openapi::CATEGORY_TAG;
use crate::dto::category_api::delete_categories::{
    DeleteCategoriesDto, DeleteCategoriesResponseDto,
};
use crate::dto::task_api::delete_task::{DeleteTasksDto, DeleteTasksResponseDto};
use axum::Json;
use axum::extract::State;
use validator::Validate;

#[utoipa::path(
    delete,
    path = "/category/deleteCategories",
    tag = CATEGORY_TAG,
    request_body = DeleteCategoriesDto,
    description = "Delete categories and their tasks",
    responses(
        (status = 200, description = "Categories deleted successfully", body = DeleteCategoriesResponseDto),
        (status = 400, description = "Bad request - validation error"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_categories_api(
    State(state): State<AppState>,
    Json(payload): Json<DeleteCategoriesDto>,
) -> Result<Json<DeleteCategoriesResponseDto>, ApiError> {
    payload.validate()?;

    let res = state
        .category_service
        .delete_categories(
            payload
                .category_ids
                .iter()
                .map(|id| id.parse().unwrap()) // should be safe due to dto validation
                .collect(),
        )
        .await;

    log::debug!("{:?}", res);

    match res {
        Ok(res) => Ok(Json(DeleteCategoriesResponseDto {
            deleted_ids: res.iter().map(|id| id.to_string()).collect(),
        })),
        Err(err) => {
            log::error!("{:?}", err);
            Err(ApiError::from(err))
        }
    }
}
