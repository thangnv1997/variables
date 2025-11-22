use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Medicine {
    pub id: u32,
    pub name: String,
    pub price: f64,
    pub quantity: u32,
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
pub struct Pharmacy {
    pub inventory: Vec<Medicine>,
    pub import_log: Vec<ImportBatch>,
    pub export_log: Vec<ExportBatch>,
}

impl Pharmacy {
    pub fn new() -> Self {
        Pharmacy {
            inventory: Vec::new(),
            import_log: Vec::new(),
            export_log: Vec::new(),
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

    pub fn list_medicines(&self) {
        // CLI method, keeping it for compatibility or debugging
        println!(
            "{:<5} | {:<20} | {:<15} | {:<10}",
            "ID", "Name", "Price", "Quantity"
        );
        println!("{}", "-".repeat(60));
        for med in &self.inventory {
            let price_str = format!("{:.2}", med.price).replace('.', ",");
            println!(
                "{:<5} | {:<20} | {:<15} | {:<10}",
                med.id, med.name, price_str, med.quantity
            );
        }
    }
}
