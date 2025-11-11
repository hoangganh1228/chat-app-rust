mod database;
mod entities;
mod dtos;
mod handlers;
mod repositories;
mod response;
mod routes;
mod security;
mod services;

use std::time::Duration;
use axum::Router;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".into())
        .parse::<u16>()
        .unwrap_or(3000);

        let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "your-secret-key-change-in-production".into());
    
    let jwt_expiration_minutes = std::env::var("JWT_EXPIRATION_MINUTES")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1440); // Default 24 hours
    
    let jwt_expiration = Duration::from_secs(jwt_expiration_minutes * 60);

    let state = database::init_app_state(jwt_secret, jwt_expiration).await?;

    let app = Router::new()
        .merge(routes::build())
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = format!("localhost:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    println!("🚀 Server is running on http://{}", addr);
    
    axum::serve(listener, app).await?;

    Ok(())
}

