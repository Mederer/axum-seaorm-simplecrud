use app::models::AppState;
use axum::http::StatusCode;
use axum::routing::{delete, get, post};
use axum::Router;
use sea_orm::Database;
use std::{error::Error, net::SocketAddr, sync::Arc};
use tracing::info;

use app::controllers::{auth_controller, post_controller, user_controller};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().expect("Error reading environment variables.");

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_test_writer()
        .init();

    let db_url = dotenvy::var("DATABASE_URL").expect("No 'DATABASE_URL' var set.");
    let port: u16 = dotenvy::var("PORT")
        .expect("No 'PORT' var set.")
        .parse()
        .expect("Invalid 'PORT' var set.");

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let db = Database::connect(db_url).await.expect(
        "Unable to connect to database. Check your 'DATABASE_URL' var and ensure the database is running.",
    );

    let state = Arc::new(AppState { db });

    let user_router = Router::new()
        .route(
            "/",
            get(user_controller::get_all_users)
                .post(user_controller::create_user)
                .put(user_controller::update_user),
        )
        .route(
            "/:id",
            delete(user_controller::delete_user).get(user_controller::get_user),
        )
        .route("/:id/post", get(user_controller::get_posts));

    let post_router = Router::new().route(
        "/",
        post(post_controller::create_post).get(post_controller::get_all_posts),
    );

    let auth_router = Router::new().route("/", post(auth_controller::authorize));

    let app = Router::new()
        .nest("/user", user_router)
        .nest("/post", post_router)
        .nest("/auth", auth_router)
        .with_state(state)
        .fallback(|| async { (StatusCode::NOT_FOUND, "Resource was not found.") });

    info!("Listening on port {port}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
