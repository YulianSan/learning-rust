use axum::{extract::{State, Path}, http::StatusCode, Json};
use bcrypt::{hash, DEFAULT_COST};
use sqlx::{Pool, Postgres};

use super::models::{RegisterUser, User};

async fn get_all(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let result = sqlx::query!("SELECT * FROM users").fetch_all(&pool).await;

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
) -> Result<Json<User>, (StatusCode, String)> {
    let hashed = hash(user.password, DEFAULT_COST).expect("Failed to hash password");
    let result = sqlx::query!(
        "INSERT INTO users(name, email, password) VALUES ($1, $2, $3) RETURNING id, name, email, password", 
        user.name, 
        user.email, 
        hashed
    ).fetch_one(&pool).await;

    match result {
        Ok(user) => Ok(Json(User {
            id: user.id,
            name: user.name.clone(),
            email: user.email.clone(),
            password: user.password.clone(),
        })),
        Err(_) => Err((StatusCode::BAD_REQUEST, String::from("error"))),
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
        .route("/", axum::routing::get(get_all))
        .route("/", axum::routing::post(create_user))
        .route("/:id", axum::routing::get(show_user))
        .route("/:id", axum::routing::delete(delete_user));

    axum::Router::new().nest("/user", router)
}
