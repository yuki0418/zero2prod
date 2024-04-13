use std::collections::HashSet;

use actix_web::dev::Response;
use reqwest::header::HeaderValue;

use crate::helpers::{assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn an_error_flash_message_is_set_on_failure() {
    let app = spawn_app().await;

    // Act
    let login_form = serde_json::json!({
      "username": "unknown_user",
      "password": "password"
    });
    let response = app.post_login(&login_form).await;

    // Assert - Try to login
    assert_is_redirect_to(&response, "/login");

    let flash_cookie = response.cookies().find(|c| c.name() == "_flash").unwrap();
    assert_eq!(flash_cookie.value(), "Authentication failed.");

    // Act 2 - Follow the redirect
    let html_page = app.get_login_html().await;
    assert!(html_page.contains(r#"<p><i>Authentication failed.</i></p>"#));

    // Act 3 - Reload the login page
    let html_page = app.get_login_html().await;
    assert!(!html_page.contains(r#"<p><i>Authentication failed.</i></p>"#));
}
