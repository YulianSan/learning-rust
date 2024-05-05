use std::cmp;
use axum::{routing::{get, post, delete, put}, extract::{State, Path, Query}, http::StatusCode, Json, middleware};
use bcrypt::{hash, DEFAULT_COST};
use sqlx::{Pool, Postgres};
use serde::Deserialize;

use crate::services;

use super::models::{RegisterUser, User, UpdateUser};

#[derive(Deserialize)]
struct Pagination {
    page: Option<i64>,
    per_page: Option<i64>,
}

async fn get_all(
    State(pool): State<Pool<Postgres>>,
    pagination: Query<Pagination>
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let limit = pagination.per_page.unwrap_or(15);
    let skip = match pagination.page {
        Some(skip) => cmp::max(skip - 1, 0) * limit,
        None => 0
    };

    let result = sqlx::query!(
        "SELECT * FROM users limit $1 offset $2", limit, skip
    ).fetch_all(&pool).await;

    match result {
        Ok(users) => Ok(Json(
            users
                .iter()
                .map(|user| User {
                    id: user.id,
                    name: user.name.clone(),
                    email: user.email.clone(),
                    password: user.password.clone(),
                })
                .collect::<Vec<User>>(),
        )),
        Err(_) => Err((StatusCode::BAD_REQUEST, String::from("error"))),
    }
}

async fn create_user(
    State(pool): State<Pool<Postgres>>,
    Json(user): Json<RegisterUser>,
) -> Result<(StatusCode, Json<User>), (StatusCode, String)> {
    let hashed = hash(user.password, DEFAULT_COST).expect("Failed to hash password");
    let result = sqlx::query!(
        "INSERT INTO users(name, email, password) VALUES ($1, $2, $3) RETURNING id, name, email, password", 
        user.name, 
        user.email, 
        hashed
    ).fetch_one(&pool).await;

    match result {
        Ok(user) => Ok((
            StatusCode::CREATED, 
            Json(User {
                id: user.id,
                name: user.name.clone(),
                email: user.email.clone(),
                password: user.password.clone(),
            })
        )),
        Err(_) => Err((StatusCode::BAD_REQUEST, String::from("error"))),
    }
}

async fn update_user(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(user): Json<UpdateUser>
) -> Result<(StatusCode, Json<User>), (StatusCode, String)> {
    let user_exists = sqlx::query!(
        "SELECT COUNT(*) total FROM users u WHERE u.id = $1",
        id
    ).fetch_one(&pool).await;

    match user_exists {
        Ok(num) => {
            if num.total.unwrap() < 1 {
                return Err((StatusCode::NOT_FOUND, "User not found".to_string()))
            }
        },
        Err(_) => {
            return Err((StatusCode::BAD_REQUEST, "Error".to_string()))
        }
    }

    let hashed = hash(user.password, DEFAULT_COST).expect("Failed to hash password");
    let result = sqlx::query!(
        "UPDATE users SET name = $1, email = $2, password = $3 RETURNING id, name, email, password",
        user.name,
        user.email,
        hashed
    ).fetch_one(&pool).await;

    match result {
        Ok(user) => Ok(
            (            
                StatusCode::OK,
                Json(User {
                    id: user.id,
                    name: user.name.clone(),
                    email: user.email.clone(),
                    password: user.password.clone(),
                })
            )        ),
        Err(_) => Err((StatusCode::BAD_REQUEST, "Error".to_string()))
    }
}

async fn show_user(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<Json<User>, (StatusCode, String)> {
    let result = sqlx::query!(
        "SELECT * FROM users u WHERE u.id = $1 limit 1", 
        id
    ).fetch_one(&pool).await;

    match result {
        Ok(user) => Ok(Json(User {
            id: user.id,
            name: user.name.clone(),
            email: user.email.clone(),
            password: user.password.clone(),
        })),
        Err(_) => Err((StatusCode::NOT_FOUND, String::from("User not found")))
    }
}

async fn delete_user(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let result = sqlx::query!(
        "DELETE FROM users u WHERE u.id = $1", 
        id
    ).execute(&pool).await;

    match result {
        Ok(lines) => match lines.rows_affected() {
            0 => Err((StatusCode::NOT_FOUND, String::from("User not found"))),
            _ => Ok((StatusCode::OK, String::from("User deleted"))),
        },
        Err(_) => Err((StatusCode::BAD_REQUEST, String::from("Error")))
    }
}

pub fn routes () -> axum::Router<Pool<Postgres>> {
    let router = axum::Router::new()
        .route("/", get(get_all))
        .route("/", post(create_user))
        .route("/:id", get(show_user))
        .route("/:id", put(update_user))
        .route("/:id", delete(delete_user))
        .layer(middleware::from_fn(services::middleware::auth::auth));

    axum::Router::new().nest("/user", router)
}
