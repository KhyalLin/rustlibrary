use actix_web::{web, HttpResponse, Responder};
use sqlx::MySqlPool;
use uuid::Uuid;
use chrono::Utc;
use serde::Deserialize;

use crate::models::book::{Book, CreateBook, UpdateBook};

pub async fn create_book(
    pool: web::Data<MySqlPool>,
    book: web::Json<CreateBook>,
) -> impl Responder {
    let book_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().naive_local();

    match sqlx::query!(
        r#"
        INSERT INTO books (id, title, author, isbn, description, type, quantity, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        book_id,
        book.title,
        book.author,
        book.isbn,
        book.description,
        book.r#type,
        book.quantity,
        now,
        now
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            let new_book = Book {
                id: book_id.clone(),
                title: book.title.clone(),
                author: book.author.clone(),
                isbn: book.isbn.clone(),
                description: book.description.clone(),
                r#type: book.r#type.clone(),
                quantity: book.quantity,
                created_at: now,
                updated_at: now,
            };
            HttpResponse::Created().json(new_book)
        }
        Err(e) => {
            eprintln!("Error creating book: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_book(
    pool: web::Data<MySqlPool>,
    book_id: web::Path<Uuid>,
) -> impl Responder {
    match sqlx::query_as!(
        Book,
        r#"
        SELECT * FROM books WHERE id = ?
        "#,
        book_id.to_string()
    )
    .fetch_optional(pool.get_ref())
    .await
    {
        Ok(Some(book)) => HttpResponse::Ok().json(book),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => {
            eprintln!("Error fetching book: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn update_book(
    pool: web::Data<MySqlPool>,
    book_id: web::Path<Uuid>,
    book_update: web::Json<UpdateBook>,
) -> impl Responder {
    let now = Utc::now().naive_local();

    let update_query = sqlx::query!(
        r#"
        UPDATE books
        SET title = COALESCE(?, title),
            author = COALESCE(?, author),
            isbn = COALESCE(?, isbn),
            description = COALESCE(?, description),
            type = COALESCE(?, type),
            quantity = COALESCE(?, quantity),
            updated_at = ?
        WHERE id = ?
        "#,
        book_update.title,
        book_update.author,
        book_update.isbn,
        book_update.description,
        book_update.r#type,
        book_update.quantity,
        now,
        book_id.to_string()
    )
    .execute(pool.get_ref())
    .await;

    match update_query {
        Ok(_) => {
            match sqlx::query_as!(
                Book,
                r#"
                SELECT * FROM books WHERE id = ?
                "#,
                book_id.to_string()
            )
            .fetch_optional(pool.get_ref())
            .await
            {
                Ok(Some(book)) => HttpResponse::Ok().json(book),
                _ => HttpResponse::NotFound().finish(),
            }
        }
        Err(e) => {
            eprintln!("Error updating book: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn delete_book(
    pool: web::Data<MySqlPool>,
    book_id: web::Path<Uuid>,
) -> impl Responder {
    match sqlx::query!(
        r#"
        DELETE FROM books WHERE id = ?
        "#,
        book_id.to_string()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            eprintln!("Error deleting book: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct BookQuery {
    pub page_no: Option<i64>,
    pub page_size: Option<i64>,
    pub id: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
}

pub async fn list_books(
    pool: web::Data<MySqlPool>,
    query: web::Query<BookQuery>,
) -> impl Responder {
    let page_no = query.page_no.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);
    let offset = (page_no - 1) * page_size;

    let mut conditions = Vec::new();
    let mut params: Vec<String> = Vec::new();

    if let Some(id) = &query.id {
        conditions.push("id = ?");
        params.push(id.clone());
    }
    if let Some(title) = &query.title {
        conditions.push("title LIKE ?");
        params.push(format!("%{}%", title));
    }
    if let Some(author) = &query.author {
        conditions.push("author LIKE ?");
        params.push(format!("%{}%", author));
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // 构建查询语句
    let query_str = format!(
        "SELECT * FROM books {} ORDER BY created_at DESC LIMIT ? OFFSET ?",
        where_clause
    );

    // 添加分页参数
    params.push(page_size.to_string());
    params.push(offset.to_string());

    // 执行查询
    let mut query_builder = sqlx::query_as::<_, Book>(&query_str);
    for param in &params {
        query_builder = query_builder.bind(param);
    }
    let books = query_builder
        .fetch_all(pool.get_ref())
        .await;

    match books {
        Ok(books) => {
            let count_query = format!(
                "SELECT COUNT(*) as total FROM books {}",
                where_clause
            );
            let mut count_query_builder = sqlx::query_scalar::<_, i64>(&count_query);
            for param in &params[..params.len()-2] { // 移除分页参数
                count_query_builder = count_query_builder.bind(param);
            }
            let total: i64 = count_query_builder
                .fetch_one(pool.get_ref())
                .await
                .unwrap_or(0);

            HttpResponse::Ok().json(serde_json::json!({
                "total": total,
                "page_no": page_no,
                "page_size": page_size,
                "data": books
            }))
        }
        Err(e) => {
            eprintln!("Error fetching books: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
} 