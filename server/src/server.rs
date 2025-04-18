mod maze;
use axum::{
    Json, Router,
    body::Body,
    extract::Query,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
};
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

use std::convert::Infallible;
use tower::ServiceExt;
use tower_http::services::{ServeDir, ServeFile};

pub fn app() -> Result<Router> {
    let static_file_service = |req: Request<Body>| async {
        let mut resp = match req.uri().to_string().as_str() {
            s if s.ends_with("css") => ServeDir::new("../web/dist/assets").oneshot(req).await,
            s if s.ends_with("js") => ServeDir::new("../web/dist/assets").oneshot(req).await,
            s if s.ends_with("ttf") => ServeDir::new("../web/dist/assets").oneshot(req).await,
            s if s.ends_with("png") => ServeDir::new("../web/dist/assets").oneshot(req).await,
            _ => ServeFile::new("../web/dist/index.html").oneshot(req).await,
        };

        if resp.as_mut().unwrap().status() == 404 {
            return Ok::<_, Infallible>(
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from("Something went wrong...\n"))
                    .unwrap(),
            );
        }

        Ok::<_, Infallible>(resp.into_response())
    };

    return Ok(Router::new()
        .nest_service("/assets", get(static_file_service))
        .route("/api/maze", get(get_maze).post(create_maze))
        .route(
            "/api/high_score",
            get(get_high_score).post(create_high_score),
        )
        .fallback_service(get(static_file_service)));
}

#[derive(Debug, Deserialize)]
struct MazeParams {
    key: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct MazeTable {
    key: String,
    maze: Vec<u8>,
}

async fn get_maze(param: Query<MazeParams>) -> impl IntoResponse {
    let connection = Connection::open("./sqlite.db3").unwrap();

    let stmt_result = connection.prepare("SELECT * FROM maze WHERE key=?");
    let mut stmt = match stmt_result {
        Ok(r) => r,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Body::from(err.to_string()),
            );
        }
    };
    let maze_ret_result = stmt.query_row(rusqlite::params![param.key], |row| {
        Ok(MazeTable {
            key: row.get(0).unwrap(),
            maze: row.get(1).unwrap(),
        })
    });
    let maze_ret = match maze_ret_result {
        Ok(r) => r,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Body::from(err.to_string()),
            );
        }
    };

    (StatusCode::OK, Body::from(maze_ret.maze))
}

#[derive(Debug, Deserialize)]
struct CreateMaze {
    key: String,
    maze: Vec<u8>,
}

async fn create_maze(Json(input): Json<CreateMaze>) -> impl IntoResponse {
    let connection = Connection::open("./sqlite.db3").unwrap();

    let query_result = connection.execute(
        "INSERT INTO maze (key, maze) VALUES (?1, ?2)",
        rusqlite::params![input.key, input.maze],
    );

    match query_result {
        Ok(_) => (StatusCode::CREATED, Body::empty()),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Body::from(err.to_string()),
        ),
    }
}

#[derive(Debug, Deserialize)]
struct HighScoreSearches {
    search_enum: HighScoreSearchEnum,
    value: String,
}

#[derive(Debug, Deserialize)]
enum HighScoreSearchEnum {
    Key,
    Time,
    User,
}

impl HighScoreSearchEnum {
    fn as_str(&self) -> &'static str {
        match self {
            HighScoreSearchEnum::Key => "key",
            HighScoreSearchEnum::Time => "time",
            HighScoreSearchEnum::User => "user",
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct HighScoreTable {
    key: String,
    user: String,
    time: String,
}

async fn get_high_score(param: Query<HighScoreSearches>) -> impl IntoResponse {
    dbg!("???????????");
    let connection = Connection::open("./sqlite.db3").unwrap();

    let stmt_result = connection.prepare("SELECT * FROM high_score WHERE ?1=?2");
    let mut stmt = match stmt_result {
        Ok(r) => r,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Body::from(err.to_string()),
            );
        }
    };

    let maze_ret_result = stmt.query_row(
        rusqlite::params![param.search_enum.as_str(), param.value],
        |row| {
            Ok(HighScoreTable {
                key: row.get(0).unwrap(),
                user: row.get(1).unwrap(),
                time: row.get(2).unwrap(),
            })
        },
    );

    let maze_ret = match maze_ret_result {
        Ok(r) => r,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Body::from(err.to_string()),
            );
        }
    };

    (StatusCode::OK, Body::from(json!(maze_ret).to_string()))
}

#[derive(Debug, Deserialize)]
struct CreateHighScore {
    key: String,
    time: String,
    user: String,
}

async fn create_high_score(Json(input): Json<CreateHighScore>) -> impl IntoResponse {
    let connection = Connection::open("./sqlite.db3").unwrap();

    let query_result = connection.execute(
        "INSERT INTO high_score (key, time, user) VALUES (?1, ?2, ?3)",
        rusqlite::params![input.key, input.time, input.user],
    );

    match query_result {
        Ok(_) => (StatusCode::CREATED, Body::empty()),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Body::from(err.to_string()),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::app;
    use crate::database;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use rusqlite::Connection;
    use tower::ServiceExt; // for `call`, `oneshot`, and `ready`

    #[tokio::test]
    async fn e2e() {
        database::migrate().unwrap();
        let connection = Connection::open("./sqlite.db3").unwrap();
        connection.execute("DELETE FROM high_score", []).unwrap();
        connection.execute("DELETE FROM maze", []).unwrap();
        connection.close().unwrap();

        let mut app = app();

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = app
            .as_mut()
            .unwrap()
            .oneshot(
                Request::builder()
                    .uri("/api/maze")
                    .method("POST")
                    .header("Content-Type", "application/json")
                    .body(Body::from(r#"{"key": "howdy", "maze": [0,2]}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let response = app
            .as_mut()
            .unwrap()
            .oneshot(
                Request::builder()
                    .uri("/api/maze?key=howdy")
                    .method("GET")
                    .body(Body::default())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();

        // Deserialize into MazeTable
        let maze_data: Vec<u8> = body_bytes.to_vec();

        // Now you can assert on the maze_table fields
        assert_eq!(maze_data, [0, 2]);

        let response = app
            .as_mut()
            .unwrap()
            .oneshot(
                Request::builder()
                    .uri("/api/high_score")
                    .method("POST")
                    .header("Content-Type", "application/json")
                    .body(Body::from(
                        r#"{"key": "howdy", "user": "PYN", "time": "2025-01-01 01:00:00"}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let response = app
            .unwrap()
            .oneshot(
                Request::builder()
                    .uri("/api/high_score?search_enum=key&value=howdy")
                    .method("GET")
                    .body(Body::default())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();

        // Deserialize into MazeTable
        let maze_data: Vec<u8> = body_bytes.to_vec();

        // Now you can assert on the maze_table fields
        assert_eq!(maze_data, [0, 2]);
    }
}
