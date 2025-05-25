mod models;
mod handlers;
mod config;

use actix_web::{web, App, HttpServer};
use handlers::{book_handler, user_handler};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let pool = config::database::establish_connection()
        .await
        .expect("Failed to create pool");

    println!("Server running at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/books")
                            .route("", web::get().to(book_handler::list_books))
                            .route("", web::post().to(book_handler::create_book))
                            .route("/{id}", web::get().to(book_handler::get_book))
                            .route("/{id}", web::put().to(book_handler::update_book))
                            .route("/{id}", web::delete().to(book_handler::delete_book)),
                    )
                    .service(
                        web::scope("/auth")
                            .route("/register", web::post().to(user_handler::register))
                            .route("/login", web::post().to(user_handler::login))
                            .route("/logout", web::post().to(user_handler::logout)),
                    ),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
