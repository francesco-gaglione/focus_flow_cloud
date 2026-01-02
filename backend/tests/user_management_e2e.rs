mod common;

use adapters::http::auth::login::{LoginDto, LoginResponseDto};
use adapters::http::users::create_user::CreateUserDto;
use adapters::http::users::update_password::UpdatePasswordDto;
use adapters::http::users::update_username::UpdateUsernameDto;
use reqwest::StatusCode;

#[tokio::test]
async fn test_update_username_and_password() {
    let ctx = common::setup().await;

    // 1. Create a new user
    let username = "testuser_mgmt";
    let password = "Password123!";
    let create_user_dto = CreateUserDto {
        username: username.to_string(),
        password: password.to_string(),
    };

    let response = ctx
        .client
        .post(format!("{}/api/users", ctx.base_url))
        .json(&create_user_dto)
        .send()
        .await
        .expect("Failed to create user");
    assert_eq!(response.status(), StatusCode::OK);

    // 2. Login
    let client = reqwest::Client::new();
    let login_dto = LoginDto {
        username: username.to_string(),
        password: password.to_string(),
    };

    let response = client
        .post(format!("{}/api/auth/login", ctx.base_url))
        .json(&login_dto)
        .send()
        .await
        .expect("Failed to login");
    assert_eq!(response.status(), StatusCode::OK);
    let login_response: LoginResponseDto = response.json().await.unwrap();

    let user_client = reqwest::Client::builder()
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", login_response.token))
                    .unwrap(),
            );
            headers
        })
        .build()
        .unwrap();

    // 3. Update Username
    let new_username = "new_testuser_mgmt";
    let update_username_dto = UpdateUsernameDto {
        new_username: new_username.to_string(),
    };

    let response = user_client
        .put(format!("{}/api/users/username", ctx.base_url))
        .json(&update_username_dto)
        .send()
        .await
        .expect("Failed to update username");
    assert_eq!(response.status(), StatusCode::OK);

    // 4. Update Password
    let new_password = "NewPassword123!";
    let update_password_dto = UpdatePasswordDto {
        old_password: password.to_string(),
        new_password: new_password.to_string(),
    };

    let response = user_client
        .put(format!("{}/api/users/password", ctx.base_url))
        .json(&update_password_dto)
        .send()
        .await
        .expect("Failed to update password");
    assert_eq!(response.status(), StatusCode::OK);

    // 5. Verify Login with NEW credentials
    let login_dto_new = LoginDto {
        username: new_username.to_string(),
        password: new_password.to_string(),
    };

    let response = client
        .post(format!("{}/api/auth/login", ctx.base_url))
        .json(&login_dto_new)
        .send()
        .await
        .expect("Failed to login with new credentials");
    assert_eq!(response.status(), StatusCode::OK);

    // 6. Verify Login fails with OLD credentials
    let login_dto_old = LoginDto {
        username: username.to_string(),
        password: password.to_string(),
    };

    let response = client
        .post(format!("{}/api/auth/login", ctx.base_url))
        .json(&login_dto_old)
        .send()
        .await
        .expect("Failed to login check");
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_update_username_taken() {
    let ctx = common::setup().await;

    // 1. Create User 1
    let username1 = "user1";
    let password = "Password123!";
    ctx.create_user(&CreateUserDto {
        username: username1.to_string(),
        password: password.to_string(),
    })
    .await;

    // 2. Create User 2
    let username2 = "user2";
    ctx.create_user(&CreateUserDto {
        username: username2.to_string(),
        password: password.to_string(),
    })
    .await;

    // 3. Login as User 2
    let client = reqwest::Client::new();
    let login_dto = LoginDto {
        username: username2.to_string(),
        password: password.to_string(),
    };
    let response = client
        .post(format!("{}/api/auth/login", ctx.base_url))
        .json(&login_dto)
        .send()
        .await
        .expect("Failed to login");
    let login_response: LoginResponseDto = response.json().await.unwrap();

    let user_client = reqwest::Client::builder()
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", login_response.token))
                    .unwrap(),
            );
            headers
        })
        .build()
        .unwrap();

    // 4. Try to update User 2's username to "user1" (already taken)
    let update_username_dto = UpdateUsernameDto {
        new_username: username1.to_string(),
    };

    let response = user_client
        .put(format!("{}/api/users/username", ctx.base_url))
        .json(&update_username_dto)
        .send()
        .await
        .expect("Failed to update username");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_update_password_invalid_old() {
    let ctx = common::setup().await;

    // 1. Create User
    let username = "user_pwd_test";
    let password = "Password123!";
    ctx.create_user(&CreateUserDto {
        username: username.to_string(),
        password: password.to_string(),
    })
    .await;

    // 2. Login
    let client = reqwest::Client::new();
    let login_dto = LoginDto {
        username: username.to_string(),
        password: password.to_string(),
    };
    let response = client
        .post(format!("{}/api/auth/login", ctx.base_url))
        .json(&login_dto)
        .send()
        .await
        .unwrap();
    let login_response: LoginResponseDto = response.json().await.unwrap();

    let user_client = reqwest::Client::builder()
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", login_response.token))
                    .unwrap(),
            );
            headers
        })
        .build()
        .unwrap();

    // 3. Try to update password with WRONG old password
    let update_password_dto = UpdatePasswordDto {
        old_password: "WrongPassword!".to_string(),
        new_password: "NewPassword123!".to_string(),
    };

    let response = user_client
        .put(format!("{}/api/users/password", ctx.base_url))
        .json(&update_password_dto)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
