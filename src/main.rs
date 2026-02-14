mod models;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{delete, get, post, put},
};
use chrono::{DateTime, Local};
use models::{
    ExportBatch, ImportBatch, InternalTransfer, Medicine, Pharmacy, StockBatch, Supplier,
    Warehouse, WarehouseType,
};
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
        // Warehouse routes
        .route(
            "/api/warehouses",
            get(list_warehouses).post(create_warehouse),
        )
        .route("/api/warehouses/{id}", put(edit_warehouse))
        .route("/api/stock-batches", get(list_stock_batches))
        .route("/api/import-batch", post(import_batch_handler))
        .route("/api/transfer-batch", post(transfer_batch_handler))
        .route("/api/expiring-batches", get(get_expiring_batches))
        .route("/api/transfers", get(get_transfers))
        // Supplier routes
        .route("/api/suppliers", get(list_suppliers).post(create_supplier))
        .route("/api/suppliers/{id}", put(edit_supplier))
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

// Warehouse handlers

async fn list_warehouses(State(state): State<AppState>) -> Json<Vec<Warehouse>> {
    let pharmacy = state.lock().unwrap();
    Json(pharmacy.warehouses.clone())
}

#[derive(Deserialize)]
struct CreateWarehouseRequest {
    name: String,
    warehouse_type: WarehouseType,
}

#[derive(Deserialize)]
struct EditWarehouseRequest {
    name: String,
    warehouse_type: WarehouseType,
}

async fn create_warehouse(
    State(state): State<AppState>,
    Json(payload): Json<CreateWarehouseRequest>,
) -> impl IntoResponse {
    let mut pharmacy = state.lock().unwrap();
    let id = pharmacy.add_warehouse(payload.name, payload.warehouse_type);
    save_data(&pharmacy);
    (StatusCode::CREATED, Json(id))
}

async fn edit_warehouse(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    Json(payload): Json<EditWarehouseRequest>,
) -> impl IntoResponse {
    let mut pharmacy = state.lock().unwrap();
    match pharmacy.edit_warehouse(id, payload.name, payload.warehouse_type) {
        Ok(_) => {
            save_data(&pharmacy);
            StatusCode::OK.into_response()
        }
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
    }
}

async fn list_stock_batches(State(state): State<AppState>) -> Json<Vec<StockBatch>> {
    let pharmacy = state.lock().unwrap();
    Json(pharmacy.stock_batches.clone())
}

#[derive(Deserialize)]
struct ImportBatchRequest {
    medicine_id: u32,
    medicine_name: String,
    warehouse_id: u32,
    quantity: u32,
    price: f64,
    expiry_date: String, // ISO 8601 format
}

async fn import_batch_handler(
    State(state): State<AppState>,
    Json(payload): Json<ImportBatchRequest>,
) -> impl IntoResponse {
    let mut pharmacy = state.lock().unwrap();

    // Parse expiry date
    let expiry_date = match DateTime::parse_from_rfc3339(&payload.expiry_date) {
        Ok(dt) => dt.with_timezone(&Local),
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid date format").into_response(),
    };

    match pharmacy.import_batch(
        payload.medicine_id,
        payload.medicine_name,
        payload.warehouse_id,
        payload.quantity,
        payload.price,
        expiry_date,
    ) {
        Ok(batch_id) => {
            save_data(&pharmacy);
            (StatusCode::CREATED, Json(batch_id)).into_response()
        }
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
    }
}

#[derive(Deserialize)]
struct TransferBatchRequest {
    batch_id: u32,
    to_warehouse_id: u32,
    quantity: u32,
}

async fn transfer_batch_handler(
    State(state): State<AppState>,
    Json(payload): Json<TransferBatchRequest>,
) -> impl IntoResponse {
    let mut pharmacy = state.lock().unwrap();

    match pharmacy.transfer_batch(payload.batch_id, payload.to_warehouse_id, payload.quantity) {
        Ok(_) => {
            save_data(&pharmacy);
            StatusCode::OK.into_response()
        }
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
    }
}

async fn get_expiring_batches(State(state): State<AppState>) -> Json<Vec<StockBatch>> {
    let pharmacy = state.lock().unwrap();
    Json(pharmacy.get_expiring_batches(90))
}

async fn get_transfers(State(state): State<AppState>) -> Json<Vec<InternalTransfer>> {
    let pharmacy = state.lock().unwrap();
    Json(pharmacy.transfer_log.clone())
}

// Supplier handlers

async fn list_suppliers(State(state): State<AppState>) -> Json<Vec<Supplier>> {
    let pharmacy = state.lock().unwrap();
    Json(pharmacy.suppliers.clone())
}

#[derive(Deserialize)]
struct CreateSupplierRequest {
    name: String,
    contact: String,
    phone: String,
    address: String,
}

#[derive(Deserialize)]
struct EditSupplierRequest {
    name: String,
    contact: String,
    phone: String,
    address: String,
}

async fn create_supplier(
    State(state): State<AppState>,
    Json(payload): Json<CreateSupplierRequest>,
) -> impl IntoResponse {
    let mut pharmacy = state.lock().unwrap();
    let id = pharmacy.add_supplier(
        payload.name,
        payload.contact,
        payload.phone,
        payload.address,
    );
    save_data(&pharmacy);
    (StatusCode::CREATED, Json(id))
}

async fn edit_supplier(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    Json(payload): Json<EditSupplierRequest>,
) -> impl IntoResponse {
    let mut pharmacy = state.lock().unwrap();
    match pharmacy.edit_supplier(
        id,
        payload.name,
        payload.contact,
        payload.phone,
        payload.address,
    ) {
        Ok(_) => {
            save_data(&pharmacy);
            StatusCode::OK.into_response()
        }
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
    }
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
