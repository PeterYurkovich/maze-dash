use axum::{
    Json, Router,
    body::Body,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
};

use rand::RngCore;
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use tower::ServiceExt;
use tower_http::services::{ServeDir, ServeFile};

#[derive(Debug, Serialize)]
struct Person {
    id: u64,
    name: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // run our app with hyper, listening globally on port 3001
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app().unwrap()).await.unwrap();
    Ok(())
}

fn app() -> Result<Router> {
    let connection = Connection::open("./sqlite.db3").unwrap();

    connection
        .execute(
            "CREATE TABLE person (
                id    INTEGER PRIMARY KEY,
                name  TEXT NOT NULL
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

        // Ok::<_, Infallible>(resp.into_response().map(|body| BoxBody::new(body)))
        Ok::<_, Infallible>(resp.into_response())
    };

    return Ok(Router::new()
        .nest_service("/assets", get(static_file_service))
        .route("/person", post(create_person))
        .fallback_service(get(static_file_service)));
}

#[derive(Debug, Deserialize)]
struct CreatePerson {
    name: String,
}

async fn create_person(Json(input): Json<CreatePerson>) -> impl IntoResponse {
    let connection = Connection::open("./sqlite.db3").unwrap();
    let mut rng = rand::rng();

    let new_person = Person {
        id: rng.next_u64(),
        name: input.name,
    };

    connection
        .execute(
            "INSERT INTO person (name) VALUES (?1)",
            rusqlite::params![new_person.name],
        )
        .unwrap();

    let mut stmt = connection.prepare("SELECT id, name FROM person").unwrap();
    let person_iter = stmt
        .query_map([], |row| {
            Ok(Person {
                id: row.get(0).unwrap(),
                name: row.get(1).unwrap(),
            })
        })
        .unwrap();

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
    (StatusCode::CREATED, Json(new_person))
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
    async fn hello_person() {
        let app = app();

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = app
            .unwrap()
            .oneshot(
                Request::builder()
                    .uri("/person")
                    .method("POST")
                    .header("Content-Type", "application/json")
                    .body(Body::from(r#"{"name": "mazie"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }
}
