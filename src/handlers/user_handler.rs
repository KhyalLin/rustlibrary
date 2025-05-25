use actix_web::{web, HttpResponse, Responder, http::header::HeaderValue, HttpRequest};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use uuid::Uuid;
use chrono::{Utc, Duration, DateTime};

use crate::models::user::{User, CreateUser, LoginUser, UserResponse};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn register(
    pool: web::Data<MySqlPool>,
    user: web::Json<CreateUser>,
) -> impl Responder {
    let hashed_password = hash(user.password.as_bytes(), DEFAULT_COST).unwrap();
    let user_id = Uuid::new_v4();
    let now = Utc::now();

    match sqlx::query!(
        r#"
        INSERT INTO users (id, username, password, email, password_hash, role, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        user_id,
        user.username,
        hashed_password,
        user.email,
        hashed_password,
        "user", // 默认角色
        now,
        now
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            let user_response = UserResponse {
                id: user_id,
                username: user.username.clone(),
                email: user.email.clone(),
                role: "user".to_string(),
                created_at: now,
                updated_at: now,
            };
            HttpResponse::Created().json(user_response)
        }
        Err(e) => {
            eprintln!("Error creating user: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn login(
    pool: web::Data<MySqlPool>,
    credentials: web::Json<LoginUser>,
) -> impl Responder {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT * FROM users WHERE username = ?
        "#,
        credentials.username
    )
    .fetch_optional(pool.get_ref())
    .await;

    match user {
        Ok(Some(user)) => {
            if verify(&credentials.password, &user.password).unwrap_or(false) {
                let expiration = Utc::now()
                    .checked_add_signed(Duration::hours(24))
                    .expect("valid timestamp")
                    .timestamp() as usize;

                let claims = Claims {
                    sub: user.id.to_string(),
                    exp: expiration,
                };

                let token = encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret(b"your-secret-key"),
                )
                .unwrap();

                // 存储 token 到数据库
                let token_id = Uuid::new_v4();
                let expires_at = Utc::now()
                    .checked_add_signed(Duration::hours(24))
                    .expect("valid timestamp");
                let now = Utc::now();

                match sqlx::query!(
                    r#"
                    INSERT INTO tokens (id, user_id, token, expires_at, created_at)
                    VALUES (?, ?, ?, ?, ?)
                    "#,
                    token_id,
                    user.id,
                    token,
                    expires_at,
                    now
                )
                .execute(pool.get_ref())
                .await
                {
                    Ok(_) => HttpResponse::Ok().json(serde_json::json!({
                        "token": token,
                        "expires_at": expires_at
                    })),
                    Err(e) => {
                        eprintln!("Error storing token: {}", e);
                        HttpResponse::InternalServerError().finish()
                    }
                }
            } else {
                HttpResponse::Unauthorized().finish()
            }
        }
        _ => HttpResponse::Unauthorized().finish(),
    }
}

pub async fn logout(
    pool: web::Data<MySqlPool>,
    req: HttpRequest,
) -> impl Responder {
    let token_str = match req.headers().get("Authorization") {
        Some(header_value) => {
            let value = header_value.to_str().unwrap_or("");
            if value.starts_with("Bearer ") {
                &value[7..]
            } else {
                value
            }
        }
        None => return HttpResponse::BadRequest().finish(),
    };

    match sqlx::query!(
        r#"
        DELETE FROM tokens WHERE token = ?
        "#,
        token_str
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            eprintln!("Error deleting token: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
} 