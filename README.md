# Pharmacy Management System

A robust and efficient web-based application for managing pharmacy operations, built with Rust and Axum. This system handles inventory management, sales recording, warehouse organization, stock batch tracking, and supplier management.

## Features

- **Medicine Management**: Add, update, delete, and list medicines in the inventory.
- **Sales Processing**: Record sales of medicines and track revenue.
- **Warehouse Management**: Create and manage multiple warehouses with different types (Store, Counter, Cold Storage).
- **Stock Batch Tracking**:
  - Import batches of medicines into specific warehouses.
  - Track expiry dates (ISO 8601 format).
  - Monitoring expiring batches (90 days lookahead).
  - Transfer stock between warehouses.
- **Supplier Management**: Maintain a database of suppliers with contact details.
- **Data Persistence**: All data is automatically saved to and loaded from a local JSON file (`data.json`).

## Tech Stack

- **Backend**: Rust
- **Web Framework**: Axum
- **Asynchronous Runtime**: Tokio
- **Serialization**: Serde & Serde JSON
- **Date/Time Handling**: Chrono
- **Frontend**: Vanilla HTML5, CSS3, and JavaScript (served statically)

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- Cargo (comes with Rust)

## Installation & Running

1. **Clone the repository** (if applicable):
   ```bash
   git clone <repository-url>
   cd variables
   ```

2. **Run the application**:
   ```bash
   cargo run
   ```

   The server will start listening on `http://127.0.0.1:3000`.

3. **Access the Application**:
   Open your web browser and navigate to:
   [http://localhost:3000](http://localhost:3000)

## Project Structure

- `src/main.rs`: Entry point of the application, server setup, and API route handlers.
- `src/models.rs`: Data structures for Pharmacy, Medicine, Warehouse, StockBatch, Supplier, etc.
- `src/config.rs`: Configuration modules (if any).
- `assets/`: Contains the frontend static files (`index.html`, `style.css`, `script.js`).
- `data.json`: Stores the persistent application data (created automatically on first run/write).

## API Endpoints

### Medicines
- `GET /api/medicines`: List all medicines.
- `POST /api/medicines`: Add a new medicine.
- `DELETE /api/medicines/{id}`: Delete a medicine.
- `POST /api/sell`: Process a sale.

### Warehouses
- `GET /api/warehouses`: List all warehouses.
- `POST /api/warehouses`: Create a new warehouse.
- `PUT /api/warehouses/{id}`: Edit a warehouse.

### Stock & Batches
- `GET /api/stock-batches`: List specific stock batches.
- `POST /api/import-batch`: Import a new batch of medicine.
- `POST /api/transfer-batch`: Transfer stock between warehouses.
- `GET /api/expiring-batches`: Get batches expiring soon.
- `GET /api/batches/import`: Get log of import actions.
- `GET /api/batches/export`: Get log of export actions.
- `GET /api/transfers`: Get log of internal transfers.

### Suppliers
- `GET /api/suppliers`: List all suppliers.
- `POST /api/suppliers`: Create a new supplier.
- `PUT /api/suppliers/{id}`: Edit a supplier details.
