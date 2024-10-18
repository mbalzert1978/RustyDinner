mod error;
mod prelude;

use axum::extract::Path;
use axum::response::Json;
use axum::routing::get;
use axum::Router;
pub use prelude::*;
use serde::Serialize;

#[derive(Serialize)]
struct Weather {
    city: String,
    temperature: f32,
    condition: String,
}

async fn get_weather(Path(city): Path<String>) -> Json<Weather> {
    let weather = Weather {
        city,
        temperature: 22.0,
        condition: "Sunny".to_string(),
    };

    Json(weather)
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new().route("/weather/:city", get(get_weather));

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", "localhost", "8000"))
        .await
        .expect("Could not bind to address");

    axum::serve(listener, app)
        .await
        .expect("Could not start server");

    Ok(())
}
