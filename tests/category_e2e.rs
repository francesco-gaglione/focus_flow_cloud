mod common;

use crate::common::setup;
use focus_flow_cloud::adapters::http::dto::category_api::create_category::CreateCategoryDto;
use focus_flow_cloud::adapters::http::dto::category_api::get_categories::GetCategoriesResponseDto;
use focus_flow_cloud::adapters::http::dto::category_api::get_category::GetCategoryResponseDto;
use focus_flow_cloud::adapters::http::dto::category_api::update_category::{
    UpdateCategoryDto, UpdateCategoryResponseDto,
};

#[tokio::test]
async fn create_and_list_category() {
    let context = setup().await;

    let create_dto = CreateCategoryDto {
        name: "Work".to_string(),
        description: Some("Work related tasks".to_string()),
        color: Some("#FF5733".to_string()),
    };

    // Create Category
    let _ = context.create_category(&create_dto).await;

    // List Categories
    let response = context
        .client
        .get(format!("{}/api/categories", context.base_url))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), 200);
    let body: GetCategoriesResponseDto = response
        .json()
        .await
        .expect("Failed to deserialize response");

    assert!(!body.categories.is_empty());
    let category = body
        .categories
        .iter()
        .find(|c| c.name == "Work")
        .expect("Category not found");
    assert_eq!(category.description, Some("Work related tasks".to_string()));
    assert_eq!(category.color, "#FF5733");
}

#[tokio::test]
async fn create_category_only_mandatory_fields_and_list_categories() {
    let context = setup().await;

    let create_dto = CreateCategoryDto {
        name: "Mandatory fields".to_string(),
        description: None,
        color: Some("#FF5733".to_string()),
    };

    let _ = context.create_category(&create_dto).await;

    let response = context
        .client
        .get(format!("{}/api/categories", context.base_url))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), 200);
    let body: GetCategoriesResponseDto = response
        .json()
        .await
        .expect("Failed to deserialize response");

    assert!(!body.categories.is_empty());
    let category = body
        .categories
        .iter()
        .find(|c| c.name == "Mandatory fields")
        .expect("Category not found");
    assert_eq!(category.description, None);
    assert_eq!(category.color, "#FF5733");
}

#[tokio::test]
async fn update_category_and_get_by_id() {
    let context = setup().await;

    // Create category and check it is created
    let create_dto = CreateCategoryDto {
        name: "Mandatory fields".to_string(),
        description: None,
        color: Some("#FF5733".to_string()),
    };
    let body_response = context.create_category(&create_dto).await;

    assert!(body_response.category_id.len() > 0);

    // Update category and check it is updated
    let update_dto = UpdateCategoryDto {
        name: Some("Updated Mandatory fields".to_string()),
        description: Some("Updated description".to_string()),
        color: Some("#00FF00".to_string()),
    };
    let response = context
        .client
        .put(format!(
            "{}/api/categories/{}",
            context.base_url, body_response.category_id
        ))
        .json(&update_dto)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), 200);
    let body_response: UpdateCategoryResponseDto = response
        .json()
        .await
        .expect("Failed to deserialize message");

    assert_eq!(
        body_response.updated_category.name,
        "Updated Mandatory fields"
    );
    assert_eq!(
        body_response.updated_category.description,
        Some("Updated description".to_string())
    );
    assert_eq!(body_response.updated_category.color, "#00FF00".to_string());

    // Get category by id and check the response
    let response = context
        .client
        .get(format!(
            "{}/api/categories/{}",
            context.base_url, body_response.updated_category.id
        ))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), 200);
    let body_response: GetCategoryResponseDto = response
        .json()
        .await
        .expect("Failed to deserialize message");

    assert_eq!(body_response.category.name, "Updated Mandatory fields");
    assert_eq!(
        body_response.category.description,
        Some("Updated description".to_string())
    );
    assert_eq!(body_response.category.color, "#00FF00".to_string());
}

#[tokio::test]
async fn delete_category() {
    let context = setup().await;

    // Create a new category
    let create_dto = CreateCategoryDto {
        name: "Test Category".to_string(),
        description: Some("Test description".to_string()),
        color: Some("#FF0000".to_string()),
    };
    let body_response = context.create_category(&create_dto).await;

    // Delete the category
    let response = context
        .client
        .delete(format!(
            "{}/api/categories/{}",
            context.base_url, body_response.category_id
        ))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), 200);

    // Verify that the category was deleted
    let response = context
        .client
        .get(format!(
            "{}/api/categories/{}",
            context.base_url, body_response.category_id
        ))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), 404);
}
