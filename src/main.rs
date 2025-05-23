// main.rs
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use std::env;

mod wasm_handler; // We'll create this module next

#[derive(Debug, Serialize, Deserialize)]
struct PharmacyData {
    name: String,
    pharmacy: String,
    region: String,
    month: String,
    unit_sold: i32,
    unit: String,
    method: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct FilterRequest {
    search: Option<String>,
    pharmacy: Option<String>,
    region: Option<String>,
    month: Option<String>,
    method: Option<String>,
    unit: Option<String>,
}

async fn filter_data(filters: web::Json<FilterRequest>) -> impl Responder {
    let handler = wasm_handler::get_wasm_handler();
    match handler.filter_data(&serde_json::to_string(&filters).unwrap()) {
        Ok(filtered_data) => HttpResponse::Ok().json(filtered_data),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn calculate_stats(data: web::Json<Vec<PharmacyData>>) -> impl Responder {
    let handler = wasm_handler::get_wasm_handler();
    match handler.calculate_stats(&serde_json::to_string(&data).unwrap()) {
        Ok(stats) => HttpResponse::Ok().json(stats),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port = port.parse::<u16>().unwrap();

    println!("Starting server on port {}", port);

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                    .route("/filter", web::post().to(filter_data))
                    .route("/stats", web::post().to(calculate_stats))
            )
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
