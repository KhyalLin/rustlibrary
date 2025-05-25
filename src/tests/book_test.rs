use actix_web::{test, web, App, HttpResponse, dev::Service};
use sqlx::MySql;
use uuid::Uuid;
use crate::{
    models::book::{Book, CreateBook, UpdateBook},
    handlers::book_handler::{create_book, get_book, update_book, delete_book, list_books},
    config::database::init_test_pool,
};

async fn setup_test_app() -> impl Service<actix_web::dev::ServiceRequest, Response = actix_web::dev::ServiceResponse, Error = actix_web::Error> {
    let pool = init_test_pool().await.expect("Failed to create test database pool");
    
    test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .service(
                web::scope("/api")
                    .service(web::resource("/books").route(web::post().to(create_book)))
                    .service(web::resource("/books/{id}").route(web::get().to(get_book)))
                    .service(web::resource("/books/{id}").route(web::put().to(update_book)))
                    .service(web::resource("/books/{id}").route(web::delete().to(delete_book)))
                    .service(web::resource("/books").route(web::get().to(list_books)))
            )
    ).await
}

#[actix_rt::test]
async fn test_create_book() {
    let app = setup_test_app().await;

    let book_data = CreateBook {
        title: format!("Test Book {}", Uuid::new_v4()),
        author: "Test Author".to_string(),
        isbn: format!("{}", Uuid::new_v4()),
        description: Some("Test Description".to_string()),
        r#type: "test".to_string(),
        quantity: 10,
    };

    let resp = test::TestRequest::post()
        .uri("/api/books")
        .set_json(&book_data)
        .send_request(&app)
        .await;

    assert!(resp.status().is_success());
    
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["id"].is_string());
}

#[actix_rt::test]
async fn test_get_book() {
    let app = setup_test_app().await;

    // 先创建一本书
    let book_data = CreateBook {
        title: format!("Test Book {}", Uuid::new_v4()),
        author: "Test Author".to_string(),
        isbn: format!("{}", Uuid::new_v4()),
        description: Some("Test Description".to_string()),
        r#type: "test".to_string(),
        quantity: 10,
    };

    let create_resp = test::TestRequest::post()
        .uri("/api/books")
        .set_json(&book_data)
        .send_request(&app)
        .await;

    let created_book: serde_json::Value = test::read_body_json(create_resp).await;
    let book_id = Uuid::parse_str(created_book["id"].as_str().unwrap()).unwrap();

    // 测试获取图书
    let get_resp = test::TestRequest::get()
        .uri(&format!("/api/books/{}", book_id))
        .send_request(&app)
        .await;

    assert!(get_resp.status().is_success());
    
    let body: serde_json::Value = test::read_body_json(get_resp).await;
    assert_eq!(body["id"], book_id.to_string());
}

#[actix_rt::test]
async fn test_update_book() {
    let app = setup_test_app().await;

    // 先创建一本书
    let book_data = CreateBook {
        title: format!("Test Book {}", Uuid::new_v4()),
        author: "Test Author".to_string(),
        isbn: format!("{}", Uuid::new_v4()),
        description: Some("Test Description".to_string()),
        r#type: "test".to_string(),
        quantity: 10,
    };

    let create_resp = test::TestRequest::post()
        .uri("/api/books")
        .set_json(&book_data)
        .send_request(&app)
        .await;

    let created_book: serde_json::Value = test::read_body_json(create_resp).await;
    let book_id = Uuid::parse_str(created_book["id"].as_str().unwrap()).unwrap();

    // 测试更新图书
    let update_data = UpdateBook {
        title: Some("Updated Title".to_string()),
        author: None,
        isbn: None,
        description: None,
        r#type: None,
        quantity: None,
    };

    let update_resp = test::TestRequest::put()
        .uri(&format!("/api/books/{}", book_id))
        .set_json(&update_data)
        .send_request(&app)
        .await;

    assert!(update_resp.status().is_success());
    
    let body: serde_json::Value = test::read_body_json(update_resp).await;
    assert_eq!(body["title"], "Updated Title");
}

#[actix_rt::test]
async fn test_delete_book() {
    let app = setup_test_app().await;

    // 先创建一本书
    let book_data = CreateBook {
        title: format!("Test Book {}", Uuid::new_v4()),
        author: "Test Author".to_string(),
        isbn: format!("{}", Uuid::new_v4()),
        description: Some("Test Description".to_string()),
        r#type: "test".to_string(),
        quantity: 10,
    };

    let create_resp = test::TestRequest::post()
        .uri("/api/books")
        .set_json(&book_data)
        .send_request(&app)
        .await;

    let created_book: serde_json::Value = test::read_body_json(create_resp).await;
    let book_id = Uuid::parse_str(created_book["id"].as_str().unwrap()).unwrap();

    // 测试删除图书
    let delete_resp = test::TestRequest::delete()
        .uri(&format!("/api/books/{}", book_id))
        .send_request(&app)
        .await;

    assert_eq!(delete_resp.status(), 204);
}

#[actix_rt::test]
async fn test_list_books() {
    let app = setup_test_app().await;

    // 先创建几本书
    for _ in 0..3 {
        let book_data = CreateBook {
            title: format!("Test Book {}", Uuid::new_v4()),
            author: "Test Author".to_string(),
            isbn: format!("{}", Uuid::new_v4()),
            description: Some("Test Description".to_string()),
            r#type: "test".to_string(),
            quantity: 10,
        };

        let create_resp = test::TestRequest::post()
            .uri("/api/books")
            .set_json(&book_data)
            .send_request(&app)
            .await;

        assert!(create_resp.status().is_success());
    }

    // 测试分页查询
    let list_resp = test::TestRequest::get()
        .uri("/api/books?page_no=1&page_size=10")
        .send_request(&app)
        .await;

    assert!(list_resp.status().is_success());
    
    let body: serde_json::Value = test::read_body_json(list_resp).await;
    assert!(body["total"].is_number());
    assert!(body["data"].is_array());

    // 测试带条件的查询
    let search_resp = test::TestRequest::get()
        .uri("/api/books?title=Test&author=Author&page_no=1&page_size=10")
        .send_request(&app)
        .await;

    assert!(search_resp.status().is_success());
    
    let body: serde_json::Value = test::read_body_json(search_resp).await;
    assert!(body["total"].is_number());
    assert!(body["data"].is_array());
} 