mod maze;
use axum::{
    Json, Router,
    body::Body,
    extract::Query,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
};
use std::convert::Infallible;
use tower::ServiceExt;
use tower_http::services::{ServeDir, ServeFile};

use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

pub fn app() -> Result<Router> {
    let connection = Connection::open("./sqlite.db3").unwrap();
    let _ = maze::get_packed_maze();

    connection
        .execute(
            "CREATE TABLE maze (
                id    INTEGER PRIMARY KEY,
                maze  BLOB
            )",
            (), // empty list of parameters.
        )
        .unwrap_or(1);

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
        .fallback_service(get(static_file_service)));
}

#[derive(Debug, Deserialize)]
struct MazeParams {
    id: u8,
}

#[derive(Debug, Deserialize, Serialize)]
struct MazeTable {
    id: u8,
    maze: Vec<u8>,
}

async fn get_maze(param: Query<MazeParams>) -> impl IntoResponse {
    let connection = Connection::open("./sqlite.db3").unwrap();

    let stmt_result = connection.prepare("SELECT * FROM maze WHERE id=?");
    let mut stmt = match stmt_result {
        Ok(r) => r,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Body::from(err.to_string()),
            );
        }
    };

    let maze_ret_result = stmt.query_row(rusqlite::params![param.id], |row| {
        Ok(MazeTable {
            id: row.get(0).unwrap(),
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
    maze: Vec<u8>,
}

async fn create_maze(Json(input): Json<CreateMaze>) -> impl IntoResponse {
    let connection = Connection::open("./sqlite.db3").unwrap();

    let query_result = connection.execute(
        "INSERT INTO maze (maze) VALUES (?1)",
        rusqlite::params![input.maze],
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
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt; // for `call`, `oneshot`, and `ready`

    #[tokio::test]
    async fn create_maze() {
        let app = app();

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = app
            .unwrap()
            .oneshot(
                Request::builder()
                    .uri("/api/maze")
                    .method("POST")
                    .header("Content-Type", "application/json")
                    .body(Body::from(r#"{"maze": [0,2]}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn get_maze() {
        let app = app();

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = app
            .unwrap()
            .oneshot(
                Request::builder()
                    .uri("/api/maze?id=1")
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

        dbg!(&body_bytes);

        // Deserialize into MazeTable
        let maze_data: Vec<u8> = body_bytes.to_vec();

        // Now you can assert on the maze_table fields
        assert_eq!(maze_data, [0, 1]);
    }
}
