use axum::{
    routing::{ get, post },
    extract::{ State, Path },
    response::{ IntoResponse, Response },
    http::{ StatusCode },
    Json,
    Router
};

use tower_http::{
    services::ServeDir,
    cors::{
        Any,
        CorsLayer
    }
};

use sqlx::{
    Row,
    sqlite::{ SqlitePool, SqliteRow }
};


use serde_derive::{ Serialize, Deserialize };


#[derive(Debug, Serialize, Deserialize)]
struct Comment {
    page : String,
    username : String,
    content : String
}


struct AppError(anyhow::Error);


impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("error: {}", self.0)).into_response()
    }
}


impl<T> From<T> for AppError where anyhow::Error : From<T> {
    fn from(item : T) -> Self {
        Self(item.into())
    }
}


#[axum::debug_handler]
async fn get_comments(State(state) : State<AppState>, Path(page) : Path<String>) -> Result<Json<Vec<Comment>>, AppError> {
    let comments : Vec<anyhow::Result<Comment>> = sqlx::query("SELECT * FROM comments WHERE page_url=? ORDER BY id DESC").bind(page).map(|row : SqliteRow| {
        Ok(Comment {
            page : row.get(0),
            username : row.get(1),
            content : row.get(2)
        })
    }).fetch_all(&state.database).await?;
    let comments : Result<Vec<Comment>, _> = comments.into_iter().collect();
    let comments = comments?;
    Ok(Json(comments))
}

async fn post_comment(State(state) : State<AppState>, Json(comment) : Json<Comment>) -> Result<(), AppError> {
    sqlx::query("INSERT INTO comments(page_url, username, content) VALUES(?, ?, ?)").bind(comment.page).bind(comment.username).bind(comment.content)
        .execute(&state.database).await?;
    Ok(())
}


#[derive(Clone)]
struct AppState {
    database : SqlitePool
}


impl AppState {
    async fn setup() -> anyhow::Result<AppState> {
        Ok(AppState {
            database : SqlitePool::connect(&std::env::var("DATABASE_PATH")?).await?
        })
    }
}


#[tokio::main]
async fn main() {
    let cors_layer = CorsLayer::new().allow_methods(Any).allow_origin(Any).allow_headers(Any);
    let app = Router::new().route("/comments/{path}", get(get_comments))
                           .route("/post/", post(post_comment))
                           .layer(cors_layer)
                           .with_state(AppState::setup().await.unwrap())
                           .fallback_service(ServeDir::new("static"));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
