use app::models::{AuthError, Credentials};
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post};
use axum::{response::IntoResponse, Router};
use entity::user::{self, NewUser, UserNoSecrets};
use migration::Condition;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use serde_json::{json, Value};
use std::sync::Arc;
use std::{error::Error, net::SocketAddr};

struct AppState {
    db: DatabaseConnection,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().expect("Error reading environment variables.");

    let db_url = dotenvy::var("DATABASE_URL").expect("No 'DATABASE_URL' var set.");
    let port: u16 = dotenvy::var("PORT")
        .expect("No 'PORT' var set.")
        .parse()
        .expect("Invalid 'PORT' var set.");

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let db = Database::connect(db_url)
        .await
        .expect("Error connecting to database!");

    let state = Arc::new(AppState { db });

    let user_router = Router::new()
        .route("/", get(get_all_users).post(create_user).put(update_user))
        .route("/login", post(login))
        .route("/:id", delete(delete_user).get(get_user));

    let app = Router::new()
        .nest("/user", user_router)
        .with_state(state)
        .fallback(|| async { (StatusCode::NOT_FOUND, "Resource was not found") });

    println!("Listening on {}", port);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn get_user(State(state): State<Arc<AppState>>, Path(id): Path<i32>) -> impl IntoResponse {
    let db = &state.db;

    let user = user::Entity::find_by_id(id)
        .into_model::<UserNoSecrets>()
        .one(db)
        .await;

    let user = if let Ok(Some(user)) = user {
        user
    } else {
        return Json(json!({
            "success": false,
            "message": "Couldnt find user"
        }));
    };

    Json(json!({
        "success": true,
        "user": user
    }))
}

async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<NewUser>,
) -> Result<Json<Value>, StatusCode> {
    let db = &state.db;

    let new_user = user::ActiveModel {
        firstname: Set(payload.firstname),
        lastname: Set(payload.lastname),
        email: Set(payload.email),
        secret: Set(payload.secret),
        ..Default::default()
    }
    .insert(db)
    .await;

    if let Ok(new_user) = new_user {
        return Ok(Json(json!({
            "status": "success",
            "user": UserNoSecrets::from(new_user)
        })));
    } else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
}

async fn get_all_users(State(state): State<Arc<AppState>>) -> Json<Value> {
    let db = &state.db;

    let users = user::Entity::find()
        .into_model::<UserNoSecrets>()
        .all(db)
        .await
        .unwrap();

    Json(json!({ "users": users }))
}

async fn login(
    State(state): State<Arc<AppState>>,
    Json(credentials): Json<Credentials>,
) -> Result<Json<Value>, AuthError> {
    let db = &state.db;

    let user = user::Entity::find()
        .filter(
            Condition::all()
                .add(user::Column::Email.eq(credentials.email))
                .add(user::Column::Secret.eq(credentials.secret)),
        )
        .into_model::<UserNoSecrets>()
        .one(db)
        .await;

    if let Ok(Some(user)) = user {
        return Ok(Json(json!({
            "status": "success",
            "message": "Successful login",
            "user": user,
        })));
    } else {
        return Err(AuthError::InvalidCredentials);
    }
}

async fn update_user(
    State(state): State<Arc<AppState>>,
    Json(updated_user): Json<UserNoSecrets>,
) -> Result<Json<Value>, impl IntoResponse> {
    let db = &state.db;

    let user = user::Entity::find_by_id(updated_user.id).one(db).await;
    let user = if let Ok(Some(user)) = user {
        user
    } else {
        return Err((StatusCode::NOT_FOUND, "User not found"));
    };
    let mut user: user::ActiveModel = user.into();
    user.firstname = Set(updated_user.firstname);
    user.lastname = Set(updated_user.lastname);
    user.email = Set(updated_user.email);
    let user = user.update(db).await;

    if let Ok(user) = user {
        return Ok(Json(json!({
            "success": true,
            "user": user,
        })));
    } else {
        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "An error occured while processing this request.",
        ))
    }
}

async fn delete_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let db = &state.db;

    let result = user::Entity::delete_by_id(id).exec(db).await.unwrap();

    if result.rows_affected > 0 {
        Ok(Json(json!({
            "success": true,
            "message": format!("User {id} deleted.")
        })))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "message": "User not found."
            })),
        ))
    }
}
