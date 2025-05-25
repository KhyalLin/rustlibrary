use actix_web::{test, web, App, HttpResponse};
use sqlx::MySqlPool;
use crate::{
    models::user::{User, CreateUser, LoginUser},
    handlers::user_handler::{register, login, logout},
    config::database::init_pool,
};

async fn setup_test_app() -> (App<()>, MySqlPool) {
    let pool = init_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::resource("/users/register").route(web::post().to(register)))
            .service(web::resource("/users/login").route(web::post().to(login)))
            .service(web::resource("/users/logout").route(web::post().to(logout)))
    ).await;
    (app, pool)
}

#[actix_rt::test]
async fn test_user_register() {
    let (app, _pool) = setup_test_app().await;

    let user_data = CreateUser {
        username: format!("testuser_{}", uuid::Uuid::new_v4()),
        password: "testpass123".to_string(),
        email: format!("test_{}@example.com", uuid::Uuid::new_v4()),
    };

    let resp = test::TestRequest::post()
        .uri("/users/register")
        .set_json(&user_data)
        .send_request(&app)
        .await;

    assert!(resp.status().is_success());
    
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["message"].is_string());
}

#[actix_rt::test]
async fn test_user_login() {
    let (app, _pool) = setup_test_app().await;

    // 先注册用户
    let user_data = CreateUser {
        username: format!("testuser_{}", uuid::Uuid::new_v4()),
        password: "testpass123".to_string(),
        email: format!("test_{}@example.com", uuid::Uuid::new_v4()),
    };

    let register_resp = test::TestRequest::post()
        .uri("/users/register")
        .set_json(&user_data)
        .send_request(&app)
        .await;

    assert!(register_resp.status().is_success());

    // 测试登录
    let login_data = LoginUser {
        username: user_data.username.clone(),
        password: user_data.password.clone(),
    };

    let login_resp = test::TestRequest::post()
        .uri("/users/login")
        .set_json(&login_data)
        .send_request(&app)
        .await;

    assert!(login_resp.status().is_success());
    
    let body: serde_json::Value = test::read_body_json(login_resp).await;
    assert!(body["token"].is_string());
}

#[actix_rt::test]
async fn test_user_logout() {
    let (app, _pool) = setup_test_app().await;

    // 先注册并登录用户
    let user_data = CreateUser {
        username: format!("testuser_{}", uuid::Uuid::new_v4()),
        password: "testpass123".to_string(),
        email: format!("test_{}@example.com", uuid::Uuid::new_v4()),
    };

    let register_resp = test::TestRequest::post()
        .uri("/users/register")
        .set_json(&user_data)
        .send_request(&app)
        .await;

    assert!(register_resp.status().is_success());

    let login_data = LoginUser {
        username: user_data.username.clone(),
        password: user_data.password.clone(),
    };

    let login_resp = test::TestRequest::post()
        .uri("/users/login")
        .set_json(&login_data)
        .send_request(&app)
        .await;

    let login_body: serde_json::Value = test::read_body_json(login_resp).await;
    let token = login_body["token"].as_str().unwrap();

    // 测试登出
    let logout_resp = test::TestRequest::post()
        .uri("/users/logout")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .send_request(&app)
        .await;

    assert!(logout_resp.status().is_success());
    
    let body: serde_json::Value = test::read_body_json(logout_resp).await;
    assert!(body["message"].is_string());
} 