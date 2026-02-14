use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Medicine {
    pub id: u32,
    pub name: String,
    pub price: f64,
    pub quantity: u32, // This will be deprecated, kept for backward compatibility
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WarehouseType {
    Main,  // Kho chính
    Store, // Kho nhà thuốc
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Warehouse {
    pub id: u32,
    pub name: String,
    pub warehouse_type: WarehouseType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StockBatch {
    pub id: u32,
    pub medicine_id: u32,
    pub medicine_name: String,
    pub warehouse_id: u32,
    pub quantity: u32,
    pub price: f64,
    pub expiry_date: DateTime<Local>,
    pub import_date: DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImportBatch {
    pub id: u32,
    pub medicine_id: u32,
    pub medicine_name: String,
    pub quantity: u32,
    pub price: f64,
    pub timestamp: DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExportBatch {
    pub id: u32,
    pub medicine_id: u32,
    pub medicine_name: String,
    pub amount: u32,
    pub price: f64,
    pub timestamp: DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InternalTransfer {
    pub id: u32,
    pub medicine_id: u32,
    pub medicine_name: String,
    pub from_warehouse_id: u32,
    pub to_warehouse_id: u32,
    pub quantity: u32,
    pub batch_id: u32,
    pub timestamp: DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Supplier {
    pub id: u32,
    pub name: String,
    pub contact: String,
    pub phone: String,
    pub address: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pharmacy {
    pub inventory: Vec<Medicine>, // Deprecated, kept for compatibility
    pub warehouses: Vec<Warehouse>,
    pub stock_batches: Vec<StockBatch>,
    pub import_log: Vec<ImportBatch>,
    pub export_log: Vec<ExportBatch>,
    pub transfer_log: Vec<InternalTransfer>,
    pub suppliers: Vec<Supplier>,
}

impl Pharmacy {
    pub fn new() -> Self {
        Pharmacy {
            inventory: Vec::new(),
            warehouses: Vec::new(),
            stock_batches: Vec::new(),
            import_log: Vec::new(),
            export_log: Vec::new(),
            transfer_log: Vec::new(),
            suppliers: Vec::new(),
        }
    }

    pub fn add_medicine(&mut self, name: String, price: f64, quantity: u32) {
        // Check if medicine exists (simple check by name for now, or just create new ID)
        // For this task, we assume adding new medicine creates a new entry.
        // In "Advanced Inventory", we will handle deduplication.

        let id = if let Some(last) = self.inventory.last() {
            last.id + 1
        } else {
            1
        };

        let medicine = Medicine {
            id,
            name: name.clone(),
            price,
            quantity,
        };
        self.inventory.push(medicine);

        // Record Import Batch
        let batch_id = self.import_log.len() as u32 + 1;
        let batch = ImportBatch {
            id: batch_id,
            medicine_id: id,
            medicine_name: name,
            quantity,
            price,
            timestamp: Local::now(),
        };
        self.import_log.push(batch);
    }

    pub fn sell_medicine(&mut self, id: u32, amount: u32) -> Result<(), String> {
        if let Some(med) = self.inventory.iter_mut().find(|m| m.id == id) {
            if med.quantity >= amount {
                med.quantity -= amount;

                // Record Export Batch
                let batch_id = self.export_log.len() as u32 + 1;
                let batch = ExportBatch {
                    id: batch_id,
                    medicine_id: id,
                    medicine_name: med.name.clone(),
                    amount,
                    price: med.price,
                    timestamp: Local::now(),
                };
                self.export_log.push(batch);

                Ok(())
            } else {
                Err(format!("Not enough quantity. Available: {}", med.quantity))
            }
        } else {
            Err("Medicine not found".to_string())
        }
    }

    pub fn delete_medicine(&mut self, id: u32) -> Result<(), String> {
        if let Some(pos) = self.inventory.iter().position(|m| m.id == id) {
            self.inventory.remove(pos);
            Ok(())
        } else {
            Err("Medicine not found".to_string())
        }
    }

    // Warehouse Management Methods

    pub fn edit_warehouse(
        &mut self,
        id: u32,
        name: String,
        warehouse_type: WarehouseType,
    ) -> Result<(), String> {
        if let Some(wh) = self.warehouses.iter_mut().find(|w| w.id == id) {
            wh.name = name;
            wh.warehouse_type = warehouse_type;
            Ok(())
        } else {
            Err("Warehouse not found".to_string())
        }
    }

    pub fn add_supplier(
        &mut self,
        name: String,
        contact: String,
        phone: String,
        address: String,
    ) -> u32 {
        let id = if let Some(last) = self.suppliers.last() {
            last.id + 1
        } else {
            1
        };
        let supplier = Supplier {
            id,
            name,
            contact,
            phone,
            address,
        };
        self.suppliers.push(supplier);
        id
    }

    pub fn edit_supplier(
        &mut self,
        id: u32,
        name: String,
        contact: String,
        phone: String,
        address: String,
    ) -> Result<(), String> {
        if let Some(supplier) = self.suppliers.iter_mut().find(|s| s.id == id) {
            supplier.name = name;
            supplier.contact = contact;
            supplier.phone = phone;
            supplier.address = address;
            Ok(())
        } else {
            Err("Supplier not found".to_string())
        }
    }

    // Restore add_warehouse method
    pub fn add_warehouse(&mut self, name: String, warehouse_type: WarehouseType) -> u32 {
        let id = if let Some(last) = self.warehouses.last() {
            last.id + 1
        } else {
            1
        };
        let warehouse = Warehouse {
            id,
            name,
            warehouse_type,
        };
        self.warehouses.push(warehouse);
        id
    }

    pub fn import_batch(
        &mut self,
        medicine_id: u32,
        medicine_name: String,
        warehouse_id: u32,
        quantity: u32,
        price: f64,
        expiry_date: DateTime<Local>,
    ) -> Result<u32, String> {
        // Verify warehouse exists
        if !self.warehouses.iter().any(|w| w.id == warehouse_id) {
            return Err("Warehouse not found".to_string());
        }

        let batch_id = if let Some(last) = self.stock_batches.last() {
            last.id + 1
        } else {
            1
        };

        let batch = StockBatch {
            id: batch_id,
            medicine_id,
            medicine_name: medicine_name.clone(),
            warehouse_id,
            quantity,
            price,
            expiry_date,
            import_date: Local::now(),
        };
        self.stock_batches.push(batch);

        // Log import
        let log_id = self.import_log.len() as u32 + 1;
        let import_log = ImportBatch {
            id: log_id,
            medicine_id,
            medicine_name,
            quantity,
            price,
            timestamp: Local::now(),
        };
        self.import_log.push(import_log);

        Ok(batch_id)
    }

    pub fn transfer_batch(
        &mut self,
        batch_id: u32,
        to_warehouse_id: u32,
        quantity: u32,
    ) -> Result<(), String> {
        // Find the source batch
        let source_batch = self
            .stock_batches
            .iter_mut()
            .find(|b| b.id == batch_id)
            .ok_or("Batch not found")?;

        if source_batch.quantity < quantity {
            return Err(format!(
                "Insufficient quantity. Available: {}",
                source_batch.quantity
            ));
        }

        let from_warehouse_id = source_batch.warehouse_id;
        let medicine_id = source_batch.medicine_id;
        let medicine_name = source_batch.medicine_name.clone();
        let price = source_batch.price;
        let expiry_date = source_batch.expiry_date;

        // Decrease source batch quantity
        source_batch.quantity -= quantity;

        // Create new batch in destination warehouse
        let new_batch_id = if let Some(last) = self.stock_batches.last() {
            last.id + 1
        } else {
            1
        };

        let new_batch = StockBatch {
            id: new_batch_id,
            medicine_id,
            medicine_name: medicine_name.clone(),
            warehouse_id: to_warehouse_id,
            quantity,
            price,
            expiry_date,
            import_date: Local::now(),
        };
        self.stock_batches.push(new_batch);

        // Log transfer
        let transfer_id = self.transfer_log.len() as u32 + 1;
        let transfer = InternalTransfer {
            id: transfer_id,
            medicine_id,
            medicine_name,
            from_warehouse_id,
            to_warehouse_id,
            quantity,
            batch_id,
            timestamp: Local::now(),
        };
        self.transfer_log.push(transfer);

        Ok(())
    }

    #[allow(dead_code)]
    pub fn sell_with_fefo(&mut self, medicine_id: u32, quantity: u32) -> Result<(), String> {
        // Find store warehouse
        let store_warehouse = self
            .warehouses
            .iter()
            .find(|w| matches!(w.warehouse_type, WarehouseType::Store))
            .ok_or("Store warehouse not found")?;

        // Get all batches for this medicine in store, sorted by expiry date (FEFO)
        let mut available_batches: Vec<_> = self
            .stock_batches
            .iter_mut()
            .filter(|b| {
                b.medicine_id == medicine_id
                    && b.warehouse_id == store_warehouse.id
                    && b.quantity > 0
            })
            .collect();

        available_batches.sort_by_key(|b| b.expiry_date);

        let mut remaining = quantity;
        let mut medicine_name = String::new();
        let mut total_price = 0.0;

        for batch in available_batches {
            if remaining == 0 {
                break;
            }

            if medicine_name.is_empty() {
                medicine_name = batch.medicine_name.clone();
            }

            let to_sell = remaining.min(batch.quantity);
            batch.quantity -= to_sell;
            remaining -= to_sell;
            total_price += batch.price * to_sell as f64;
        }

        if remaining > 0 {
            return Err(format!("Insufficient stock. Short by {} units", remaining));
        }

        // Log export
        let export_id = self.export_log.len() as u32 + 1;
        let export = ExportBatch {
            id: export_id,
            medicine_id,
            medicine_name,
            amount: quantity,
            price: total_price / quantity as f64, // Average price
            timestamp: Local::now(),
        };
        self.export_log.push(export);

        Ok(())
    }

    pub fn get_expiring_batches(&self, days: i64) -> Vec<StockBatch> {
        use chrono::Duration;
        let threshold = Local::now() + Duration::days(days);

        self.stock_batches
            .iter()
            .filter(|b| b.expiry_date <= threshold && b.quantity > 0)
            .cloned()
            .collect()
    }

    #[allow(dead_code)]
    pub fn get_stock_by_warehouse(&self, warehouse_id: u32) -> Vec<StockBatch> {
        self.stock_batches
            .iter()
            .filter(|b| b.warehouse_id == warehouse_id && b.quantity > 0)
            .cloned()
            .collect()
    }
}
