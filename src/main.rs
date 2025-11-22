mod config;
mod models;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{delete, get, post},
};
use models::{ExportBatch, ImportBatch, Medicine, Pharmacy};
use serde::Deserialize;
use std::{
    fs,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tower_http::services::ServeDir;

const DATA_FILE: &str = "data.json";

type AppState = Arc<Mutex<Pharmacy>>;

#[tokio::main]
async fn main() {
    // Load initial data
    let pharmacy = load_data();
    let state = Arc::new(Mutex::new(pharmacy));

    // Define routes
    let app = Router::new()
        .route("/api/medicines", get(list_medicines).post(add_medicine))
        .route("/api/medicines/{id}", delete(delete_medicine))
        .route("/api/sell", post(sell_medicine))
        .route("/api/batches/import", get(get_import_batches))
        .route("/api/batches/export", get(get_export_batches))
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(index_handler))
        .with_state(state.clone());

    // Address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    // Start server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index_handler() -> impl IntoResponse {
    match fs::read_to_string("assets/index.html") {
        Ok(html) => Html(html).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Index file not found").into_response(),
    }
}

async fn list_medicines(State(state): State<AppState>) -> Json<Vec<Medicine>> {
    let pharmacy = state.lock().unwrap();
    Json(pharmacy.inventory.clone())
}

#[derive(Deserialize)]
struct AddMedicineRequest {
    name: String,
    price: f64,
    quantity: u32,
}

async fn add_medicine(
    State(state): State<AppState>,
    Json(payload): Json<AddMedicineRequest>,
) -> impl IntoResponse {
    let mut pharmacy = state.lock().unwrap();
    pharmacy.add_medicine(payload.name, payload.price, payload.quantity);
    save_data(&pharmacy);
    StatusCode::CREATED
}

async fn delete_medicine(State(state): State<AppState>, Path(id): Path<u32>) -> impl IntoResponse {
    let mut pharmacy = state.lock().unwrap();
    match pharmacy.delete_medicine(id) {
        Ok(_) => {
            save_data(&pharmacy);
            StatusCode::OK
        }
        Err(_) => StatusCode::NOT_FOUND,
    }
}

#[derive(Deserialize)]
struct SellRequest {
    id: u32,
    amount: u32,
}

async fn sell_medicine(
    State(state): State<AppState>,
    Json(payload): Json<SellRequest>,
) -> impl IntoResponse {
    let mut pharmacy = state.lock().unwrap();
    match pharmacy.sell_medicine(payload.id, payload.amount) {
        Ok(_) => {
            save_data(&pharmacy);
            StatusCode::OK.into_response()
        }
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
    }
}

async fn get_import_batches(State(state): State<AppState>) -> Json<Vec<ImportBatch>> {
    let pharmacy = state.lock().unwrap();
    Json(pharmacy.import_log.clone())
}

async fn get_export_batches(State(state): State<AppState>) -> Json<Vec<ExportBatch>> {
    let pharmacy = state.lock().unwrap();
    Json(pharmacy.export_log.clone())
}

fn load_data() -> Pharmacy {
    if let Ok(data) = fs::read_to_string(DATA_FILE) {
        serde_json::from_str(&data).unwrap_or_else(|_| Pharmacy::new())
    } else {
        Pharmacy::new()
    }
}

fn save_data(pharmacy: &Pharmacy) {
    let data = serde_json::to_string_pretty(pharmacy).expect("Failed to serialize data");
    fs::write(DATA_FILE, data).expect("Failed to write data file");
}
