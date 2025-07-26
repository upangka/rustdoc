use axum::http::StatusCode;
use axum::{
    Json, Router,
    response::{IntoResponse, Response},
    routing::get,
    routing::post,
};
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // 初始化日志订阅
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum_practice=info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer()) // 实际打印日志的格式层
        .init();

    let app = Router::new()
        .route("/test", post(create_user))
        .route("/", get(hello_handler))
        .layer(TraceLayer::new_for_http());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello_handler() -> &'static str {
    "Hello, World!"
}

#[derive(Debug, Deserialize, Serialize)]
struct NewUser {
    email: String,
    username: String,
}

#[derive(Debug, Clone)]
pub struct HttpError {
    pub message: String,
    pub status: StatusCode,
}

impl Default for HttpError {
    fn default() -> Self {
        HttpError {
            message: "".to_string(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        Default::default()
    }
}

async fn create_user(Json(payload): Json<NewUser>) -> Result<impl IntoResponse, HttpError> {
    info!("Got a request: {:#?}", payload);
    Ok(Json(payload))
}

#[test]
fn test() {
    let a = (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(NewUser {
            email: "".to_string(),
            username: "".to_string(),
        }),
    );
    a.into_response();
}
